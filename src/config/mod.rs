use std::sync::RwLock;

use config::Config;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(
        Config::builder()
            .add_source(config::File::with_name("config/marla.yml").required(false))
            .add_source(config::Environment::with_prefix("MARLA"))
            .build()
            .unwrap()
    );
}
