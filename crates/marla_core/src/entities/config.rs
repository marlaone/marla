use std::path::PathBuf;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    pub content_path: PathBuf,
    pub data_path: PathBuf,
    pub theme_path: PathBuf,
    pub http_host: String,
    pub http_port: u16,
}
