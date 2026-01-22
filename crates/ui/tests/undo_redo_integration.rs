//! Integration tests for Undo/Redo functionality

use std::sync::{Arc, Mutex};
use testruct_core::document::{
    Document, DocumentBuilder, DocumentElement, Page, ShapeElement, ShapeKind,
};
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

fn create_test_shape() -> DocumentElement {
    DocumentElement::Shape(ShapeElement {
        id: Uuid::new_v4(),
        kind: ShapeKind::Rectangle,
        bounds: Rect {
            origin: Point { x: 10.0, y: 20.0 },
            size: Size {
                width: 100.0,
                height: 50.0,
            },
        },
        stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
        stroke_width: 1.0,
        fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
        visible: true,
        locked: false,
    })
}

#[test]
fn test_undo_stack_creation() {
    let stack = testruct_ui::undo_redo::UndoRedoStack::new();

    assert!(!stack.can_undo());
    assert!(!stack.can_redo());
}

#[test]
fn test_create_command_undo_redo() {
    let doc = create_test_document();
    let shape = create_test_shape();

    let mut cmd = testruct_ui::undo_redo::CreateCommand::new(doc.clone(), shape, 0);

    // Execute command
    assert!(cmd.execute().is_ok());
    assert_eq!(cmd.description(), "Create object");
}

#[test]
fn test_delete_command_creation() {
    let doc = create_test_document();
    let shape = create_test_shape();
    let shape_id = shape.id();

    // Add shape to document first
    {
        let mut d = doc.lock().unwrap();
        if let Some(page) = d.pages.first_mut() {
            page.add_element(shape);
        }
    }

    // Create delete command
    let mut cmd = testruct_ui::undo_redo::DeleteCommand::new(doc.clone(), shape_id, 0);

    // Execute should succeed
    assert!(cmd.execute().is_ok());
    assert_eq!(cmd.description(), "Delete object");
}

#[test]
fn test_stack_push_command() {
    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();
    let doc = create_test_document();
    let shape = create_test_shape();

    let cmd = testruct_ui::undo_redo::CreateCommand::new(doc, shape, 0);

    stack.push(Box::new(cmd));

    assert!(stack.can_undo());
    assert!(!stack.can_redo());
}

#[test]
fn test_undo_operation() {
    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();
    let doc = create_test_document();
    let shape = create_test_shape();

    let cmd = testruct_ui::undo_redo::CreateCommand::new(doc, shape, 0);

    stack.push(Box::new(cmd));
    assert!(stack.can_undo());

    // Perform undo
    let result = stack.undo();
    assert!(result);

    // After undo, can_redo should be true
    assert!(stack.can_redo());
}

#[test]
fn test_redo_operation() {
    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();
    let doc = create_test_document();
    let shape = create_test_shape();

    let cmd = testruct_ui::undo_redo::CreateCommand::new(doc, shape, 0);

    stack.push(Box::new(cmd));

    // Perform undo
    stack.undo();
    assert!(stack.can_redo());

    // Perform redo
    let result = stack.redo();
    assert!(result);

    // After redo, can_undo should be true again
    assert!(stack.can_undo());
}

#[test]
fn test_multiple_commands_stack() {
    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();
    let doc = create_test_document();

    // Push 3 create commands
    for _ in 0..3 {
        let shape = create_test_shape();
        let cmd = testruct_ui::undo_redo::CreateCommand::new(doc.clone(), shape, 0);
        stack.push(Box::new(cmd));
    }

    assert!(stack.can_undo());

    // Undo all 3 commands
    assert!(stack.undo());
    assert!(stack.undo());
    assert!(stack.undo());

    // All commands undone
    assert!(!stack.can_undo());
    assert!(stack.can_redo());
}

#[test]
fn test_redo_clears_after_new_command() {
    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();
    let doc = create_test_document();

    // Push command and undo
    let shape1 = create_test_shape();
    let cmd1 = testruct_ui::undo_redo::CreateCommand::new(doc.clone(), shape1, 0);
    stack.push(Box::new(cmd1));
    stack.undo();

    assert!(stack.can_redo());

    // Push new command (this should clear redo history)
    let shape2 = create_test_shape();
    let cmd2 = testruct_ui::undo_redo::CreateCommand::new(doc.clone(), shape2, 0);
    stack.push(Box::new(cmd2));

    // Redo should be cleared
    assert!(!stack.can_redo());
}

#[test]
fn test_command_description() {
    let doc = create_test_document();
    let shape = create_test_shape();

    let cmd = testruct_ui::undo_redo::CreateCommand::new(doc, shape, 0);
    let description = cmd.description();

    assert!(!description.is_empty());
    assert_eq!(description, "Create object");
}

