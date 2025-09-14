// html.rs

use scraper::{Html, Selector};
use std::fs;
use std::path::Path;
use crate::parsers::documents_types::DocumentElement;




// Main parsing function
pub fn parse_html_file(path: &Path) -> Result<Vec<DocumentElement>, Box<dyn std::error::Error>> {
    println!("\nOxidoc HTML Parser");
    println!("Parsing HTML file: {:?}", path);
    
    // Check if file exists
    if !path.exists() {
        eprintln!("❌ HTML file not found: {:?}", path);
        return Ok(vec![]);
    }
    
    // Read the HTML file
    let html_content = fs::read_to_string(path)?;
    
    // Parse the HTML document
    let document = Html::parse_document(&html_content);
    
    // Extract metadata first
    let mut elements = extract_metadata(&document);
    
    // Extract all content elements
    elements.extend(extract_elements(&document));
    
    // Display detailed results
    display_parsing_results(&elements);
    
    Ok(elements)
}

// Display detailed parsing results
fn display_parsing_results(elements: &[DocumentElement]) {
    println!("\nParsing Results:");
    
    let mut metadata_count = 0;
    let mut headings_count = 0;
    let mut paragraphs_count = 0;
    let mut lists_count = 0;
    let mut tables_count = 0;
    let mut code_count = 0;
    let mut links_count = 0;
    let mut images_count = 0;
    let mut blockquotes_count = 0;
    
    for element in elements {
        match element {
            DocumentElement::HtmldocumentTitle { .. } |
            DocumentElement::HtmldocumentDescription { .. } |
            DocumentElement::HtmldocumentKeywords { .. } |
            DocumentElement::HtmldocumentAuthor { .. } |
            DocumentElement::HtmldocumentLanguage { .. } => metadata_count += 1,
            DocumentElement::HtmlHeading { .. } => headings_count += 1,
            DocumentElement::HtmlParagraph { .. } => paragraphs_count += 1,
            DocumentElement::HtmlList { .. } => lists_count += 1,
            DocumentElement::HtmlTable { .. } => tables_count += 1,
            DocumentElement::HtmlCode { .. } => code_count += 1,
            DocumentElement::HtmlLink { .. } => links_count += 1,
            DocumentElement::HtmlImageDescription { .. } => images_count += 1,
            DocumentElement::HtmlBlockquote { .. } => blockquotes_count += 1,
        }
    }
    
    println!("Metadata: {}", metadata_count);
    println!("Headings: {}", headings_count);
    println!("Paragraphs: {}", paragraphs_count);
    println!("Lists: {}", lists_count);
    println!("Tables: {}", tables_count);
    println!("Code blocks: {}", code_count);
    println!("Links: {}", links_count);
    println!("Images: {}", images_count);
    println!("Blockquotes: {}", blockquotes_count);
    println!("Total elements: {}", elements.len());
    
}





//-------------------------------------------------
// Block of Functions related to Metadata elements
//-------------------------------------------------

// Main metadata extraction function
fn extract_metadata(document: &Html) -> Vec<DocumentElement> {
    let mut metadata = Vec::new();
    
    // Extract title
    if let Some(title) = extract_title(document) {
        metadata.push(DocumentElement::HtmldocumentTitle { text: title });
    }
    
    // Extract description
    if let Some(description) = extract_meta_description(document) {
        metadata.push(DocumentElement::HtmldocumentDescription { text: description });
    }
    
    // Extract keywords
    if let Some(keywords) = extract_meta_keywords(document) {
        metadata.push(DocumentElement::HtmldocumentKeywords { text: keywords });
    }
    
    // Extract author
    if let Some(author) = extract_meta_author(document) {
        metadata.push(DocumentElement::HtmldocumentAuthor { text: author });
    }
    
    // Extract language
    if let Some(language) = extract_document_language(document) {
        metadata.push(DocumentElement::HtmldocumentLanguage { text: language });
    }
    
    metadata
}

// Extract title from <title> tag
fn extract_title(document: &Html) -> Option<String> {
    let title_selector = Selector::parse("title").unwrap();
    document.select(&title_selector)
        .next()
        .map(|element| element.text().collect::<String>().trim().to_string())
        .filter(|title| !title.is_empty())
}

