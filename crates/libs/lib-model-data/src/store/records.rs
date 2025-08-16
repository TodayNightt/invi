use chrono::{DateTime, Utc};
use lib_model::ModelManager;
use lib_model::{Error, Result};
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode, FromRow};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// region : Types
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, FromRow)]
pub struct RawRecord {
    pub id: i64,
    pub item_id: i64,
    pub date: DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub quantity: u32,
    pub total: u32,
    pub adjustment_remarks: Option<i64>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TransactionType {
    In = 1,
    Out = 2,
    AdjustmentIn = 3,
    AdjustmentOut = 4,
}

impl TransactionType {
    pub fn new_in() -> TransactionType {
        TransactionType::In
    }

    pub fn new_out() -> TransactionType {
        TransactionType::Out
    }

    pub fn new_adjustment_in() -> TransactionType {
        TransactionType::AdjustmentIn
    }

    pub fn new_adjustment_out() -> TransactionType {
        TransactionType::AdjustmentOut
    }

    pub fn is_adjustment(&self) -> bool {
        self == &TransactionType::AdjustmentIn || self == &TransactionType::AdjustmentOut
    }
    pub fn do_arithmetic(&self, last_total: u32, quantity: u32) -> u32 {
        match self {
            TransactionType::In => last_total + quantity,
            TransactionType::Out => last_total - quantity,
            TransactionType::AdjustmentIn => last_total + quantity,
            TransactionType::AdjustmentOut => last_total - quantity,
        }
    }

    fn casting_to_signed(&self, num: u32) -> i32 {
        match self {
            TransactionType::In => num as i32,
            TransactionType::Out => -(num as i32),
            _ => num as i32,
        }
    }

    pub fn eval_adjustment(&self, current_quantity: u32, new_quantity: u32) -> Option<Self> {
        if current_quantity.eq(&new_quantity) {
            return None;
        }
        let signed_current = self.casting_to_signed(current_quantity);
        let signed_new = self.casting_to_signed(new_quantity);

        if signed_current > signed_new {
            Some(TransactionType::AdjustmentOut)
        } else {
            Some(TransactionType::AdjustmentIn)
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            TransactionType::In => TransactionType::AdjustmentOut,
            TransactionType::Out => TransactionType::AdjustmentIn,
            TransactionType::AdjustmentIn => TransactionType::AdjustmentOut,
            TransactionType::AdjustmentOut => TransactionType::AdjustmentIn,
        }
    }
}

impl<'r, DB: Database> Decode<'r, DB> for TransactionType
where
    u8: Decode<'r, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> core::result::Result<Self, BoxDynError> {
        let value = <u8 as Decode<DB>>::decode(value)?;

