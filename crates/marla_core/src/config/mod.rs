use std::{path::Path, sync::RwLock};

use config::Config;
use home::home_dir;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(
        Config::builder()
            .add_source(
                config::File::from(
                    Path::new(&home_dir().unwrap_or_default()).join(".marla.config.yml")
                )
                .required(false)
            )
            .add_source(
                config::File::from(std::env::current_dir().unwrap().join("./site/config.yml"))
                    .required(false)
            )
            .add_source(config::Environment::with_prefix("MARLA"))
            .set_default(
                "site.content",
                std::env::current_dir()
                    .unwrap()
                    .join("./site/content/")
                    .to_str()
                    .to_owned()
            )
            .unwrap()
            .set_default(
                "site.data",
                std::env::current_dir()
                    .unwrap()
                    .join("./site/data/")
                    .to_str()
                    .to_owned()
            )
            .unwrap()
            .set_default("http.host", "localhost")
            .unwrap()
            .set_default::<&str, u16>("http.port", 1809)
            .unwrap()
            .set_default(
                "site.theme",
                std::env::current_dir()
                    .unwrap()
                    .join("./site/themes/marla/")
                    .to_str()
                    .to_owned()
            )
            .unwrap()
            .build()
            .unwrap()
    );
}

pub fn site_debug() -> bool {
    let default = false;
    match SETTINGS.read() {
        Ok(reader) => reader.get_bool("site.debug").unwrap_or(default),
        Err(_) => default,
    }
}

pub fn site_content_path() -> String {
    let default = "./site/content/".to_string();
    match SETTINGS.read() {
        Ok(reader) => reader.get_string("site.content").unwrap_or(default),
        Err(_) => default,
    }
}

pub fn site_theme_path() -> String {
    let default = "./site/themes/marla/".to_string();
    match SETTINGS.read() {
        Ok(reader) => reader.get_string("site.theme").unwrap_or(default),
        Err(_) => default,
    }
}

pub fn site_output_path() -> String {
    match SETTINGS.read() {
        Ok(reader) => reader.get_string("site.output").unwrap_or_default(),
        Err(_) => "".to_string(),
    }
}

pub fn site_data_path() -> String {
    let default = "./site/data/".to_string();
    match SETTINGS.read() {
        Ok(reader) => reader.get_string("site.data").unwrap_or(default),
        Err(_) => default,
    }
}

pub fn http_host() -> String {
    let default = "localhost".to_string();
    match SETTINGS.read() {
        Ok(reader) => reader.get_string("http.host").unwrap_or(default),
        Err(_) => default,
    }
}

pub fn http_port() -> u16 {
    let default = 1809;
    match SETTINGS.read() {
        Ok(reader) => reader.get::<u16>("http.port").unwrap_or(default),
        Err(_) => default,
    }
}

pub fn logger_level() -> String {
    let default = "info".to_string();
    match SETTINGS.read() {
        Ok(reader) => reader.get_string("logger.level").unwrap_or(default),
        Err(_) => default,
    }
}

pub fn version() -> String {
    let default = "0.1".to_string();
    match SETTINGS.read() {
        Ok(reader) => reader.get_string("version").unwrap_or(default),
        Err(_) => default,
    }
}
