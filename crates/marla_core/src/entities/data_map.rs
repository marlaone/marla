use std::collections::BTreeMap;

#[cfg(feature = "serde")]
pub type DataMap = serde_json::Map<String, serde_json::Value>;

#[cfg(not(feature = "serde"))]
pub type DataMap = BTreeMap<String, String>;
