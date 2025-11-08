//! Tests for page thumbnail/preview generation functionality

use std::sync::{Arc, Mutex};
use testruct_core::document::{Document, DocumentBuilder, Page, DocumentElement, ShapeKind, ShapeElement, TextElement, ImageElement, FrameElement, GroupElement};
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::{Color, TextStyle};
use testruct_core::workspace::assets::AssetRef;
use uuid::Uuid;

fn create_test_document_with_shapes() -> Arc<Mutex<Document>> {
    let mut doc = DocumentBuilder::new()
        .with_title("Thumbnail Test Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    if let Some(page) = doc.pages.first_mut() {
        // Add rectangle shape
        page.add_element(DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Rectangle,
            bounds: Rect {
                origin: Point { x: 50.0, y: 50.0 },
                size: Size { width: 100.0, height: 80.0 },
            },
            stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
        }));

        // Add ellipse shape
        page.add_element(DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Ellipse,
            bounds: Rect {
                origin: Point { x: 200.0, y: 100.0 },
                size: Size { width: 120.0, height: 100.0 },
            },
            stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            fill: Some(Color::from_rgb(0.0, 1.0, 0.0)),
        }));

        // Add line shape
        page.add_element(DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Line,
            bounds: Rect {
                origin: Point { x: 50.0, y: 200.0 },
                size: Size { width: 200.0, height: 50.0 },
            },
            stroke: Some(Color::from_rgb(0.0, 0.0, 1.0)),
            fill: None,
        }));
    }

    Arc::new(Mutex::new(doc))
}

fn create_test_document_with_text() -> Arc<Mutex<Document>> {
    let mut doc = DocumentBuilder::new()
        .with_title("Text Thumbnail Test")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    if let Some(page) = doc.pages.first_mut() {
        page.add_element(DocumentElement::Text(TextElement {
            id: Uuid::new_v4(),
            content: "Sample Text Content".to_string(),
            bounds: Rect {
                origin: Point { x: 50.0, y: 50.0 },
                size: Size { width: 300.0, height: 50.0 },
            },
            style: TextStyle::default(),
            auto_resize_height: false,
        }));

        page.add_element(DocumentElement::Text(TextElement {
            id: Uuid::new_v4(),
            content: "More text here".to_string(),
            bounds: Rect {
                origin: Point { x: 100.0, y: 150.0 },
                size: Size { width: 200.0, height: 40.0 },
            },
            style: TextStyle::default(),
            auto_resize_height: false,
        }));
    }

    Arc::new(Mutex::new(doc))
}

fn create_test_document_with_images() -> Arc<Mutex<Document>> {
    let mut doc = DocumentBuilder::new()
        .with_title("Image Thumbnail Test")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    if let Some(page) = doc.pages.first_mut() {
        page.add_element(DocumentElement::Image(ImageElement {
            id: Uuid::new_v4(),
            bounds: Rect {
                origin: Point { x: 50.0, y: 50.0 },
                size: Size { width: 200.0, height: 150.0 },
            },
            source: AssetRef::new(),
        }));

        page.add_element(DocumentElement::Image(ImageElement {
            id: Uuid::new_v4(),
            bounds: Rect {
                origin: Point { x: 300.0, y: 100.0 },
                size: Size { width: 150.0, height: 120.0 },
            },
            source: AssetRef::new(),
        }));
    }

    Arc::new(Mutex::new(doc))
}

fn create_test_document_with_mixed_elements() -> Arc<Mutex<Document>> {
    let mut doc = DocumentBuilder::new()
        .with_title("Mixed Elements Thumbnail Test")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    if let Some(page) = doc.pages.first_mut() {
        // Shape
        page.add_element(DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Rectangle,
            bounds: Rect {
                origin: Point { x: 50.0, y: 50.0 },
                size: Size { width: 100.0, height: 100.0 },
            },
            stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            fill: Some(Color::from_rgb(1.0, 0.5, 0.0)),
        }));

        // Text
        page.add_element(DocumentElement::Text(TextElement {
            id: Uuid::new_v4(),
            content: "Mixed content".to_string(),
            bounds: Rect {
                origin: Point { x: 200.0, y: 50.0 },
                size: Size { width: 150.0, height: 40.0 },
            },
            style: TextStyle::default(),
            auto_resize_height: false,
        }));

        // Image
        page.add_element(DocumentElement::Image(ImageElement {
            id: Uuid::new_v4(),
            bounds: Rect {
                origin: Point { x: 50.0, y: 200.0 },
                size: Size { width: 120.0, height: 100.0 },
            },
            source: AssetRef::new(),
        }));

        // Frame
        page.add_element(DocumentElement::Frame(FrameElement {
            id: Uuid::new_v4(),
            bounds: Rect {
                origin: Point { x: 250.0, y: 200.0 },
                size: Size { width: 200.0, height: 150.0 },
            },
            children: vec![],
        }));
    }

    Arc::new(Mutex::new(doc))
}

