use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub listen_address: String,
    pub port: Option<u16>,
    pub service_name: String,
    pub database_path: String,
}