# Oxidoc Architecture & Development Plan

## 🎯 Project Vision

**Oxidoc** is a Rust-native library and CLI tool for converting diverse file formats into AI training-ready formats, with a focus on **JSONL** and **plain text** outputs.

## 📊 Target Audience

### Primary Users
- **AI Engineers**: Converting documents to training data
- **Data Scientists**: Preprocessing text datasets
- **Rust Developers**: Integrating document parsing into their projects

### Use Cases
- Convert PDFs/Office docs and other files to JSONL for LLM training
- Batch process document collections
- Extract text from various formats for NLP pipelines
- Integrate document parsing into Rust applications



## 🚀 Development Roadmap

### **Phase 1: Foundation (Current)**
- Basic parsing + JSONL and TXT output

- Parsers
    - [ ] **TXT Parser**: Direct text extraction
    - [ ] **Markdown Parser**: Convert to plain text
    - [ ] **HTML Parser**: Strip tags, preserve text structure

- Output Formats
    - [ ] **Plain Text**: Clean, normalized text
    - [ ] **JSONL**: One document per line with metadata

#### **Phase 1.1: Expand types**
- [ ] DOCX
- [ ] PPTX
- [ ] XLSX
- [ ] PDF

### **Phase 2: Structure Intelligence**
- Document structure detection
- Table extraction
- Section identification

### **Phase 3: AI Enhancement**
- Code block analysis
- Image processing
- Content enrichment

### **Phase 4: Advanced Types**
- [ ] OCR
- [ ] Audio
- [ ] Metadata
...

### **Phase 5: Production Features**
- REST API
- Batch processing
- Progress reporting

