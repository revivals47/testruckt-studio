//! Integration tests for document I/O (Save/Load)

use std::fs;
use std::path::PathBuf;
use testruct_core::document::{Document, DocumentBuilder, Page, DocumentElement, ShapeKind, ShapeElement};
use testruct_core::layout::Rect;
use testruct_core::typography::Color;
use uuid::Uuid;

fn create_test_document() -> Document {
    let mut doc = DocumentBuilder::new()
        .with_title("Test Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    // Add a shape to the first page
    if let Some(page) = doc.pages.first_mut() {
        let shape = DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Rectangle,
            bounds: Rect {
                origin: testruct_core::layout::Point { x: 10.0, y: 20.0 },
                size: testruct_core::layout::Size { width: 100.0, height: 50.0 },
            },
            stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
        });
        page.add_element(shape);
    }

    doc
}

#[test]
fn test_save_document() {
    let file_path = {
        let mut path = std::env::temp_dir();
        path.push("testruct_save_test.json");
        path
    };

    // Clean up any existing file
    let _ = fs::remove_file(&file_path);

    let document = create_test_document();
    let result = testruct_ui::io::save_document(&document, &file_path);

    assert!(result.is_ok(), "Failed to save document");
    assert!(file_path.exists(), "File was not created");

    // Clean up
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_document() {
    let file_path = {
        let mut path = std::env::temp_dir();
        path.push("testruct_load_test.json");
        path
    };

    // Clean up any existing file
    let _ = fs::remove_file(&file_path);

    let original_doc = create_test_document();
    testruct_ui::io::save_document(&original_doc, &file_path)
        .expect("Failed to save document");

    // Give filesystem time to flush
    std::thread::sleep(std::time::Duration::from_millis(50));

    let loaded_doc = testruct_ui::io::load_document(&file_path)
        .expect("Failed to load document");

    assert_eq!(loaded_doc.metadata.title, original_doc.metadata.title);
    assert_eq!(loaded_doc.pages.len(), original_doc.pages.len());

    // Clean up
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_save_load_roundtrip() {
    let file_path = {
        let mut path = std::env::temp_dir();
        path.push("roundtrip_test.json");
        path
    };

    // Clean up any existing file
    let _ = fs::remove_file(&file_path);

    let original_doc = create_test_document();

    // Save
    testruct_ui::io::save_document(&original_doc, &file_path)
        .expect("Failed to save document");

    // Verify file was written
    assert!(file_path.exists(), "File should exist after saving");

    // Give filesystem time to flush
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Load
    let loaded_doc = testruct_ui::io::load_document(&file_path)
        .expect("Failed to load document");

    // Verify content matches
    assert_eq!(loaded_doc.id, original_doc.id);
    assert_eq!(loaded_doc.metadata.title, original_doc.metadata.title);

    // Check first page content
    assert_eq!(loaded_doc.pages.len(), 1);
    assert_eq!(original_doc.pages[0].elements.len(), loaded_doc.pages[0].elements.len());

    if let Some(element) = loaded_doc.pages[0].elements.first() {
        if let DocumentElement::Shape(shape) = element {
            assert_eq!(shape.kind, ShapeKind::Rectangle);
            assert_eq!(shape.bounds.size.width, 100.0);
            assert_eq!(shape.bounds.size.height, 50.0);
        }
    }

    // Clean up
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_default_filename_format() {
    let filename = testruct_ui::io::default_filename();

    // Should contain "document_" prefix
    assert!(filename.starts_with("document_"));
    // Should end with .json
    assert!(filename.ends_with(".json"));
    // Should be at least minimum length (document_YYYYMMDD_HHMMSS.json)
    assert!(filename.len() > 20);
}

#[test]
fn test_save_creates_valid_json() {
    let file_path = {
        let mut path = std::env::temp_dir();
        path.push("json_test.json");
        path
    };

    // Clean up
    let _ = fs::remove_file(&file_path);

    let document = create_test_document();
    testruct_ui::io::save_document(&document, &file_path)
        .expect("Failed to save document");

    let content = fs::read_to_string(&file_path)
        .expect("Failed to read saved file");

    // Verify it's valid JSON
    let _json: serde_json::Value = serde_json::from_str(&content)
        .expect("Saved file is not valid JSON");

    // Clean up
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_load_nonexistent_file_fails() {
    let file_path = PathBuf::from("/nonexistent/path/document.json");
    let result = testruct_ui::io::load_document(&file_path);

    assert!(result.is_err(), "Should fail when loading nonexistent file");
}

#[test]
fn test_save_invalid_json_handling() {
    let file_path = {
        let mut path = std::env::temp_dir();
        path.push("invalid.json");
        path
    };

    // Create a file with invalid JSON
    fs::write(&file_path, "{ invalid json }").expect("Failed to create test file");

    let result = testruct_ui::io::load_document(&file_path);
    assert!(result.is_err(), "Should fail when loading invalid JSON");

    // Clean up
    let _ = fs::remove_file(&file_path);
}
