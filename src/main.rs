// main.rs

use oxidoc::parsers::html_parser::parse_html_file;
use oxidoc::converters::jsonl_converter::export_to_jsonl;
use oxidoc::converters::txt_converter::export_to_txt;
use oxidoc::downloader::download::{save_to_downloads_jsonl, save_to_downloads_txt};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!(
        "\n\x1b[38;5;208m
      ░ ░░░░░░░░░░░░░░░░░░░░░░░░░░ ░ ░  ░   
     ░ ░░░█▀█░█░█░▀█▀░█▀▄░█▀█░█▀▀░░ ░ ░   ░    
    ░ ░░░░█░█░▄▀▄░░█░░█░█░█░█░█░░░░░ ░  ░    ░ 
     ░ ░░░▀▀▀░▀░▀░▀▀▀░▀▀▀░▀▀▀░▀▀▀░░ ░ ░   ░    
      ░ ░░░░░░░░░░░░░░░░░░░░░░░░░░ ░ ░  ░   

      Rust-powered document distillation 🦀
\x1b[0m"
    );

    // Path to your test HTML file
    let html_file_path = Path::new("test_files/sample.html");
    
    // Parse the HTML file
    let elements = parse_html_file(html_file_path)?;
    
    // Export to JSONL
    let jsonl_content = export_to_jsonl(&elements, "sample.html")?;
    let jsonl_path = save_to_downloads_jsonl(&jsonl_content, "oxidoc_sample")?;
    
    // Export to TXT
    let txt_content = export_to_txt(&elements, "sample.html")?;
    let txt_path = save_to_downloads_txt(&txt_content, "oxidoc_sample")?;
    
    println!("\n✅ Files saved:");
    println!("   📄 JSONL: {}", jsonl_path);
    println!("   📝 TXT: {}", txt_path);
    
    println!("\n📄 JSONL Preview (first 3 lines):");
    let lines: Vec<&str> = jsonl_content.lines().take(3).collect();
    for (i, line) in lines.iter().enumerate() {
        println!("{}. {}", i + 1, line);
    }
    
    println!("\n📝 TXT Preview (first 10 lines):");
    let txt_lines: Vec<&str> = txt_content.lines().take(10).collect();
    for (i, line) in txt_lines.iter().enumerate() {
        println!("{}. {}", i + 1, line);
    }
    
    Ok(())
}