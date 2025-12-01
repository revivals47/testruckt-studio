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

mod alignment_actions;
mod clipboard_actions;
mod common;
mod edit_actions;
mod export_actions;
mod file_actions;
mod group_actions;
mod help_actions;
mod layer_actions;
mod tools_actions;
mod view_actions;

use crate::window::actions::common::add_window_action;
use gtk4::prelude::*;
use gtk4::Box as GtkBox;

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
    file_actions::register(window, state.clone(), canvas_view);
    export_actions::register(window, state.clone());
    edit_actions::register(window, state.clone(), canvas_view);
    view_actions::register(
        window,
        canvas_view,
        tool_palette,
        properties_panel,
        toolbar_buttons,
    );
    tools_actions::register(window, state.clone(), canvas_view, property_components);
    group_actions::register(window, state.clone(), canvas_view, property_components);
    clipboard_actions::register(window, state.clone(), canvas_view);
    layer_actions::register(window, state.clone(), canvas_view);
    alignment_actions::register(window, state.clone(), canvas_view, property_components);
    help_actions::register(window, state.clone());

    // Register block tools toggle action
    add_window_action(window, "toggle-block-tools", |_| {
        tracing::info!("Action: toggle block tools");
        // Block tools panel toggle is handled by the toolbar button binding
        // The visibility state is managed by the GTK button's active state
    });

    // Register lock action
    let lock_state = state.clone();
    let lock_render_state = canvas_view.render_state().selected_ids.clone();
    let lock_drawing_area = canvas_view.drawing_area();
    add_window_action(window, "lock", move |_| {
        tracing::info!("Action: lock selected objects");
        let selected_ids_vec: Vec<uuid::Uuid> = {
            let selected_ids = lock_render_state.borrow();
            if selected_ids.is_empty() {
                tracing::warn!("⚠️  No objects selected to lock");
                return;
            }
            selected_ids.clone().into_iter().collect()
        };

        let locked_count = lock_state.with_active_page(|page| {
            let mut count = 0;
            for element in &mut page.elements {
                if selected_ids_vec.contains(&element.id()) {
                    element.set_locked(true);
                    count += 1;
                }
            }
            count
        }).unwrap_or(0);

        tracing::info!("✅ {} object(s) locked", locked_count);
        lock_drawing_area.queue_draw();
    });

    // Register unlock action
    let unlock_state = state.clone();
    let unlock_render_state = canvas_view.render_state().selected_ids.clone();
    let unlock_drawing_area = canvas_view.drawing_area();
    add_window_action(window, "unlock", move |_| {
        tracing::info!("Action: unlock selected objects");
        let selected_ids_vec: Vec<uuid::Uuid> = {
            let selected_ids = unlock_render_state.borrow();
            if selected_ids.is_empty() {
                tracing::warn!("⚠️  No objects selected to unlock");
                return;
            }
            selected_ids.clone().into_iter().collect()
        };

        let unlocked_count = unlock_state.with_active_page(|page| {
            let mut count = 0;
            for element in &mut page.elements {
                if selected_ids_vec.contains(&element.id()) {
                    element.set_locked(false);
                    count += 1;
                }
            }
            count
        }).unwrap_or(0);

        tracing::info!("✅ {} object(s) unlocked", unlocked_count);
        unlock_drawing_area.queue_draw();
    });

    // Set keyboard accelerators
    common::set_accelerators(window);
}
