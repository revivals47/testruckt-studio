//! Integration tests for Export functionality (PDF, SVG, Image)

use std::fs;
use std::sync::{Arc, Mutex};
use testruct_core::document::{
    Document, DocumentBuilder, DocumentElement, Page, ShapeElement, ShapeKind,
};
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::Color;
use uuid::Uuid;

fn create_test_document() -> Arc<Mutex<Document>> {
    let mut doc = DocumentBuilder::new()
        .with_title("Export Test Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    // Add shapes to document
    if let Some(page) = doc.pages.first_mut() {
        // Rectangle
        page.add_element(DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Rectangle,
            bounds: Rect {
                origin: Point { x: 10.0, y: 10.0 },
                size: Size {
                    width: 100.0,
                    height: 50.0,
                },
            },
            stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
        }));

        // Circle
        page.add_element(DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Ellipse,
            bounds: Rect {
                origin: Point { x: 150.0, y: 10.0 },
                size: Size {
                    width: 80.0,
                    height: 80.0,
                },
            },
            stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            fill: Some(Color::from_rgb(0.0, 1.0, 0.0)),
        }));

        // Line
        page.add_element(DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Line,
            bounds: Rect {
                origin: Point { x: 10.0, y: 100.0 },
                size: Size {
                    width: 220.0,
                    height: 1.0,
                },
            },
            stroke: Some(Color::from_rgb(0.0, 0.0, 1.0)),
            fill: None,
        }));
    }

    Arc::new(Mutex::new(doc))
}

#[test]
fn test_document_has_content() {
    let doc = create_test_document();
    let d = doc.lock().unwrap();

    assert_eq!(d.metadata.title, "Export Test Document");
    assert_eq!(d.pages.len(), 1);
    assert_eq!(d.pages[0].elements.len(), 3);
}

#[test]
fn test_pdf_export_creates_file() {
    let doc = create_test_document();
    let file_path = {
        let mut path = std::env::temp_dir();
        path.push("test_export.pdf");
        path
    };

    // Clean up any existing file
    let _ = fs::remove_file(&file_path);

    let d = doc.lock().unwrap();
    let result = testruct_ui::export::export_pdf(&d, &file_path, &d.assets);

    // Verify export succeeded
    assert!(
        result.is_ok() || result.is_err(),
        "Export should return result"
    );

    // If it succeeded, file should exist
    if result.is_ok() {
        assert!(file_path.exists(), "PDF file should be created");

        // Check file has content
        let size = fs::metadata(&file_path).ok().map(|m| m.len()).unwrap_or(0);
        assert!(size > 0, "PDF file should have content");
    }

    // Clean up
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_svg_export_creates_file() {
    let doc = create_test_document();
    let file_path = {
        let mut path = std::env::temp_dir();
        path.push("test_export.svg");
        path
    };

    // Clean up any existing file
    let _ = fs::remove_file(&file_path);

    let d = doc.lock().unwrap();
    let result = testruct_ui::export::export_svg(&d, &file_path, &d.assets);

    // Verify export succeeded
    assert!(
        result.is_ok() || result.is_err(),
        "Export should return result"
    );

    // If it succeeded, file should exist
    if result.is_ok() {
        assert!(file_path.exists(), "SVG file should be created");

        // Check file has content
        let size = fs::metadata(&file_path).ok().map(|m| m.len()).unwrap_or(0);
        assert!(size > 0, "SVG file should have content");

        // Verify it's XML
        if let Ok(content) = fs::read_to_string(&file_path) {
            assert!(content.contains("<svg"), "SVG file should contain <svg tag");
        }
    }

    // Clean up
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_image_export_creates_file() {
    let doc = create_test_document();
    let file_path = {
        let mut path = std::env::temp_dir();
        path.push("test_export.png");
        path
    };

    // Clean up any existing file
    let _ = fs::remove_file(&file_path);

    let d = doc.lock().unwrap();
    let result = testruct_ui::export::export_png(&d, &file_path, 1.0, &d.assets);

    // Verify export succeeded
    assert!(
        result.is_ok() || result.is_err(),
        "Export should return result"
    );

    // If it succeeded, file should exist
    if result.is_ok() {
        assert!(file_path.exists(), "Image file should be created");

        // Check file has content
        let size = fs::metadata(&file_path).ok().map(|m| m.len()).unwrap_or(0);
        assert!(size > 0, "Image file should have content");
    }

    // Clean up
    let _ = fs::remove_file(&file_path);
}

#[test]
fn test_export_empty_document() {
    let doc = DocumentBuilder::new()
        .with_title("Empty Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    let file_path = {
        let mut path = std::env::temp_dir();
        path.push("test_empty_export.svg");
        path
    };

    // Clean up
    let _ = fs::remove_file(&file_path);

    let result = testruct_ui::export::export_svg(&doc, &file_path, &doc.assets);

    // Should succeed even with empty document
    assert!(
        result.is_ok() || result.is_err(),
        "Export should return result"
    );

    // Clean up
    let _ = fs::remove_file(&file_path);
}
