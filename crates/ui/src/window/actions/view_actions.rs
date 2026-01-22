//! View action handlers (grid, guides, rulers, panels visibility, zoom)

use super::common::add_window_action;
use crate::app::AppState;
use gtk4::{prelude::*, Box as GtkBox};

/// Format zoom percentage for display
fn format_zoom_percent(zoom: f64) -> String {
    format!("{:.0}%", zoom * 100.0)
}

/// Register view menu actions
pub fn register(
    window: &gtk4::ApplicationWindow,
    canvas_view: &crate::canvas::CanvasView,
    tool_palette: &GtkBox,
    properties_panel: &GtkBox,
    toolbar_buttons: &crate::toolbar::ToolbarButtons,
    app_state: &AppState,
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
        grid_drawing_area.queue_draw();
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
        guides_drawing_area.queue_draw();
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
        rulers_drawing_area.queue_draw();
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

    // Zoom controls with level display update
    let zoom_out_canvas = canvas_view.drawing_area();
    let zoom_out_state = canvas_view.render_state().clone();
    let zoom_out_btn = toolbar_buttons.zoom_100_btn.clone();
    add_window_action(window, "zoom-out", move |_| {
        tracing::info!("Action: zoom out");
        let mut config = zoom_out_state.config.borrow_mut();
        let new_zoom = (config.zoom * 0.8).max(0.1); // Minimum 10% zoom
        config.zoom = new_zoom;
        drop(config);
        zoom_out_btn.set_label(&format_zoom_percent(new_zoom));
        tracing::info!("✅ Zoom set to {:.0}%", new_zoom * 100.0);
        zoom_out_canvas.queue_draw();
    });

    let zoom_reset_canvas = canvas_view.drawing_area();
    let zoom_reset_state = canvas_view.render_state().clone();
    let zoom_reset_btn = toolbar_buttons.zoom_100_btn.clone();
    add_window_action(window, "zoom-100", move |_| {
        tracing::info!("Action: zoom to 100%");
        let mut config = zoom_reset_state.config.borrow_mut();
        config.zoom = 1.0;
        drop(config);
        zoom_reset_btn.set_label("100%");
        tracing::info!("✅ Zoom reset to 100%");
        zoom_reset_canvas.queue_draw();
    });

    let zoom_in_canvas = canvas_view.drawing_area();
    let zoom_in_state = canvas_view.render_state().clone();
    let zoom_in_btn = toolbar_buttons.zoom_100_btn.clone();
    add_window_action(window, "zoom-in", move |_| {
        tracing::info!("Action: zoom in");
        let mut config = zoom_in_state.config.borrow_mut();
        let new_zoom = (config.zoom * 1.25).min(4.0); // Maximum 400% zoom
        config.zoom = new_zoom;
        drop(config);
        zoom_in_btn.set_label(&format_zoom_percent(new_zoom));
        tracing::info!("✅ Zoom set to {:.0}%", new_zoom * 100.0);
        zoom_in_canvas.queue_draw();
    });

    // Zoom to fit window (Ctrl+1)
    let zoom_fit_canvas = canvas_view.drawing_area();
    let zoom_fit_state = canvas_view.render_state().clone();
    let zoom_fit_btn = toolbar_buttons.zoom_100_btn.clone();
    let zoom_fit_app_state = app_state.clone();
    add_window_action(window, "zoom-fit-window", move |_| {
        tracing::info!("Action: zoom to fit window");

        // Get page size from document
        let page_size = zoom_fit_app_state
            .active_document()
            .map(|doc| doc.metadata.page_size.to_size())
            .unwrap_or(testruct_core::layout::Size::new(800.0, 600.0));

        // Get canvas size
        let canvas_width = zoom_fit_canvas.width() as f64 - 40.0; // margin for rulers
        let canvas_height = zoom_fit_canvas.height() as f64 - 40.0;

        // Calculate zoom to fit
        let zoom_x = canvas_width / page_size.width as f64;
        let zoom_y = canvas_height / page_size.height as f64;
        let new_zoom = zoom_x.min(zoom_y).min(4.0).max(0.1);

        let mut config = zoom_fit_state.config.borrow_mut();
        config.zoom = new_zoom;
        config.pan_x = 0.0;
        config.pan_y = 0.0;
        drop(config);

        zoom_fit_btn.set_label(&format_zoom_percent(new_zoom));
        tracing::info!("✅ Zoom fit to window: {:.0}%", new_zoom * 100.0);
        zoom_fit_canvas.queue_draw();
    });

    // Zoom to fit selection
    let zoom_selection_canvas = canvas_view.drawing_area();
    let zoom_selection_state = canvas_view.render_state().clone();
    let zoom_selection_btn = toolbar_buttons.zoom_100_btn.clone();
    let zoom_selection_app_state = app_state.clone();
    add_window_action(window, "zoom-fit-selection", move |_| {
        tracing::info!("Action: zoom to fit selection");

        let selected_ids = zoom_selection_state.selected_ids.borrow();
        if selected_ids.is_empty() {
            tracing::warn!("⚠️ No objects selected for zoom to fit");
            return;
        }

        // Calculate bounding box of selected elements
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        zoom_selection_app_state.with_active_document(|doc| {
            if let Some(page) = doc.pages.first() {
                for element in &page.elements {
                    if selected_ids.contains(&element.id()) {
                        let bounds = element.bounds();
                        min_x = min_x.min(bounds.origin.x);
                        min_y = min_y.min(bounds.origin.y);
                        max_x = max_x.max(bounds.origin.x + bounds.size.width);
                        max_y = max_y.max(bounds.origin.y + bounds.size.height);
                    }
                }
            }
        });
        drop(selected_ids);

        if min_x == f32::MAX {
            tracing::warn!("⚠️ Could not calculate selection bounds");
            return;
        }

        // Add padding
        let padding = 50.0;
        let selection_width = (max_x - min_x) as f64 + padding * 2.0;
        let selection_height = (max_y - min_y) as f64 + padding * 2.0;

        // Get canvas size
        let canvas_width = zoom_selection_canvas.width() as f64 - 40.0;
        let canvas_height = zoom_selection_canvas.height() as f64 - 40.0;

        // Calculate zoom to fit selection
        let zoom_x = canvas_width / selection_width;
        let zoom_y = canvas_height / selection_height;
        let new_zoom = zoom_x.min(zoom_y).min(4.0).max(0.1);

        // Calculate pan to center selection
        let center_x = (min_x as f64 + max_x as f64) / 2.0;
        let center_y = (min_y as f64 + max_y as f64) / 2.0;
        let pan_x = (canvas_width / 2.0) - (center_x * new_zoom);
        let pan_y = (canvas_height / 2.0) - (center_y * new_zoom);

        let mut config = zoom_selection_state.config.borrow_mut();
        config.zoom = new_zoom;
        config.pan_x = pan_x;
        config.pan_y = pan_y;
        drop(config);

        zoom_selection_btn.set_label(&format_zoom_percent(new_zoom));
        tracing::info!("✅ Zoom fit to selection: {:.0}%", new_zoom * 100.0);
        zoom_selection_canvas.queue_draw();
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
        snap_guides_canvas.queue_draw();
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
        snap_grid_canvas.queue_draw();
    });

    // Toggle layers panel
    add_window_action(window, "toggle-layers", move |_| {
        tracing::info!("Action: toggle layers panel");
        // Layers panel is not yet implemented - placeholder for future integration
        tracing::warn!("⚠️  Layers panel toggle not yet implemented");
    });

    // Toggle properties panel (the right side panel)
    let properties_toggle = properties_panel.clone();
    add_window_action(window, "toggle-properties", move |_| {
        tracing::info!("Action: toggle properties panel");
        let is_visible = properties_toggle.is_visible();
        properties_toggle.set_visible(!is_visible);
        tracing::info!("✅ Properties panel visibility toggled: {}", !is_visible);
    });
}
