use figment::providers;
use figment::providers::Format;
use figment::Figment;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        Figment::new()
            .merge(providers::Json::file("config.json"))
            .merge(providers::Env::prefixed("ARKALIS_"))
            .extract()
            .expect("Failed to load configuration")
    }
}
