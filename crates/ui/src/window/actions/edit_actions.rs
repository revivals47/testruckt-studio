//! Edit action handlers (undo, redo, select-all)

use super::common::add_window_action;
use gtk4::prelude::*;

/// Register edit menu actions
pub fn register(
    window: &gtk4::ApplicationWindow,
    state: crate::app::AppState,
    canvas_view: &crate::canvas::CanvasView,
) {
    let undo_state = state.clone();
    let undo_drawing_area = canvas_view.drawing_area();
    add_window_action(window, "undo", move |_| {
        tracing::info!("Action: undo");
        if undo_state.undo() {
            tracing::info!("✅ Undo successful");
            let _ = undo_drawing_area.queue_draw();
        } else {
            tracing::info!("⚠️  Nothing to undo");
        }
    });

    let redo_state = state.clone();
    let redo_drawing_area = canvas_view.drawing_area();
    add_window_action(window, "redo", move |_| {
        tracing::info!("Action: redo");
        if redo_state.redo() {
            tracing::info!("✅ Redo successful");
            let _ = redo_drawing_area.queue_draw();
        } else {
            tracing::info!("⚠️  Nothing to redo");
        }
    });

    let select_all_state = state.clone();
    let select_all_drawing_area = canvas_view.drawing_area();
    let select_all_render_state = canvas_view.render_state().clone();
    add_window_action(window, "select-all", move |_| {
        tracing::info!("Action: select all objects");

        let all_ids = select_all_state.get_all_object_ids();

        if all_ids.is_empty() {
            tracing::info!("⚠️  No objects to select");
        } else {
            let mut selected = select_all_render_state.selected_ids.borrow_mut();
            selected.clear();
            for id in &all_ids {
                selected.push(*id);
            }
            drop(selected);
            let _ = select_all_drawing_area.queue_draw();
            tracing::info!("✅ Selected {} objects", all_ids.len());
        }
    });

    // Delete selected objects
    let delete_state = state.clone();
    let delete_drawing_area = canvas_view.drawing_area();
    let delete_render_state = canvas_view.render_state().clone();
    add_window_action(window, "delete", move |_| {
        tracing::info!("Action: delete selected objects");

        let selected_ids: Vec<uuid::Uuid> = {
            delete_render_state.selected_ids.borrow().clone()
        };

        if selected_ids.is_empty() {
            tracing::info!("⚠️  No objects selected to delete");
            return;
        }

        let deleted_count = delete_state.with_active_page(|page| {
            let before_count = page.elements.len();
            page.elements.retain(|element| {
                let element_id = match element {
                    testruct_core::document::DocumentElement::Shape(s) => s.id,
                    testruct_core::document::DocumentElement::Text(t) => t.id,
                    testruct_core::document::DocumentElement::Image(i) => i.id,
                    testruct_core::document::DocumentElement::Frame(f) => f.id,
                    testruct_core::document::DocumentElement::Group(g) => g.id,
                };
                !selected_ids.contains(&element_id)
            });
            before_count - page.elements.len()
        }).unwrap_or(0);

        // Clear selection
        delete_render_state.selected_ids.borrow_mut().clear();
        delete_drawing_area.queue_draw();

        tracing::info!("✅ Deleted {} object(s)", deleted_count);
    });

    // Duplicate selected objects
    let duplicate_state = state.clone();
    let duplicate_drawing_area = canvas_view.drawing_area();
    let duplicate_render_state = canvas_view.render_state().clone();
    add_window_action(window, "duplicate", move |_| {
        tracing::info!("Action: duplicate selected objects");

        let selected_ids: Vec<uuid::Uuid> = {
            duplicate_render_state.selected_ids.borrow().clone()
        };

        if selected_ids.is_empty() {
            tracing::info!("⚠️  No objects selected to duplicate");
            return;
        }

        let new_ids = duplicate_state.with_active_page(|page| {
            let mut new_elements = Vec::new();
            let mut new_ids = Vec::new();

            for element in &page.elements {
                let element_id = match element {
                    testruct_core::document::DocumentElement::Shape(s) => s.id,
                    testruct_core::document::DocumentElement::Text(t) => t.id,
                    testruct_core::document::DocumentElement::Image(i) => i.id,
                    testruct_core::document::DocumentElement::Frame(f) => f.id,
                    testruct_core::document::DocumentElement::Group(g) => g.id,
                };

                if selected_ids.contains(&element_id) {
                    let mut cloned = element.clone();
                    let new_id = uuid::Uuid::new_v4();
                    // Offset the duplicated element
                    match &mut cloned {
                        testruct_core::document::DocumentElement::Shape(s) => {
                            s.id = new_id;
                            s.bounds.origin.x += 20.0;
                            s.bounds.origin.y += 20.0;
                        }
                        testruct_core::document::DocumentElement::Text(t) => {
                            t.id = new_id;
                            t.bounds.origin.x += 20.0;
                            t.bounds.origin.y += 20.0;
                        }
                        testruct_core::document::DocumentElement::Image(i) => {
                            i.id = new_id;
                            i.bounds.origin.x += 20.0;
                            i.bounds.origin.y += 20.0;
                        }
                        testruct_core::document::DocumentElement::Frame(f) => {
                            f.id = new_id;
                            f.bounds.origin.x += 20.0;
                            f.bounds.origin.y += 20.0;
                        }
                        testruct_core::document::DocumentElement::Group(g) => {
                            g.id = new_id;
                            g.bounds.origin.x += 20.0;
                            g.bounds.origin.y += 20.0;
                        }
                    }
                    new_ids.push(new_id);
                    new_elements.push(cloned);
                }
            }

            page.elements.extend(new_elements);
            new_ids
        }).unwrap_or_default();

        // Select the duplicated objects
        {
            let mut selected = duplicate_render_state.selected_ids.borrow_mut();
            selected.clear();
            selected.extend(new_ids.iter());
        }
        duplicate_drawing_area.queue_draw();

        tracing::info!("✅ Duplicated {} object(s)", new_ids.len());
    });

    // Cut selected objects to clipboard
    let cut_state = state.clone();
    let cut_drawing_area = canvas_view.drawing_area();
    let cut_render_state = canvas_view.render_state().clone();
    add_window_action(window, "cut", move |_| {
        tracing::info!("Action: cut selected objects");

        let selected_ids: Vec<uuid::Uuid> = {
            cut_render_state.selected_ids.borrow().clone()
        };

        if selected_ids.is_empty() {
            tracing::info!("⚠️  No objects selected to cut");
            return;
        }

        // Copy to clipboard first
        let copied = cut_state.with_active_page(|page| {
            let mut copied_elements = Vec::new();
            for element in &page.elements {
                let element_id = match element {
                    testruct_core::document::DocumentElement::Shape(s) => s.id,
                    testruct_core::document::DocumentElement::Text(t) => t.id,
                    testruct_core::document::DocumentElement::Image(i) => i.id,
                    testruct_core::document::DocumentElement::Frame(f) => f.id,
                    testruct_core::document::DocumentElement::Group(g) => g.id,
                };
                if selected_ids.contains(&element_id) {
                    copied_elements.push(element.clone());
                }
            }
            copied_elements
        }).unwrap_or_default();

        // Store in clipboard
        crate::clipboard::copy_to_clipboard(copied);

        // Delete the elements
        let deleted_count = cut_state.with_active_page(|page| {
            let before_count = page.elements.len();
            page.elements.retain(|element| {
                let element_id = match element {
                    testruct_core::document::DocumentElement::Shape(s) => s.id,
                    testruct_core::document::DocumentElement::Text(t) => t.id,
                    testruct_core::document::DocumentElement::Image(i) => i.id,
                    testruct_core::document::DocumentElement::Frame(f) => f.id,
                    testruct_core::document::DocumentElement::Group(g) => g.id,
                };
                !selected_ids.contains(&element_id)
            });
            before_count - page.elements.len()
        }).unwrap_or(0);

        // Clear selection
        cut_render_state.selected_ids.borrow_mut().clear();
        cut_drawing_area.queue_draw();

        tracing::info!("✅ Cut {} object(s) to clipboard", deleted_count);
    });
}
