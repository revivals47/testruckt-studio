//! Integration tests for Clipboard functionality
//!
//! Note: These tests use a global mutex to prevent race conditions
//! when accessing the shared clipboard state.

use std::sync::Mutex;
use testruct_core::document::{DocumentElement, ShapeElement, ShapeKind, TextElement};
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::{Color, TextStyle};
use testruct_ui::clipboard;
use uuid::Uuid;

/// Global lock for clipboard tests to prevent race conditions
static CLIPBOARD_TEST_LOCK: Mutex<()> = Mutex::new(());

fn create_test_shape(x: f32, y: f32) -> DocumentElement {
    DocumentElement::Shape(ShapeElement {
        id: Uuid::new_v4(),
        kind: ShapeKind::Rectangle,
        bounds: Rect {
            origin: Point { x, y },
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

fn create_test_text(x: f32, y: f32, content: &str) -> DocumentElement {
    DocumentElement::Text(TextElement {
        id: Uuid::new_v4(),
        content: content.to_string(),
        style: TextStyle::default(),
        bounds: Rect {
            origin: Point { x, y },
            size: Size {
                width: 200.0,
                height: 30.0,
            },
        },
        auto_resize_height: false,
        visible: true,
        locked: false,
    })
}

// ============================================
// Basic Clipboard Tests
// ============================================

#[test]
fn test_clipboard_copy_single_element() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();

    let shape = create_test_shape(10.0, 20.0);
    let original_id = shape.id();

    clipboard::copy_to_clipboard(vec![shape]);

    assert!(clipboard::has_clipboard_content());
    assert_eq!(clipboard::clipboard_content_count(), 1);

    // Paste and verify
    let pasted = clipboard::paste_from_clipboard();
    assert!(pasted.is_some());

    let pasted_elems = pasted.unwrap();
    assert_eq!(pasted_elems.len(), 1);

    // ID should be different
    assert_ne!(pasted_elems[0].id(), original_id);

    clipboard::clear_clipboard();
}

#[test]
fn test_clipboard_copy_multiple_elements() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();

    let shape1 = create_test_shape(10.0, 20.0);
    let shape2 = create_test_shape(100.0, 100.0);
    let text = create_test_text(50.0, 50.0, "Test text");

    let elements = vec![shape1, shape2, text];
    clipboard::copy_to_clipboard(elements);

    assert_eq!(clipboard::clipboard_content_count(), 3);

    let pasted = clipboard::paste_from_clipboard().unwrap();
    assert_eq!(pasted.len(), 3);

    clipboard::clear_clipboard();
}

#[test]
fn test_clipboard_paste_offset() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();
    clipboard::reset_paste_offset();

    let shape = create_test_shape(0.0, 0.0);
    clipboard::copy_to_clipboard(vec![shape]);

    // First paste - should have offset
    if let Some(pasted1) = clipboard::paste_from_clipboard() {
        let bounds1 = pasted1[0].bounds();
        assert!(bounds1.origin.x > 0.0, "Offset should be applied: got {}", bounds1.origin.x);

        // Second paste - should have larger offset
        if let Some(pasted2) = clipboard::paste_from_clipboard() {
            let bounds2 = pasted2[0].bounds();
            assert!(bounds2.origin.x > bounds1.origin.x,
                "Second offset should be larger: {} vs {}", bounds2.origin.x, bounds1.origin.x);
        }
    }

    clipboard::clear_clipboard();
}

#[test]
fn test_clipboard_unique_ids_per_paste() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();
    clipboard::reset_paste_offset();

    let shape = create_test_shape(10.0, 10.0);
    clipboard::copy_to_clipboard(vec![shape]);

    if let (Some(pasted1), Some(pasted2), Some(pasted3)) = (
        clipboard::paste_from_clipboard(),
        clipboard::paste_from_clipboard(),
        clipboard::paste_from_clipboard(),
    ) {
        // All IDs should be different
        assert_ne!(pasted1[0].id(), pasted2[0].id());
        assert_ne!(pasted2[0].id(), pasted3[0].id());
        assert_ne!(pasted1[0].id(), pasted3[0].id());
    }

    clipboard::clear_clipboard();
}

