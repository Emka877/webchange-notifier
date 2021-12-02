#[cfg(test)]
mod tests;
mod tools;
mod models;
mod errors;
mod config;

use std::path::PathBuf;
use crate::config::{BASE_FILENAME, COMPARISON_FILENAME};
use crate::tools::{ensure_store_exists, fetch_remote_page, load_configuration, write_page_to_file, WriteType};
use crate::models::AppConfig;

#[tokio::main]
async fn main() {
    let config: AppConfig = load_configuration().unwrap_or_else(|error| {
        eprintln!("Cannot read the config file: {}", error);
        std::process::exit(1);
    });
    let check_interval: u64 = config.timeout_seconds;
    let duration: tokio::time::Duration = tokio::time::Duration::from_secs(check_interval as u64);

    // Preflight
    ensure_store_exists(&config);
    let base_exists: bool = PathBuf::from(format!("{}/{}", config.relative_store_path, BASE_FILENAME)).exists();
    // TODO: Get the overwrite toggle from console argument.
    let mut overwrite_base = false;

    loop {
        let mut write_type: WriteType = match base_exists {
            true => WriteType::Comparison,
            false => WriteType::Base,
        };

        // Overwrite override
        if overwrite_base {
            write_type = WriteType::Base;
        }

        tokio::spawn(async {
            let page_content = fetch_remote_page(config.target.clone()).await?;
            if !page_content.is_empty() {
                write_page_to_file(&config, page_content, write_type, false);
            }
        });

        tokio::time::sleep(duration);
        // Overwrite must only be run once
        overwrite_base = false;
    }
}
