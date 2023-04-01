use super::{config::Config, data_map::DataMap, page::Page};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Site {
    pub path: String,
    pub page: Page,
    pub pages: Vec<Page>,
    pub config: Config,
    pub data: DataMap,
    pub lang: Option<String>,
}
