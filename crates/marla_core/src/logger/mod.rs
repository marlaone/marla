use std::env;

use env_logger::Builder;

use crate::config::logger_level;

pub fn setup_logger() {
    env::set_var("RUST_LOG", logger_level());
    Builder::new().parse_default_env().init();
}
