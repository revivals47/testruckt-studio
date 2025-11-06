use super::WindowComponents;
use crate::app::AppState;
use gtk4::prelude::*;

pub fn bind_events(components: &WindowComponents, state: AppState) {
    // CanvasView is already configured with event handlers and draw function
    // during CanvasView::new() in layout::build_widgets()

    // Bind toolbar buttons to window actions
    bind_toolbar_buttons(components);

    // Bind tool palette buttons to tool switching
    bind_tool_selection(components);

    // Monitor tool state changes and update UI buttons accordingly
    setup_tool_monitor(components);

    // Monitor selection changes to update property panel
    setup_selection_monitor(components, state);

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

    // Zoom controls
    bind_button(&toolbar.zoom_out_btn, window, "win.zoom-out");
    bind_button(&toolbar.zoom_100_btn, window, "win.zoom-100");
    bind_button(&toolbar.zoom_in_btn, window, "win.zoom-in");

    // Object operations (Secondary toolbar)
    bind_button(&toolbar.group_btn, window, "win.group");
    bind_button(&toolbar.ungroup_btn, window, "win.ungroup");
    bind_button(&toolbar.lock_btn, window, "win.lock");
    bind_button(&toolbar.unlock_btn, window, "win.unlock");

    // Snap toggles
    bind_toggle_button(
        &toolbar.snap_to_guides_toggle,
        window,
        "win.toggle-snap-guides",
    );

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
fn bind_toggle_button(
    button: &gtk4::ToggleButton,
    window: &gtk4::ApplicationWindow,
    action_name: &str,
) {
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
    use crate::canvas::tools::ToolMode;

    let canvas_view = &components.canvas_view;
    let render_state = canvas_view.render_state().clone();
    let tool_buttons = &components.tool_palette_buttons;
    let drawing_area = canvas_view.drawing_area();

    // Select tool button
    let state_select = render_state.clone();
    let drawing_area_select = drawing_area.clone();
    let all_buttons = tool_buttons.clone();
    tool_buttons.select_btn.connect_toggled(move |btn| {
        if btn.is_active() {
            all_buttons.text_btn.set_active(false);
            all_buttons.image_btn.set_active(false);
            all_buttons.rect_btn.set_active(false);
            all_buttons.circle_btn.set_active(false);
            all_buttons.line_btn.set_active(false);
            all_buttons.arrow_btn.set_active(false);
            let mut tool_state = state_select.tool_state.borrow_mut();
            tool_state.current_tool = ToolMode::Select;
            tracing::info!("✅ Tool switched to: Select");
            drop(tool_state);
            drawing_area_select.queue_draw();
        }
    });

    // Text tool button
    let state_text = render_state.clone();
    let drawing_area_text = drawing_area.clone();
    let all_buttons = tool_buttons.clone();
    tool_buttons.text_btn.connect_toggled(move |btn| {
        if btn.is_active() {
            all_buttons.select_btn.set_active(false);
            all_buttons.image_btn.set_active(false);
            all_buttons.rect_btn.set_active(false);
            all_buttons.circle_btn.set_active(false);
            all_buttons.line_btn.set_active(false);
            all_buttons.arrow_btn.set_active(false);
            let mut tool_state = state_text.tool_state.borrow_mut();
            tool_state.current_tool = ToolMode::Text;
            tracing::info!("✅ Tool switched to: 📝 Text");
            drop(tool_state);
            drawing_area_text.queue_draw();
        }
    });

    // Rectangle tool button
    let state_rect = render_state.clone();
    let drawing_area_rect = drawing_area.clone();
    let all_buttons = tool_buttons.clone();
    tool_buttons.rect_btn.connect_toggled(move |btn| {
        if btn.is_active() {
            all_buttons.select_btn.set_active(false);
            all_buttons.text_btn.set_active(false);
            all_buttons.image_btn.set_active(false);
            all_buttons.circle_btn.set_active(false);
            all_buttons.line_btn.set_active(false);
            all_buttons.arrow_btn.set_active(false);
            let mut tool_state = state_rect.tool_state.borrow_mut();
            tool_state.current_tool = ToolMode::Rectangle;
            tracing::info!("✅ Tool switched to: Rectangle");
            drop(tool_state);
            drawing_area_rect.queue_draw();
        }
    });

    // Circle tool button
    let state_circle = render_state.clone();
    let drawing_area_circle = drawing_area.clone();
    let all_buttons = tool_buttons.clone();
    tool_buttons.circle_btn.connect_toggled(move |btn| {
        if btn.is_active() {
            all_buttons.select_btn.set_active(false);
            all_buttons.text_btn.set_active(false);
            all_buttons.image_btn.set_active(false);
            all_buttons.rect_btn.set_active(false);
            all_buttons.line_btn.set_active(false);
            all_buttons.arrow_btn.set_active(false);
            let mut tool_state = state_circle.tool_state.borrow_mut();
            tool_state.current_tool = ToolMode::Circle;
            tracing::info!("✅ Tool switched to: Circle");
            drop(tool_state);
            drawing_area_circle.queue_draw();
        }
    });

    // Line tool button
    let state_line = render_state.clone();
    let drawing_area_line = drawing_area.clone();
    let all_buttons = tool_buttons.clone();
    tool_buttons.line_btn.connect_toggled(move |btn| {
        if btn.is_active() {
            all_buttons.select_btn.set_active(false);
            all_buttons.text_btn.set_active(false);
            all_buttons.image_btn.set_active(false);
            all_buttons.rect_btn.set_active(false);
            all_buttons.circle_btn.set_active(false);
            all_buttons.arrow_btn.set_active(false);
            let mut tool_state = state_line.tool_state.borrow_mut();
            tool_state.current_tool = ToolMode::Line;
            tracing::info!("✅ Tool switched to: Line");
            drop(tool_state);
            drawing_area_line.queue_draw();
        }
    });

    // Arrow tool button
    let state_arrow = render_state.clone();
    let drawing_area_arrow = drawing_area.clone();
    let all_buttons = tool_buttons.clone();
    tool_buttons.arrow_btn.connect_toggled(move |btn| {
        if btn.is_active() {
            all_buttons.select_btn.set_active(false);
            all_buttons.text_btn.set_active(false);
            all_buttons.image_btn.set_active(false);
            all_buttons.rect_btn.set_active(false);
            all_buttons.circle_btn.set_active(false);
            all_buttons.line_btn.set_active(false);
            let mut tool_state = state_arrow.tool_state.borrow_mut();
            tool_state.current_tool = ToolMode::Arrow;
            tracing::info!("✅ Tool switched to: Arrow");
            drop(tool_state);
            drawing_area_arrow.queue_draw();
        }
    });

    // Image tool button
    let state_image = render_state.clone();
    let drawing_area_image = drawing_area.clone();
    let all_buttons = tool_buttons.clone();
    tool_buttons.image_btn.connect_toggled(move |btn| {
        if btn.is_active() {
            all_buttons.select_btn.set_active(false);
            all_buttons.text_btn.set_active(false);
            all_buttons.rect_btn.set_active(false);
            all_buttons.circle_btn.set_active(false);
            all_buttons.line_btn.set_active(false);
            all_buttons.arrow_btn.set_active(false);
            let mut tool_state = state_image.tool_state.borrow_mut();
            tool_state.current_tool = ToolMode::Image;
            tracing::info!("✅ Tool switched to: Image");
            drop(tool_state);
            drawing_area_image.queue_draw();
        }
    });
}

/// Monitor tool state changes and update UI buttons accordingly
fn setup_tool_monitor(components: &WindowComponents) {
    use crate::canvas::tools::ToolMode;
    use gtk4::glib;

    let render_state = components.canvas_view.render_state().clone();
    let tool_buttons = components.tool_palette_buttons.clone();
    let mut last_tool = ToolMode::Select;

    // Set up a periodic check every 100ms to detect tool changes
    glib::source::timeout_add_local(std::time::Duration::from_millis(100), move || {
        let current_tool = render_state.tool_state.borrow().current_tool;

        // Only update UI if tool has changed
        if current_tool != last_tool {
            last_tool = current_tool;

            // Deactivate all buttons first
            tool_buttons.select_btn.set_active(false);
            tool_buttons.text_btn.set_active(false);
            tool_buttons.image_btn.set_active(false);
            tool_buttons.rect_btn.set_active(false);
            tool_buttons.circle_btn.set_active(false);
            tool_buttons.line_btn.set_active(false);
            tool_buttons.arrow_btn.set_active(false);

            // Activate the correct button for the current tool
            match current_tool {
                ToolMode::Select => tool_buttons.select_btn.set_active(true),
                ToolMode::Text => tool_buttons.text_btn.set_active(true),
                ToolMode::Image => tool_buttons.image_btn.set_active(true),
                ToolMode::Rectangle => tool_buttons.rect_btn.set_active(true),
                ToolMode::Circle => tool_buttons.circle_btn.set_active(true),
                ToolMode::Line => tool_buttons.line_btn.set_active(true),
                ToolMode::Arrow => tool_buttons.arrow_btn.set_active(true),
                ToolMode::Pan => {} // Pan tool doesn't have a button in the palette
            }
        }

        glib::ControlFlow::Continue
    });
}

/// Monitor selection changes and update property panel accordingly
fn setup_selection_monitor(components: &WindowComponents, app_state: AppState) {
    use gtk4::glib;

    let render_state = components.canvas_view.render_state().clone();
    let property_components = components.property_components.clone();
    let mut last_selection: Vec<uuid::Uuid> = Vec::new();

    // Set up a periodic check every 50ms to detect selection changes
    glib::source::timeout_add_local(std::time::Duration::from_millis(50), move || {
        let current_selection = render_state.selected_ids.borrow().clone();

        // Only update property panel if selection has changed
        if current_selection != last_selection {
            last_selection = current_selection.clone();
            tracing::debug!("✅ Selection changed, updating property panel");

            // Update the property panel text buffer with the selected element's content
            crate::panels::update_property_panel_on_selection(
                &property_components,
                &app_state,
                &current_selection,
            );
        }

        glib::ControlFlow::Continue
    });
}
