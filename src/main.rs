// main.rs

use oxidoc::parsers::html_parser::parse_html_file;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!(
        "\n\x1b[38;5;208m
░█▀█░█░█░▀█▀░█▀▄░█▀█░█▀▀
░█░█░▄▀▄░░█░░█░█░█░█░█░░
░▀▀▀░▀░▀░▀▀▀░▀▀▀░▀▀▀░▀▀▀

Rust-powered document distillation 🦀
\x1b[0m"
    );


    // Path to your test HTML file
    let html_file_path = Path::new("test_files/sample.html");
    
    // Parse the HTML file - all output comes from the parser function
    let _elements = parse_html_file(html_file_path)?;
    
    Ok(())
}