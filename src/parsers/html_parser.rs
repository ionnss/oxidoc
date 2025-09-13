// html.rs

use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub enum HtmlElement {
    // Document metadata (from <head>)
    DocumentTitle { text: String },
    DocumentDescription { text: String },
    DocumentKeywords { text: String },
    DocumentAuthor { text: String },
    DocumentLanguage { text: String },
    
    // Content elements (from <body>)
    Heading { level: u8, text: String},
    Paragraph { text: String},
    Blockquote { text: String},
    List { items: Vec<String>, ordered: bool},
    Table { headers: Vec<String>, rows: Vec<Vec<String>>},
    Code { code: String, language: Option<String>, inline: bool},
    Link { text: String, url: String},
    ImageDescription { text: String},
}

//pub fn parse_html_file

//pub fn extract_metadata

//pub fn extract_elements

/// Functions to Enums for Metadata elements
/// 
/// 

/// Enums for Enums for HTML elements
/// 
/// 



