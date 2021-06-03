use config::{Config, ConfigError, Environment, File};

use serde::Deserialize;
use std::fmt;

const CONFIG_FILE_PATH: &str = "./config/Default.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Templating {
    pub path: String,
    pub static_dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub api_enabled: bool,
    pub icon_domain: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub general: General,
    pub server: Server,
    pub database: Database,
    pub templating: Templating,
    pub log: Log,
    pub env: ENV,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUST_ENV").unwrap_or_else(|_| "Development".into());
        let mut s = Config::new();
        s.set("env", env)?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;

        // This makes it so "EA_SERVER__PORT overrides server.port
        s.merge(Environment::with_prefix("ariadne").separator("_"))?;

        s.try_into()
    }
}
