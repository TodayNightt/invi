pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToCreatePool(String),
    FailToOpenCache(String),
    ParseError(String),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::FailToCreatePool(err.to_string())
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Self {
        Self::FailToOpenCache(err.to_string())
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
