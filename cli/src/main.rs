use reqwest::Error;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Define the URL of the web page to scrape
    let url = "https://example.com";

    // Send an HTTP GET request to the URL
    let body = reqwest::get(url).await?.text().await?;

    // Parse the HTML body using the scraper crate
    let document = Html::parse_document(&body);

    // Define a CSS selector to extract specific elements
    let selector = Selector::parse("h1, h2, h3").unwrap();

    // Iterate over the matched elements and extract their text
    for element in document.select(&selector) {
        println!("{}", element.text().collect::<String>());
    }

    Ok(())
}
