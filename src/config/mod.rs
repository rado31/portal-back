mod state;

use config::{Config, ConfigError, File};
use serde::Deserialize;
pub use state::State;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub app: App,
    pub database: Database,
    pub auth: Auth,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct App {
    pub url: String,
    pub upload_path: String,
    pub media_password: String,
    pub release: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Auth {
    pub access_key: String,
    pub access_time: u64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let setting = Config::builder()
            .add_source(File::with_name("src/config/default"))
            .build()?;

        setting.try_deserialize()
    }
}
