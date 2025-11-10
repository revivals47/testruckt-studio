//! Japanese IME (Input Method Editor) support for text editing
//!
//! This module provides IMContext integration for handling Japanese, Chinese, and other
//! multi-key composition input methods through GTK4's IMMulticontext.

use gtk4::prelude::*;
use gtk4::{gdk, glib, IMMulticontext};
use std::cell::RefCell;
use std::rc::Rc;

/// Manages Input Method (IME) context for text editing
///
/// Handles Japanese IME and other input methods by integrating with GTK4's
/// IMMulticontext. This allows proper composition of multi-key input sequences.
pub struct ImeManager {
    context: Rc<RefCell<Option<IMMulticontext>>>,
    text_insertion_callback: Rc<RefCell<Option<Box<dyn Fn(String)>>>>,
}

impl ImeManager {
    /// Create a new ImeManager instance
    pub fn new() -> Self {
        Self {
            context: Rc::new(RefCell::new(None)),
            text_insertion_callback: Rc::new(RefCell::new(None)),
        }
    }

    /// Set up IMContext with a keyboard event controller
    ///
    /// This connects the IMContext to the keyboard controller so that
    /// key events are properly routed through the input method layer.
    pub fn setup_with_controller(&self, key_controller: &gtk4::EventControllerKey) {
        let context = IMMulticontext::new();
        eprintln!("🔧 IMMulticontext created");

        // Connect the commit signal to handle composed text
        let callback_ref = self.text_insertion_callback.clone();
        context.connect_commit(move |_, text: &str| {
            eprintln!("🎌 IME COMMIT SIGNAL: '{}'", text);
            // Insert the composed text through the callback
            if let Some(ref callback) = *callback_ref.borrow() {
                eprintln!("  ✅ Calling insertion callback with text: '{}'", text);
                callback(text.to_string());
            } else {
                eprintln!("  ⚠️  No callback registered!");
            }
        });
        eprintln!("🔗 Connected commit signal handler");

        // Connect preedit signals to detect IME composition
        context.connect_preedit_start(|_| {
            eprintln!("📝 IME preedit-start: composition begins");
        });
        eprintln!("🔗 Connected preedit-start signal handler");

        context.connect_preedit_end(|_| {
            eprintln!("📝 IME preedit-end: composition ends");
        });
        eprintln!("🔗 Connected preedit-end signal handler");

        context.connect_preedit_changed(|_| {
            eprintln!("📝 IME preedit-changed: composition updated");
        });
        eprintln!("🔗 Connected preedit-changed signal handler");

        // Set the IMContext as the input method handler for this controller
        // This allows GTK4 to automatically route key events through the IME
        eprintln!("🔌 Setting IMContext on EventControllerKey");
        key_controller.set_im_context(Some(&context));
        eprintln!("✅ IMContext set on controller");

        // NOTE: We intentionally do NOT call set_client_widget() on the context
        // because DrawingArea is not a standard text widget and this causes
        // macOS-specific issues with IMKCFRunLoopWakeUpReliable errors.
        // GTK4's EventControllerKey handles composition automatically.

        // Store the context
        *self.context.borrow_mut() = Some(context);

        eprintln!("✅ IME context initialized with key controller and signal handlers");
        tracing::debug!("✅ IME context initialized with key controller");
    }

    /// Filter a key event through the Input Method Engine
    ///
    /// Returns `true` if the IME handled the key (composition in progress),
    /// `false` if the key should be processed directly (ASCII, arrows, etc).
    ///
    /// This is the critical step that allows Japanese IME to intercept keys
    /// and manage composition.
    ///
    /// NOTE: In GTK4, we let the EventControllerKey handle key filtering through
    /// the IMContext automatically when we call set_im_context(). This method
    /// is kept for potential future use.
    pub fn filter_key_legacy(&self, _keyval: u32, _state: gdk::ModifierType) -> bool {
        // In GTK4, key filtering is handled automatically by EventControllerKey
        // when IMContext is set via set_im_context()
        // This method is kept for reference but not actively used
        false
    }

    /// Notify IME that text editing has begun (focus gained)
    pub fn focus_in(&self) {
        if let Some(ref context) = *self.context.borrow() {
            context.focus_in();
            tracing::debug!("📱 IME focus_in() called");
        }
    }

    /// Notify IME that text editing has ended (focus lost)
    pub fn focus_out(&self) {
        if let Some(ref context) = *self.context.borrow() {
            context.focus_out();
            tracing::debug!("📱 IME focus_out() called");
        }
    }

    /// Register a callback for composed text insertion
    ///
    /// This callback will be invoked by the commit signal handler
    /// when the IME delivers composed text (e.g., Japanese characters).
    pub fn set_text_insertion_callback<F>(&self, callback: F)
    where
        F: Fn(String) + 'static,
    {
        *self.text_insertion_callback.borrow_mut() = Some(Box::new(callback));
        tracing::debug!("✅ Text insertion callback registered for IME");
    }

    /// Reset the IMContext to initial state
    ///
    /// Call this when canceling text composition or switching modes
    pub fn reset(&self) {
        if let Some(ref context) = *self.context.borrow() {
            context.reset();
            tracing::debug!("🔄 IME context reset");
        }
    }
}

impl Default for ImeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ime_manager_creation() {
        let ime = ImeManager::new();
        assert!(ime.context.borrow().is_none());
    }

    #[test]
    fn test_callback_registration() {
        let ime = ImeManager::new();
        let called = Rc::new(RefCell::new(false));
        let called_clone = called.clone();

        ime.set_text_insertion_callback(move |_text| {
            *called_clone.borrow_mut() = true;
        });

        assert!(ime.text_insertion_callback.borrow().is_some());
    }
}
