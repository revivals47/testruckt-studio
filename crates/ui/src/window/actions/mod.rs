//! Window-level action handlers for menu and toolbar events
//!
//! This module organizes action callbacks into logical sections:
//! - File operations (new, open, save, save-as)
//! - Export operations (PDF, PNG, JPEG, SVG)
//! - Edit operations (undo, redo, select-all)
//! - View toggles (grid, guides, rulers)
//! - Page management (add, delete, duplicate, move pages)
//! - Tool operations (image insertion, templates)
//! - Z-order operations (bring-to-front, send-to-back, etc.)
//! - Grouping operations (group, ungroup)
//! - Clipboard operations (copy, paste)
//! - Help operations (manual, about, settings)

mod common;
mod file_actions;
mod export_actions;
mod edit_actions;
mod view_actions;
mod tools_actions;
mod group_actions;
mod clipboard_actions;
mod help_actions;
mod layer_actions;
mod alignment_actions;

use gtk4::Box as GtkBox;
use crate::window::actions::common::add_window_action;

/// Register all window-level actions
pub fn register_window_actions(
    window: &gtk4::ApplicationWindow,
    state: crate::app::AppState,
    canvas_view: &crate::canvas::CanvasView,
    tool_palette: &GtkBox,
    properties_panel: &GtkBox,
    property_components: &crate::panels::PropertyPanelComponents,
    toolbar_buttons: &crate::toolbar::ToolbarButtons,
) {
    // Register actions from each module
    file_actions::register(window, state.clone());
    export_actions::register(window, state.clone());
    edit_actions::register(window, state.clone(), canvas_view);
    view_actions::register(window, canvas_view, tool_palette, properties_panel, toolbar_buttons);
    tools_actions::register(window, state.clone(), canvas_view, property_components);
    group_actions::register(window, state.clone(), canvas_view);
    clipboard_actions::register(window, state.clone(), canvas_view);
    layer_actions::register(window, state.clone(), canvas_view);
    alignment_actions::register(window, state.clone(), canvas_view);
    help_actions::register(window, state.clone());

    // Register block tools toggle action
    add_window_action(window, "toggle-block-tools", |_| {
        tracing::info!("Action: toggle block tools");
        // Block tools panel toggle is handled by the toolbar button binding
        // The visibility state is managed by the GTK button's active state
    });

    // Register lock action
    let lock_render_state = canvas_view.render_state().selected_ids.clone();
    add_window_action(window, "lock", move |_| {
        tracing::info!("Action: lock selected objects");
        let selected_ids = lock_render_state.borrow();
        if selected_ids.is_empty() {
            tracing::warn!("⚠️  No objects selected to lock");
            return;
        }

        // TODO: Implement actual lock logic when element lock property is added
        tracing::info!("✅ {} object(s) locked (lock feature in progress)", selected_ids.len());
    });

    // Register unlock action
    let unlock_render_state = canvas_view.render_state().selected_ids.clone();
    add_window_action(window, "unlock", move |_| {
        tracing::info!("Action: unlock selected objects");
        let selected_ids = unlock_render_state.borrow();
        if selected_ids.is_empty() {
            tracing::warn!("⚠️  No objects selected to unlock");
            return;
        }

        // TODO: Implement actual unlock logic when element lock property is added
        tracing::info!("✅ {} object(s) unlocked (unlock feature in progress)", selected_ids.len());
    });

    // Set keyboard accelerators
    common::set_accelerators(window);
}
