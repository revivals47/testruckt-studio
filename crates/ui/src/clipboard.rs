//! Clipboard management for copy/paste/duplicate operations
//!
//! Provides a global clipboard for storing document elements that can be
//! pasted into the document. Supports:
//! - Internal element copy/paste
//! - External text paste (creates TextElement)
//! - External image paste (creates ImageElement)
//! - Cumulative paste offset to avoid overlapping

use once_cell::sync::Lazy;
use std::sync::Mutex;
use testruct_core::document::{DocumentElement, ImageElement, TextElement};
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::TextStyle;
use testruct_core::workspace::assets::AssetRef;

/// Base offset for paste operations
const BASE_PASTE_OFFSET: f32 = 20.0;

/// Maximum cumulative offset before reset
const MAX_PASTE_OFFSET: f32 = 200.0;

/// Clipboard data structure holding copied elements and metadata
#[derive(Clone, Debug)]
pub struct ClipboardData {
    /// The elements that were copied
    pub elements: Vec<DocumentElement>,
    /// Current paste count (for cumulative offset)
    pub paste_count: u32,
}

impl ClipboardData {
    /// Create new clipboard data with elements
    pub fn new(elements: Vec<DocumentElement>) -> Self {
        Self {
            elements,
            paste_count: 0,
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

    /// Calculate current paste offset based on paste count
    pub fn current_offset(&self) -> (f32, f32) {
        let offset = BASE_PASTE_OFFSET * (self.paste_count as f32 + 1.0);
        let clamped = offset % MAX_PASTE_OFFSET;
        (clamped, clamped)
    }

    /// Increment paste count
    pub fn increment_paste_count(&mut self) {
        self.paste_count += 1;
    }

    /// Reset paste count (called when new copy is made)
    pub fn reset_paste_count(&mut self) {
        self.paste_count = 0;
    }
}

/// External clipboard content types
#[derive(Clone, Debug)]
pub enum ExternalClipboardContent {
    /// Plain text content
    Text(String),
    /// Image data (PNG bytes)
    Image(Vec<u8>),
    /// No supported content
    None,
}

/// Global clipboard storage
static CLIPBOARD: Lazy<Mutex<Option<ClipboardData>>> = Lazy::new(|| Mutex::new(None));

/// Paste count tracker for cumulative offset
static PASTE_COUNT: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

/// Copy elements to clipboard (resets paste count)
pub fn copy_to_clipboard(elements: Vec<DocumentElement>) {
    let len = elements.len();
    let mut clipboard = CLIPBOARD.lock().expect("clipboard lock");
    *clipboard = Some(ClipboardData::new(elements));

    // Reset paste count on new copy
    let mut count = PASTE_COUNT.lock().expect("paste count lock");
    *count = 0;

    tracing::info!("📋 Copied {} elements to clipboard", len);
}

/// Get elements from clipboard (deep cloned with new IDs and cumulative offset)
pub fn paste_from_clipboard() -> Option<Vec<DocumentElement>> {
    let mut clipboard = CLIPBOARD.lock().expect("clipboard lock");

    clipboard.as_mut().map(|data| {
        let offset = data.current_offset();
        data.increment_paste_count();

        data.elements
            .iter()
            .map(|elem| {
                let mut new_elem = elem.clone();
                regenerate_element_id(&mut new_elem);
                offset_element_bounds(&mut new_elem, offset);
                new_elem
            })
            .collect()
    })
}

/// Get current paste offset (for external paste operations)
pub fn get_paste_offset() -> (f32, f32) {
    let mut count = PASTE_COUNT.lock().expect("paste count lock");
    let offset = BASE_PASTE_OFFSET * (*count as f32 + 1.0);
    let clamped = offset % MAX_PASTE_OFFSET;
    *count += 1;
    (clamped, clamped)
}

/// Reset paste offset counter (called when pasting different content)
pub fn reset_paste_offset() {
    let mut count = PASTE_COUNT.lock().expect("paste count lock");
    *count = 0;
}

/// Create a TextElement from external clipboard text
pub fn create_text_element_from_clipboard(text: &str, position: Option<(f32, f32)>) -> DocumentElement {
    let offset = get_paste_offset();
    let (x, y) = position.unwrap_or((100.0, 100.0));

    // Estimate text bounds based on content length
    let line_count = text.lines().count().max(1);
    let max_line_len = text.lines().map(|l| l.len()).max().unwrap_or(10);
    let estimated_width = (max_line_len as f32 * 8.0).clamp(100.0, 400.0);
    let estimated_height = (line_count as f32 * 20.0).clamp(24.0, 300.0);

    let text_element = TextElement {
        id: uuid::Uuid::new_v4(),
        content: text.to_string(),
        style: TextStyle::default(),
        bounds: Rect::new(
            Point::new(x + offset.0, y + offset.1),
            Size::new(estimated_width, estimated_height),
        ),
        auto_resize_height: true,
        visible: true,
        locked: false,
    };

    tracing::info!("📝 Created TextElement from clipboard text ({} chars)", text.len());
    DocumentElement::Text(text_element)
}

/// Create an ImageElement from external clipboard image data
/// Returns None if image data is invalid or can't be processed
pub fn create_image_element_from_clipboard(
    image_data: &[u8],
    position: Option<(f32, f32)>,
    asset_ref: AssetRef,
) -> Option<DocumentElement> {
    let offset = get_paste_offset();
    let (x, y) = position.unwrap_or((100.0, 100.0));

    // Try to determine image dimensions from PNG header
    let (width, height) = parse_png_dimensions(image_data).unwrap_or((200.0, 200.0));

    // Scale down if too large
    let max_size = 400.0;
    let scale = if width > max_size || height > max_size {
        max_size / width.max(height)
    } else {
        1.0
    };

    let scaled_width = width * scale;
    let scaled_height = height * scale;

    let image_element = ImageElement {
        id: uuid::Uuid::new_v4(),
        source: asset_ref,
        bounds: Rect::new(
            Point::new(x + offset.0, y + offset.1),
            Size::new(scaled_width, scaled_height),
        ),
        visible: true,
        locked: false,
    };

    tracing::info!("🖼️ Created ImageElement from clipboard ({:.0}x{:.0})", scaled_width, scaled_height);
    Some(DocumentElement::Image(image_element))
}

/// Parse PNG dimensions from header
fn parse_png_dimensions(data: &[u8]) -> Option<(f32, f32)> {
    // PNG header: 8 bytes signature, then IHDR chunk
    // IHDR starts at byte 8, width at offset 16 (4 bytes), height at offset 20 (4 bytes)
    if data.len() < 24 {
        return None;
    }

    // Check PNG signature
    let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    if data[0..8] != png_signature {
        return None;
    }

    // Read width and height from IHDR (big-endian)
    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]) as f32;
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]) as f32;

    Some((width, height))
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
        DocumentElement::Group(group) => {
            group.id = uuid::Uuid::new_v4();
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
        DocumentElement::Group(group) => {
            group.bounds.origin.x += offset.0;
            group.bounds.origin.y += offset.1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testruct_core::document::TextElement;
    use testruct_core::layout::{Point, Rect, Size};
    use testruct_core::typography::TextStyle;

    fn create_test_text(x: f32, y: f32, content: &str) -> TextElement {
        TextElement {
            id: uuid::Uuid::new_v4(),
            content: content.to_string(),
            style: TextStyle::default(),
            bounds: Rect::new(Point::new(x, y), Size::new(100.0, 20.0)),
            auto_resize_height: false,
            visible: true,
            locked: false,
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
        clear_clipboard();
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
        clear_clipboard();
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
        clear_clipboard();
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
            if let (DocumentElement::Text(t1), DocumentElement::Text(t2)) = (&p1[0], &p2[0]) {
                assert_ne!(t1.id, t2.id);
            }
        }
        clear_clipboard();
    }
}