// ============================================
// Text Clipboard Tests
// ============================================

#[test]
fn test_create_text_element_from_clipboard() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::reset_paste_offset();

    let text = "Hello, World!";
    let element = clipboard::create_text_element_from_clipboard(text, Some((100.0, 100.0)));

    if let DocumentElement::Text(text_elem) = element {
        assert_eq!(text_elem.content, "Hello, World!");
        assert!(text_elem.bounds.origin.x >= 100.0);
        assert!(text_elem.bounds.origin.y >= 100.0);
    } else {
        panic!("Expected TextElement");
    }
}

#[test]
fn test_create_multiline_text_element() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::reset_paste_offset();

    let text = "Line 1\nLine 2\nLine 3";
    let element = clipboard::create_text_element_from_clipboard(text, Some((50.0, 50.0)));

    if let DocumentElement::Text(text_elem) = element {
        assert_eq!(text_elem.content, text);
        // Height should be larger for multiline
        assert!(text_elem.bounds.size.height >= 24.0);
    } else {
        panic!("Expected TextElement");
    }
}

#[test]
fn test_text_element_bounds_estimation() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::reset_paste_offset();

    // Short text
    let short = clipboard::create_text_element_from_clipboard("Hi", None);
    let short_bounds = short.bounds();

    // Long text
    clipboard::reset_paste_offset();
    let long = clipboard::create_text_element_from_clipboard(
        "This is a much longer piece of text that should have larger bounds",
        None,
    );
    let long_bounds = long.bounds();

    // Long text should have wider bounds
    assert!(long_bounds.size.width >= short_bounds.size.width);
}

// ============================================
// Clipboard State Tests
// ============================================

#[test]
fn test_clipboard_clear() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();

    let shape = create_test_shape(10.0, 10.0);
    clipboard::copy_to_clipboard(vec![shape]);
    assert!(clipboard::has_clipboard_content());

    clipboard::clear_clipboard();
    assert!(!clipboard::has_clipboard_content());
    assert_eq!(clipboard::clipboard_content_count(), 0);
}

#[test]
fn test_clipboard_overwrite() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();

    // Copy first set
    let shape1 = create_test_shape(10.0, 10.0);
    clipboard::copy_to_clipboard(vec![shape1]);
    assert_eq!(clipboard::clipboard_content_count(), 1);

    // Copy second set (should overwrite)
    let shape2 = create_test_shape(20.0, 20.0);
    let shape3 = create_test_shape(30.0, 30.0);
    clipboard::copy_to_clipboard(vec![shape2, shape3]);
    assert_eq!(clipboard::clipboard_content_count(), 2);

    clipboard::clear_clipboard();
}

#[test]
fn test_paste_from_empty_clipboard() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();

    let result = clipboard::paste_from_clipboard();
    assert!(result.is_none());
}

// ============================================
// Mixed Content Tests
// ============================================

#[test]
fn test_copy_paste_mixed_elements() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();

    let elements = vec![
        create_test_shape(10.0, 10.0),
        create_test_text(50.0, 50.0, "Text 1"),
        create_test_shape(100.0, 100.0),
        create_test_text(150.0, 150.0, "Text 2"),
    ];

    clipboard::copy_to_clipboard(elements);
    assert_eq!(clipboard::clipboard_content_count(), 4);

    let pasted = clipboard::paste_from_clipboard().unwrap();
    assert_eq!(pasted.len(), 4);

    // Verify element types are preserved
    let shape_count = pasted
        .iter()
        .filter(|e| matches!(e, DocumentElement::Shape(_)))
        .count();
    let text_count = pasted
        .iter()
        .filter(|e| matches!(e, DocumentElement::Text(_)))
        .count();

    assert_eq!(shape_count, 2);
    assert_eq!(text_count, 2);

    clipboard::clear_clipboard();
}

