#[derive(Debug)]
pub struct ErrorDescriptor {
    kind: String,
    reason: Option<String>,
}

impl std::fmt::Display for ErrorDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(reason) = &self.reason {
            write!(f, "{}: {}", self.kind, reason)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

use jsonschema::error::ValidationError;

use super::get_type_name;

impl From<ValidationError<'_>> for ErrorDescriptor {
    fn from(error: ValidationError) -> Self {
        Self {
            kind: get_type_name(&error.kind).to_string(),
            reason: Some(error.to_string()),
        }
    }
}
