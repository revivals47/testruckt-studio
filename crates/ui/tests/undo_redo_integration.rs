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
        fill: Some(Color::from_rgb(1.0, 0.0, 0.0)),
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
