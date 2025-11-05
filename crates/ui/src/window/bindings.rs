use super::WindowComponents;
use crate::app::AppState;
use gtk4::prelude::*;

pub fn bind_events(components: &WindowComponents, _state: AppState) {
    // CanvasView is already configured with event handlers and draw function
    // during CanvasView::new() in layout::build_widgets()

    // Bind toolbar buttons to window actions
    bind_toolbar_buttons(components);

    // Bind tool palette buttons to tool switching
    bind_tool_selection(components);

    components.window.set_focus_visible(true);
}

/// Bind toolbar buttons to corresponding window actions
fn bind_toolbar_buttons(components: &WindowComponents) {
    let toolbar = &components.toolbar.buttons;
    let window = &components.window;

    // Document operations
    bind_button(&toolbar.new_btn, window, "win.new");
    bind_button(&toolbar.open_btn, window, "win.open");
    bind_button(&toolbar.save_btn, window, "win.save");
    bind_button(&toolbar.export_btn, window, "win.export-pdf");
    bind_button(&toolbar.image_export_btn, window, "win.export-png");

    // History operations
    bind_button(&toolbar.undo_btn, window, "win.undo");
    bind_button(&toolbar.redo_btn, window, "win.redo");

    // Workflow shortcuts
    bind_button(&toolbar.template_btn, window, "win.templates");
    bind_button(&toolbar.json_editor_btn, window, "win.open-json-editor");
    bind_button(&toolbar.settings_btn, window, "win.settings");

    // View toggles
    bind_toggle_button(&toolbar.grid_toggle_btn, window, "win.toggle-grid");
    bind_toggle_button(&toolbar.guides_visible_btn, window, "win.toggle-guides");
    bind_toggle_button(&toolbar.rulers_visible_btn, window, "win.toggle-rulers");
    bind_toggle_button(&toolbar.item_library_btn, window, "win.toggle-item-library");
    bind_toggle_button(&toolbar.block_tools_btn, window, "win.toggle-block-tools");

    // Popover toggles
    bind_toggle_button(&toolbar.ruler_menu_toggle, window, "win.toggle-rulers");
    bind_toggle_button(&toolbar.guides_menu_toggle, window, "win.toggle-guides");
}

/// Bind a regular button to a window action
fn bind_button(button: &gtk4::Button, window: &gtk4::ApplicationWindow, action_name: &str) {
    let action_name = action_name.to_string();
    let window_weak = window.downgrade();

    button.connect_clicked(move |_| {
        if let Some(window) = window_weak.upgrade() {
            // Activate the action
            if let Some(action) = window.lookup_action(&action_name[4..]) {
                action.activate(None);
            }
        }
    });
}

/// Bind a toggle button to a window action
fn bind_toggle_button(button: &gtk4::ToggleButton, window: &gtk4::ApplicationWindow, action_name: &str) {
    let action_name = action_name.to_string();
    let window_weak = window.downgrade();

    button.connect_toggled(move |_| {
        if let Some(window) = window_weak.upgrade() {
            // Activate the action
            if let Some(action) = window.lookup_action(&action_name[4..]) {
                action.activate(None);
            }
        }
    });
}

/// Bind tool palette buttons to tool mode switching
fn bind_tool_selection(components: &WindowComponents) {
    let canvas_view = &components.canvas_view;
    let render_state = canvas_view.render_state().clone();

    // Tool buttons are in the left tool palette (built in layout_v2)
    // For now, we'll connect to the toolbar tool buttons if available
    // TODO: Connect to left palette buttons when available in layout

    // Select tool button
    let state_select = render_state.clone();
    // TODO: Connect select_btn from tool palette to switching to Select mode
    let _state = state_select; // Placeholder

    // Rectangle tool button
    let state_rect = render_state.clone();
    let _state = state_rect; // Placeholder

    // Circle tool button
    let state_circle = render_state.clone();
    let _state = state_circle; // Placeholder

    // Text tool button
    let state_text = render_state.clone();
    let _state = state_text; // Placeholder

    // Line tool button
    let state_line = render_state.clone();
    let _state = state_line; // Placeholder
}
