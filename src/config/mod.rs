use std::{
    path::{Path, PathBuf},
    sync::RwLock,
};

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
            .add_source(config::File::from(PathBuf::from("./site/config.yml")).required(false))
            .add_source(config::Environment::with_prefix("MARLA"))
            .set_default("site.content", "./site/content/")
            .unwrap()
            .set_default("http.host", "localhost")
            .unwrap()
            .set_default::<&str, u16>("http.port", 1809)
            .unwrap()
            .set_default("graphql.host", "localhost")
            .unwrap()
            .set_default::<&str, u16>("graphql.port", 1808)
            .unwrap()
            .set_default("site.theme", "./site/themes/marla/")
            .unwrap()
            .build()
            .unwrap()
    );
}