// ============================================
// Move Command Tests
// ============================================

fn create_document_with_shape() -> (Arc<Mutex<Document>>, Uuid) {
    let doc = create_test_document();
    let shape = create_test_shape();
    let shape_id = shape.id();

    {
        let mut d = doc.lock().unwrap();
        if let Some(page) = d.pages.first_mut() {
            page.add_element(shape);
        }
    }

    (doc, shape_id)
}

#[test]
fn test_move_command_basic() {
    let (doc, shape_id) = create_document_with_shape();

    // Get original position
    let original_pos = {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();
        let elem = page.elements.iter().find(|e| e.id() == shape_id).unwrap();
        let bounds = elem.bounds();
        (bounds.origin.x, bounds.origin.y)
    };

    assert_eq!(original_pos, (10.0, 20.0));

    // Create move command
    let mut cmd = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        shape_id,
        0,
        50.0,
        30.0,
    );

    // Execute move
    assert!(cmd.execute().is_ok());

    // Check new position
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();
        let elem = page.elements.iter().find(|e| e.id() == shape_id).unwrap();
        let bounds = elem.bounds();
        assert_eq!(bounds.origin.x, 60.0); // 10 + 50
        assert_eq!(bounds.origin.y, 50.0); // 20 + 30
    }

    // Undo move
    assert!(cmd.undo().is_ok());

    // Check position restored
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();
        let elem = page.elements.iter().find(|e| e.id() == shape_id).unwrap();
        let bounds = elem.bounds();
        assert_eq!(bounds.origin.x, 10.0);
        assert_eq!(bounds.origin.y, 20.0);
    }
}

#[test]
fn test_move_undo_redo_cycle() {
    let (doc, shape_id) = create_document_with_shape();

    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();

    // Push move command
    let cmd = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        shape_id,
        0,
        100.0,
        100.0,
    );
    stack.push(Box::new(cmd));

    // Verify position changed
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();
        let elem = page.elements.iter().find(|e| e.id() == shape_id).unwrap();
        let bounds = elem.bounds();
        assert_eq!(bounds.origin.x, 110.0);
        assert_eq!(bounds.origin.y, 120.0);
    }

    // Undo
    assert!(stack.undo());

    // Verify original position
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();
        let elem = page.elements.iter().find(|e| e.id() == shape_id).unwrap();
        let bounds = elem.bounds();
        assert_eq!(bounds.origin.x, 10.0);
        assert_eq!(bounds.origin.y, 20.0);
    }

    // Redo
    assert!(stack.redo());

    // Verify moved position again
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();
        let elem = page.elements.iter().find(|e| e.id() == shape_id).unwrap();
        let bounds = elem.bounds();
        assert_eq!(bounds.origin.x, 110.0);
        assert_eq!(bounds.origin.y, 120.0);
    }
}

// ============================================
// Multiple Operations Tests
// ============================================

#[test]
fn test_multiple_move_operations() {
    let (doc, shape_id) = create_document_with_shape();
    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();

    // Move 1
    let cmd1 = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        shape_id,
        0,
        10.0,
        10.0,
    );
    stack.push(Box::new(cmd1));

    // Move 2
    let cmd2 = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        shape_id,
        0,
        20.0,
        20.0,
    );
    stack.push(Box::new(cmd2));

    // Move 3
    let cmd3 = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        shape_id,
        0,
        30.0,
        30.0,
    );
    stack.push(Box::new(cmd3));

    // Final position: 10+10+20+30 = 70, 20+10+20+30 = 80
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();
        let elem = page.elements.iter().find(|e| e.id() == shape_id).unwrap();
        let bounds = elem.bounds();
        assert_eq!(bounds.origin.x, 70.0);
        assert_eq!(bounds.origin.y, 80.0);
    }

    // Undo all 3
    assert!(stack.undo()); // undo move 3
    assert!(stack.undo()); // undo move 2
    assert!(stack.undo()); // undo move 1

    // Back to original
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();
        let elem = page.elements.iter().find(|e| e.id() == shape_id).unwrap();
        let bounds = elem.bounds();
        assert_eq!(bounds.origin.x, 10.0);
        assert_eq!(bounds.origin.y, 20.0);
    }
}

