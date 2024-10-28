use std::fs::File;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub app: AppDetails,
}

#[derive(Deserialize, Debug)]
pub struct AppDetails {
    pub binance: BinanceAccountConfig,
}

#[derive(Deserialize, Debug)]
pub struct BinanceAccountConfig {
    api_key:  String,
    api_secret: String,
    base_url: String,
}

impl BinanceAccountConfig {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn api_secret(&self) -> &str {
        &self.api_secret
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

pub fn read_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let file = File::open("resources/config.yml")?;
    let config = serde_yaml::from_reader(file)?;
    Ok(config)
}