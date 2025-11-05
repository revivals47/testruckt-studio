//! Clipboard integration for copy/paste operations

use super::common::add_window_action;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// Thread-local storage for clipboard data
thread_local! {
    static CLIPBOARD_CONTENT: RefCell<Option<String>> = RefCell::new(None);
}

/// Register copy/paste actions
pub fn register(
    window: &gtk4::ApplicationWindow,
    state: crate::app::AppState,
    canvas_view: &crate::canvas::CanvasView,
) {
    let copy_state = state.clone();
    let copy_selected_ids = canvas_view.render_state().selected_ids.clone();
    add_window_action(window, "copy", move |_| {
        tracing::info!("Action: copy selected objects");

        let selected_ids = copy_selected_ids.borrow();

        if selected_ids.is_empty() {
            tracing::warn!("⚠️  No objects selected for copying");
            return;
        }

        copy_state.with_active_document(|doc| {
            if let Some(page) = doc.pages.first() {
                let mut elements_to_copy = Vec::new();

                // Find all selected elements
                for element in &page.elements {
                    if selected_ids.contains(&element.id()) {
                        elements_to_copy.push(element.clone());
                    }
                }

                // Serialize elements to JSON
                if let Ok(json) = serde_json::to_string(&elements_to_copy) {
                    CLIPBOARD_CONTENT.with(|cb| {
                        *cb.borrow_mut() = Some(json);
                    });
                    tracing::info!("✅ Copied {} objects to clipboard", elements_to_copy.len());
                } else {
                    tracing::error!("❌ Failed to serialize clipboard content");
                }
            }
        });
    });

    let paste_state = state.clone();
    let paste_drawing_area = canvas_view.drawing_area();
    let paste_selected_ids = canvas_view.render_state().selected_ids.clone();
    add_window_action(window, "paste", move |_| {
        tracing::info!("Action: paste from clipboard");

        let content = CLIPBOARD_CONTENT.with(|cb| cb.borrow().clone());

        let Some(json_content) = content else {
            tracing::warn!("⚠️  Clipboard is empty");
            return;
        };

        // Deserialize elements from JSON
        let elements: Result<Vec<testruct_core::document::DocumentElement>, _> =
            serde_json::from_str(&json_content);

        let Ok(mut elements_to_paste) = elements else {
            tracing::error!("❌ Failed to deserialize clipboard content");
            return;
        };

        // Generate new UUIDs for pasted elements and offset position
        const PASTE_OFFSET: f32 = 20.0;
        for element in &mut elements_to_paste {
            // Generate new ID
            let new_id = uuid::Uuid::new_v4();

            // Update element with new ID and offset position
            match element {
                testruct_core::document::DocumentElement::Frame(ref mut f) => {
                    f.id = new_id;
                    f.bounds.origin.x += PASTE_OFFSET;
                    f.bounds.origin.y += PASTE_OFFSET;
                }
                testruct_core::document::DocumentElement::Text(ref mut t) => {
                    t.id = new_id;
                    t.bounds.origin.x += PASTE_OFFSET;
                    t.bounds.origin.y += PASTE_OFFSET;
                }
                testruct_core::document::DocumentElement::Image(ref mut i) => {
                    i.id = new_id;
                    i.bounds.origin.x += PASTE_OFFSET;
                    i.bounds.origin.y += PASTE_OFFSET;
                }
                testruct_core::document::DocumentElement::Shape(ref mut s) => {
                    s.id = new_id;
                    s.bounds.origin.x += PASTE_OFFSET;
                    s.bounds.origin.y += PASTE_OFFSET;
                }
            }
        }

        paste_state.with_active_document(|doc| {
            if let Some(page) = doc.pages.get_mut(0) {
                // Clear current selection and add pasted elements
                let pasted_count = elements_to_paste.len();
                {
                    let mut selected = paste_selected_ids.borrow_mut();
                    selected.clear();

                    // Add pasted elements and track their IDs
                    for element in elements_to_paste {
                        let element_id = element.id();
                        page.add_element(element);
                        selected.push(element_id);
                    }
                }

                tracing::info!("✅ Pasted {} objects from clipboard", pasted_count);
            }
        });

        // Redraw canvas
        let _ = paste_drawing_area.queue_draw();
    });
}
