use std::path::Path;

use figment::providers;
use figment::providers::Format;
use figment::Figment;
use serde::Deserialize;

use super::arguments::Cli;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub database_url: String,
    pub admin_master_key: String,
    pub bind_url: Option<String>,
}

impl Config {
    pub fn new(cli_args: &Cli) -> Self {
        let path = Path::new(&cli_args.configs_path);

        Figment::new()
            .merge(providers::Json::file(path.join("config.json")))
            .merge(providers::Toml::file(path.join("config.toml")))
            .merge(providers::Env::prefixed("ARKALIS_"))
            .extract()
            .expect("Failed to load configuration")
    }
}
