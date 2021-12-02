mod config;
mod errors;
mod models;
#[cfg(test)]
mod tests;
mod tools;

use crate::config::{BASE_FILENAME, COMPARISON_FILENAME};
use crate::models::AppConfig;
use crate::tools::{
    ensure_store_exists, fetch_remote_page, load_configuration, write_page_to_file, WriteType,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the configuration from the conf/config.ron file.
    // Bail if an error is detected (configuration is critical, can't work without it)
    let config: AppConfig = load_configuration().unwrap_or_else(|error| {
        eprintln!("Cannot read the config file: {}", error);
        std::process::exit(1);
    });
    // Setup the cooldown timer duration
    let check_interval: u64 = config.timeout_seconds;
    let duration: tokio::time::Duration = tokio::time::Duration::from_secs(check_interval as u64);

    // Preflight
    ensure_store_exists(&config);
    let base_exists: bool =
        PathBuf::from(format!("{}/{}", config.relative_store_path, BASE_FILENAME)).exists();
    // TODO: Implement CLI args parsing
    let mut overwrite_base = false;
    // TODO: Get this from the appconfig and implement its behaviour
    let replace_base = false;

    // main loop
    loop {
        // Get the page content write type (create a base vs create a comparison)
        // If a base already exists (see check above), it will (over)write the comparison file instead.
        // If no base is detected, the type will always be BASE.
        let mut write_type: WriteType = match base_exists {
            true => WriteType::Comparison,
            false => WriteType::Base,
        };

        // Take care of the eventual "overwrite" override
        // TODO: Implement CLI arguments parsing
        if overwrite_base {
            write_type = WriteType::Base;
        }

        // Get the remote page asynchronously, and handle the response
        match fetch_remote_page(config.target.clone()).await {
            Ok(content) => {
                // All good, check if the content is not empty first
                if !content.is_empty() {
                    // Write the page content to a file in the store, or print an error if it failed somehow
                    if let Err(err) = write_page_to_file(
                        &config,
                        content,
                        write_type,
                        overwrite_base || replace_base,
                    ) {
                        eprintln!("Warn: Could not write the page content to file: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error trying to retrieve the remote page: {}", err);
            }
        }

        // TODO: Compare the base and the comparison files contents

        // TODO: If a change is detected, send a push notification

        // Overwrite must only be run once
        overwrite_base = false;

        // Cooldown take a coffee/tea
        tokio::time::sleep(duration).await;
    }
}