#[test]
fn test_thumbnail_generation_basic() {
    use testruct_ui::canvas::page_thumbnail::generate_page_thumbnail;

    let doc = create_test_document_with_shapes();
    let d = doc.lock().unwrap();

    if let Some(page) = d.pages.first() {
        let result = generate_page_thumbnail(page);
        assert!(result.is_ok(), "Thumbnail generation should succeed");

        let png_data = result.unwrap();
        assert!(!png_data.is_empty(), "PNG data should not be empty");
        assert!(png_data.len() > 100, "PNG data should have reasonable size");

        // Check PNG magic bytes
        assert!(png_data[0] == 0x89, "PNG should start with correct magic byte");
        assert!(png_data[1] == 0x50, "PNG should have correct magic bytes"); // 'P'
        assert!(png_data[2] == 0x4E, "PNG should have correct magic bytes"); // 'N'
        assert!(png_data[3] == 0x47, "PNG should have correct magic bytes"); // 'G'
    }
}

#[test]
fn test_thumbnail_generation_with_text() {
    use testruct_ui::canvas::page_thumbnail::generate_page_thumbnail;

    let doc = create_test_document_with_text();
    let d = doc.lock().unwrap();

    if let Some(page) = d.pages.first() {
        let result = generate_page_thumbnail(page);
        assert!(result.is_ok(), "Thumbnail generation should succeed with text");

        let png_data = result.unwrap();
        assert!(!png_data.is_empty(), "PNG data should not be empty");

        // Verify PNG format
        assert_eq!(&png_data[0..4], &[0x89, 0x50, 0x4E, 0x47], "Should be valid PNG format");
    }
}

#[test]
fn test_thumbnail_generation_with_images() {
    use testruct_ui::canvas::page_thumbnail::generate_page_thumbnail;

    let doc = create_test_document_with_images();
    let d = doc.lock().unwrap();

    if let Some(page) = d.pages.first() {
        let result = generate_page_thumbnail(page);
        assert!(result.is_ok(), "Thumbnail generation should succeed with images");

        let png_data = result.unwrap();
        assert!(!png_data.is_empty(), "PNG data should not be empty");

        // Verify PNG format
        assert_eq!(&png_data[0..4], &[0x89, 0x50, 0x4E, 0x47], "Should be valid PNG format");
    }
}

#[test]
fn test_thumbnail_generation_with_mixed_elements() {
    use testruct_ui::canvas::page_thumbnail::generate_page_thumbnail;

    let doc = create_test_document_with_mixed_elements();
    let d = doc.lock().unwrap();

    if let Some(page) = d.pages.first() {
        let result = generate_page_thumbnail(page);
        assert!(result.is_ok(), "Thumbnail generation should succeed with mixed elements");

        let png_data = result.unwrap();
        assert!(!png_data.is_empty(), "PNG data should not be empty");

        // Verify PNG format
        assert_eq!(&png_data[0..4], &[0x89, 0x50, 0x4E, 0x47], "Should be valid PNG format");
    }
}

#[test]
fn test_thumbnail_empty_page() {
    use testruct_ui::canvas::page_thumbnail::generate_page_thumbnail;

    let page = Page::empty();
    let result = generate_page_thumbnail(&page);

    assert!(result.is_ok(), "Thumbnail generation should succeed for empty page");

    let png_data = result.unwrap();
    assert!(!png_data.is_empty(), "PNG data should not be empty for empty page");

    // Verify PNG format
    assert_eq!(&png_data[0..4], &[0x89, 0x50, 0x4E, 0x47], "Should be valid PNG format");
}

#[test]
fn test_thumbnail_cache_key_generation() {
    use testruct_ui::canvas::page_thumbnail::get_thumbnail_cache_key;

    let page_id = testruct_core::document::PageId::new();
    let cache_key = get_thumbnail_cache_key(page_id);

    assert!(cache_key.starts_with("page_thumbnail_"), "Cache key should have correct prefix");
    assert!(!cache_key.is_empty(), "Cache key should not be empty");
}

#[test]
fn test_page_change_detection() {
    use testruct_ui::canvas::page_thumbnail::has_page_changed;

    let mut page = Page::empty();

    // Empty page (0 elements) with no cached hash - should detect as changed
    assert!(has_page_changed(&page, None), "Empty page with no hash should be detected as changed");

    // Empty page (0 elements) with wrong hash - should detect as changed
    assert!(has_page_changed(&page, Some(5u64)), "Empty page with wrong hash should be detected as changed");

    // Empty page (0 elements) with correct hash (0) - should NOT detect as changed
    assert!(!has_page_changed(&page, Some(0u64)), "Empty page with correct hash should not be detected as changed");

    // Add an element - now page has 1 element
    page.add_element(DocumentElement::Shape(ShapeElement {
        id: Uuid::new_v4(),
        kind: ShapeKind::Rectangle,
        bounds: Rect {
            origin: Point { x: 50.0, y: 50.0 },
            size: Size { width: 100.0, height: 100.0 },
        },
        stroke: None,
        fill: None,
    }));

    // Page with 1 element, old hash of 0 - should detect as changed
    assert!(has_page_changed(&page, Some(0u64)), "Page with new element should be detected as changed");

    // Page with 1 element, correct hash of 1 - should NOT detect as changed
    assert!(!has_page_changed(&page, Some(1u64)), "Page with correct hash should not be detected as changed");
}

