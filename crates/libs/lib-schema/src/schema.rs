use serde::{Deserialize, Serialize};
use lib_commons::{FieldType, Value};
use crate::Field;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    id : Option<i64>,
    name: String,
    fields: Vec<Field>,
}

impl Schema {
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.name() == name)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn create(name: &str, fields: Vec<Field>) -> Self {
        Schema {
            name: name.to_string(),
            fields,
            id : None,
        }
    }

    pub fn push_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    pub fn remove_field(&mut self, index: usize) {
        self.fields.remove(index);
    }

    pub fn rename_field(&mut self, index: usize, new_name: &str)-> Option<()> {
        self.fields.get_mut(index)?.set_name(new_name);
        Some(())
    }

    pub fn change_type_field(&mut self, index: usize, new_field_type : FieldType) -> Option<()>{
        self.fields.get_mut(index)?.set_field_type(new_field_type);
        Some(())
    }

    pub fn change_default_field(&mut self, index: usize, default_value: Value ) -> Option<()> {
        self.fields.get_mut(index)?.set_default(default_value);
        Some(())
    }

    pub fn set_required_field(&mut self, index: usize, required : bool) -> Option<()> {
        self.fields.get_mut(index)?.set_required(required);
        Some(())
    }

    pub fn set_id(&mut self, id : i64) {
        self.id = Some(id);
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }

}

impl From<lib_model_schema::types::Schema> for Schema {
    fn from(value: lib_model_schema::types::Schema) -> Self {
        Schema {
            id : Some(value.id()),
            name : value.name().to_string(),
            fields : value.fields().as_ref().to_vec(),
        }
    }
}

impl From<&lib_model_schema::types::Schema> for Schema {
    fn from(value: &lib_model_schema::types::Schema) -> Self {
        value.clone().into()
    }
}
