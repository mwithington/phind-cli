use prettytable::{Cell, Row, Table};
use reqwest::Error;
use scraper::{Html, Selector};
use std::env;
use html_entities::decode_html_entities;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Read the website URL from command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut url = None;

    for (index, arg) in args.iter().enumerate() {
        if arg == "--site" || arg == "-s" {
            if let Some(site) = args.get(index + 1) {
                url = Some(decode_html_entities(site).unwrap());
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

    // Define an XPath expression to extract specific elements
    let xpath_expr = "//div[@name='answer-i']";

    // Create a table to display the extracted data
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Heading"),
        Cell::new("Text"),
    ]));

    // Find the elements using the XPath expression
    let selector = Selector::parse(xpath_expr).unwrap();
    if let Some(element) = document.select(&selector).next() {
        let heading = "Answer";
        let text = element.text().collect::<String>();
        table.add_row(Row::new(vec![
            Cell::new(&heading),
            Cell::new(&text),
        ]));
    } else {
        eprintln!("No elements matching the selector were found.");
        return Ok(());
    }

    // Print the table
    table.printstd();

    Ok(())
}
