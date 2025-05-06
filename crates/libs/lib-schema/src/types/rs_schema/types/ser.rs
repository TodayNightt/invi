use serde::{Serialize, Serializer};

use super::{SchemaType, wrapper::TypeWrapper};

// Implement Serialize for Type
impl Serialize for SchemaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wrapper = TypeWrapper::new(self.clone());
        wrapper.serialize(serializer)
    }
}
