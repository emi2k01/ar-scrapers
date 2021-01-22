use std::sync::Arc;

use reqwest::Client;

mod anime;
mod browse;
mod util;

const DEFAULT_USER_AGENT: &'static str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:83.0) Gecko/20100101 Firefox/83.0";

pub async fn scrape() -> anyhow::Result<()> {
    let client = setup_client();

    browse::scrape_browse_pages(&client).await?;

    Ok(())
}

fn setup_client() -> Arc<Client> {
    let user_agent = dotenv::var("AR_USER_AGENT").unwrap_or(DEFAULT_USER_AGENT.to_string());

    let client = reqwest::ClientBuilder::new()
        .user_agent(&user_agent)
        .build()
        .unwrap();

    Arc::new(client)
}
