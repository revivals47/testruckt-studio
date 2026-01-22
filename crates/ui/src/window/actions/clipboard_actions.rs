//! Clipboard integration for copy/paste operations
//!
//! Supports:
//! - Internal element copy/paste (JSON serialization)
//! - External text paste (creates TextElement)
//! - External image paste (creates ImageElement)
//! - System clipboard integration via GTK4

use super::common::add_window_action;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// Thread-local storage for internal clipboard data (JSON serialized elements)
thread_local! {
    static CLIPBOARD_CONTENT: RefCell<Option<String>> = const { RefCell::new(None) };
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
    let paste_window = window.clone();

    add_window_action(window, "paste", move |_| {
        tracing::info!("Action: paste from clipboard");

        // First, try internal clipboard content
        let internal_content = CLIPBOARD_CONTENT.with(|cb| cb.borrow().clone());

        if let Some(json_content) = internal_content {
            // Try to parse as internal elements
            let elements: Result<Vec<testruct_core::document::DocumentElement>, _> =
                serde_json::from_str(&json_content);

            if let Ok(elements_to_paste) = elements {
                paste_internal_elements(
                    elements_to_paste,
                    &paste_state,
                    &paste_selected_ids,
                    &paste_drawing_area,
                );
                return;
            }
        }

        // If no internal content, try system clipboard
        let clipboard = gtk4::prelude::WidgetExt::display(&paste_window).clipboard();
        let state_for_text = paste_state.clone();
        let selected_for_text = paste_selected_ids.clone();
        let drawing_for_text = paste_drawing_area.clone();

        // Try to read text from system clipboard
        clipboard.read_text_async(
            gtk4::gio::Cancellable::NONE,
            move |result| {
                if let Ok(Some(text)) = result {
                    if !text.is_empty() {
                        paste_external_text(
                            &text,
                            &state_for_text,
                            &selected_for_text,
                            &drawing_for_text,
                        );
                    }
                }
            },
        );
    });

    // Register paste-special action for explicit external paste
    let paste_special_state = state.clone();
    let paste_special_drawing_area = canvas_view.drawing_area();
    let paste_special_selected_ids = canvas_view.render_state().selected_ids.clone();
    let paste_special_window = window.clone();

    add_window_action(window, "paste-external", move |_| {
        tracing::info!("Action: paste from external clipboard");

        let clipboard = gtk4::prelude::WidgetExt::display(&paste_special_window).clipboard();
        let state_for_paste = paste_special_state.clone();
        let selected_for_paste = paste_special_selected_ids.clone();
        let drawing_for_paste = paste_special_drawing_area.clone();

        // Try text first
        clipboard.read_text_async(
            gtk4::gio::Cancellable::NONE,
            move |result| {
                if let Ok(Some(text)) = result {
                    if !text.is_empty() {
                        paste_external_text(
                            &text,
                            &state_for_paste,
                            &selected_for_paste,
                            &drawing_for_paste,
                        );
                    }
                }
            },
        );
    });
}

/// Paste internal elements (from copy action)
fn paste_internal_elements(
    elements_to_paste: Vec<testruct_core::document::DocumentElement>,
    paste_state: &crate::app::AppState,
    paste_selected_ids: &Rc<RefCell<Vec<uuid::Uuid>>>,
    paste_drawing_area: &gtk4::DrawingArea,
) {
    // Use clipboard module for cumulative offset
    let offset = crate::clipboard::get_paste_offset();

    let mut elements_with_new_ids: Vec<testruct_core::document::DocumentElement> = elements_to_paste
        .into_iter()
        .map(|mut element| {
            let new_id = uuid::Uuid::new_v4();

            match &mut element {
                testruct_core::document::DocumentElement::Frame(ref mut f) => {
                    f.id = new_id;
                    f.bounds.origin.x += offset.0;
                    f.bounds.origin.y += offset.1;
                }
                testruct_core::document::DocumentElement::Text(ref mut t) => {
                    t.id = new_id;
                    t.bounds.origin.x += offset.0;
                    t.bounds.origin.y += offset.1;
                }
                testruct_core::document::DocumentElement::Image(ref mut i) => {
                    i.id = new_id;
                    i.bounds.origin.x += offset.0;
                    i.bounds.origin.y += offset.1;
                }
                testruct_core::document::DocumentElement::Shape(ref mut s) => {
                    s.id = new_id;
                    s.bounds.origin.x += offset.0;
                    s.bounds.origin.y += offset.1;
                }
                testruct_core::document::DocumentElement::Group(ref mut g) => {
                    g.id = new_id;
                    g.bounds.origin.x += offset.0;
                    g.bounds.origin.y += offset.1;
                }
            }
            element
        })
        .collect();

    paste_state.with_active_document(|doc| {
        let page_index = paste_state.active_page_index();
        if let Some(page) = doc.pages.get_mut(page_index) {
            let pasted_count = elements_with_new_ids.len();
            {
                let mut selected = paste_selected_ids.borrow_mut();
                selected.clear();

                for element in elements_with_new_ids.drain(..) {
                    let element_id = element.id();
                    page.add_element(element);
                    selected.push(element_id);
                }
            }

            tracing::info!("✅ Pasted {} objects from internal clipboard", pasted_count);
        }
    });

    paste_drawing_area.queue_draw();
}

/// Paste external text as TextElement
fn paste_external_text(
    text: &str,
    paste_state: &crate::app::AppState,
    paste_selected_ids: &Rc<RefCell<Vec<uuid::Uuid>>>,
    paste_drawing_area: &gtk4::DrawingArea,
) {
    // Create TextElement from clipboard text
    let text_element = crate::clipboard::create_text_element_from_clipboard(text, None);

    paste_state.with_active_document(|doc| {
        let page_index = paste_state.active_page_index();
        if let Some(page) = doc.pages.get_mut(page_index) {
            let element_id = text_element.id();

            let mut selected = paste_selected_ids.borrow_mut();
            selected.clear();

            page.add_element(text_element);
            selected.push(element_id);

            tracing::info!("✅ Pasted text from external clipboard ({} chars)", text.len());
        }
    });

    paste_drawing_area.queue_draw();
}
