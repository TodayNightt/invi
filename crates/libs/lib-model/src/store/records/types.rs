use std::sync::Arc;
use chrono::Utc;

pub(crate) struct Records {
    item_id: u32,
    records: Arc<[Record]>,
}

impl Records {
    pub fn new(item_id: u32, records: Vec<Record>) -> Self {
        Records { item_id, records : records.into() }
    }

    pub fn item_id(&self) -> u32 {
        self.item_id
    }

    pub fn records(&self) -> Arc<[Record]> {
        self.records.clone()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Record {
    id: u32,
    date: chrono::DateTime<Utc>,
    transaction_type: TransactionType,
    quantity: u32,
    correction: bool,
}

impl Record {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn date(&self) -> chrono::DateTime<Utc> {
        self.date
    }

    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn correction(&self) -> bool {
        self.correction
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TransactionType {
    In,
    Out,
}

impl TransactionType {
    pub fn opposite(&self) -> Self {
        match self {
            TransactionType::In => TransactionType::Out,
            TransactionType::Out => TransactionType::In,
        }
    }
}

impl From<TransactionType> for bool {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::In => false,
            TransactionType::Out => true,
        }
    }
}

impl From<bool> for TransactionType {
    fn from(value: bool) -> Self {
        match value {
            false => TransactionType::In,
            true => TransactionType::Out,
        }
    }
}

#[derive(Debug, Copy, Clone, sqlx::FromRow)]
pub(crate) struct RawRecord {
    id: u32,
    date: chrono::DateTime<Utc>,
    transaction_type: bool,
    quantity: u32,
    correction: bool,
}

impl From<RawRecord> for Record {
    fn from(raw: RawRecord) -> Self {
        Record {
            id: raw.id,
            date: raw.date,
            transaction_type: raw.transaction_type.into(),
            quantity: raw.quantity,
            correction: raw.correction,
        }
    }
}
