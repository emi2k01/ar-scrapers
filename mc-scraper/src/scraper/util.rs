use adler::adler32_slice;
use anyhow::{Context, Result};
use futures::{stream, StreamExt};
use reqwest::Client;

use crate::page_checksum::{self, PageChecksum};

/// Fetches the given url and returns a tuple containing (url, body)
pub async fn fetch_page(client: &Client, url: String) -> Result<(String, String)> {
    debug!("requesting {}", url);
    let body = client.get(&url).send().await?.text().await?;

    Ok((url, body))
}

/// Fetches the given urls and returns a `Vec` containing the url and the body of the request
pub async fn fetch_pages(client: &Client, urls: Vec<String>) -> Vec<(String, String)> {
    let fut_res = stream::iter(urls)
        .map(|url| {
            let client = client.clone();

            // Spawn a task that might execute parallelly
            tokio::spawn(async move {
                debug!("requesting {}", url);
                let body = client
                    .get(&url)
                    .send()
                    .await
                    .context(format_err!("request to [{}] failed", &url))?
                    .text()
                    .await?;

                Ok((url, body)) as anyhow::Result<_, anyhow::Error>
            })
        })
        // Execute at most 4 futures concurrently
        .buffer_unordered(4)
        .collect::<Vec<_>>()
        .await;

    let res: Vec<_> = fut_res
        .into_iter()
        // Unwrap Result<Result<_>>
        .filter_map(|outer_res| match outer_res {
            Ok(inner_res) => match inner_res {
                Ok(_) => inner_res.ok(),
                Err(e) => {
                    error!("error on request: {}", e);
                    None
                }
            },
            _ => None,
        })
        .collect();

    res
}

/// Returns true if it needed to be updated or inserted
pub fn update_or_insert(url: &str, body: &str, section: &str) -> Result<bool> {
    let checksum = adler32_slice(body.as_bytes()) as i32;

    if let Some(mut stored_page) = page_checksum::select_by_url(section, url) {
        if checksum != stored_page.checksum {
            stored_page.checksum = checksum;
            stored_page.update()?;

            return Ok(true);
        }
    } else {
        let new_page = PageChecksum {
            id: 0,
            checksum,
            section: section.to_string(),
            url: url.to_string(),
        };
        new_page.insert()?;

        return Ok(true);
    }

    return Ok(false);
}