// Extract meta description
fn extract_meta_description(document: &Html) -> Option<String> {
    let meta_selector = Selector::parse("meta[name='description']").unwrap();
    document.select(&meta_selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .map(|content| content.to_string())
        .filter(|content| !content.trim().is_empty())
}

// Extract meta keywords
fn extract_meta_keywords(document: &Html) -> Option<String> {
    let meta_selector = Selector::parse("meta[name='keywords']").unwrap();
    document.select(&meta_selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .map(|content| content.to_string())
        .filter(|content| !content.trim().is_empty())
}

// Extract meta author
fn extract_meta_author(document: &Html) -> Option<String> {
    let meta_selector = Selector::parse("meta[name='author']").unwrap();
    document.select(&meta_selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .map(|content| content.to_string())
        .filter(|content| !content.trim().is_empty())
}

// Extract document language
fn extract_document_language(document: &Html) -> Option<String> {
    // Try <html lang="..."> first
    let html_selector = Selector::parse("html").unwrap();
    if let Some(html_element) = document.select(&html_selector).next() {
        if let Some(lang) = html_element.value().attr("lang") {
            return Some(lang.to_string());
        }
    }
    
    // Fallback to meta http-equiv
    let meta_selector = Selector::parse("meta[http-equiv='content-language']").unwrap();
    document.select(&meta_selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .map(|content| content.to_string())
        .filter(|content| !content.trim().is_empty())
}


//---------------------------------------------
// Block of Functions related to HTML elements
//---------------------------------------------

// Main element extraction function
pub fn extract_elements(htmldocument: &Html) -> Vec<DocumentElement> {
    let mut elements = Vec::new();

    // Extract different types of elements
    elements.extend(extract_headings(htmldocument));
    elements.extend(extract_paragraphs(htmldocument));
    elements.extend(extract_blockquotes(htmldocument));
    elements.extend(extract_lists(htmldocument));
    elements.extend(extract_tables(htmldocument));
    elements.extend(extract_code_blocks(htmldocument));
    elements.extend(extract_inline_codes(htmldocument));
    elements.extend(extract_links(htmldocument));
    elements.extend(extract_image_descriptions(htmldocument));

    // Return the elements
    elements
}

// Extract headings (h1, h2, h3, h4, h5, h6)
pub fn extract_headings(htmldocument: &Html) -> Vec<DocumentElement> {
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
            headings.push(DocumentElement::HtmlHeading { level, text });
        }
    }
    // Return the headings
    headings
}

pub fn extract_paragraphs(htmldocument:&Html) -> Vec<DocumentElement> {
    let mut paragraphs= Vec::new();
    let paragraph_selector = Selector::parse("p").unwrap();

    for paragraph in htmldocument.select(&paragraph_selector) {
        let text = paragraph.text().collect::<String>().trim().to_string();
        if !text.is_empty() {
            paragraphs.push(DocumentElement::HtmlParagraph { text });
        }
    }
    paragraphs
}

pub fn extract_blockquotes(htmldocument: &Html) -> Vec<DocumentElement> {
    let mut blockquotes= Vec::new();
    let blockquote_selector = Selector::parse("blockquote").unwrap();

    for blockquote in htmldocument.select(&blockquote_selector) {
        let text = blockquote.text().collect::<String>().trim().to_string();
        if !text.is_empty() {
            blockquotes.push(DocumentElement::HtmlBlockquote { text });
        }
    }
    blockquotes
}

pub fn extract_lists(htmldocument: &Html) -> Vec<DocumentElement> {
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
            lists.push(DocumentElement::HtmlList { items, ordered: false });
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
            lists.push(DocumentElement::HtmlList { items, ordered: true });
        }
    }
    
    lists
}

// Extract tables
fn extract_tables(htmldocument: &Html) -> Vec<DocumentElement> {
    let mut tables = Vec::new();
    let table_selector = Selector::parse("table").unwrap();
    
    for table_element in htmldocument.select(&table_selector) {
        // Extract headers
        let th_selector = Selector::parse("th").unwrap();
        let headers: Vec<String> = table_element.select(&th_selector)
            .map(|th| th.text().collect::<String>().trim().to_string())
            .collect();
        
        // Extract rows
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let mut rows = Vec::new();
        
        for row in table_element.select(&tr_selector) {
            let cells: Vec<String> = row.select(&td_selector)
                .map(|td| td.text().collect::<String>().trim().to_string())
                .collect();
            
            if !cells.is_empty() {
                rows.push(cells);
            }
        }
        
        if !headers.is_empty() || !rows.is_empty() {
            tables.push(DocumentElement::HtmlTable { headers, rows });
        }
    }
    
    tables
}

