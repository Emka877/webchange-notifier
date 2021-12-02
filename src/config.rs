use std::path::PathBuf;

use crate::tools::WriteType;
use crate::models::AppConfig;

type StoreFileType = WriteType;

pub const BASE_FILENAME: &'static str = "base.txt";
pub const COMPARISON_FILENAME: &'static str = "comparison.txt";

pub fn get_relative_path_to(app_config: &AppConfig, file_type: StoreFileType) -> String {
    get_relative_pathbuf_to(app_config, file_type).to_str().unwrap().to_owned()
}

pub fn get_relative_pathbuf_to(app_config: &AppConfig, file_type: StoreFileType) -> PathBuf {
    let path: String = format!("{}/{}", &app_config.relative_store_path, BASE_FILENAME);
    std::fs::canonicalize(&path).expect(format!("Error trying to normalize this path: {}", &path).as_str())
}
