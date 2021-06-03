use std::sync::{Arc, RwLock};

use crate::state::config::Settings;

pub mod config;
pub mod session;

//region ConfigManager
pub struct ConfigManager {
    pub config: Arc<Settings>,
}
impl From<Settings> for ConfigManager {
    fn from(config: Settings) -> Self {
        ConfigManager {
            config: Arc::new(config),
        }
    }
}
impl Clone for ConfigManager {
    fn clone(&self) -> Self {
        ConfigManager {
            config: self.config.clone(),
        }
    }
}
//endregion
