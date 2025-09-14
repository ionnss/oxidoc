// jsonl_converter.rs


// src/converters/jsonl_converter.rs
use crate::parsers::documents_types::DocumentElement;
use crate::converters::converter_types::{TrainingRecord, RecordMetadata};


pub fn export_to_jsonl(
    elements: &[DocumentElement], 
    source_file: &str,
    document_type: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let mut jsonl_lines = Vec::new();
    
    for element in elements {
        let record = match element {
            DocumentElement::Title { text } => {
                TrainingRecord {
                    text: text.clone(),
                    element_type: "title".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Description { text } => {
                TrainingRecord {
                    text: text.clone(),
                    element_type: "description".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Heading { level, text } => {
                TrainingRecord {
                    text: text.clone(),
                    element_type: format!("heading_{}", level),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Paragraph { text } => {
                TrainingRecord {
                    text: text.clone(),
                    element_type: "paragraph".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Blockquote { text } => {
                TrainingRecord {
                    text: text.clone(),
                    element_type: "blockquote".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::List { items, ordered } => {
                let list_text = items.join(" | ");
                TrainingRecord {
                    text: list_text.clone(),
                    element_type: if *ordered { "ordered_list".to_string() } else { "unordered_list".to_string() },
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: list_text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Code { code, language, inline } => {
                TrainingRecord {
                    text: code.clone(),
                    element_type: if *inline { "inline_code".to_string() } else { "code_block".to_string() },
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: code.len(),
                        language: language.clone(),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Link { text, url } => {
                let link_text = format!("{} -> {}", text, url);
                TrainingRecord {
                    text: link_text.clone(),
                    element_type: "link".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: link_text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Image { alt, url } => {
                let image_text = if let Some(url) = url {
                    format!("{} ({})", alt, url)
                } else {
                    alt.clone()
                };
                TrainingRecord {
                    text: image_text.clone(),
                    element_type: "image".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: image_text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Table { headers, rows } => {
                let mut table_text = format!("Headers: {} | ", headers.join(", "));
                for row in rows {
                    table_text.push_str(&format!("Row: {} | ", row.join(", ")));
                }
                TrainingRecord {
                    text: table_text.clone(),
                    element_type: "table".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: table_text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Keywords { text } => {
                TrainingRecord {
                    text: text.clone(),
                    element_type: "keywords".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Author { text } => {
                TrainingRecord {
                    text: text.clone(),
                    element_type: "author".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: text.len(),
                        language: None,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
            DocumentElement::Language { text } => {
                TrainingRecord {
                    text: text.clone(),
                    element_type: "language".to_string(),
                    metadata: RecordMetadata {
                        source_file: source_file.to_string(),
                        document_type: document_type.to_string(),
                        content_length: text.len(),
                        language: Some(text.clone()),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    },
                }
            },
        };
        
        let json_line = serde_json::to_string(&record)?;
        jsonl_lines.push(json_line);
    }
    
    Ok(jsonl_lines.join("\n"))
}
    