// Extract code blocks
fn extract_code_blocks(htmldocument: &Html) -> Vec<DocumentElement> {
    let mut code_blocks = Vec::new();
    let pre_selector = Selector::parse("pre").unwrap();
    
    for pre_element in htmldocument.select(&pre_selector) {
        let code_selector = Selector::parse("code").unwrap();
        let code_element = pre_element.select(&code_selector).next();
        
        if let Some(code_elem) = code_element {
            let code = code_elem.text().collect::<String>();
            let language = extract_code_language(&code_elem);
            
            code_blocks.push(DocumentElement::HtmlCode { 
                code, 
                language, 
                inline: false 
            });
        } else {
            // No <code> inside <pre>, just use the pre content
            let code = pre_element.text().collect::<String>();
            code_blocks.push(DocumentElement::HtmlCode { 
                code, 
                language: None, 
                inline: false 
            });
        }
    }
    
    code_blocks
}

// Extract inline code
fn extract_inline_codes(htmldocument: &Html) -> Vec<DocumentElement> {
    let mut inline_codes = Vec::new();
    let code_selector = Selector::parse("code").unwrap();
    
    for element in htmldocument.select(&code_selector) {
        // Skip if it's inside a <pre> (already handled by code blocks)
        if element.parent().map_or(false, |parent| {
            parent.value().is_element() && parent.value().as_element().unwrap().name() == "pre"
        }) {
            continue;
        }
        
        let code = element.text().collect::<String>().trim().to_string();
        
        if !code.is_empty() {
            inline_codes.push(DocumentElement::HtmlCode { 
                code, 
                language: None, 
                inline: true 
            });
        }
    }
    
    inline_codes
}

// Extract links
fn extract_links(htmldocument: &Html) -> Vec<DocumentElement> {
    let mut links = Vec::new();
    let link_selector = Selector::parse("a[href]").unwrap();
    
    for element in htmldocument.select(&link_selector) {
        let url = element.value().attr("href").unwrap_or("").to_string();
        let text = element.text().collect::<String>().trim().to_string();
        
        if !url.is_empty() && !text.is_empty() {
            links.push(DocumentElement::HtmlLink { text, url });
        }
    }
    
    links
}

// Extract image descriptions (alt text and captions)
fn extract_image_descriptions(htmldocument: &Html) -> Vec<DocumentElement> {
    let mut image_descriptions = Vec::new();
    let img_selector = Selector::parse("img").unwrap();
    
    for element in htmldocument.select(&img_selector) {
        // Extract alt text
        if let Some(alt) = element.value().attr("alt") {
            if !alt.trim().is_empty() && alt != "image" && alt != "photo" {
                image_descriptions.push(DocumentElement::HtmlImageDescription { 
                    text: alt.to_string() 
                });
            }
        }
        
        // Try to find caption in parent figure
        if let Some(parent) = element.parent() {
            if parent.value().is_element() {
                // Find all figcaptions and check if any are children of this parent
                let figcaption_selector = Selector::parse("figcaption").unwrap();
                for caption in htmldocument.select(&figcaption_selector) {
                    if caption.parent().map_or(false, |p| p == parent) {
                        let caption_text = caption.text().collect::<String>().trim().to_string();
                        if !caption_text.is_empty() {
                            image_descriptions.push(DocumentElement::HtmlImageDescription { 
                                text: caption_text 
                            });
                        }
                        break; // Found the caption for this image
                    }
                }
            }
        }
    }
    
    image_descriptions
}

// Helper function to extract code language
fn extract_code_language(element: &scraper::ElementRef) -> Option<String> {
    element.value().attr("class")
        .and_then(|class| {
            // Look for language-* classes
            if class.starts_with("language-") {
                Some(class.strip_prefix("language-").unwrap().to_string())
            } else {
                None
            }
        })
}