// ============================================
// Paste Offset Tests
// ============================================

#[test]
fn test_paste_offset_reset_on_new_copy() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();
    clipboard::reset_paste_offset();

    let shape1 = create_test_shape(0.0, 0.0);
    clipboard::copy_to_clipboard(vec![shape1]);

    // Paste multiple times to increase offset
    let _ = clipboard::paste_from_clipboard();
    let _ = clipboard::paste_from_clipboard();

    if let Some(pasted_before) = clipboard::paste_from_clipboard() {
        let offset_before = pasted_before[0].bounds().origin.x;

        // New copy should reset offset
        let shape2 = create_test_shape(0.0, 0.0);
        clipboard::copy_to_clipboard(vec![shape2]);

        if let Some(pasted_after) = clipboard::paste_from_clipboard() {
            let offset_after = pasted_after[0].bounds().origin.x;

            // After new copy, offset should be reset (smaller than before)
            assert!(offset_after < offset_before,
                "Offset should reset: {} should be < {}", offset_after, offset_before);
        }
    }

    clipboard::clear_clipboard();
}

#[test]
fn test_external_paste_offset() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::reset_paste_offset();

    let offset1 = clipboard::get_paste_offset();
    let offset2 = clipboard::get_paste_offset();
    let offset3 = clipboard::get_paste_offset();

    // Offsets should increase
    assert!(offset2.0 > offset1.0);
    assert!(offset3.0 > offset2.0);

    // Reset
    clipboard::reset_paste_offset();
    let offset_after_reset = clipboard::get_paste_offset();
    assert!(offset_after_reset.0 < offset3.0);
}

// ============================================
// Element Property Preservation Tests
// ============================================

#[test]
fn test_element_properties_preserved() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();

    let original = DocumentElement::Shape(ShapeElement {
        id: Uuid::new_v4(),
        kind: ShapeKind::Ellipse,
        bounds: Rect {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 150.0,
                height: 100.0,
            },
        },
        stroke: Some(Color::from_rgb(0.5, 0.3, 0.1)),
        stroke_width: 3.5,
        fill: Some(Color::from_rgb(0.8, 0.6, 0.4)),
        visible: true,
        locked: false,
    });

    clipboard::copy_to_clipboard(vec![original]);
    let pasted = clipboard::paste_from_clipboard().unwrap();

    if let DocumentElement::Shape(shape) = &pasted[0] {
        assert_eq!(shape.kind, ShapeKind::Ellipse);
        assert_eq!(shape.bounds.size.width, 150.0);
        assert_eq!(shape.bounds.size.height, 100.0);
        assert_eq!(shape.stroke_width, 3.5);
        assert!(shape.stroke.is_some());
        assert!(shape.fill.is_some());
    } else {
        panic!("Expected ShapeElement");
    }

    clipboard::clear_clipboard();
}

#[test]
fn test_text_content_preserved() {
    let _lock = CLIPBOARD_TEST_LOCK.lock().unwrap();
    clipboard::clear_clipboard();

    let original_content = "Hello, World!\nThis is a test.\nMultiple lines!";
    let original = DocumentElement::Text(TextElement {
        id: Uuid::new_v4(),
        content: original_content.to_string(),
        style: TextStyle::default(),
        bounds: Rect {
            origin: Point { x: 10.0, y: 10.0 },
            size: Size {
                width: 200.0,
                height: 100.0,
            },
        },
        auto_resize_height: true,
        visible: true,
        locked: false,
    });

    clipboard::copy_to_clipboard(vec![original]);
    let pasted = clipboard::paste_from_clipboard().unwrap();

    if let DocumentElement::Text(text) = &pasted[0] {
        assert_eq!(text.content, original_content);
        assert!(text.auto_resize_height);
    } else {
        panic!("Expected TextElement");
    }

    clipboard::clear_clipboard();
}
