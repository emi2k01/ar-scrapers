use anyhow::{Context, Result};
use futures::{stream, StreamExt};
use once_cell::sync::Lazy;
use reqwest::Client;
use scrap::{Html, Selector};

const DEFAULT_USER_AGENT: &'static str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:83.0) Gecko/20100101 Firefox/83.0";

const BROWSE_URL: &'static str = "https://monoschinos2.com/animes";

/// Fetches the pages from given urls and returns a `Vec` containing the url and the body of the request
pub async fn fetch_pages(urls: Vec<String>) -> Vec<(String, String)> {
    let client = setup_client();

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
            Err(e) => {
                error!("{}", e);
                None
            }
        })
        .collect();

    res
}

/// Fetches the browse pages and returns a `Vec` containing the url and the body of the request
pub async fn fetch_browse_pages() -> Vec<(String, String)> {
    let client = setup_client();

    let number_of_pages = number_of_browse_pages(&client).await.unwrap();

    let browse_urls = build_browse_urls(number_of_pages);

    fetch_pages(browse_urls).await
}

/// Selects all anchors in `doc` matched by the given `anchors_selector` and
/// fetches all the urls in the attr attribute.
///
/// Returns a `Vec` containing the url and the body of the request
pub async fn fetch_pages_from_anchors(body: &str, anchors_selector: &str) -> Vec<(String, String)> {
    let urls = {
        let doc = Html::parse_document(&body);
        let sel = Selector::parse(anchors_selector).unwrap();
        let anchors = doc.select(&sel);

        let mut urls = Vec::with_capacity(30);
        for anchor in anchors {
            let href = anchor.value().attr("href").unwrap().to_string();
            urls.push(href);
        }
        urls
    };

    fetch_pages(urls).await
}

fn setup_client() -> Client {
    static CLIENT: Lazy<Client> = once_cell::sync::Lazy::new(|| {
        let user_agent = dotenv::var("AR_USER_AGENT").unwrap_or(DEFAULT_USER_AGENT.to_string());
        reqwest::ClientBuilder::new()
            .user_agent(&user_agent)
            .build()
            .unwrap()
    });

    CLIENT.clone()
}

fn build_browse_urls(num_pages: u32) -> Vec<String> {
    let mut urls = Vec::with_capacity(num_pages as usize);

    for i in 1..=num_pages {
        urls.push(format!("{Url}?page={page}", Url = BROWSE_URL, page = i));
    }

    urls
}

async fn number_of_browse_pages(client: &Client) -> Result<u32> {
    info!(
        "getting the total number of browse pages from {}",
        BROWSE_URL
    );

    let browse_page_html = client
        .get(BROWSE_URL)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let browse_doc = Html::parse_document(&browse_page_html);

    let last_page_sel = Selector::parse("li.page-item:nth-last-child(2) > a.page-link")
        .map_err(|e| anyhow::format_err!("could not parse {:#?}", e))?;

    let last_page_a = browse_doc
        .select(&last_page_sel)
        .next()
        .ok_or(format_err!("could not find the last page button element"))?;

    let last_page_text = last_page_a
        .text()
        .next()
        .ok_or(format_err!("last page button element doesn't have text"))?;

    let last_page = last_page_text.parse()?;

    Ok(last_page)
}
