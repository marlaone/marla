use std::env;

use env_logger::Builder;

use crate::config::SETTINGS;

pub fn setup_logger() {
    let logger_level = SETTINGS
        .read()
        .unwrap()
        .get_string("logger.level")
        .unwrap_or("info".to_string());
    env::set_var("RUST_LOG", format!("actix_web={}", logger_level));
    Builder::new().parse_default_env().init();
}
