//! Clipboard management for copy/paste/duplicate operations
//!
//! Provides a global clipboard for storing document elements that can be
//! pasted into the document.

use std::sync::Mutex;
use testruct_core::document::DocumentElement;
use once_cell::sync::Lazy;

/// Clipboard data structure holding copied elements and metadata
#[derive(Clone, Debug)]
pub struct ClipboardData {
    /// The elements that were copied
    pub elements: Vec<DocumentElement>,
    /// Default offset for pasting (to avoid exact overlap)
    pub offset: (f32, f32),
}

impl ClipboardData {
    /// Create new clipboard data with elements
    pub fn new(elements: Vec<DocumentElement>) -> Self {
        Self {
            elements,
            offset: (20.0, 20.0),
        }
    }

    /// Get the number of elements in clipboard
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Check if clipboard is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// Global clipboard storage
static CLIPBOARD: Lazy<Mutex<Option<ClipboardData>>> = Lazy::new(|| Mutex::new(None));

/// Copy elements to clipboard
pub fn copy_to_clipboard(elements: Vec<DocumentElement>) {
    let len = elements.len();
    let mut clipboard = CLIPBOARD.lock().expect("clipboard lock");
    *clipboard = Some(ClipboardData::new(elements));
    tracing::info!("📋 Copied {} elements to clipboard", len);
}

/// Get elements from clipboard (deep cloned with new IDs)
pub fn paste_from_clipboard() -> Option<Vec<DocumentElement>> {
    let clipboard = CLIPBOARD.lock().expect("clipboard lock");

    clipboard.as_ref().map(|data| {
        data.elements
            .iter()
            .map(|elem| {
                let mut new_elem = elem.clone();
                regenerate_element_id(&mut new_elem);
                offset_element_bounds(&mut new_elem, data.offset);
                new_elem
            })
            .collect()
    })
}

/// Check if clipboard has content
pub fn has_clipboard_content() -> bool {
    let clipboard = CLIPBOARD.lock().expect("clipboard lock");
    clipboard.is_some()
}

/// Get clipboard content count
pub fn clipboard_content_count() -> usize {
    let clipboard = CLIPBOARD.lock().expect("clipboard lock");
    clipboard.as_ref().map(|d| d.len()).unwrap_or(0)
}

/// Clear the clipboard
pub fn clear_clipboard() {
    let mut clipboard = CLIPBOARD.lock().expect("clipboard lock");
    *clipboard = None;
    tracing::info!("🗑️  Clipboard cleared");
}

/// Regenerate element ID (create new UUID)
fn regenerate_element_id(element: &mut DocumentElement) {
    use testruct_core::document::DocumentElement;

    match element {
        DocumentElement::Text(text) => {
            text.id = uuid::Uuid::new_v4();
        }
        DocumentElement::Image(image) => {
            image.id = uuid::Uuid::new_v4();
        }
        DocumentElement::Shape(shape) => {
            shape.id = uuid::Uuid::new_v4();
        }
        DocumentElement::Frame(frame) => {
            frame.id = uuid::Uuid::new_v4();
        }
    }
}

/// Offset element bounds (for paste positioning)
fn offset_element_bounds(element: &mut DocumentElement, offset: (f32, f32)) {
    use testruct_core::document::DocumentElement;

    match element {
        DocumentElement::Text(text) => {
            text.bounds.origin.x += offset.0;
            text.bounds.origin.y += offset.1;
        }
        DocumentElement::Image(image) => {
            image.bounds.origin.x += offset.0;
            image.bounds.origin.y += offset.1;
        }
        DocumentElement::Shape(shape) => {
            shape.bounds.origin.x += offset.0;
            shape.bounds.origin.y += offset.1;
        }
        DocumentElement::Frame(frame) => {
            frame.bounds.origin.x += offset.0;
            frame.bounds.origin.y += offset.1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testruct_core::document::TextElement;
    use testruct_core::layout::{Point, Size, Rect};
    use testruct_core::typography::TextStyle;

    fn create_test_text(x: f32, y: f32, content: &str) -> TextElement {
        TextElement {
            id: uuid::Uuid::new_v4(),
            content: content.to_string(),
            style: TextStyle::default(),
            bounds: Rect::new(Point::new(x, y), Size::new(100.0, 20.0)),
            auto_resize_height: false,
        }
    }

    #[test]
    fn test_copy_to_clipboard() {
        clear_clipboard();

        let text = create_test_text(0.0, 0.0, "Test");
        let elements = vec![DocumentElement::Text(text)];
        copy_to_clipboard(elements);

        assert!(has_clipboard_content());
        assert_eq!(clipboard_content_count(), 1);
    }

    #[test]
    fn test_paste_from_clipboard() {
        clear_clipboard();

        let text = create_test_text(0.0, 0.0, "Test");
        let original_id = text.id;

        let elements = vec![DocumentElement::Text(text)];
        copy_to_clipboard(elements);

        let pasted = paste_from_clipboard();
        assert!(pasted.is_some());

        let pasted_elems = pasted.unwrap();
        assert_eq!(pasted_elems.len(), 1);

        // Check that ID was regenerated
        if let DocumentElement::Text(t) = &pasted_elems[0] {
            assert_ne!(t.id, original_id);
            // Check offset applied
            assert_eq!(t.bounds.origin.x, 20.0);
            assert_eq!(t.bounds.origin.y, 20.0);
        }
    }

    #[test]
    fn test_clear_clipboard() {
        let text = create_test_text(0.0, 0.0, "Test");
        copy_to_clipboard(vec![DocumentElement::Text(text)]);
        assert!(has_clipboard_content());

        clear_clipboard();
        assert!(!has_clipboard_content());
    }

    #[test]
    fn test_clipboard_content_count() {
        clear_clipboard();

        let text1 = create_test_text(0.0, 0.0, "Test1");
        let text2 = create_test_text(0.0, 30.0, "Test2");

        copy_to_clipboard(vec![
            DocumentElement::Text(text1),
            DocumentElement::Text(text2),
        ]);

        assert_eq!(clipboard_content_count(), 2);
    }

    #[test]
    fn test_multiple_paste_operations() {
        clear_clipboard();

        let text = create_test_text(0.0, 0.0, "Test");
        copy_to_clipboard(vec![DocumentElement::Text(text)]);

        // First paste
        let pasted1 = paste_from_clipboard();
        let pasted2 = paste_from_clipboard();

        assert!(pasted1.is_some());
        assert!(pasted2.is_some());

        // IDs should be different
        if let (Some(p1), Some(p2)) = (pasted1, pasted2) {
            if let (DocumentElement::Text(t1), DocumentElement::Text(t2)) =
                (&p1[0], &p2[0])
            {
                assert_ne!(t1.id, t2.id);
            }
        }
    }
}
