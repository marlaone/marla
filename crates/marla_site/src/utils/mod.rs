use std::path::PathBuf;

use path_clean::PathClean;

pub fn clean_path(path: &PathBuf) -> PathBuf {
    path.clean()
}
