use std::fs;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let page_urls = [
        "https://ssr1.scrape.center/page/1",
        "https://ssr1.scrape.center/page/2",
        "https://ssr1.scrape.center/page/3",
        "https://ssr1.scrape.center/page/4",
        "https://ssr1.scrape.center/page/5",
    ];

    for page in page_urls {
        download(page).await?
    }

    Ok(())
}

async fn download(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let text = client.get(url).send().await?.text().await?;

    let suffix = url.split('/').last().unwrap();
    let filename = format!("{}.html", suffix);
    fs::write(filename, &text)?;
    Ok(())
}
