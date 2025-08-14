use crate::{Error, Result};
use redb::{Database, TableDefinition, WriteTransaction};
use std::path::Path;
use std::sync::Arc;

use redb::Error as RedbError;

pub struct ImageStore {
    db: Database,
}

const TABLE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("images");

impl ImageStore {
    pub fn new(url: impl AsRef<Path>) -> Result<Self> {
        let db = Database::create(url).map_err(RedbError::from)?;
        Ok(ImageStore { db })
    }

    pub fn store(&self, key: &str, data: Vec<u8>) -> Result<()> {
        let write_txn = self.db.begin_write().map_err(RedbError::from)?;
        {
            let mut table = write_txn.open_table(TABLE).map_err(RedbError::from)?;

            table.insert(key, data).map_err(RedbError::from)?;
        }
        write_txn.commit().map_err(RedbError::from)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Vec<u8>> {
        let read_txn = self.db.begin_read().map_err(RedbError::from)?;
        let table = read_txn.open_table(TABLE).map_err(RedbError::from)?;
        let result = table.get(key).map_err(RedbError::from)?;

        let Some(data) = result else {
            return Err(Error::ImageNotFound(key.into()));
        };

        Ok(data.value().to_vec())
    }

    pub fn remove_last(&self) -> Result<()> {
        let write_txn = self.db.begin_write().map_err(RedbError::from)?;
        {
            let mut table = write_txn.open_table(TABLE).map_err(RedbError::from)?;
            table.pop_last().map_err(RedbError::from)?;
        }
        write_txn.commit().map_err(RedbError::from)?;

        Ok(())
    }
}
