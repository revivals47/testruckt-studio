//! View action handlers (grid, guides, rulers, panels visibility)

use super::common::add_window_action;
use gtk4::{prelude::*, Box as GtkBox, ToggleButton};

/// Register view menu actions
pub fn register(
    window: &gtk4::ApplicationWindow,
    canvas_view: &crate::canvas::CanvasView,
    tool_palette: &GtkBox,
    properties_panel: &GtkBox,
    toolbar_buttons: &crate::toolbar::ToolbarButtons,
) {
    let grid_drawing_area = canvas_view.drawing_area();
    let grid_render_state = canvas_view.render_state().clone();
    let grid_btn = toolbar_buttons.grid_toggle_btn.clone();
    add_window_action(window, "toggle-grid", move |_| {
        tracing::info!("Action: toggle grid visibility");
        let mut config = grid_render_state.config.borrow_mut();
        config.show_grid = !config.show_grid;
        let new_state = config.show_grid;
        drop(config);
        tracing::info!("✅ Grid visibility toggled: {}", new_state);
        grid_btn.set_label(if new_state {
            "グリッド: ON"
        } else {
            "グリッド: OFF"
        });
        let _ = grid_drawing_area.queue_draw();
    });

    let guides_drawing_area = canvas_view.drawing_area();
    let guides_render_state = canvas_view.render_state().clone();
    let guides_btn = toolbar_buttons.guides_visible_btn.clone();
    add_window_action(window, "toggle-guides", move |_| {
        tracing::info!("Action: toggle guides visibility");
        let mut config = guides_render_state.config.borrow_mut();
        config.show_guides = !config.show_guides;
        let new_state = config.show_guides;
        drop(config);
        tracing::info!("✅ Guides visibility toggled: {}", new_state);
        guides_btn.set_label(if new_state {
            "ガイド: ON"
        } else {
            "ガイド: OFF"
        });
        let _ = guides_drawing_area.queue_draw();
    });

    let rulers_drawing_area = canvas_view.drawing_area();
    let rulers_render_state = canvas_view.render_state().clone();
    let rulers_btn = toolbar_buttons.rulers_visible_btn.clone();
    add_window_action(window, "toggle-rulers", move |_| {
        tracing::info!("Action: toggle rulers");
        let mut config = rulers_render_state.config.borrow_mut();
        config.show_rulers = !config.show_rulers;
        let new_state = config.show_rulers;
        drop(config);
        tracing::info!("✅ Rulers visibility toggled: {}", new_state);
        rulers_btn.set_label(if new_state {
            "ルーラー: ON"
        } else {
            "ルーラー: OFF"
        });
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

    // Zoom controls
    let zoom_out_canvas = canvas_view.drawing_area();
    let zoom_out_state = canvas_view.render_state().clone();
    add_window_action(window, "zoom-out", move |_| {
        tracing::info!("Action: zoom out");
        let mut config = zoom_out_state.config.borrow_mut();
        let new_zoom = (config.zoom * 0.8).max(0.1); // Minimum 10% zoom
        config.zoom = new_zoom;
        drop(config);
        tracing::info!("✅ Zoom set to {:.0}%", new_zoom * 100.0);
        let _ = zoom_out_canvas.queue_draw();
    });

    let zoom_reset_canvas = canvas_view.drawing_area();
    let zoom_reset_state = canvas_view.render_state().clone();
    add_window_action(window, "zoom-100", move |_| {
        tracing::info!("Action: zoom to 100%");
        let mut config = zoom_reset_state.config.borrow_mut();
        config.zoom = 1.0;
        drop(config);
        tracing::info!("✅ Zoom reset to 100%");
        let _ = zoom_reset_canvas.queue_draw();
    });

    let zoom_in_canvas = canvas_view.drawing_area();
    let zoom_in_state = canvas_view.render_state().clone();
    add_window_action(window, "zoom-in", move |_| {
        tracing::info!("Action: zoom in");
        let mut config = zoom_in_state.config.borrow_mut();
        let new_zoom = (config.zoom * 1.25).min(4.0); // Maximum 400% zoom
        config.zoom = new_zoom;
        drop(config);
        tracing::info!("✅ Zoom set to {:.0}%", new_zoom * 100.0);
        let _ = zoom_in_canvas.queue_draw();
    });

    // Snap to guides toggle
    let snap_guides_canvas = canvas_view.drawing_area();
    let snap_guides_state = canvas_view.render_state().clone();
    add_window_action(window, "toggle-snap-guides", move |_| {
        tracing::info!("Action: toggle snap to guides");
        let mut config = snap_guides_state.config.borrow_mut();
        config.snap_to_guides = !config.snap_to_guides;
        let new_state = config.snap_to_guides;
        drop(config);
        tracing::info!("✅ Snap to guides toggled: {}", new_state);
        let _ = snap_guides_canvas.queue_draw();
    });

    // Snap to grid toggle
    let snap_grid_canvas = canvas_view.drawing_area();
    let snap_grid_state = canvas_view.render_state().clone();
    add_window_action(window, "toggle-snap-grid", move |_| {
        tracing::info!("Action: toggle snap to grid");
        let mut config = snap_grid_state.config.borrow_mut();
        config.snap_to_grid = !config.snap_to_grid;
        let new_state = config.snap_to_grid;
        drop(config);
        tracing::info!("✅ Snap to grid toggled: {}", new_state);
        let _ = snap_grid_canvas.queue_draw();
    });
}
