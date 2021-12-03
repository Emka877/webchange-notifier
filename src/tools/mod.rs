use crate::config::{get_absolute_pathbuf_to, FileType};
use crate::errors::{BaseOverwriteError, FileReadError};
use crate::models::AppConfig;
use crate::{BASE_FILENAME, COMPARISON_FILENAME};
use reqwest::Response;
use ron::de::from_reader;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

/* Enums */

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CompareResult {
    Same,
    Different,
    EmptyBase,
    EmptyNewest,
}

impl std::fmt::Display for CompareResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: &'static str = match *self {
            CompareResult::Same => "The base and the comparison are the same.",
            CompareResult::Different => "The base and the comparison are different.",
            CompareResult::EmptyBase => "The base does not exist.",
            CompareResult::EmptyNewest => "The comparison does not exist.",
        };
        write!(f, "{}", message)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WriteType {
    Base,
    Comparison,
}

/* Helper functions */

/// Compares a base to the newest version of a page.
// TODO: Find a way to compare page strings faster (eg: for big pages)
// Example: At the first difference, bail and return different.
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

pub fn read_file_content(cfg: &AppConfig, which_file: WriteType) -> Result<String, FileReadError> {
    let path: String = match which_file {
        WriteType::Base => format!("{}/{}", cfg.relative_store_path.clone(), BASE_FILENAME),
        WriteType::Comparison => format!(
            "{}/{}",
            cfg.relative_store_path.clone(),
            COMPARISON_FILENAME
        ),
    };
    let pathbuf: PathBuf = PathBuf::from(path);

    let read_result = File::open(pathbuf);

    if read_result.is_err() {
        let cause = read_result.unwrap_err().to_string();
        return Err(FileReadError::new(&cause));
    }

    let mut content_buf: String = "".to_owned();
    if let Err(err) = read_result.unwrap().read_to_string(&mut content_buf) {
        return Err(FileReadError::new(&err.to_string()));
    }

    Ok(content_buf)
}

pub async fn fetch_remote_page(url: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response: Response = client
        .get(url)
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
    if !path.exists() {
        if let Err(err) = std::fs::create_dir_all(path) {
            eprintln!(
                "Warn: Could not create directories to ensure store's existence: {}",
                err
            );
        }
    }
}

pub fn write_page_to_file(
    app_config: &AppConfig,
    page_content: String,
    write_type: WriteType,
    overwrite: bool,
) -> Result<(), Box<dyn Error>> {
    let base_exists: bool = get_absolute_pathbuf_to(app_config, FileType::WriteType(WriteType::Base)).exists();
    if write_type.clone() == WriteType::Base && base_exists && !overwrite {
        return Err(Box::new(BaseOverwriteError::new(
            "Trying to overwrite existing base without the overwrite toggle.",
        )));
    }

    let path = get_absolute_pathbuf_to(app_config, FileType::WriteType(write_type));
    let exists_already = path.exists();

    if !exists_already {
        File::create(&path).expect(&format!("Could not create unexisting file: {}", path.to_str().unwrap()).to_owned());
    }
    
    let mut file: File = OpenOptions::new().write(true).open(path).expect("Could not open the comparison store.");
    file.write_all(page_content.as_bytes())?;

    Ok(())
}
