mod extractor;
// mod generator;
mod registry;
mod validator;
pub use error::{Error, Result};
mod types;
mod error {
    pub type Result<T> = core::result::Result<T, Error>;

    use lib_utils::types::ErrorDescriptor;

    #[derive(Debug)]
    pub enum Error {
        FailToCreateValidator(ErrorDescriptor),
        UnknownType(String),
        ValidateError(Vec<ErrorDescriptor>),
        RootSchemaNotAnObject,
        MissingKeyword(String),
        InvalidTypeFor(String, String),
        WrongTypeInside,
    }

    impl std::error::Error for Error {}

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
}
