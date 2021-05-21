use std::borrow::Borrow;
use std::sync::{Arc, RwLock};

use handlebars::Handlebars;

use handlebars_utils::HandlebarsUtils;
use crate::state::config::Settings;

pub mod handlebars_utils;
pub mod config;
pub mod session;


//region HandlebarManager
pub struct HandlebarManager {
    pub handlebars: Arc<RwLock<HandlebarsUtils<'static>>>,
}
impl HandlebarManager {
    pub fn default(cfg: Arc<Settings>) -> Self {
        HandlebarManager {
            handlebars: Arc::new(RwLock::new(HandlebarsUtils::default(cfg))),
        }
    }
}
impl HandlebarManager {
    pub fn register(self) -> Self {
        self.handlebars.write().unwrap().register_handlebars();
        self
    }
}
impl Clone for HandlebarManager {
    fn clone(&self) -> Self {
        HandlebarManager {
            handlebars: self.handlebars.clone()
        }
    }
}
//endregion

//region ConfigManager
pub struct ConfigManager {
    pub config: Arc<Settings>
}
impl From<Settings> for ConfigManager {
    fn from(config: Settings) -> Self {
        ConfigManager {
            config: Arc::new(config)
        }
    }
}
impl Clone for ConfigManager {
    fn clone(&self) -> Self {
        ConfigManager {
            config: self.config.clone()
        }
    }
}
//endregion
