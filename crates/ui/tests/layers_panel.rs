//! Integration tests for Layers Panel functionality

use std::sync::{Arc, Mutex};
use testruct_core::document::{
    Document, DocumentBuilder, DocumentElement, Page, ShapeElement, ShapeKind,
};
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::Color;
use uuid::Uuid;

fn create_test_document_with_shapes() -> Arc<Mutex<Document>> {
    let mut doc = DocumentBuilder::new()
        .with_title("Layers Test Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    if let Some(page) = doc.pages.first_mut() {
        // Add 3 shapes
        for i in 0..3 {
            page.add_element(DocumentElement::Shape(ShapeElement {
                id: Uuid::new_v4(),
                kind: ShapeKind::Rectangle,
                bounds: Rect {
                    origin: Point {
                        x: (i as f32) * 50.0,
                        y: 10.0,
                    },
                    size: Size {
                        width: 40.0,
                        height: 40.0,
                    },
                },
                stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
                fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
            }));
        }
    }

    Arc::new(Mutex::new(doc))
}

#[test]
fn test_document_has_layers() {
    let doc = create_test_document_with_shapes();
    let d = doc.lock().unwrap();

    assert_eq!(d.pages.len(), 1);
    assert_eq!(d.pages[0].elements.len(), 3);
}

#[test]
fn test_layer_element_ids_unique() {
    let doc = create_test_document_with_shapes();
    let d = doc.lock().unwrap();
    let page = &d.pages[0];

    let ids: Vec<_> = page.elements.iter().map(|e| e.id()).collect();

    // All IDs should be unique
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            assert_ne!(ids[i], ids[j], "Layer IDs should be unique");
        }
    }
}

#[test]
fn test_layer_ordering() {
    let doc = create_test_document_with_shapes();
    let d = doc.lock().unwrap();
    let page = &d.pages[0];

    // Elements should maintain order (bottom to top)
    assert_eq!(page.elements.len(), 3);

    // First element is bottom, last is top
    if let DocumentElement::Shape(s0) = &page.elements[0] {
        assert_eq!(s0.bounds.origin.x, 0.0);
    }

    if let DocumentElement::Shape(s1) = &page.elements[1] {
        assert_eq!(s1.bounds.origin.x, 50.0);
    }

    if let DocumentElement::Shape(s2) = &page.elements[2] {
        assert_eq!(s2.bounds.origin.x, 100.0);
    }
}

#[test]
fn test_layer_z_order_function() {
    let doc = create_test_document_with_shapes();
    let d = doc.lock().unwrap();
    let page = &d.pages[0];

    // Get the first element's ID
    let first_id = page.elements[0].id();

    // Check z_order returns correct index
    assert_eq!(page.z_order(first_id), Some(0));
}

#[test]
fn test_layer_visibility_representation() {
    let doc = create_test_document_with_shapes();
    let d = doc.lock().unwrap();

    // All layers should be initially visible (in the data)
    // Visibility toggle would be UI state, stored separately
    assert_eq!(d.pages[0].elements.len(), 3);
}

#[test]
fn test_layer_names_generation() {
    let doc = create_test_document_with_shapes();
    let d = doc.lock().unwrap();
    let page = &d.pages[0];

    // Check that we can get display names for each layer
    for (index, element) in page.elements.iter().enumerate() {
        let name = match element {
            DocumentElement::Shape(shape) => format!("{:?}", shape.kind),
            DocumentElement::Text(_) => "Text".to_string(),
            DocumentElement::Image(_) => "Image".to_string(),
            DocumentElement::Frame(_) => "Frame".to_string(),
            DocumentElement::Group(group) => format!("Group: {}", group.name),
        };

        assert!(!name.is_empty(), "Layer {} should have a name", index);
    }
}

#[test]
fn test_empty_document_layers() {
    let doc = DocumentBuilder::new()
        .with_title("Empty Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    let d = Arc::new(Mutex::new(doc));
    let doc_lock = d.lock().unwrap();

    assert_eq!(doc_lock.pages[0].elements.len(), 0);
}

#[test]
fn test_layer_bounds_accessibility() {
    let doc = create_test_document_with_shapes();
    let d = doc.lock().unwrap();
    let page = &d.pages[0];

    // Each layer's bounds should be accessible
    for element in &page.elements {
        let bounds = match element {
            DocumentElement::Shape(s) => &s.bounds,
            DocumentElement::Text(t) => &t.bounds,
            DocumentElement::Image(i) => &i.bounds,
            DocumentElement::Frame(f) => &f.bounds,
            DocumentElement::Group(g) => &g.bounds,
        };

        assert!(bounds.size.width > 0.0);
        assert!(bounds.size.height > 0.0);
    }
}

#[test]
fn test_layer_children_in_group() {
    let mut doc = DocumentBuilder::new()
        .with_title("Group Layers Test")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    // Add child elements
    let child1 = DocumentElement::Shape(ShapeElement {
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
        fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
    });

    let child2 = DocumentElement::Shape(ShapeElement {
        id: Uuid::new_v4(),
        kind: ShapeKind::Ellipse,
        bounds: Rect {
            origin: Point { x: 60.0, y: 0.0 },
            size: Size {
                width: 50.0,
                height: 50.0,
            },
        },
        stroke: None,
        fill: Some(Color::from_rgb(0.0, 1.0, 0.0)),
    });

    // Create group
    let group = DocumentElement::Group(testruct_core::document::GroupElement {
        id: Uuid::new_v4(),
        name: "Test Group".to_string(),
        bounds: Rect {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 110.0,
                height: 50.0,
            },
        },
        children: vec![child1, child2],
    });

    if let Some(page) = doc.pages.first_mut() {
        page.add_element(group);
    }

    let d = Arc::new(Mutex::new(doc));
    let doc_lock = d.lock().unwrap();
    let page = &doc_lock.pages[0];

    // Verify group has 2 children
    if let DocumentElement::Group(g) = &page.elements[0] {
        assert_eq!(g.children.len(), 2);
    } else {
        panic!("First element should be a group");
    }
}
