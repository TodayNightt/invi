use serde::{Serialize, Serializer, ser::SerializeMap};

use super::Property;

impl Serialize for Property {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.name, &self.property_type)?;
        map.end()
    }
}