#[test]
fn test_create_move_delete_sequence() {
    let doc = create_test_document();
    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();

    // Create shape
    let shape = create_test_shape();
    let shape_id = shape.id();
    let create_cmd = testruct_ui::undo_redo::CreateCommand::new(doc.clone(), shape, 0);
    stack.push(Box::new(create_cmd));

    // Verify created
    {
        let d = doc.lock().unwrap();
        assert_eq!(d.pages[0].elements.len(), 1);
    }

    // Move shape
    let move_cmd = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        shape_id,
        0,
        50.0,
        50.0,
    );
    stack.push(Box::new(move_cmd));

    // Delete shape
    let delete_cmd = testruct_ui::undo_redo::DeleteCommand::new(doc.clone(), shape_id, 0);
    stack.push(Box::new(delete_cmd));

    // Verify deleted
    {
        let d = doc.lock().unwrap();
        assert_eq!(d.pages[0].elements.len(), 0);
    }

    // Undo delete
    assert!(stack.undo());
    {
        let d = doc.lock().unwrap();
        assert_eq!(d.pages[0].elements.len(), 1);
    }

    // Undo move
    assert!(stack.undo());
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();
        let elem = page.elements.iter().find(|e| e.id() == shape_id).unwrap();
        let bounds = elem.bounds();
        assert_eq!(bounds.origin.x, 10.0);
        assert_eq!(bounds.origin.y, 20.0);
    }

    // Undo create
    assert!(stack.undo());
    {
        let d = doc.lock().unwrap();
        assert_eq!(d.pages[0].elements.len(), 0);
    }
}

#[test]
fn test_undo_stack_limit() {
    let doc = create_test_document();
    let mut stack = testruct_ui::undo_redo::UndoRedoStack::with_capacity(5);

    // Push 10 commands (exceeds limit of 5)
    for i in 0..10 {
        let shape = create_test_shape();
        let cmd = testruct_ui::undo_redo::CreateCommand::new(doc.clone(), shape, 0);
        stack.push(Box::new(cmd));
    }

    // Stack should have max 5 commands
    let mut undo_count = 0;
    while stack.undo() {
        undo_count += 1;
    }
    assert_eq!(undo_count, 5);
}

#[test]
fn test_redo_cleared_on_new_action() {
    let (doc, shape_id) = create_document_with_shape();
    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();

    // Move and undo
    let cmd1 = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        shape_id,
        0,
        10.0,
        10.0,
    );
    stack.push(Box::new(cmd1));
    stack.undo();

    assert!(stack.can_redo());

    // New action should clear redo
    let cmd2 = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        shape_id,
        0,
        20.0,
        20.0,
    );
    stack.push(Box::new(cmd2));

    assert!(!stack.can_redo());
}

#[test]
fn test_move_two_elements_separately() {
    let doc = create_test_document();

    // Add two shapes
    let shape1 = create_test_shape();
    let shape2 = DocumentElement::Shape(ShapeElement {
        id: Uuid::new_v4(),
        kind: ShapeKind::Ellipse,
        bounds: Rect {
            origin: Point { x: 100.0, y: 100.0 },
            size: Size { width: 50.0, height: 50.0 },
        },
        stroke: Some(Color::from_rgb(0.0, 0.0, 0.0)),
        stroke_width: 1.0,
        fill: Some(Color::from_rgb(0.0, 1.0, 0.0)),
        visible: true,
        locked: false,
    });

    let id1 = shape1.id();
    let id2 = shape2.id();

    {
        let mut d = doc.lock().unwrap();
        if let Some(page) = d.pages.first_mut() {
            page.add_element(shape1);
            page.add_element(shape2);
        }
    }

    let mut stack = testruct_ui::undo_redo::UndoRedoStack::new();

    // Move first element
    let cmd1 = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        id1,
        0,
        25.0,
        25.0,
    );
    stack.push(Box::new(cmd1));

    // Move second element
    let cmd2 = testruct_ui::undo_redo::MoveCommand::new(
        doc.clone(),
        id2,
        0,
        25.0,
        25.0,
    );
    stack.push(Box::new(cmd2));

    // Verify both moved
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();

        let elem1 = page.elements.iter().find(|e| e.id() == id1).unwrap();
        assert_eq!(elem1.bounds().origin.x, 35.0); // 10 + 25

        let elem2 = page.elements.iter().find(|e| e.id() == id2).unwrap();
        assert_eq!(elem2.bounds().origin.x, 125.0); // 100 + 25
    }

    // Undo both
    assert!(stack.undo());
    assert!(stack.undo());

    // Verify both restored
    {
        let d = doc.lock().unwrap();
        let page = d.pages.first().unwrap();

        let elem1 = page.elements.iter().find(|e| e.id() == id1).unwrap();
        assert_eq!(elem1.bounds().origin.x, 10.0);

        let elem2 = page.elements.iter().find(|e| e.id() == id2).unwrap();
        assert_eq!(elem2.bounds().origin.x, 100.0);
    }
}