#[test]
fn test_thumbnail_multiple_shapes() {
    use testruct_ui::canvas::page_thumbnail::generate_page_thumbnail;

    let mut doc = DocumentBuilder::new()
        .with_title("Multiple Shapes")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create document");

    if let Some(page) = doc.pages.first_mut() {
        // Add multiple shapes of different types
        for i in 0..5 {
            let kind = match i % 5 {
                0 => ShapeKind::Rectangle,
                1 => ShapeKind::Ellipse,
                2 => ShapeKind::Line,
                3 => ShapeKind::Arrow,
                4 => ShapeKind::Polygon,
                _ => ShapeKind::Rectangle,
            };

            page.add_element(DocumentElement::Shape(ShapeElement {
                id: Uuid::new_v4(),
                kind,
                bounds: Rect {
                    origin: Point { x: (i as f32 * 50.0), y: (i as f32 * 50.0) },
                    size: Size { width: 60.0, height: 60.0 },
                },
                stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
                fill: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            }));
        }
    }

    let doc_lock = Arc::new(Mutex::new(doc));
    let d = doc_lock.lock().unwrap();

    if let Some(page) = d.pages.first() {
        let result = generate_page_thumbnail(page);
        assert!(result.is_ok(), "Thumbnail should generate for multiple shapes");

        let png_data = result.unwrap();
        assert!(!png_data.is_empty(), "PNG data should not be empty");
    }
}

#[test]
fn test_thumbnail_with_frames_and_groups() {
    use testruct_ui::canvas::page_thumbnail::generate_page_thumbnail;

    let mut doc = DocumentBuilder::new()
        .with_title("Frames and Groups")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create document");

    if let Some(page) = doc.pages.first_mut() {
        // Add frame
        page.add_element(DocumentElement::Frame(FrameElement {
            id: Uuid::new_v4(),
            bounds: Rect {
                origin: Point { x: 50.0, y: 50.0 },
                size: Size { width: 300.0, height: 200.0 },
            },
            children: vec![],
        }));

        // Add group
        page.add_element(DocumentElement::Group(GroupElement {
            id: Uuid::new_v4(),
            bounds: Rect {
                origin: Point { x: 150.0, y: 150.0 },
                size: Size { width: 250.0, height: 180.0 },
            },
            name: "Group 1".to_string(),
            children: vec![],
        }));
    }

    let doc_lock = Arc::new(Mutex::new(doc));
    let d = doc_lock.lock().unwrap();

    if let Some(page) = d.pages.first() {
        let result = generate_page_thumbnail(page);
        assert!(result.is_ok(), "Thumbnail should generate for frames and groups");

        let png_data = result.unwrap();
        assert!(!png_data.is_empty(), "PNG data should not be empty");
        assert_eq!(&png_data[0..4], &[0x89, 0x50, 0x4E, 0x47], "Should be valid PNG");
    }
}

#[test]
fn test_thumbnail_consistency() {
    use testruct_ui::canvas::page_thumbnail::generate_page_thumbnail;

    let doc = create_test_document_with_shapes();
    let d = doc.lock().unwrap();

    if let Some(page) = d.pages.first() {
        // Generate thumbnail twice from same page
        let result1 = generate_page_thumbnail(page);
        let result2 = generate_page_thumbnail(page);

        assert!(result1.is_ok() && result2.is_ok(), "Both thumbnails should generate successfully");

        let png_data1 = result1.unwrap();
        let png_data2 = result2.unwrap();

        // Same page should produce same thumbnail size (though byte-exact comparison might fail due to timestamps)
        assert_eq!(png_data1.len(), png_data2.len(), "Thumbnails from same page should have same size");
    }
}

#[test]
fn test_thumbnail_different_pages_different_sizes() {
    use testruct_ui::canvas::page_thumbnail::generate_page_thumbnail;

    let doc1 = create_test_document_with_shapes();
    let doc2 = create_test_document_with_images();

    let d1 = doc1.lock().unwrap();
    let d2 = doc2.lock().unwrap();

    if let (Some(page1), Some(page2)) = (d1.pages.first(), d2.pages.first()) {
        let result1 = generate_page_thumbnail(page1);
        let result2 = generate_page_thumbnail(page2);

        let png_data1 = result1.unwrap();
        let png_data2 = result2.unwrap();

        // Both should be valid PNG
        assert_eq!(&png_data1[0..4], &[0x89, 0x50, 0x4E, 0x47], "Both should be valid PNG");
        assert_eq!(&png_data2[0..4], &[0x89, 0x50, 0x4E, 0x47], "Both should be valid PNG");

        // Both should be non-empty
        assert!(!png_data1.is_empty(), "First thumbnail should not be empty");
        assert!(!png_data2.is_empty(), "Second thumbnail should not be empty");
    }
}
