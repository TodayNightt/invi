use sqlx::error::BoxDynError;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Database, Decode, FromRow, Type};
use std::sync::Arc;

// region : Record
pub type Records = Arc<[Record]>;
#[derive(Debug, Copy, Clone, FromRow)]
pub struct Record {
    id: u32,
    item_id: u32,
    date: DateTime<Utc>,
    transaction_type: TransactionType,
    quantity: u32,
    total: u32,
    adjustment_remarks: Option<u32>,
}

impl Record {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn item_id(&self) -> u32 {
        self.item_id
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn total(&self) -> u32 {
        self.total
    }

    pub fn adjustment_remarks(&self) -> Option<u32> {
        self.adjustment_remarks
    }
}

pub struct ItemRecords {
    item_id: u32,
    records: Arc<[ItemRecord]>,
}

impl ItemRecords {
    pub fn new(item_id: u32, records: Vec<ItemRecord>) -> Self {
        ItemRecords {
            item_id,
            records: records.into(),
        }
    }

    pub fn item_id(&self) -> u32 {
        self.item_id
    }

    pub fn records(&self) -> Arc<[ItemRecord]> {
        self.records.clone()
    }
}
// endregion

// region : ItemRecord
#[derive(Debug, Copy, Clone, sqlx::FromRow)]
pub struct ItemRecord {
    id: u32,
    date: DateTime<Utc>,
    transaction_type: TransactionType,
    quantity: u32,
    adjustment_remarks: Option<u32>,
}
impl ItemRecord {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn adjustment_remarks(&self) -> Option<u32> {
        self.adjustment_remarks
    }
}

// endregion

// region : TransactionType
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum TransactionType {
    In = 1,
    Out = 2,
    AdjustmentIn = 3,
    AdjustmentOut = 4,
}

impl<'r, DB: Database> Decode<'r, DB> for TransactionType
where
    u8: Decode<'r, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
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

impl From<i64> for TransactionType {
    fn from(value: i64) -> Self {
        match value {
            1 => TransactionType::In,
            2 => TransactionType::Out,
            3 => TransactionType::AdjustmentIn,
            4 => TransactionType::AdjustmentOut,
            _ => TransactionType::In,
        }
    }
}

impl<DB> Type<DB> for TransactionType
where
    DB: Database,
    bool: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <bool as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <bool as Type<DB>>::compatible(ty)
    }
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

// endregion


#[cfg(test)]
mod tests {
    use crate::store::records::types::TransactionType;

    #[test]
    fn test_eval_adjustment() {
        let current = 40;

        // IN current > new => OUT
        let tt = TransactionType::new_in();
        let adjustment = tt.eval_adjustment(current, 10).unwrap();
        assert_eq!(adjustment, TransactionType::AdjustmentOut);

        // IN current < new => IN
        let tt = TransactionType::new_in();
        let adjustment = tt.eval_adjustment(current, 100).unwrap();
        assert_eq!(adjustment, TransactionType::AdjustmentIn);

        // IN current == new => None
        let tt = TransactionType::new_in();
        let adjustment = tt.eval_adjustment(current, 40);
        assert!(adjustment.is_none());

        // OUT current > new => IN
        let tt = TransactionType::new_out();
        let adjustment = tt.eval_adjustment(current, 10).unwrap();
        assert_eq!(adjustment, TransactionType::AdjustmentIn);

        // OUT current < new => OUT
        let tt = TransactionType::new_out();
        let adjustment = tt.eval_adjustment(current, 100).unwrap();
        assert_eq!(adjustment, TransactionType::AdjustmentOut);

        // OUT current == new => None
        let tt = TransactionType::new_out();
        let adjustment = tt.eval_adjustment(current, 40);
        assert!(adjustment.is_none());
    }
}
