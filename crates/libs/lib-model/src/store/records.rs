use crate::ModelManager;
use crate::store::records::types::{RawRecord, Record, Records};
use crate::{Error, Result};

pub(crate) mod types;

//```sql
// CREATE TABLE records (
// id               INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
// item_id          INTEGER REFERENCES items (id) ON UPDATE CASCADE,
// date             INTEGER NOT NULL UNIQUE,
// transaction_type INTEGER NOT NULL CHECK (transaction_type IN (0, 1) ),
// quantity         INTEGER NOT NULL,
// total            INTEGER NOT NULL
// );
//```
pub(crate) struct RecordsBmc;

impl RecordsBmc {
    pub async fn create(
        mm: &ModelManager,
        item_id: u32,
        date_create: i64,
        transaction_type: bool,
        quantity: u32,
        correction: bool,
    ) -> Result<u32> {
        let db = mm.db();
        // Calculate the total based on the previous record
        let last_total = Self::get_last_total(mm, item_id).await?;

        let total = do_arithmetic(transaction_type, last_total, quantity);
        // Implementation for creating a record
        let id = sqlx::query("INSERT into records (item_id, date, transaction_type, quantity, total, correction) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(item_id)
            .bind(date_create)
            .bind(transaction_type)
            .bind(quantity)
            .bind(total)
            .bind(correction)
            .execute(db)
            .await
            .map_err(|err| match err {
                sqlx::error::Error::RowNotFound => {
                    Error::QueryError(format!("Failed to create record for item_id: {}", item_id))
                }
                _ => err.into(),
            })?.last_insert_rowid();

        Ok(id.try_into()?)
    }

    pub async fn get(mm: &ModelManager, record_id: u32) -> Result<Record> {
        // Implementation for retrieving records by item_id
        let db = mm.db();

        let record = sqlx::query_as::<_, RawRecord>(
            "SELECT id, date, transaction_type, quantity, correction FROM records WHERE id = $1",
        )
        .bind(record_id)
        .fetch_one(db)
        .await?;

        Ok(record.into())
    }

    pub async fn get_all(mm: &ModelManager, item_id: u32) -> Result<Records> {
        let db = mm.db();

        let records = sqlx::query_as::<_, RawRecord>
            ("SELECT id, date, transaction_type, quantity, correction FROM records WHERE item_id = $1 ORDER BY date DESC")
            .bind(item_id)
            .fetch_all(db)
            .await?;

        let records: Vec<Record> = records.into_iter().map(|rec| rec.into()).collect();

        Ok(Records::new(item_id, records))
    }

    pub async fn get_in_timeframe(
        mm: &ModelManager,
        item_id: u32,
        start: i64,
        end: i64,
    ) -> Result<Records> {
        let db = mm.db();

        let records = sqlx::query_as::<_, RawRecord>(
            "SELECT id, date, transaction_type, quantity, correction FROM records WHERE item_id = $1 AND date BETWEEN $2 AND $3 ORDER BY date DESC",
        ).bind(item_id)
            .bind(start)
            .bind(end)
            .fetch_all(db)
            .await?;

        let records: Vec<Record> = records.into_iter().map(|rec| rec.into()).collect();

        Ok(Records::new(item_id, records))
    }

    pub async fn get_last_total(mm: &ModelManager, item_id: u32) -> Result<u32> {
        let db = mm.db();

        let result = sqlx::query_as::<_, (u32,)>(
            "SELECT total FROM records WHERE item_id = $1 ORDER BY date DESC LIMIT 1",
        )
        .bind(item_id)
        .fetch_optional(db)
        .await?;

        Ok(result.map(|r| r.0).unwrap_or(0))
    }

    pub async fn get_last(mm: &ModelManager, item_id: u32) -> Result<Option<Record>> {
        // Implementation for retrieving the last record
        let db = mm.db();

        let result = sqlx::query_as::<_, RawRecord>(
            "SELECT id, date, transaction_type, quantity, total, correction  FROM records WHERE item_id = $1 ORDER BY date DESC LIMIT 1",
        )
            .bind(item_id)
            .fetch_optional(db)
            .await?;

        Ok(result.map(|r| r.into()))
    }

    pub async fn update(mm : &ModelManager, item_id : u32, record_id : u32,quantity : u32)-> Result<()> {
        let db = mm.db();
        // Implementation for updating a record
        // check if the record to edit is the last one
        // we only allow to edit the last record as it will affect the total if not
        let last_record = Self::get_last(mm, item_id).await?;

        let Some(last_record) = last_record else {
            // FIX : More appropriate error
            return Err(Error::RecordUpdateForbidden(format!(
                "Record data not existent for item_id {}",
                item_id
            )));
        };

        // Check if the record_id matches the last record
        if !last_record.id().eq(&record_id) {
            return Err(Error::RecordUpdateForbidden(format!(
                "Record with id {} is not the last record for item_id {}",
                record_id, item_id
            )));
        }

        let last_total = Self::get_last_total(mm, item_id).await?;

        // Get the second last record total
        let previous_total = do_arithmetic(
            last_record.transaction_type().opposite().into(),
            last_total,
            last_record.quantity(),
        );

        let new_total = do_arithmetic(
            last_record.transaction_type().into(),
            previous_total,
            quantity,
        );

        sqlx::query("UPDATE records SET quantity = $1, total = $2 WHERE id = $3")
            .bind(quantity)
            .bind(new_total)
            .bind(record_id)
            .execute(db)
            .await
            .map_err(|err| match err {
                sqlx::error::Error::RowNotFound => {
                    Error::QueryError(format!("Failed to update record with id: {}", record_id))
                }
                _ => err.into(),
            })?;

        Ok(())


    }

    pub async fn delete(mm: &ModelManager, record_id: u32, item_id: u32) -> Result<()> {
        let db = mm.db();
        // Check whether the record is the last one
        let last_record = Self::get_last(mm, item_id).await?;

        let Some(last_record) = last_record else {
            // FIX : More appropriate error
            return Err(Error::QueryError(format!(
                "Record with id {} not found",
                record_id
            )));
        };

        // If the record is the last one, we can delete it
        if last_record.id().eq(&record_id) {
            sqlx::query("DELETE FROM records WHERE id = $1")
                .bind(record_id)
                .execute(db)
                .await
                .map_err(|err| match err {
                    sqlx::error::Error::RowNotFound => {
                        Error::QueryError(format!("Failed to delete record with id: {}", record_id))
                    }
                    _ => err.into(),
                })?;
        // else, we need to create a new record with the correction flag on, to correct the total
        } else {
            // Get the target record, if it is not the last one
            let target_record = Self::get(mm, record_id).await?;

            // Create a new correction record
            let time = chrono::Utc::now().timestamp();
            let transaction_type = target_record.transaction_type().opposite().into();
            let _ = RecordsBmc::create(
                mm,
                item_id,
                time,
                transaction_type,
                target_record.quantity(),
                true,
            )
            .await?;
        }
        Ok(())
    }
}

fn do_arithmetic(transaction_type: bool, last_total: u32, quantity: u32) -> u32 {
    match transaction_type {
        false => last_total + quantity,
        true => last_total - quantity,
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils::get_dev_env;
    use crate::store::records::RecordsBmc;
    use serial_test::serial;
    use std::time::Duration;
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
    use tokio::time::sleep;
    use crate::Error;

    #[tokio::test]
    #[serial]
    async fn test_get_last_total() {
        // Test for getting the last total
        let mm = get_dev_env().await.unwrap();

        let result = RecordsBmc::get_last_total(&mm, 1).await.unwrap();

        assert_eq!(result, 23);

        // Test for getting the last total with no records
        let result = RecordsBmc::get_last_total(&mm, 9999).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_last() {
        let mm = get_dev_env().await.unwrap();

        // Test for getting the last record
        let last_record = RecordsBmc::get_last(&mm, 1).await.unwrap();

        assert!(last_record.is_some());
        assert_eq!(last_record.unwrap().id(), 4);
        assert_eq!(last_record.unwrap().date().timestamp(), 1685577600);
        assert_eq!(last_record.unwrap().quantity(), 10);
        assert!(!last_record.unwrap().correction());

        // Test for getting the last record with no records
        let last_record = RecordsBmc::get_last(&mm, 9999).await.unwrap();
        assert!(last_record.is_none());
    }

    #[tokio::test]
    #[serial]
    async fn test_get(){
        let mm = get_dev_env().await.unwrap();

        let record_id = 2;

        let record = RecordsBmc::get(&mm, record_id).await.unwrap();

        assert_eq!(record.id(), record_id);
        assert_eq!(record.date().timestamp(), 1675123200);
        assert_eq!(record.quantity(), 3);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_all(){
        let mm = get_dev_env().await.unwrap();

        let item_id = 2;

        let records = RecordsBmc::get_all(&mm, item_id).await.unwrap();

        assert_eq!(records.item_id(), item_id);
        assert_eq!(records.records().len(),9);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_in_timeframe() {
        let mm = get_dev_env().await.unwrap();

        let start_time = Utc.with_ymd_and_hms(2024,12,1,0,0,0).unwrap();
        let end_time = Utc.with_ymd_and_hms(2024,12,31,23,59,59).unwrap();

        let records = RecordsBmc::get_in_timeframe(&mm, 2, start_time.timestamp(), end_time.timestamp()).await.unwrap();

        assert_eq!(records.records().len(),2);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_record() {
        let mm = get_dev_env().await.unwrap();

        let item_id = 1;

        // Create first record
        let time = Utc::now().timestamp();
        let _id = RecordsBmc::create(&mm, item_id, time, false, 10, false)
            .await
            .unwrap();

        let current_total = RecordsBmc::get_last_total(&mm, item_id).await.unwrap();
        assert_eq!(current_total, 33);

        sleep(Duration::from_secs(1)).await;

        // Create second record
        let time = Utc::now().timestamp();
        let _id = RecordsBmc::create(&mm, item_id, time, true, 3, false)
            .await
            .unwrap();

        let current_total = RecordsBmc::get_last_total(&mm, item_id).await.unwrap();

        assert_eq!(current_total, 30);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_record() {
        let mm = get_dev_env().await.unwrap();

        // Update record where it is not the last one
        let result = RecordsBmc::update(&mm,1,2,10).await;

        assert!(matches!(result, Err(Error::RecordUpdateForbidden(_))));

        // Update the last record
        RecordsBmc::update(&mm,1,4,5).await.unwrap();

        let last_total = RecordsBmc::get_last_total(&mm, 1).await.unwrap();

        assert_eq!(last_total, 28);

        RecordsBmc::update(&mm,2,14,100).await.unwrap();

        let last_total = RecordsBmc::get_last_total(&mm, 2).await.unwrap();

        assert_eq!(last_total, 173);

    }

    #[tokio::test]
    #[serial]
    async fn test_delete_record() {
        let mm = get_dev_env().await.unwrap();

        let item_id = 1;

        // Get the last record before deletion
        let last_record = RecordsBmc::get_last(&mm, item_id).await.unwrap();
        assert!(last_record.is_some());

        // Delete the record
        RecordsBmc::delete(&mm, last_record.unwrap().id(), item_id)
            .await
            .unwrap();

        // Check that the record is deleted
        let last_record_after_delete = RecordsBmc::get_last(&mm, item_id).await.unwrap();

        assert_ne!(
            last_record_after_delete.unwrap().id(),
            last_record.unwrap().id()
        );

        let target_record = RecordsBmc::get(&mm, 2).await.unwrap();

        let target_quantity = target_record.quantity();

        // Check delete record in the middle it will insert a correction record at the end
        RecordsBmc::delete(&mm, 2, item_id).await.unwrap();

        // Check that the last record is now the correction record
        let last_record_after_correction = RecordsBmc::get_last(&mm, item_id).await.unwrap();

        assert!(last_record_after_correction.is_some());
        assert!(last_record_after_correction.unwrap().correction());
        assert_eq!(last_record_after_correction.unwrap().quantity(), target_quantity);
        assert_eq!(last_record_after_correction.unwrap().transaction_type(), target_record.transaction_type().opposite());
    }
}
