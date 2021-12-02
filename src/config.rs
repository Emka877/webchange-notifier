use std::path::PathBuf;

use crate::tools::WriteType;
use crate::models::AppConfig;

pub enum FileType {
    WriteType(WriteType),
    Configuration,
}

pub const BASE_FILENAME: &'static str = "base.txt";
pub const COMPARISON_FILENAME: &'static str = "comparison.txt";

pub fn get_relative_pathbuf_to(app_config: &AppConfig, file_type: FileType) -> PathBuf {
    let working_dir: PathBuf = std::env::current_dir().unwrap();
    let path: String = match file_type {
        FileType::WriteType(WriteType::Base) => format!("{}/{}", app_config.relative_store_path.clone(), BASE_FILENAME),
        FileType::WriteType(WriteType::Comparison) => format!("{}/{}", app_config.relative_store_path.clone(), COMPARISON_FILENAME),
        FileType::Configuration => "conf/config.ron".to_owned(),
    };
    working_dir.join(path)
}
