use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentElement {
    // htmldocument metadata (from <head>)
    HtmldocumentTitle { text: String },
    HtmldocumentDescription { text: String },
    HtmldocumentKeywords { text: String },
    HtmldocumentAuthor { text: String },
    HtmldocumentLanguage { text: String },
    
    // Content elements (from <body>)
    HtmlHeading { level: u8, text: String},
    HtmlParagraph { text: String},
    HtmlBlockquote { text: String},
    HtmlList { items: Vec<String>, ordered: bool},
    HtmlTable { headers: Vec<String>, rows: Vec<Vec<String>>},
    HtmlCode { code: String, language: Option<String>, inline: bool},
    HtmlLink { text: String, url: String},
    HtmlImageDescription { text: String},
}