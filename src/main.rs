use std::fs;

#[tokio::main]
async fn main() {
    let f1 = download("https://ssr1.scrape.center/page/1");
    let f2 = download("https://ssr1.scrape.center/page/2");
    let f3 = download("https://ssr1.scrape.center/page/3");
    let f4 = download("https://ssr1.scrape.center/page/4");
    let f5 = download("https://ssr1.scrape.center/page/5");

    futures::join!(f1, f2, f3, f4, f5);
}

async fn download(url: &str) {
    let client = reqwest::Client::new();
    let text = client.get(url).send().await.unwrap().text().await.unwrap();

    let suffix = url.split('/').last().unwrap();
    let filename = format!("{}.html", suffix);
    fs::write(filename, &text).unwrap();
}
