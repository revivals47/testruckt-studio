//! View action handlers (grid, guides, rulers, panels visibility)

use super::common::add_window_action;
use gtk4::{prelude::*, Box as GtkBox};

/// Register view menu actions
pub fn register(
    window: &gtk4::ApplicationWindow,
    canvas_view: &crate::canvas::CanvasView,
    tool_palette: &GtkBox,
    properties_panel: &GtkBox,
) {
    let grid_drawing_area = canvas_view.drawing_area();
    let grid_render_state = canvas_view.render_state().clone();
    add_window_action(window, "toggle-grid", move |_| {
        tracing::info!("Action: toggle grid visibility");
        let mut config = grid_render_state.config.borrow_mut();
        config.show_grid = !config.show_grid;
        let new_state = config.show_grid;
        drop(config);
        tracing::info!("✅ Grid visibility toggled: {}", new_state);
        let _ = grid_drawing_area.queue_draw();
    });

    let guides_drawing_area = canvas_view.drawing_area();
    let guides_render_state = canvas_view.render_state().clone();
    add_window_action(window, "toggle-guides", move |_| {
        tracing::info!("Action: toggle guides visibility");
        let mut config = guides_render_state.config.borrow_mut();
        config.show_guides = !config.show_guides;
        let new_state = config.show_guides;
        drop(config);
        tracing::info!("✅ Guides visibility toggled: {}", new_state);
        let _ = guides_drawing_area.queue_draw();
    });

    let rulers_drawing_area = canvas_view.drawing_area();
    let rulers_render_state = canvas_view.render_state().clone();
    add_window_action(window, "toggle-rulers", move |_| {
        tracing::info!("Action: toggle rulers");
        let mut config = rulers_render_state.config.borrow_mut();
        config.show_rulers = !config.show_rulers;
        let new_state = config.show_rulers;
        drop(config);
        tracing::info!("✅ Rulers visibility toggled: {}", new_state);
        let _ = rulers_drawing_area.queue_draw();
    });

    let tool_palette_toggle = tool_palette.clone();
    add_window_action(window, "toggle-tool-palette", move |_| {
        tracing::info!("Action: toggle tool palette");
        let is_visible = tool_palette_toggle.is_visible();
        tool_palette_toggle.set_visible(!is_visible);
        tracing::info!("✅ Tool palette visibility toggled: {}", !is_visible);
    });

    let item_library_panel = properties_panel.clone();
    add_window_action(window, "toggle-item-library", move |_| {
        tracing::info!("Action: toggle item library");
        let is_visible = item_library_panel.is_visible();
        item_library_panel.set_visible(!is_visible);
        tracing::info!("✅ Item library visibility toggled: {}", !is_visible);
    });
}
