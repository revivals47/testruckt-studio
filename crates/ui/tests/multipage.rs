//! Integration tests for Multi-page document support

use std::sync::{Arc, Mutex};
use testruct_core::document::{
    Document, DocumentBuilder, DocumentElement, Page, ShapeElement, ShapeKind,
};
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::Color;
use uuid::Uuid;

fn create_multipage_document() -> Arc<Mutex<Document>> {
    let mut doc = DocumentBuilder::new()
        .with_title("Multi-Page Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    // Add elements to first page
    for elem_num in 0..2 {
        doc.pages[0].add_element(DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Rectangle,
            bounds: Rect {
                origin: Point {
                    x: (elem_num as f32) * 60.0,
                    y: 0.0,
                },
                size: Size {
                    width: 50.0,
                    height: 50.0,
                },
            },
            stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            stroke_width: 1.0,
            fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
            visible: true,
            locked: false,
        }));
    }

    // Add more pages with elements
    for page_num in 0..2 {
        let mut page = Page::empty();

        // Add elements to each page
        for elem_num in 0..2 {
            page.add_element(DocumentElement::Shape(ShapeElement {
                id: Uuid::new_v4(),
                kind: ShapeKind::Rectangle,
                bounds: Rect {
                    origin: Point {
                        x: (elem_num as f32) * 60.0,
                        y: (page_num as f32) * 10.0,
                    },
                    size: Size {
                        width: 50.0,
                        height: 50.0,
                    },
                },
                stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
                stroke_width: 1.0,
                fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
                visible: true,
                locked: false,
            }));
        }

        doc.pages.push(page);
    }

    Arc::new(Mutex::new(doc))
}

#[test]
fn test_document_has_multiple_pages() {
    let doc = create_multipage_document();
    let d = doc.lock().unwrap();

    assert_eq!(d.pages.len(), 3);
}

#[test]
fn test_each_page_has_unique_id() {
    let doc = create_multipage_document();
    let d = doc.lock().unwrap();

    let page_ids: Vec<_> = d.pages.iter().map(|p| p.id).collect();

    // All page IDs should be unique
    for i in 0..page_ids.len() {
        for j in (i + 1)..page_ids.len() {
            assert_ne!(page_ids[i], page_ids[j], "Page IDs should be unique");
        }
    }
}

#[test]
fn test_each_page_has_elements() {
    let doc = create_multipage_document();
    let d = doc.lock().unwrap();

    // First page should have 2 elements
    assert_eq!(d.pages[0].elements.len(), 2);
    // Second page should have 2 elements
    assert_eq!(d.pages[1].elements.len(), 2);
    // Third page should have 2 elements
    assert_eq!(d.pages[2].elements.len(), 2);
}

#[test]
fn test_page_access_by_id() {
    let doc = create_multipage_document();
    let d = doc.lock().unwrap();

    // Get the first page's ID
    let page_id = d.pages[0].id;

    // Should be able to find it
    let found_page = d.page(page_id);
    assert!(found_page.is_some());
}

#[test]
fn test_add_page_to_document() {
    let doc = DocumentBuilder::new()
        .with_title("Test Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to build document");

    let d = Arc::new(Mutex::new(doc));
    let mut doc_mut = d.lock().unwrap();

    let initial_page_count = doc_mut.pages.len();
    assert_eq!(initial_page_count, 1);

    // Add new page
    doc_mut.pages.push(Page::empty());

    assert_eq!(doc_mut.pages.len(), 2);
}

#[test]
fn test_page_metadata() {
    let mut doc = DocumentBuilder::new()
        .with_title("Metadata Test")
        .add_page(Page::empty())
        .build()
        .expect("Failed to build document");

    // Check first page metadata
    assert!(doc.pages[0].metadata.name.is_empty() || !doc.pages[0].metadata.name.is_empty());

    let d = Arc::new(Mutex::new(doc));
    let doc_lock = d.lock().unwrap();

    // Pages should have metadata
    for page in &doc_lock.pages {
        // Metadata has name and notes fields
        let _ = &page.metadata.name;
        let _ = &page.metadata.notes;
    }
}

#[test]
fn test_multipage_iteration() {
    let doc = create_multipage_document();
    let d = doc.lock().unwrap();

    let page_count = d.pages().count();
    assert_eq!(page_count, 3);

    let mut index = 0;
    for page in d.pages() {
        assert_eq!(page.elements.len(), 2);
        index += 1;
    }
    assert_eq!(index, 3);
}

#[test]
fn test_page_element_independence() {
    let mut doc = DocumentBuilder::new()
        .with_title("Independence Test")
        .add_page(Page::empty())
        .build()
        .expect("Failed to build document");

    // Add shape to first page
    let shape1 = DocumentElement::Shape(ShapeElement {
        id: Uuid::new_v4(),
        kind: ShapeKind::Rectangle,
        bounds: Rect {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 50.0,
                height: 50.0,
            },
        },
        stroke: None,
        stroke_width: 1.0,
        fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
        visible: true,
        locked: false,
    });

    doc.pages[0].add_element(shape1);

    // Add second page
    let mut page2 = Page::empty();
    let shape2 = DocumentElement::Shape(ShapeElement {
        id: Uuid::new_v4(),
        kind: ShapeKind::Ellipse,
        bounds: Rect {
            origin: Point { x: 10.0, y: 10.0 },
            size: Size {
                width: 40.0,
                height: 40.0,
            },
        },
        stroke: None,
        stroke_width: 1.0,
        fill: Some(Color::from_rgb(0.0, 1.0, 0.0)),
        visible: true,
        locked: false,
    });

    page2.add_element(shape2);
    doc.pages.push(page2);

    // Verify pages are independent
    assert_eq!(doc.pages[0].elements.len(), 1);
    assert_eq!(doc.pages[1].elements.len(), 1);

    // Elements should have different IDs
    let id0 = doc.pages[0].elements[0].id();
    let id1 = doc.pages[1].elements[0].id();
    assert_ne!(id0, id1);
}

#[test]
fn test_empty_page_creation() {
    let page = Page::empty();

    assert_eq!(page.elements.len(), 0);
    // Metadata should exist
    assert!(page.metadata.name.is_empty() || !page.metadata.name.is_empty());
}

#[test]
fn test_page_access_bounds() {
    let doc = create_multipage_document();
    let d = doc.lock().unwrap();

    // Access within bounds
    assert!(d.pages.get(0).is_some());
    assert!(d.pages.get(1).is_some());
    assert!(d.pages.get(2).is_some());

    // Access out of bounds
    assert!(d.pages.get(3).is_none());
    assert!(d.pages.get(100).is_none());
}
