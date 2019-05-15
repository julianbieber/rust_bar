use dirs::home_dir;
use std::path::{Path, PathBuf};

pub fn base_path() -> PathBuf {
    home_dir()
        .expect("Home directory must be set")
        .join(Path::new(".tracking"))
}
