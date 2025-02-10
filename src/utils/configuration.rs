use dotenvy::dotenv;
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub real_mode: bool,
    pub use_onchain: bool,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();
        let config = Config {
            real_mode: env::var("REAL_MODE")
                .map(|val| val.parse().unwrap_or(true))
                .unwrap_or(true),
            use_onchain: env::var("USE_ONCHAIN")
                .map(|val| val.parse().unwrap_or(true))
                .unwrap_or(true),
        };

        CONFIG
            .set(config)
            .map_err(|_| "Config already initialized".into())
    }

    pub fn get() -> &'static Config {
        CONFIG.get().expect("Config not initialized")
    }
}
