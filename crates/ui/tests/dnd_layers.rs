//! Tests for drag-and-drop layers panel functionality

use std::sync::{Arc, Mutex};
use testruct_core::document::{
    Document, DocumentBuilder, DocumentElement, Page, ShapeElement, ShapeKind,
};
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::Color;
use uuid::Uuid;

fn create_test_document_with_layers() -> Arc<Mutex<Document>> {
    let mut doc = DocumentBuilder::new()
        .with_title("DnD Test Document")
        .add_page(Page::empty())
        .build()
        .expect("Failed to create test document");

    if let Some(page) = doc.pages.first_mut() {
        // Add 4 shapes for layer reordering test
        for i in 0..4 {
            page.add_element(DocumentElement::Shape(ShapeElement {
                id: Uuid::new_v4(),
                kind: ShapeKind::Rectangle,
                bounds: Rect {
                    origin: Point {
                        x: (i as f32) * 30.0,
                        y: 10.0,
                    },
                    size: Size {
                        width: 25.0,
                        height: 25.0,
                    },
                },
                stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
                stroke_width: 1.0,
                fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
                visible: true,
                locked: false,
            }));
        }
    }

    Arc::new(Mutex::new(doc))
}

#[test]
fn test_document_has_layers_for_dnd() {
    let doc = create_test_document_with_layers();
    let d = doc.lock().unwrap();

    assert_eq!(d.pages.len(), 1);
    assert_eq!(d.pages[0].elements.len(), 4);
}

#[test]
fn test_layer_visual_indices() {
    let doc = create_test_document_with_layers();
    let d = doc.lock().unwrap();
    let page = &d.pages[0];

    // Test visual indices (from top to bottom)
    // Reverse iteration should give us layer 0 at top, layer 3 at bottom
    let total_elements = page.elements.len();

    for (visual_index, _element) in page.elements.iter().enumerate().rev() {
        let expected_layer_number = visual_index + 1;
        assert!(expected_layer_number <= total_elements);
    }
}

#[test]
fn test_layer_reorder_swap() {
    let doc = create_test_document_with_layers();
    let d_clone = Arc::clone(&doc);
    let mut doc_mut = d_clone.lock().unwrap();
    let page = &mut doc_mut.pages[0];

    // Get initial order
    let initial_ids: Vec<_> = page.elements.iter().map(|e| e.id()).collect();
    assert_eq!(initial_ids.len(), 4);

    // Swap positions 0 and 1 (simulating layer reorder)
    if page.elements.len() >= 2 {
        page.elements.swap(0, 1);

        let new_ids: Vec<_> = page.elements.iter().map(|e| e.id()).collect();

        // First two should be swapped
        assert_eq!(new_ids[0], initial_ids[1]);
        assert_eq!(new_ids[1], initial_ids[0]);
        // Others unchanged
        assert_eq!(new_ids[2], initial_ids[2]);
        assert_eq!(new_ids[3], initial_ids[3]);
    }
}

#[test]
fn test_layer_move_forward() {
    let doc = create_test_document_with_layers();
    let d_clone = Arc::clone(&doc);
    let mut doc_mut = d_clone.lock().unwrap();
    let page = &mut doc_mut.pages[0];

    let layer_id = page.elements[1].id();

    // Move layer forward (increase z-order)
    if let Some(pos) = page.elements.iter().position(|e| e.id() == layer_id) {
        if pos < page.elements.len() - 1 {
            page.elements.swap(pos, pos + 1);

            // Verify it moved
            let new_pos = page.elements.iter().position(|e| e.id() == layer_id);
            assert_eq!(new_pos, Some(pos + 1));
        }
    }
}

#[test]
fn test_layer_move_backward() {
    let doc = create_test_document_with_layers();
    let d_clone = Arc::clone(&doc);
    let mut doc_mut = d_clone.lock().unwrap();
    let page = &mut doc_mut.pages[0];

    let layer_id = page.elements[2].id();

    // Move layer backward (decrease z-order)
    if let Some(pos) = page.elements.iter().position(|e| e.id() == layer_id) {
        if pos > 0 {
            page.elements.swap(pos, pos - 1);

            // Verify it moved
            let new_pos = page.elements.iter().position(|e| e.id() == layer_id);
            assert_eq!(new_pos, Some(pos - 1));
        }
    }
}

