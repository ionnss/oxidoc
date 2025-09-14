use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentElement {
    // Document metadata (from any document type)
    Title { text: String },
    Description { text: String },
    Keywords { text: String },
    Author { text: String },
    Language { text: String },
    
    // Content elements (generic for any document type)
    Heading { level: u8, text: String },
    Paragraph { text: String },
    Blockquote { text: String },
    List { items: Vec<String>, ordered: bool },
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    Code { code: String, language: Option<String>, inline: bool },
    Link { text: String, url: String },
    Image { alt: String, url: Option<String> },
}