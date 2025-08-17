pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToCreatePool(String),
    FailToOpenCache(String),
    ParseError(String),

    ItemNotFound(i64),
    LocationMetadataNotFound(i64),
    SchemaNotFound(i64),
    
    QueryNotFound(u32),
    QueryError(String),
    DatabaseError(String),

    ImageNotFound(String),

    RecordUpdateForbidden(String),
    RecordCreationForbidden(String),

    IntegerConversionError(String),

    RedDbError(redb::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Configuration(err) => Self::FailToCreatePool(err.to_string()),
            sqlx::Error::Io(err) => Self::FailToCreatePool(err.to_string()),
            sqlx::Error::Protocol(err) => Self::FailToCreatePool(err.to_string()),
            sqlx::Error::Database(err) => Self::DatabaseError(err.to_string()),
            _ => Self::ParseError(err.to_string()),
        }
    }
}

impl From<redb::Error> for Error {
    fn from(err: redb::Error) -> Self {
        Self::RedDbError(err)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::IntegerConversionError(err.to_string())
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
