use std::fs;
use std::path::PathBuf;

pub fn save_to_downloads_jsonl(jsonl_content: &str, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Get Downloads directory
    let home_dir = std::env::var("HOME")?;
    let downloads_path = PathBuf::from(home_dir).join("Downloads");
    
    // Create filename with .jsonl extension if not present
    let filename = if filename.ends_with(".jsonl") {
        filename.to_string()
    } else {
        format!("{}.jsonl", filename)
    };
    
    let file_path = downloads_path.join(&filename);
    
    // Save file
    fs::write(&file_path, jsonl_content)?;
    
    // Return path
    Ok(file_path.to_string_lossy().to_string())
}

pub fn save_to_downloads_txt(txt_content: &str, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Get Downloads directory
    let home_dir = std::env::var("HOME")?;
    let downloads_path = PathBuf::from(home_dir).join("Downloads");
    
    // Create filename with .txt extension if not present
    let filename = if filename.ends_with(".txt") {
        filename.to_string()
    } else {
        format!("{}.txt", filename)
    };
    
    let file_path = downloads_path.join(&filename);
    
    // Save file
    fs::write(&file_path, txt_content)?;
    
    // Return path
    Ok(file_path.to_string_lossy().to_string())
}