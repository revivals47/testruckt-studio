//! Layer management actions (reordering, visibility, deletion)

use super::common::add_window_action;
use gtk4::prelude::*;

/// Register layer management actions
pub fn register(
    window: &gtk4::ApplicationWindow,
    state: crate::app::AppState,
    canvas_view: &crate::canvas::CanvasView,
) {
    let drawing_area = canvas_view.drawing_area();
    let render_state = canvas_view.render_state().clone();

    // Move layer up (Ctrl+])
    let move_up_state = state.clone();
    let move_up_drawing = drawing_area.clone();
    let move_up_render = render_state.clone();
    add_window_action(window, "move-layer-up", move |_| {
        let selected = move_up_render.selected_ids.borrow();
        if let Some(element_id) = selected.first().copied() {
            drop(selected);
            crate::panels::reorder_layer(
                &move_up_state,
                element_id,
                crate::panels::LayerDirection::Up,
                move_up_drawing.clone(),
            );
        }
    });

    // Move layer down (Ctrl+[)
    let move_down_state = state.clone();
    let move_down_drawing = drawing_area.clone();
    let move_down_render = render_state.clone();
    add_window_action(window, "move-layer-down", move |_| {
        let selected = move_down_render.selected_ids.borrow();
        if let Some(element_id) = selected.first().copied() {
            drop(selected);
            crate::panels::reorder_layer(
                &move_down_state,
                element_id,
                crate::panels::LayerDirection::Down,
                move_down_drawing.clone(),
            );
        }
    });
}
