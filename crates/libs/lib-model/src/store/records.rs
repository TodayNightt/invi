use crate::store::records::types::{ItemRecord, ItemRecords, Record, Records, TransactionType};
use crate::ModelManager;
use crate::{Error, Result};
use chrono::Utc;

pub(crate) mod types;

//```sql
// CREATE TABLE records (
// id                 INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
// item_id            INTEGER REFERENCES items (id) ON UPDATE CASCADE,
// date               INTEGER NOT NULL UNIQUE,
// transaction_type   INTEGER NOT NULL CHECK (transaction_type IN (0, 1) ),
// quantity           INTEGER NOT NULL,
// total              INTEGER NOT NULL,
// adjustment_remarks INTEGER
// );
//```
pub(crate) struct RecordsBmc;

impl RecordsBmc {
    pub async fn create(
        mm: &ModelManager,
        item_id: u32,
        date_create: i64,
        transaction_type: TransactionType,
        quantity: u32,
        adjustment_remarks: Option<u32>,
    ) -> Result<u32> {
        let db = mm.db();
        // Calculate the total based on the previous record
        let last_total = Self::get_last_total(mm, item_id).await?;

        let total = transaction_type.do_arithmetic(last_total, quantity);

        if transaction_type.is_adjustment() && adjustment_remarks.is_none() {
            return Err(Error::RecordCreationForbidden(
                "Adjustment remarks needs to be fill for adjustment record".to_string(),
            ));
        }
        // Implementation for creating a record
        let id = sqlx::query(
            "INSERT into records (item_id, date, transaction_type, quantity, total, adjustment_remarks) VALUES ($1, $2, $3, $4, $5, $6)",
        )
            .bind(item_id)
            .bind(date_create)
            .bind(transaction_type as u8)
            .bind(quantity)
            .bind(total)
            .bind(adjustment_remarks)
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

    pub async fn get_all(mm: ModelManager) -> Result<Records> {
        let db = mm.db();

        let records = sqlx::query_as::<_,Record>("SELECT id, item_id, date, transaction_type, quantity, total, adjustment_remarks FROM records")
            .fetch_all(db)
            .await?;

        Ok(records.into())
    }

    pub async fn get_in_timeframe(mm: ModelManager, start: i64, end: i64) -> Result<Records> {
        let db = mm.db();

        let records = sqlx::query_as::<_, Record>(
            "SELECT * FROM records WHERE date BETWEEN $1 AND $2 ORDER BY date DESC",
        )
        .bind(start)
        .bind(end)
        .fetch_all(db)
        .await?;

        Ok(records.into())
    }

    pub async fn get(mm: &ModelManager, record_id: u32) -> Result<Record> {
        // Implementation for retrieving records by item_id
        let db = mm.db();

        let record = sqlx::query_as::<_, Record>(
            "SELECT id,item_id, date, transaction_type, quantity, total, adjustment_remarks FROM records WHERE id = $1",
        )
        .bind(record_id)
        .fetch_one(db)
        .await?;

        Ok(record)
    }

    pub async fn get_all_for_item(mm: &ModelManager, item_id: u32) -> Result<ItemRecords> {
        let db = mm.db();

        let records = sqlx::query_as::<_,ItemRecord>
            ("SELECT id, date, transaction_type, quantity, adjustment_remarks FROM records WHERE item_id = $1 ORDER BY date DESC")
            .bind(item_id)
            .fetch_all(db)
            .await?;

        Ok(ItemRecords::new(item_id, records))
    }

    pub async fn get_in_timeframe_for_item(
        mm: &ModelManager,
        item_id: u32,
        start: i64,
        end: i64,
    ) -> Result<ItemRecords> {
        let db = mm.db();

        let records = sqlx::query_as::<_, ItemRecord>(
            "SELECT id, date, transaction_type, quantity, adjustment_remarks FROM records WHERE item_id = $1 AND date BETWEEN $2 AND $3 ORDER BY date DESC",
        ).bind(item_id)
            .bind(start)
            .bind(end)
            .fetch_all(db)
            .await?;

        Ok(ItemRecords::new(item_id, records))
    }

    pub async fn get_last_total(mm: &ModelManager, item_id: u32) -> Result<u32> {
        let db = mm.db();

        let result = sqlx::query_as::<_, (u32,)>(
            "SELECT total FROM records WHERE item_id = $1 ORDER BY date DESC, id DESC LIMIT 1",
        )
        .bind(item_id)
        .fetch_optional(db)
        .await?;

        Ok(result.map(|r| r.0).unwrap_or(0))
    }

    pub async fn get_last(mm: &ModelManager, item_id: u32) -> Option<ItemRecord> {
        // Implementation for retrieving the last record
        let db = mm.db();

        sqlx::query_as::<_, ItemRecord>(
            "SELECT id, date, transaction_type, quantity, total, adjustment_remarks  FROM records WHERE item_id = $1 ORDER BY date DESC, id DESC LIMIT 1",
        )
            .bind(item_id)
            .fetch_optional(db)
            .await.ok()?
    }

    pub async fn update(
        mm: &ModelManager,
        item_id: u32,
        record_id: u32,
        quantity: u32,
    ) -> Result<()> {
        // Implementation for updating a record
        // It will create a new record and mark the transaction
        // type to be `Adjustment`
        // get the corresponding record first
        let current_record = RecordsBmc::get(mm, record_id).await?;

        if current_record.item_id() != item_id {
            return Err(Error::RecordUpdateForbidden(format!(
                "Item ID ({item_id}) does not match record id ({record_id})",
            )));
        }

        // eval that the transaction type need to be IN or OUT
        let eval_tt = current_record
            .transaction_type()
            .eval_adjustment(current_record.quantity(), quantity);

        let Some(tt_new) = eval_tt else {
            return Err(Error::RecordUpdateForbidden(
                "The quantity is the same".to_string(),
            ));
        };

        // Get the quantity for adjustment to tally out the difference
        let quantity_new = (current_record.quantity() as i32 - quantity as i32).unsigned_abs();
        let date = Utc::now().timestamp();
        let _new_record_id = RecordsBmc::create(
            mm,
            current_record.item_id(),
            date,
            tt_new,
            quantity_new,
            Some(record_id),
        )
        .await?;

        Ok(())
    }

    pub async fn delete(mm: &ModelManager, record_id: u32, item_id: u32) -> Result<()> {
        // Add an adjustment record to cancel out the transaction
        let current_record = RecordsBmc::get(mm, record_id).await?;
        if current_record.item_id() != item_id {
            return Err(Error::RecordUpdateForbidden(format!(
                "Item ID does not match record ID {}",
                item_id
            )));
        }

        let opposite_tt = current_record.transaction_type().opposite();

        let date = Utc::now().timestamp();
        let _new_record = RecordsBmc::create(
            mm,
            item_id,
            date,
            opposite_tt,
            current_record.quantity(),
            Some(record_id),
        )
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::_dev_utils::get_dev_env;
    use crate::store::records::types::TransactionType;
    use crate::store::records::RecordsBmc;
    use chrono::{TimeZone, Utc};
    use serial_test::serial;
    use std::time::Duration;
    use tokio::time::sleep;

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
        let last_record = RecordsBmc::get_last(&mm, 1).await;

        assert!(last_record.is_some());
        let last_record = last_record.unwrap();
        assert_eq!(last_record.id(), 4);
        assert_eq!(last_record.date().timestamp(), 1685577600);
        assert_eq!(last_record.quantity(), 10);
        assert!(last_record.adjustment_remarks().is_none());

        // Test for getting the last record with no records
        let last_record = RecordsBmc::get_last(&mm, 9999).await;
        assert!(last_record.is_none());
    }

    #[tokio::test]
    #[serial]
    async fn test_get() {
        let mm = get_dev_env().await.unwrap();

        let record_id = 2;

        let record = RecordsBmc::get(&mm, record_id).await.unwrap();

        assert_eq!(record.id(), record_id);
        assert_eq!(record.date().timestamp(), 1675123200);
        assert_eq!(record.quantity(), 3);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_all_for_item() {
        let mm = get_dev_env().await.unwrap();

        let item_id = 2;

        let records = RecordsBmc::get_all_for_item(&mm, item_id).await.unwrap();

        assert_eq!(records.item_id(), item_id);
        assert_eq!(records.records().len(), 9);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_in_timeframe() {
        let mm = get_dev_env().await.unwrap();

        let start_time = Utc.with_ymd_and_hms(2024, 12, 1, 0, 0, 0).unwrap();
        let end_time = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();

        let records = RecordsBmc::get_in_timeframe_for_item(
            &mm,
            2,
            start_time.timestamp(),
            end_time.timestamp(),
        )
        .await
        .unwrap();

        assert_eq!(records.records().len(), 2);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_record() {
        let mm = get_dev_env().await.unwrap();

        let item_id = 1;

        // Create first record
        let time = Utc::now().timestamp();
        let _id = RecordsBmc::create(&mm, item_id, time, TransactionType::new_in(), 10, None)
            .await
            .unwrap();

        let current_total = RecordsBmc::get_last_total(&mm, item_id).await.unwrap();
        assert_eq!(current_total, 33);

        sleep(Duration::from_secs(1)).await;

        // Create second record
        let time = Utc::now().timestamp();
        let _id = RecordsBmc::create(&mm, item_id, time, TransactionType::new_out(), 3, None)
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
        RecordsBmc::update(&mm, 1, 2, 10).await.unwrap();

        let last = RecordsBmc::get_last(&mm, 1).await.unwrap();

        assert!(matches!(last.adjustment_remarks(), Some(a) if a == 2));
        assert!(matches!(
            last.transaction_type(),
            TransactionType::AdjustmentOut
        ));
        assert_eq!(last.quantity(), 7);

        // Update the last record
        RecordsBmc::update(&mm, 1, 4, 5).await.unwrap();

        let last = RecordsBmc::get_last(&mm, 1).await.unwrap();

        assert!(matches!(last.adjustment_remarks(), Some(a) if a == 4));
        assert!(matches!(
            last.transaction_type(),
            TransactionType::AdjustmentIn
        ));
        assert_eq!(last.quantity(), 5);

        RecordsBmc::update(&mm, 2, 14, 100).await.unwrap();

        let last = RecordsBmc::get_last(&mm, 2).await.unwrap();

        assert!(matches!(last.adjustment_remarks(), Some(a) if a == 14));
        assert!(matches!(
            last.transaction_type(),
            TransactionType::AdjustmentIn
        ));
        assert_eq!(last.quantity(), 60);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_record() {
        let mm = get_dev_env().await.unwrap();

        let item_id = 1;

        // Get the last record before deletion
        let last_record = RecordsBmc::get_last(&mm, item_id).await;
        assert!(last_record.is_some());

        let last_record = last_record.unwrap();

        // Delete the record
        RecordsBmc::delete(&mm, last_record.id(), item_id)
            .await
            .unwrap();

        // Check that the record is deleted
        let last_record_after_delete = RecordsBmc::get_last(&mm, item_id).await.unwrap();

        assert!(
            matches!(last_record_after_delete.adjustment_remarks(), Some(a) if a == last_record.id())
        );
        assert_eq!(
            last_record_after_delete.transaction_type(),
            last_record.transaction_type().opposite()
        );
        assert_eq!(last_record_after_delete.quantity(), last_record.quantity());
    }
}
