//! Template system for document templates
//!
//! Provides functionality to save, load, and manage document templates.

use std::fs;
use std::path::PathBuf;
use testruct_core::document::Document;

/// Get the templates directory path
pub fn templates_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("testruct")
        .join("templates")
}

/// Initialize the templates directory
pub fn init_templates_dir() -> std::io::Result<()> {
    let dir = templates_dir();
    fs::create_dir_all(&dir)?;
    Ok(())
}

/// Save document as a template
pub fn save_template(name: &str, document: &Document) -> std::io::Result<()> {
    init_templates_dir()?;

    let dir = templates_dir();
    let file_path = dir.join(format!("{}.json", name));

    let json = serde_json::to_string_pretty(document)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    fs::write(&file_path, json)?;
    tracing::info!("✅ Template saved: {}", file_path.display());
    Ok(())
}

/// Load document from template
pub fn load_template(name: &str) -> std::io::Result<Document> {
    let dir = templates_dir();
    let file_path = dir.join(format!("{}.json", name));

    let json = fs::read_to_string(&file_path)?;
    let document: Document = serde_json::from_str(&json)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    tracing::info!("✅ Template loaded: {}", file_path.display());
    Ok(document)
}

/// List all available templates
pub fn list_templates() -> std::io::Result<Vec<String>> {
    init_templates_dir()?;

    let dir = templates_dir();
    let mut templates = Vec::new();

    if !dir.exists() {
        return Ok(templates);
    }

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "json") {
            if let Some(name) = path.file_stem() {
                if let Some(name_str) = name.to_str() {
                    templates.push(name_str.to_string());
                }
            }
        }
    }

    templates.sort();
    Ok(templates)
}

/// Delete a template
pub fn delete_template(name: &str) -> std::io::Result<()> {
    let dir = templates_dir();
    let file_path = dir.join(format!("{}.json", name));

    if file_path.exists() {
        fs::remove_file(&file_path)?;
        tracing::info!("✅ Template deleted: {}", name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_templates_dir_exists() {
        let dir = templates_dir();
        assert!(!dir.as_os_str().is_empty());
    }

    #[test]
    fn test_list_templates() {
        let result = list_templates();
        assert!(result.is_ok());
    }
}
