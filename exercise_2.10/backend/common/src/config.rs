use serde::Deserialize;
use std::path::PathBuf;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub http: HttpConfig,
    pub database: DatabaseConfig,
    pub app: AppConfig,
}

fn default_data_path() -> PathBuf {
    PathBuf::try_from("/tmp").unwrap()
}

#[derive(Clone, Deserialize, Debug)]
pub struct AppConfig {
    #[serde(default = "default_data_path")]
    pub data_path: PathBuf,
}

#[derive(Clone, Deserialize, Debug)]
pub struct HttpConfig {
    pub port: u16,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::with_prefix("TODO").separator("_"))
        .build()?;

    let app_config = settings.try_deserialize()?;

    Ok(app_config)
}
