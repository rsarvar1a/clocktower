use anyhow::{anyhow, Context, Error, Result};
use lazy_static::lazy_static;
use serde::{self, Deserialize, Serialize};
use toml;

use std::sync::RwLock;

#[derive(Clone, Serialize, Deserialize)]
pub struct PostgresConfig
{
    pub max_connections: u32,
    pub postgres_url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerConfig
{
    pub host: String,
    pub port: u32,
    pub root: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SslConfig
{
    pub key_file: String,
    pub cert_file: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config
{
    pub db: PostgresConfig,
    pub server: ServerConfig,
    pub ssl: SslConfig,
}

lazy_static! {
    static ref CONFIG: RwLock<Option<Config>> = RwLock::new(None);
}

pub fn get() -> Result<Config, Error>
{
    let conf = CONFIG.read().unwrap();
    conf.clone().ok_or(anyhow!("no configuration loaded"))
}

pub fn load(config_path: &str) -> Result<(), Error>
{
    let toml_str = std::fs::read_to_string(config_path).context("failed to read config file")?;
    let update: Config = toml::from_str(&toml_str).context("invalid config file")?;

    let mut config = CONFIG.write().unwrap();
    *config = Some(update);
    Ok(())
}
