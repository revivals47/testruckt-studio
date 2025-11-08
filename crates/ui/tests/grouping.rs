//! Integration tests for Grouping functionality

use std::sync::{Arc, Mutex};
use testruct_core::document::{Document, DocumentBuilder, Page, DocumentElement, ShapeKind, ShapeElement};
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::Color;
use testruct_ui::undo_redo::Command;
use uuid::Uuid;

fn create_test_document() -> Arc<Mutex<Document>> {
    let doc = DocumentBuilder::new()
        .with_title("Test Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    Arc::new(Mutex::new(doc))
}

fn create_test_shape(x: f32, y: f32) -> DocumentElement {
    DocumentElement::Shape(ShapeElement {
        id: Uuid::new_v4(),
        kind: ShapeKind::Rectangle,
        bounds: Rect {
            origin: Point { x, y },
            size: Size { width: 100.0, height: 50.0 },
        },
        stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
        fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
    })
}

#[test]
fn test_group_creation() {
    let doc = create_test_document();
    let shape1 = create_test_shape(10.0, 10.0);
    let shape1_id = shape1.id();
    let shape2 = create_test_shape(120.0, 10.0);
    let shape2_id = shape2.id();

    // Add shapes to document
    {
        let mut d = doc.lock().unwrap();
        if let Some(page) = d.pages.first_mut() {
            page.add_element(shape1);
            page.add_element(shape2);
            assert_eq!(page.elements.len(), 2);
        }
    }

    // Create group command
    let mut cmd = testruct_ui::undo_redo::GroupCommand::new(
        doc.clone(),
        vec![shape1_id, shape2_id],
        0,
        "Test Group".to_string(),
    );

    // Execute group
    assert!(cmd.execute().is_ok());
    assert_eq!(cmd.description(), "Group objects");

    // Verify group was created
    {
        let d = doc.lock().unwrap();
        if let Some(page) = d.pages.first() {
            assert_eq!(page.elements.len(), 1);

            // Check if element is a group
            if let DocumentElement::Group(group) = &page.elements[0] {
                assert_eq!(group.children.len(), 2);
            } else {
                panic!("Element should be a group");
            }
        }
    }
}

#[test]
fn test_group_bounds_calculation() {
    let doc = create_test_document();
    let shape1 = create_test_shape(10.0, 10.0); // x: 10-110, y: 10-60
    let shape1_id = shape1.id();
    let shape2 = create_test_shape(200.0, 100.0); // x: 200-300, y: 100-150

    let shape2_id = shape2.id();

    // Add shapes
    {
        let mut d = doc.lock().unwrap();
        if let Some(page) = d.pages.first_mut() {
            page.add_element(shape1);
            page.add_element(shape2);
        }
    }

    // Create and execute group
    let mut cmd = testruct_ui::undo_redo::GroupCommand::new(
        doc.clone(),
        vec![shape1_id, shape2_id],
        0,
        "Bounds Test".to_string(),
    );

    cmd.execute().expect("Group creation failed");

    // Verify bounds are correct
    {
        let d = doc.lock().unwrap();
        if let Some(page) = d.pages.first() {
            if let DocumentElement::Group(group) = &page.elements[0] {
                // Bounds should be from (10, 10) to (300, 150)
                assert_eq!(group.bounds.origin.x, 10.0);
                assert_eq!(group.bounds.origin.y, 10.0);
                assert_eq!(group.bounds.size.width, 290.0);
                assert_eq!(group.bounds.size.height, 140.0);
            }
        }
    }
}

#[test]
fn test_group_undo() {
    let doc = create_test_document();
    let shape1 = create_test_shape(10.0, 10.0);
    let shape1_id = shape1.id();
    let shape2 = create_test_shape(120.0, 10.0);
    let shape2_id = shape2.id();

    // Add shapes
    {
        let mut d = doc.lock().unwrap();
        if let Some(page) = d.pages.first_mut() {
            page.add_element(shape1);
            page.add_element(shape2);
        }
    }

    // Create and execute group
    let mut cmd = testruct_ui::undo_redo::GroupCommand::new(
        doc.clone(),
        vec![shape1_id, shape2_id],
        0,
        "Undo Test".to_string(),
    );

    cmd.execute().expect("Group creation failed");

    // Verify group exists
    {
        let d = doc.lock().unwrap();
        if let Some(page) = d.pages.first() {
            assert_eq!(page.elements.len(), 1);
        }
    }

    // Undo group
    assert!(cmd.undo().is_ok());

    // Verify shapes are back
    {
        let d = doc.lock().unwrap();
        if let Some(page) = d.pages.first() {
            assert_eq!(page.elements.len(), 2);
        }
    }
}

#[test]
fn test_group_command_with_stack() {
    let doc = create_test_document();

    // Add multiple shapes
    let shapes: Vec<_> = (0..3)
        .map(|i| {
            let shape = create_test_shape((i as f32) * 110.0, 10.0);
            (shape.id(), shape)
        })
        .collect();

    {
        let mut d = doc.lock().unwrap();
        if let Some(page) = d.pages.first_mut() {
            for (_, shape) in &shapes {
                page.add_element(shape.clone());
            }
        }
    }

    let shape_ids: Vec<_> = shapes.iter().map(|(id, _)| *id).collect();

    // Create group command
    let mut cmd = testruct_ui::undo_redo::GroupCommand::new(
        doc.clone(),
        shape_ids,
        0,
        "Stack Test".to_string(),
    );

    // Execute
    assert!(cmd.execute().is_ok());

    // Check elements
    {
        let d = doc.lock().unwrap();
        if let Some(page) = d.pages.first() {
            assert_eq!(page.elements.len(), 1);
        }
    }

    // Undo
    assert!(cmd.undo().is_ok());

    // Check shapes are back
    {
        let d = doc.lock().unwrap();
        if let Some(page) = d.pages.first() {
            assert_eq!(page.elements.len(), 3);
        }
    }
}
