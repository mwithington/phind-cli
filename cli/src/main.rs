use reqwest::Error;
use scraper::{Html, Selector};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Read the website URL from command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut url = None;

    for (index, arg) in args.iter().enumerate() {
        if arg == "--site" || arg == "-s" {
            if let Some(site) = args.get(index + 1) {
                url = Some(site.clone());
                break;
            }
        }
    }

    if url.is_none() {
        eprintln!("Usage: website-scraper --site <URL>");
        return Ok(());
    }

    let url = url.unwrap();

    // Send an HTTP GET request to the URL
    let body = reqwest::get(&url).await?.text().await?;

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
