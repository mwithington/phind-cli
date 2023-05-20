use prettytable::{Cell, Row, Table};
use reqwest::Error as ReqwestError;
use scraper::{Html, Selector};
use std::env;
use html_entities::decode_html_entities;

#[tokio::main]
async fn main() -> Result<(), ReqwestError> {
    // Read the search query from command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut query: Option<String> = None;

    for (index, arg) in args.iter().enumerate() {
        if arg == "--query" || arg == "-q" {
            if let Some(q) = args.get(index + 1) {
                query = Some(decode_html_entities(q).unwrap());
                break;
            }
        }
    }

    if query.is_none() {
        eprintln!("Usage: website-scraper --query <search query>");
        return Ok(());
    }

    let query: String = query.unwrap();

    // Construct the search URL
    let base_url = "https://phind.com/";
    let search_url: String = format!("{}?q={}", base_url, query);

    // Send an HTTP GET request to the URL
    let body: String = reqwest::get(search_url).await?.text().await?;
    println!("Body of document {}", body);

    // Parse the HTML body using the scraper crate
    let document: Html = Html::parse_document(&body);

    // Define an XPath expression to extract specific elements
    let xpath_expr: &str = "//div[@name='answer-0']";

    // Create a table to display the extracted data
    let mut table: Table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Heading"),
        Cell::new("Text"),
    ]));

    // Find the elements using the XPath expression
    let selector: Selector = Selector::parse(xpath_expr).unwrap();
    if let Some(element) = document.select(&selector).next() {
        let heading: &str = "Answer";
        let text: String = element.text().collect::<String>();
        table.add_row(Row::new(vec![
            Cell::new(heading),
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
