// main.rs

use oxidoc::parsers::html_parser::parse_html_file;
use oxidoc::parsers::pdf_parser::parse_pdf_file;
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

    // Test HTML parsing
    println!("\n🔍 Testing HTML Parser...");
    let html_file_path = Path::new("test_files/sample.html");
    let html_elements = parse_html_file(html_file_path)?;
    
    // Export HTML to JSONL
    let html_jsonl_content = export_to_jsonl(&html_elements, "sample.html", "html")?;
    let html_jsonl_path = save_to_downloads_jsonl(&html_jsonl_content, "oxidoc_html_sample")?;
    
    // Export HTML to TXT
    let html_txt_content = export_to_txt(&html_elements, "sample.html")?;
    let html_txt_path = save_to_downloads_txt(&html_txt_content, "oxidoc_html_sample")?;
    
    println!("\n✅ HTML Files saved:");
    println!("   📄 JSONL: {}", html_jsonl_path);
    println!("   📝 TXT: {}", html_txt_path);
    
    // Test PDF parsing
    println!("\n🔍 Testing PDF Parser...");
    let pdf_file_path = Path::new("test_files/pdf_test.pdf");
    let pdf_elements = parse_pdf_file(pdf_file_path)?;
    
    // Export PDF to JSONL
    let pdf_jsonl_content = export_to_jsonl(&pdf_elements, "pdf_test.pdf", "pdf")?;
    let pdf_jsonl_path = save_to_downloads_jsonl(&pdf_jsonl_content, "oxidoc_pdf_sample")?;
    
    // Export PDF to TXT
    let pdf_txt_content = export_to_txt(&pdf_elements, "pdf_test.pdf")?;
    let pdf_txt_path = save_to_downloads_txt(&pdf_txt_content, "oxidoc_pdf_sample")?;
    
    println!("\n✅ PDF Files saved:");
    println!("   📄 JSONL: {}", pdf_jsonl_path);
    println!("   📝 TXT: {}", pdf_txt_path);
    
    // Show previews
    println!("\n📄 HTML JSONL Preview (first 3 lines):");
    let html_lines: Vec<&str> = html_jsonl_content.lines().take(3).collect();
    for (i, line) in html_lines.iter().enumerate() {
        println!("{}. {}", i + 1, line);
    }
    
    println!("\n📄 PDF JSONL Preview (first 3 lines):");
    let pdf_lines: Vec<&str> = pdf_jsonl_content.lines().take(3).collect();
    for (i, line) in pdf_lines.iter().enumerate() {
        println!("{}. {}", i + 1, line);
    }
    
    println!("\n📝 PDF TXT Preview (first 10 lines):");
    let pdf_txt_lines: Vec<&str> = pdf_txt_content.lines().take(10).collect();
    for (i, line) in pdf_txt_lines.iter().enumerate() {
        println!("{}. {}", i + 1, line);
    }
    
    Ok(())
}