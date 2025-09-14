// txt_converter.rs

use crate::parsers::documents_types::DocumentElement;

pub fn export_to_txt(
    elements: &[DocumentElement], 
    source_file: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let mut txt_lines = Vec::new();
    
    // Add header
    txt_lines.push(format!("# Document: {}", source_file));
    txt_lines.push("".to_string()); // Empty line
    
    for element in elements {
        let txt_line = match element {
            DocumentElement::HtmldocumentTitle { text } => {
                format!("TITLE: {}", text)
            },
            DocumentElement::HtmldocumentDescription { text } => {
                format!("DESCRIPTION: {}", text)
            },
            DocumentElement::HtmlHeading { level, text } => {
                let prefix = "#".repeat(*level as usize);
                format!("{} {}", prefix, text)
            },
            DocumentElement::HtmlParagraph { text } => {
                text.clone()
            },
            DocumentElement::HtmlBlockquote { text } => {
                format!("> {}", text)
            },
            DocumentElement::HtmlList { items, ordered } => {
                if *ordered {
                    items.iter().enumerate()
                        .map(|(i, item)| format!("{}. {}", i + 1, item))
                        .collect::<Vec<String>>()
                        .join("\n")
                } else {
                    items.iter()
                        .map(|item| format!("- {}", item))
                        .collect::<Vec<String>>()
                        .join("\n")
                }
            },
            DocumentElement::HtmlCode { code, language, inline } => {
                if *inline {
                    format!("`{}`", code)
                } else {
                    let lang = language.as_ref().map(|l| l.as_str()).unwrap_or("");
                    format!("```{}\n{}\n```", lang, code)
                }
            },
            DocumentElement::HtmlLink { text, url } => {
                format!("[{}]({})", text, url)
            },
            DocumentElement::HtmlImageDescription { text } => {
                format!("[IMAGE: {}]", text)
            },
            DocumentElement::HtmlTable { headers, rows } => {
                let mut table_text = Vec::new();
                
                // Add headers
                table_text.push(format!("| {} |", headers.join(" | ")));
                table_text.push(format!("|{}|", "---|".repeat(headers.len())));
                
                // Add rows
                for row in rows {
                    table_text.push(format!("| {} |", row.join(" | ")));
                }
                
                table_text.join("\n")
            },
            DocumentElement::HtmldocumentKeywords { text } => {
                format!("KEYWORDS: {}", text)
            },
            DocumentElement::HtmldocumentAuthor { text } => {
                format!("AUTHOR: {}", text)
            },
            DocumentElement::HtmldocumentLanguage { text } => {
                format!("LANGUAGE: {}", text)
            },
        };
        
        txt_lines.push(txt_line);
        txt_lines.push("".to_string()); // Add empty line after each element
    }
    
    Ok(txt_lines.join("\n"))
}
