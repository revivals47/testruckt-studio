//! Text editing functionality for canvas text elements
//!
//! Provides in-app text editing with cursor, selection, and text manipulation.

use testruct_core::layout::Rect;
use testruct_core::typography::TextStyle;
use uuid::Uuid;

/// Represents an active text editing session
#[derive(Clone, Debug)]
pub struct TextEditor {
    pub text_id: Uuid,
    pub content: String,
    pub cursor_pos: usize,
    pub selection_start: Option<usize>,
    pub selection_end: Option<usize>,
    pub bounds: Rect,
    pub style: TextStyle,
}

impl TextEditor {
    /// Create a new text editor for the given text element
    pub fn new(text_id: Uuid, content: String, bounds: Rect, style: TextStyle) -> Self {
        Self {
            text_id,
            content,
            cursor_pos: 0,
            selection_start: None,
            selection_end: None,
            bounds,
            style,
        }
    }

    /// Handle text input (character insertion)
    pub fn insert_text(&mut self, text: &str) {
        // Delete selection if any
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (min, max) = if start < end {
                (start, end)
            } else {
                (end, start)
            };
            self.content.drain(min..max);
            self.cursor_pos = min;
            self.selection_start = None;
            self.selection_end = None;
        }

        // Insert new text at cursor position
        if self.cursor_pos <= self.content.len() {
            self.content.insert_str(self.cursor_pos, text);
            self.cursor_pos += text.len();
        }
    }

    /// Handle backspace (delete character before cursor)
    pub fn handle_backspace(&mut self) {
        // Delete selection if any
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (min, max) = if start < end {
                (start, end)
            } else {
                (end, start)
            };
            self.content.drain(min..max);
            self.cursor_pos = min;
            self.selection_start = None;
            self.selection_end = None;
            return;
        }

        if self.cursor_pos > 0 {
            let prev_char_pos = self.cursor_pos - 1;
            self.content.remove(prev_char_pos);
            self.cursor_pos = prev_char_pos;
        }
    }

    /// Handle delete key (delete character after cursor)
    pub fn handle_delete(&mut self) {
        // Delete selection if any
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (min, max) = if start < end {
                (start, end)
            } else {
                (end, start)
            };
            self.content.drain(min..max);
            self.cursor_pos = min;
            self.selection_start = None;
            self.selection_end = None;
            return;
        }

        if self.cursor_pos < self.content.len() {
            self.content.remove(self.cursor_pos);
        }
    }

    /// Move cursor left (by character)
    pub fn move_cursor_left(&mut self, select: bool) {
        if !select {
            self.selection_start = None;
            self.selection_end = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }

        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            if select {
                self.selection_end = Some(self.cursor_pos);
            }
        }
    }

    /// Move cursor right (by character)
    pub fn move_cursor_right(&mut self, select: bool) {
        if !select {
            self.selection_start = None;
            self.selection_end = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }

        if self.cursor_pos < self.content.len() {
            self.cursor_pos += 1;
            if select {
                self.selection_end = Some(self.cursor_pos);
            }
        }
    }

    /// Move cursor to beginning of text
    pub fn move_cursor_home(&mut self, select: bool) {
        if !select {
            self.selection_start = None;
            self.selection_end = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }

        self.cursor_pos = 0;
        if select {
            self.selection_end = Some(self.cursor_pos);
        }
    }

    /// Move cursor to end of text
    pub fn move_cursor_end(&mut self, select: bool) {
        if !select {
            self.selection_start = None;
            self.selection_end = None;
        } else if self.selection_start.is_none() {
            self.selection_start = Some(self.cursor_pos);
        }

        self.cursor_pos = self.content.len();
        if select {
            self.selection_end = Some(self.cursor_pos);
        }
    }

    /// Select all text
    pub fn select_all(&mut self) {
        self.selection_start = Some(0);
        self.selection_end = Some(self.content.len());
        self.cursor_pos = self.content.len();
    }

    /// Get the current selection as a string
    pub fn get_selection(&self) -> Option<String> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (min, max) = if start < end {
                (start, end)
            } else {
                (end, start)
            };
            if min < max && max <= self.content.len() {
                return Some(self.content[min..max].to_string());
            }
        }
        None
    }

    /// Delete the current selection
    pub fn delete_selection(&mut self) {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (min, max) = if start < end {
                (start, end)
            } else {
                (end, start)
            };
            if min < max {
                self.content.drain(min..max);
                self.cursor_pos = min;
                self.selection_start = None;
                self.selection_end = None;
            }
        }
    }

    /// Clear all selection
    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }

    /// Get the cursor position in pixels (for cursor line drawing)
    /// This is a simplified version - actual implementation would need Pango metrics
    pub fn get_cursor_x(&self) -> f32 {
        let char_width = self.style.font_size * 0.5; // Approximate
        let x_offset = self.cursor_pos as f32 * char_width;
        self.bounds.origin.x + x_offset
    }

    /// Check if cursor is at valid position
    pub fn is_cursor_valid(&self) -> bool {
        self.cursor_pos <= self.content.len()
    }

    /// Get the edited content
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Get the edited content as owned String
    pub fn into_content(self) -> String {
        self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testruct_core::layout::Point;
    use testruct_core::layout::Size;

    fn create_test_editor() -> TextEditor {
        TextEditor::new(
            uuid::Uuid::new_v4(),
            "Hello World".to_string(),
            Rect {
                origin: Point { x: 0.0, y: 0.0 },
                size: Size {
                    width: 100.0,
                    height: 20.0,
                },
            },
            TextStyle::default(),
        )
    }

    #[test]
    fn test_insert_text() {
        let mut editor = create_test_editor();
        editor.cursor_pos = 0;
        editor.insert_text("Hi ");
        assert_eq!(editor.content, "Hi Hello World");
        assert_eq!(editor.cursor_pos, 3);
    }

    #[test]
    fn test_backspace() {
        let mut editor = create_test_editor();
        editor.cursor_pos = 5; // After "Hello"
        editor.handle_backspace();
        assert_eq!(editor.content, "Hell World");
        assert_eq!(editor.cursor_pos, 4);
    }

    #[test]
    fn test_delete() {
        let mut editor = create_test_editor();
        editor.cursor_pos = 5; // After "Hello"
        editor.handle_delete();
        assert_eq!(editor.content, "HelloWorld");
        assert_eq!(editor.cursor_pos, 5);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut editor = create_test_editor();
        editor.cursor_pos = 5;
        editor.move_cursor_left(false);
        assert_eq!(editor.cursor_pos, 4);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut editor = create_test_editor();
        editor.cursor_pos = 5;
        editor.move_cursor_right(false);
        assert_eq!(editor.cursor_pos, 6);
    }

    #[test]
    fn test_select_all() {
        let mut editor = create_test_editor();
        editor.select_all();
        assert_eq!(editor.selection_start, Some(0));
        assert_eq!(editor.selection_end, Some(11));
        assert_eq!(editor.cursor_pos, 11);
    }

    #[test]
    fn test_delete_selection() {
        let mut editor = create_test_editor();
        editor.selection_start = Some(0);
        editor.selection_end = Some(5);
        editor.delete_selection();
        assert_eq!(editor.content, " World");
        assert_eq!(editor.cursor_pos, 0);
    }
}
