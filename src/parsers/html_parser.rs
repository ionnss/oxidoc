// html.rs

use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};

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


// Main parsing function
//
// TODO: Add function 






//-------------------------------------------------
// Block of Functions related to Metadata elements
//-------------------------------------------------

// Main metadata extraction function
//pub fn extract_metadata




//---------------------------------------------
// Block of Functions related to HTML elements
//---------------------------------------------

// Main element extraction function
pub fn extract_elements(htmldocument: &Html) -> Vec<HtmlElement> {
    let mut elements = Vec::new();

    // Extract different types of elements
    elements.extend(extract_headings(htmldocument));
    elements.extend(extract_paragraphs(htmldocument));
    elements.extend(extract_blockquotes(htmldocument));
    // TODO: Add more element extraction functions here

    // Return the elements
    elements
}

// Extract headings (h1, h2, h3, h4, h5, h6)
pub fn extract_headings(htmldocument: &Html) -> Vec<HtmlElement> {
    // Vector to store the headings
    let mut headings = Vec::new();
    // Selector to find the headings
    let heading_selector = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();

    // Iterate over the headings
    for heading in htmldocument.select(&heading_selector) {
        // Extract heading level from tag name
        let level = heading.value().name().chars().last().unwrap_or('1').to_digit(10).unwrap_or(1) as u8;
        // Extract heading text
        let text = heading.text().collect::<String>().trim().to_string();

        // If the heading text is not empty, add it to the vector
        if !text.is_empty() {
            headings.push(HtmlElement::Heading { level, text });
        }
    }
    // Return the headings
    headings
}

pub fn extract_paragraphs(htmldocument:&Html) -> Vec<HtmlElement> {
    let mut paragraphs= Vec::new();
    let paragraph_selector = Selector::parse("p").unwrap();

    for paragraph in htmldocument.select(&paragraph_selector) {
        let text = paragraph.text().collect::<String>().trim().to_string();
        if !text.is_empty() {
            paragraphs.push(HtmlElement::Paragraph { text });
        }
    }
    paragraphs
}

pub fn extract_blockquotes(htmldocument: &Html) -> Vec<HtmlElement> {
    let mut blockquotes= Vec::new();
    let blockquote_selector = Selector::parse("blockquote").unwrap();

    for blockquote in htmldocument.select(&blockquote_selector) {
        let text = blockquote.text().collect::<String>().trim().to_string();
        if !text.is_empty() {
            blockquotes.push(HtmlElement::Blockquote { text });
        }
    }
    blockquotes
}

pub fn extract_lists(htmldocument: &Html) -> Vec<HtmlElement> {
    let mut lists = Vec::new();

    // Extract unordered lists
    let ul_selector = Selector::parse("ul").unwrap();
    for ul_element in htmldocument.select(&ul_selector) {
        let li_selector = Selector::parse("li").unwrap();
        let items: Vec<String> = ul_element.select(&li_selector)
            .map(|li| li.text().collect::<String>().trim().to_string())
            .filter(|item| !item.is_empty())
            .collect();
        
        if !items.is_empty() {
            lists.push(HtmlElement::List { items, ordered: false });
        }
    }
    
    // Extract ordered lists
    let ol_selector = Selector::parse("ol").unwrap();
    for ol_element in htmldocument.select(&ol_selector) {
        let li_selector = Selector::parse("li").unwrap();
        let items: Vec<String> = ol_element.select(&li_selector)
            .map(|li| li.text().collect::<String>().trim().to_string())
            .filter(|item| !item.is_empty())
            .collect();
        
        if !items.is_empty() {
            lists.push(HtmlElement::List { items, ordered: true });
        }
    }
    
    lists
}


