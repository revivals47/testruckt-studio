//! Auto-save functionality for Testruct Studio
//!
//! Provides automatic saving of documents after a configurable delay.

use crate::app::AppState;
use gtk4::glib;
use std::time::Duration;

/// Default auto-save delay in seconds
pub const DEFAULT_AUTO_SAVE_DELAY_SECS: u64 = 30;

/// Start the auto-save timer
///
/// This function sets up a periodic timer that checks if auto-save should be
/// triggered. The timer runs every second and checks if:
/// 1. Auto-save is enabled
/// 2. Document has a file path (not a new unsaved document)
/// 3. Document is modified
/// 4. Enough time has passed since last modification
///
/// # Arguments
///
/// * `state` - The application state
/// * `delay_secs` - Number of seconds to wait after modification before auto-saving
///
/// # Returns
///
/// A `glib::SourceId` that can be used to cancel the timer if needed
pub fn start_auto_save_timer(state: AppState, delay_secs: u64) -> glib::SourceId {
    tracing::info!(
        "💾 Auto-save timer started (delay: {}s)",
        delay_secs
    );

    // Check every second if we should auto-save
    glib::timeout_add_local(Duration::from_secs(1), move || {
        if state.should_auto_save(delay_secs) {
            perform_auto_save(&state);
        }
        glib::ControlFlow::Continue
    })
}

/// Perform the actual auto-save operation
fn perform_auto_save(state: &AppState) {
    // Get the file path
    let path = match state.current_file_path() {
        Some(p) => p,
        None => {
            tracing::debug!("Auto-save skipped: no file path");
            return;
        }
    };

    // Get the active document
    let document = match state.active_document() {
        Some(doc) => doc,
        None => {
            tracing::warn!("Auto-save skipped: no active document");
            return;
        }
    };

    // Perform the save
    match crate::io::file_io::save_document(&document, &path) {
        Ok(_) => {
            // Mark as saved (this also updates window title)
            state.mark_as_saved(path.clone());
            // Clear the last modified time to prevent immediate re-save
            state.clear_last_modified_time();
            tracing::info!("💾 Auto-saved: {}", path.display());
        }
        Err(e) => {
            tracing::error!("❌ Auto-save failed: {}", e);
        }
    }
}

/// Stop the auto-save timer
///
/// # Arguments
///
/// * `source_id` - The source ID returned by `start_auto_save_timer`
pub fn stop_auto_save_timer(source_id: glib::SourceId) {
    source_id.remove();
    tracing::info!("💾 Auto-save timer stopped");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_delay() {
        assert_eq!(DEFAULT_AUTO_SAVE_DELAY_SECS, 30);
    }
}