        match value {
            1 => Ok(TransactionType::In),
            2 => Ok(TransactionType::Out),
            3 => Ok(TransactionType::AdjustmentIn),
            4 => Ok(TransactionType::AdjustmentOut),
            _ => Err(format!("Unknown transaction type {}", value).into()),
        }
    }
}
// endregion

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
        item_id: i64,
        date_create: i64,
        transaction_type: TransactionType,
        quantity: u32,
        adjustment_remarks: Option<i64>,
    ) -> Result<i64> {
        let db = mm.db();
        // Calculate the total based on the previous record
        let last_total = Self::get_last_total(mm, item_id).await?;

        let total = transaction_type.do_arithmetic(last_total, quantity);

        if transaction_type.is_adjustment() && adjustment_remarks.is_none() {
            return Err(Error::RecordCreationForbidden(
                "Adjustment remarks needs to be fill for adjustment record".to_string(),
            ));
        }

        let tt = transaction_type as u8;
        // Implementation for creating a record
        let id = sqlx::query!(
            "INSERT into records (item_id, date, transaction_type, quantity, total, adjustment_remarks) VALUES ($1, $2, $3, $4, $5, $6)",
            item_id, date_create, tt, quantity, total, adjustment_remarks
        )
            .execute(db)
            .await
            .map_err(|err| match err {
                sqlx::error::Error::RowNotFound => {
                    Error::QueryError(format!("Failed to create record for item_id: {}", item_id))
                }
                _ => err.into(),
            })?.last_insert_rowid();

        Ok(id)
    }

    pub async fn get_all(mm: &ModelManager) -> Result<Vec<RawRecord>> {
        let db = mm.db();

        let records = sqlx::query_as!(
            RawRecord,
            r#"SELECT id, item_id, date as "date: DateTime<Utc>", transaction_type as "transaction_type: TransactionType", quantity as "quantity: u32", total as "total: u32", adjustment_remarks FROM records"#
        )
            .fetch_all(db)
            .await?;

        Ok(records)
    }

    pub async fn get_in_timeframe(
        mm: &ModelManager,
        start: i64,
        end: i64,
    ) -> Result<Vec<RawRecord>> {
        let db = mm.db();

        let records = sqlx::query_as!(
            RawRecord,
            r#"SELECT id, item_id, date as "date: DateTime<Utc>", transaction_type as "transaction_type: TransactionType", quantity as "quantity: u32", total as "total: u32", adjustment_remarks FROM records WHERE date BETWEEN $1 AND $2 ORDER BY date DESC"#,
            start, end
        )
            .fetch_all(db)
            .await?;

        Ok(records)
    }

    pub async fn get(mm: &ModelManager, record_id: i64) -> Result<RawRecord> {
        // Implementation for retrieving records by item_id
        let db = mm.db();

        let record = sqlx::query_as!(
            RawRecord,
            r#"SELECT id, item_id, date as "date: DateTime<Utc>", transaction_type as "transaction_type: TransactionType", quantity as "quantity: u32", total as "total: u32", adjustment_remarks FROM records WHERE id = $1"#,
            record_id
        )
            .fetch_one(db)
            .await?;

        Ok(record)
    }

    pub async fn get_all_for_item(mm: &ModelManager, item_id: i64) -> Result<Vec<RawRecord>> {
        let db = mm.db();

        let records = sqlx::query_as!(
            RawRecord,
            r#"SELECT id,item_id, date as "date: DateTime<Utc>", transaction_type as "transaction_type: TransactionType", quantity as "quantity: u32", total as "total: u32", adjustment_remarks FROM records WHERE item_id = $1 ORDER BY date DESC"#,
            item_id
        )
            .fetch_all(db)
            .await?;

        Ok(records)
    }

    pub async fn get_in_timeframe_for_item(
        mm: &ModelManager,
        item_id: i64,
        start: i64,
        end: i64,
    ) -> Result<Vec<RawRecord>> {
        let db = mm.db();

        let records = sqlx::query_as!(
            RawRecord,
            r#"SELECT id, item_id, date as "date: DateTime<Utc>", transaction_type as "transaction_type: TransactionType", quantity as "quantity: u32", total as "total: u32", adjustment_remarks FROM records WHERE item_id = $1 AND date BETWEEN $2 AND $3 ORDER BY date DESC"#,
            item_id, start, end
        )
            .fetch_all(db)
            .await?;

        Ok(records)
    }

    pub async fn get_last_total(mm: &ModelManager, item_id: i64) -> Result<u32> {
        let db = mm.db();

        let result = sqlx::query!(
            r#"SELECT total as "total: u32" FROM records WHERE item_id = $1 ORDER BY date DESC, id DESC LIMIT 1"#,
            item_id
        )
            .fetch_optional(db)
            .await?;

        Ok(result.map(|r| r.total).unwrap_or(0))
    }

    pub async fn get_last(mm: &ModelManager, item_id: i64) -> Option<RawRecord> {
        // Implementation for retrieving the last record
        let db = mm.db();

        sqlx::query_as!(
            RawRecord,
            r#"SELECT id, item_id, date as "date: DateTime<Utc>", transaction_type as "transaction_type: TransactionType", quantity as "quantity: u32", total as "total: u32", adjustment_remarks  FROM records WHERE item_id = $1 ORDER BY date DESC, id DESC LIMIT 1"#,
            item_id
        )
            .fetch_optional(db)
            .await.ok()?
    }

    pub async fn update(
        mm: &ModelManager,
        item_id: i64,
        record_id: i64,
        quantity: u32,
    ) -> Result<()> {
        // Implementation for updating a record
        // It will create a new record and mark the transaction
        // type to be `Adjustment`
        // get the corresponding record first
        let current_record = RecordsBmc::get(mm, record_id).await?;

        if current_record.item_id != item_id {
            return Err(Error::RecordUpdateForbidden(format!(
                "Item ID ({item_id}) does not match record id ({record_id})",
            )));
        }

        // eval that the transaction type need to be IN or OUT
        let eval_tt = current_record
            .transaction_type
            .eval_adjustment(current_record.quantity, quantity);

        let Some(tt_new) = eval_tt else {
            return Err(Error::RecordUpdateForbidden(
                "The quantity is the same".to_string(),
            ));
        };

        // Get the quantity for adjustment to tally out the difference
        let quantity_new = (current_record.quantity as i32 - quantity as i32).unsigned_abs();
        let date = Utc::now().timestamp();
        let _new_record_id = RecordsBmc::create(
            mm,
            current_record.item_id,
            date,
            tt_new,
            quantity_new,
            Some(record_id),
        )
        .await?;

        Ok(())
    }

    pub async fn delete(mm: &ModelManager, record_id: i64, item_id: i64) -> Result<()> {
        // Add an adjustment record to cancel out the transaction
        let current_record = RecordsBmc::get(mm, record_id).await?;
        if current_record.item_id != item_id {
            return Err(Error::RecordUpdateForbidden(format!(
                "Item ID does not match record ID {}",
                item_id
            )));
        }

        let opposite_tt = current_record.transaction_type.opposite();

        let date = Utc::now().timestamp();
        let _new_record = RecordsBmc::create(
            mm,
            item_id,
            date,
            opposite_tt,
            current_record.quantity,
            Some(record_id),
        )
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::store::records::RecordsBmc;
    use crate::store::records::TransactionType;
    use chrono::{TimeZone, Utc};
    use lib_model::_dev_utils::get_dev_env;
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
        assert_eq!(last_record.id, 4);
        assert_eq!(last_record.date.timestamp(), 1685577600);
        assert_eq!(last_record.quantity, 10);
        assert!(last_record.adjustment_remarks.is_none());

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

        assert_eq!(record.id, record_id);
        assert_eq!(record.date.timestamp(), 1675123200);
        assert_eq!(record.quantity, 3);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_all_for_item() {
        let mm = get_dev_env().await.unwrap();

        let item_id = 2;

        let records = RecordsBmc::get_all_for_item(&mm, item_id).await.unwrap();

        assert_eq!(records.len(), 9);
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

        assert_eq!(records.len(), 2);
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

        assert!(matches!(last.adjustment_remarks, Some(a) if a == 2));
        assert!(matches!(
            last.transaction_type,
            TransactionType::AdjustmentOut
        ));
        assert_eq!(last.quantity, 7);

        // Update the last record
        RecordsBmc::update(&mm, 1, 4, 5).await.unwrap();

        let last = RecordsBmc::get_last(&mm, 1).await.unwrap();

        assert!(matches!(last.adjustment_remarks, Some(a) if a == 4));
        assert!(matches!(
            last.transaction_type,
            TransactionType::AdjustmentIn
        ));
        assert_eq!(last.quantity, 5);

        RecordsBmc::update(&mm, 2, 14, 100).await.unwrap();

        let last = RecordsBmc::get_last(&mm, 2).await.unwrap();

        assert!(matches!(last.adjustment_remarks, Some(a) if a == 14));
        assert!(matches!(
            last.transaction_type,
            TransactionType::AdjustmentIn
        ));
        assert_eq!(last.quantity, 60);
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
        RecordsBmc::delete(&mm, last_record.id, item_id)
            .await
            .unwrap();

        // Check that the record is deleted
        let last_record_after_delete = RecordsBmc::get_last(&mm, item_id).await.unwrap();

        assert!(
            matches!(last_record_after_delete.adjustment_remarks, Some(a) if a == last_record.id)
        );
        assert_eq!(
            last_record_after_delete.transaction_type,
            last_record.transaction_type.opposite()
        );
        assert_eq!(last_record_after_delete.quantity, last_record.quantity);
    }
}
