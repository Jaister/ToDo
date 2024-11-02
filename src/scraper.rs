use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn my_scraper(url: &str) -> Result<(), Box<dyn Error>> {
    // Fetch and parse the document from the given URL
    let response = get(url)?.text()?;
    let document = Html::parse_document(&response);
    
    // Define selectors for images and links
    let img_selector = Selector::parse("img").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    
    // Collect image sources
    let mut images = Vec::new();
    for node in document.select(&img_selector) {
        if let Some(img_src) = node.value().attr("src") {
            images.push(img_src.to_string());
        } else {
            eprintln!("Warning: Found an image without a src attribute.");
        }
    }

    // Collect link text
    let mut text_content = Vec::new();
    for node in document.select(&link_selector) {
        let link_text = node.text().collect::<Vec<_>>().join(" ");
        text_content.push(link_text);
    }

    // Write images to file
    let mut img_file = File::create("images.txt")?;
    for img in images {
        writeln!(img_file, "{}", img)?;
    }

    // Write links to file
    let mut link_file = File::create("links.txt")?;
    for text in text_content {
        writeln!(link_file, "{}", text)?;
    }

    println!("Scraping completed successfully.");
    Ok(())
}
