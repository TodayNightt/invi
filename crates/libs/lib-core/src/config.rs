use std::sync::OnceLock;

use crate::{Error, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| todo!())
}

#[allow(non_snake_case)]
pub struct Config {
    global_config: String,
    user_config: UserConfig,
}

#[allow(non_snake_case)]
pub struct UserConfig {
    pub(crate) DB_URL: String,
}

impl Config {
    
}
