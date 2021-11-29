use std::error::Error;
use reqwest::Response;

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
