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
}
