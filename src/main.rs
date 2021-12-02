mod config;
mod errors;
mod models;
#[cfg(test)]
mod tests;
mod tools;

use config::{get_relative_pathbuf_to, FileType};
use errors::FileReadError;
use pushover_rs::{Message, send_pushover_request};
use tools::{read_file_content, compare_pages, CompareResult};

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

        // Overwrite must only be run once
        overwrite_base = false;

        // Now we should have at least the base page in a file, possibly the comparison as well
        // If the comparison doesn't exist yet, don't run a comparison (duh)
        let base_read_result: Result<String, FileReadError> = read_file_content(&config, WriteType::Base);
        let cmp_read_result: Result<String, FileReadError> = read_file_content(&config, WriteType::Comparison);

        if base_read_result.is_ok() && cmp_read_result.is_ok() {
            // Compare the base and the comparison files contents
            let base_content: String = base_read_result.unwrap();
            let cmp_content: String = cmp_read_result.unwrap();
            let result: CompareResult = compare_pages(&base_content, &cmp_content);
            println!("{}", result);

            // Force overwriting the base if it's empty (which is not normal)
            if result == CompareResult::EmptyBase {
                overwrite_base = true;
            }
            
            // If a change is detected, send a push notification
            if result == CompareResult::Different {
                let push_notification: Message = pushover_rs::MessageBuilder::new(
                    &config.pushover.user_key, 
                    &config.pushover.app_token, 
                    &config.push_message
                )
                .build();
                send_pushover_request(push_notification).await?;
            }
        }

        // Cooldown take a coffee/tea
        tokio::time::sleep(duration).await;
    }
}
