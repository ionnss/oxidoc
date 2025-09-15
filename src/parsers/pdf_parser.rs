use pdf_extract;
use std::path::Path;
use crate::parsers::documents_types::DocumentElement;

pub fn parse_pdf_file(path: &Path) -> Result<Vec<DocumentElement>, Box<dyn std::error::Error>> {
    println!("\nOxidoc PDF Parser");
    println!("Parsing PDF file: {:?}", path);

    // Check if file exists
    if !path.exists() {
       println!("❌ PDF file not found: {:?}", path);
       return Ok(vec![]);
    }
    
    // Extract metadata (simple approach for now)
    let mut elements = extract_pdf_metadata(path);
    
    // Extract text from PDF using pdf-extract
    let pdf_content = pdf_extract::extract_text(path)?;

    // Split paragraphs
    let paragraphs: Vec<&str> = pdf_content.split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty() && p.len() > 20)
        .collect();

    for paragraph in paragraphs {
        elements.push(DocumentElement::Paragraph {
            text: paragraph.to_string(),
        });
    }

    // Debug: Uncomment the next line to see PDF content in terminal
    // println!("Extracted text from PDF: {}", pdf_content);

    Ok(elements)
}

pub fn extract_pdf_metadata(path: &Path) -> Vec<DocumentElement> {
    let mut metadata = Vec::new();

    // Simple metadata extraction using filename for now
    // TODO: Add proper PDF metadata extraction later
    if let Some(filename) = path.file_stem() {
        if let Some(title_str) = filename.to_str() {
            metadata.push(DocumentElement::Title { 
                text: title_str.to_string() 
            });
        }
    }

    metadata
}
