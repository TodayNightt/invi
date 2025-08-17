pub(crate) mod schema {
    use lib_commons::Field;
    use lib_model::{Error, ModelManager, Result};
    use sqlx::types::Json;

    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, sqlx::FromRow)]
    pub struct RawSchema {
        pub id: i64,
        pub name: String,
        pub fields: Json<Vec<Field>>,
    }

    /// ```sql
    /// table schema {
    ///    id     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    //     name   TEXT                              NOT NULL,
    //     fields TEXT                              NOT NULL
    /// }
    /// ```
    pub(crate) struct SchemaBmc;

    impl SchemaBmc {
        // Implement CRUD operations
        pub async fn create(mm: &ModelManager, name: &str, fields: &str) -> Result<i64> {
            // Create an item
            let db = mm.schema_db();

            let id = sqlx::query!(
                "INSERT INTO schema (name, fields) VALUES ($1, $2)",
                name,
                fields
            )
            .execute(db)
            .await
            .map_err(|err| match err {
                sqlx::error::Error::RowNotFound => {
                    Error::QueryError(format!("Failed to create item: {}", name))
                }
                _ => err.into(),
            })?
            .last_insert_rowid();

            Ok(id)
        }

        pub async fn get_all(mm: &ModelManager) -> Result<Vec<RawSchema>> {
            let db = mm.schema_db();

            let result = sqlx::query_as!(
                RawSchema,
                r#"SELECT id, name, fields as "fields: Json<Vec<Field>>" FROM schema"#
            )
            .fetch_all(db)
            .await?;

            Ok(result)
        }

        pub async fn get(mm: &ModelManager, id: i64) -> Option<RawSchema> {
            let db = mm.schema_db();

            sqlx::query_as!(
                RawSchema,
                r#"SELECT id, name, fields as "fields: Json<Vec<Field>>" FROM schema WHERE id = $1"#,
                id
            )
            .fetch_optional(db)
            .await
            .ok()?
        }

        pub async fn update_name(mm: &ModelManager, id: i64, name: &str) -> Option<()> {
            let db = mm.schema_db();

            sqlx::query!("UPDATE schema SET name = $1 WHERE id = $2", name, id)
                .execute(db)
                .await
                .ok()?;

            Some(())
        }

        pub async fn update_fields(mm: &ModelManager, id: i64, fields: &str) -> Option<()>{
            let db = mm.schema_db();

            sqlx::query!("UPDATE schema SET fields = $1 WHERE id = $2", fields, id)
                .execute(db)
                .await
                .ok()?;

            Some(())
        }

        pub async fn delete(mm: &ModelManager, id: i64) -> Option<()> {
            let db = mm.schema_db();
            sqlx::query!("DELETE FROM schema WHERE id = $1", id)
                .execute(db)
                .await
                .ok()?;

            Some(())
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::store::schema::SchemaBmc;
        use lib_commons::{Field, FieldType, Value};
        use lib_model::_dev_utils::get_dev_env;
        use serde_json::json;
        use serial_test::serial;

        #[tokio::test]
        #[serial]
        async fn test_schema_create() {
            let mm = get_dev_env().await.unwrap();
            let name = "Test 1";
            let field = Field::create("Field 1", FieldType::String, true, Value::Null);
            let fields = json!(vec![field]).to_string();

            SchemaBmc::create(&mm, name, &fields).await.unwrap();
        }
    }
}
