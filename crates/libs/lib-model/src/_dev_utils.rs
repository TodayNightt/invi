use crate::ModelManager;

use crate::Result;

pub async  fn get_dev_env() -> Result<ModelManager> {
    let db_url = env!("LIBMODEL_DEV_DB_URL");

    Ok(ModelManager::new(db_url).await?)
}