//! macOS native IME support via NSTextInputContext
//!
//! This module provides a workaround for the GTK4 + macOS DrawingArea limitation
//! by directly accepting pasted text and monitoring system pasteboard changes.
//!
//! Future enhancement: Could add NSTextInputContext bridge for full native IME support.

use std::cell::RefCell;
use std::rc::Rc;

/// macOS IME workaround handler
///
/// Since GTK4 on macOS doesn't route IME signals through custom DrawingArea widgets,
/// this provides an alternative by:
/// 1. Accepting pasted text via Ctrl+V / Cmd+V
/// 2. Monitoring clipboard changes
/// 3. Supporting system IME indirectly
pub struct MacOSImeWorkaround {
    last_clipboard_content: Rc<RefCell<String>>,
}

impl MacOSImeWorkaround {
    /// Create a new macOS IME workaround handler
    pub fn new() -> Self {
        Self {
            last_clipboard_content: Rc::new(RefCell::new(String::new())),
        }
    }

    /// Check if clipboard content changed and return the new content
    ///
    /// This is a polling approach - in a full implementation, would use
    /// native pasteboard notifications via NSPasteboard
    pub fn check_clipboard_change(&self) -> Option<String> {
        // Get current clipboard content using pbpaste
        if let Ok(output) = std::process::Command::new("pbpaste").output() {
            if let Ok(clipboard_text) = String::from_utf8(output.stdout) {
                let current = clipboard_text.trim().to_string();
                let last = self.last_clipboard_content.borrow();

                if !current.is_empty() && current != *last {
                    drop(last);
                    *self.last_clipboard_content.borrow_mut() = current.clone();
                    return Some(current);
                }
            }
        }
        None
    }

    /// Get the current clipboard content
    pub fn get_clipboard_content(&self) -> Option<String> {
        if let Ok(output) = std::process::Command::new("pbpaste").output() {
            if let Ok(text) = String::from_utf8(output.stdout) {
                let trimmed = text.trim().to_string();
                if !trimmed.is_empty() {
                    return Some(trimmed);
                }
            }
        }
        None
    }
}

impl Default for MacOSImeWorkaround {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workaround_creation() {
        let workaround = MacOSImeWorkaround::new();
        assert!(workaround.last_clipboard_content.borrow().is_empty());
    }
}
