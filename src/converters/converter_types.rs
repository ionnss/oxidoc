use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TrainingRecord {
    pub text: String,
    pub element_type: String,
    pub metadata: RecordMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct RecordMetadata {
    pub source_file: String,
    pub document_type: String, // "html", "md", "txt", "pdf", "transcripted_audio"...
    pub content_length: usize,
    pub language: Option<String>,
    pub timestamp: String,
}