use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use reqwest::Response;
use ron::de::from_reader;
use crate::{BASE_FILENAME, COMPARISON_FILENAME};
use crate::errors::BaseOverwriteError;
use crate::models::AppConfig;
use crate::tools::WriteType::Base;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CompareResult {
    Same,
    Different,
    EmptyBase,
    EmptyNewest,
}

/// Compares a base to the newest version of a page.
// TODO: Find a way to compare page strings faster (eg: for big pages)
pub fn compare_pages(base: &str, newest: &str) -> CompareResult {
    if base.is_empty() {
        return CompareResult::EmptyBase;
    }

    if newest.is_empty() {
        return CompareResult::EmptyNewest;
    }

    if base.eq(newest) {
        return CompareResult::Same;
    }

    CompareResult::Different
}

pub async fn fetch_remote_page(url: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response: Response = client.get(url)
        .timeout(core::time::Duration::from_secs(20))
        .send()
        .await?;
    let page_text = response.text().await?;
    Ok(page_text)
}

pub fn load_configuration() -> Result<AppConfig, ron::error::Error> {
    let path: String = "conf/config.ron".to_owned();
    let file: File = File::open(path)
        .expect("Cannot open the configuration file, check if conf/config.ron exists.");
    match from_reader(file) {
        Ok(config) => Ok(config),
        Err(err) => Err(err.into()),
    }
}

pub fn ensure_store_exists(config: &AppConfig) -> () {
    let path: PathBuf = PathBuf::from(config.relative_store_path.clone());
    if ! path.exists() {
        std::fs::create_dir_all(path);
    }
}

pub enum WriteType {
    Base,
    Comparison,
}

pub fn write_page_to_file(app_config: &AppConfig,
                          page_content: String,
                          write_type: WriteType,
                          overwrite: bool) -> Result<(), Box<dyn Error>> {
    if write_type == Base && !overwrite {
        return Err(Box::new(BaseOverwriteError::new("Trying to overwrite existing base without the overwrite toggle.")));
    }

    let file_name: String = match write_type {
        WriteType::Base => BASE_FILENAME.to_owned(),
        WriteType::Comparison => COMPARISON_FILENAME.to_owned(),
    };

    let path: PathBuf = PathBuf::from(format!("{}/{}", app_config.relative_store_path.clone(), file_name));
    let mut file: File = File::open(path).expect("Could not open the comparison store.");
    file.write_all(page_content.as_bytes())?;

    Ok(())
}