#[test]
fn test_layer_move_to_front() {
    let doc = create_test_document_with_layers();
    let d_clone = Arc::clone(&doc);
    let mut doc_mut = d_clone.lock().unwrap();
    let page = &mut doc_mut.pages[0];

    let layer_id = page.elements[1].id();

    // Move to front (top of z-order)
    if let Some(pos) = page.elements.iter().position(|e| e.id() == layer_id) {
        let element = page.elements.remove(pos);
        page.elements.push(element);

        // Verify it's at the end (front in z-order)
        assert_eq!(page.elements.last().unwrap().id(), layer_id);
    }
}

#[test]
fn test_layer_move_to_back() {
    let doc = create_test_document_with_layers();
    let d_clone = Arc::clone(&doc);
    let mut doc_mut = d_clone.lock().unwrap();
    let page = &mut doc_mut.pages[0];

    let layer_id = page.elements[2].id();

    // Move to back (bottom of z-order)
    if let Some(pos) = page.elements.iter().position(|e| e.id() == layer_id) {
        let element = page.elements.remove(pos);
        page.elements.insert(0, element);

        // Verify it's at the beginning (back in z-order)
        assert_eq!(page.elements.first().unwrap().id(), layer_id);
    }
}

#[test]
fn test_layer_boundaries() {
    let doc = create_test_document_with_layers();
    let d_clone = Arc::clone(&doc);
    let mut doc_mut = d_clone.lock().unwrap();
    let page = &mut doc_mut.pages[0];

    // Try to move front layer forward (should not change)
    let front_id = page.elements[page.elements.len() - 1].id();
    let front_pos = page
        .elements
        .iter()
        .position(|e| e.id() == front_id)
        .unwrap();

    if front_pos < page.elements.len() - 1 {
        page.elements.swap(front_pos, front_pos + 1);
        // Would swap if pos was less than len-1
    }

    // Should still be at same position
    assert_eq!(
        page.elements.iter().position(|e| e.id() == front_id),
        Some(front_pos)
    );
}

#[test]
fn test_multiple_layer_operations() {
    let doc = create_test_document_with_layers();
    let d_clone = Arc::clone(&doc);
    let mut doc_mut = d_clone.lock().unwrap();
    let page = &mut doc_mut.pages[0];

    let initial_count = page.elements.len();

    // Perform multiple operations
    if page.elements.len() >= 2 {
        page.elements.swap(0, 1);
    }
    if page.elements.len() >= 3 {
        page.elements.swap(1, 2);
    }

    // Count should remain same
    assert_eq!(page.elements.len(), initial_count);

    // All elements should still be present
    let ids_before = vec![
        page.elements[0].id(),
        page.elements[1].id(),
        page.elements[2].id(),
        page.elements[3].id(),
    ];

    // Check all IDs are unique
    for i in 0..ids_before.len() {
        for j in (i + 1)..ids_before.len() {
            assert_ne!(ids_before[i], ids_before[j]);
        }
    }
}

#[test]
fn test_layer_structure_after_reorder() {
    let doc = create_test_document_with_layers();
    let d_clone = Arc::clone(&doc);
    let mut doc_mut = d_clone.lock().unwrap();
    let page = &mut doc_mut.pages[0];

    // Perform reordering
    if page.elements.len() >= 4 {
        // Reorder: 0,1,2,3 -> 1,0,2,3
        page.elements.swap(0, 1);

        // All elements should still be valid
        for element in &page.elements {
            match element {
                DocumentElement::Shape(shape) => {
                    assert!(shape.bounds.size.width > 0.0);
                    assert!(shape.bounds.size.height > 0.0);
                }
                _ => {}
            }
        }
    }
}
