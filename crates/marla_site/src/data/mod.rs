use std::path::PathBuf;

use glob::glob;
use log::warn;
use marla_core::config::site_data_path;
use path_clean::PathClean;
use serde_json::{from_str, Map};
use tera::Value;

fn clean_data_path(data_path: &PathBuf) -> PathBuf {
    let site_data_path = PathBuf::from(site_data_path())
        .clean()
        .to_str()
        .unwrap_or_default()
        .to_string();
    let cleaned_path = PathBuf::from(
        data_path
            .to_str()
            .unwrap_or_default()
            .replace(site_data_path.as_str(), ""),
    );
    return cleaned_path.clean();
}

fn fill_data_map(
    data_map: &mut Map<String, serde_json::Value>,
    data_path: PathBuf,
    file_path: PathBuf,
) {
    let end_path = data_path.as_path().file_name().unwrap_or_default();
    let end_key = data_path.as_path().file_stem().unwrap_or_default();
    let mut cur_map = data_map;
    for path in data_path.iter() {
        if path == "/" {
            continue;
        } else if path == end_path {
            let file_content_result = std::fs::read_to_string(&file_path);
            match file_content_result {
                Ok(file_content) => match from_str(&file_content) {
                    Ok(json_value) => {
                        cur_map
                            .insert(end_key.to_str().unwrap_or_default().to_string(), json_value);
                    }
                    Err(e) => warn!(
                        "failed to parse json for {:?}: {}",
                        file_path,
                        e.to_string()
                    ),
                },
                Err(e) => warn!(
                    "failed to read data content for {:?}: {}",
                    file_path,
                    e.to_string()
                ),
            }
        } else {
            let key = path.to_str().unwrap_or_default().to_string();
            cur_map = cur_map
                .entry(key)
                .or_insert(Value::Object(Map::new()))
                .as_object_mut()
                .unwrap();
        }
    }
}

pub fn get_site_data() -> anyhow::Result<Value> {
    let mut site_data_pattern = site_data_path();
    site_data_pattern.push_str("/**/*.json");
    let mut data_map = serde_json::Map::new();
    for data_entry in glob(&site_data_pattern)? {
        match data_entry {
            Ok(data_path) => {
                let cleaned_data_path = clean_data_path(&data_path);
                fill_data_map(&mut data_map, cleaned_data_path, data_path);
            }
            Err(e) => {
                log::error!("failed to parse site data path: {}", e)
            }
        }
    }

    Ok(Value::Object(data_map))
}
