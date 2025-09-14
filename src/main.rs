// main.rs

use oxidoc::parsers::html_parser::{HtmlElement, extract_headings};
use scraper::Html;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Oxidoc HTML Parser!");
    
    // Path to your test HTML file
    let html_file_path = Path::new("test_files/sample.html");
    
    // Check if file exists
    if !html_file_path.exists() {
        eprintln!("❌ HTML file not found: {:?}", html_file_path);
        return Ok(());
    }
    
    // Read the HTML file
    println!("📖 Reading HTML file: {:?}", html_file_path);
    let html_content = fs::read_to_string(html_file_path)?;
    
    // Parse the HTML document
    println!("Parsing HTML document...");
    let document = Html::parse_document(&html_content);
    
    // Extract headings
    println!("📑 Extracting headings...");
    let headings = extract_headings(&document);
    
    // Display results
    println!("\n📊 Results:");
    println!("Found {} headings:", headings.len());
    
    for (i, heading) in headings.iter().enumerate() {
        match heading {
            HtmlElement::Heading { level, text } => {
                println!("  {}. Level {}: {}", i + 1, level, text);
            },
            _ => {} // This shouldn't happen, but just in case
        }
    }
    
    // Also show the raw HTML content for debugging
    println!("\n🔍 Raw HTML content (first 500 chars):");
    println!("{}", &html_content.chars().take(500).collect::<String>());
    
    Ok(())
}