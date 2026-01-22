//! Object grouping/ungrouping action handlers

use super::common::add_window_action;
use gtk4::prelude::*;

/// Register group/ungroup actions
pub fn register(
    window: &gtk4::ApplicationWindow,
    state: crate::app::AppState,
    canvas_view: &crate::canvas::CanvasView,
    property_components: &crate::panels::PropertyPanelComponents,
) {
    let group_state = state.clone();
    let group_drawing_area = canvas_view.drawing_area();
    let group_render_state = canvas_view.render_state().selected_ids.clone();
    add_window_action(window, "group", move |_| {
        tracing::info!("Action: group selected objects");

        let selected_ids: Vec<uuid::Uuid> = {
            let ids = group_render_state.borrow();
            if ids.is_empty() {
                tracing::warn!("⚠️  No objects selected for grouping");
                return;
            }

            if ids.len() < 2 {
                tracing::warn!("⚠️  Need at least 2 objects to group");
                return;
            }
            ids.clone()
        };

        // Create and execute group command with undo support
        let page_index = group_state.active_page_index();
        let command = crate::undo_redo::AppGroupCommand::new(
            group_state.clone(),
            selected_ids,
            page_index,
        );
        group_state.push_command(Box::new(command));
        group_state.mark_as_modified();

        // Clear selection and redraw
        group_render_state.borrow_mut().clear();
        group_drawing_area.queue_draw();
        tracing::info!("✅ Grouped objects (with undo support)");
    });

    let ungroup_state = state.clone();
    let ungroup_drawing_area = canvas_view.drawing_area();
    let ungroup_render_state = canvas_view.render_state().selected_ids.clone();
    add_window_action(window, "ungroup", move |_| {
        tracing::info!("Action: ungroup selected objects");

        // Extract selected IDs and drop the borrow immediately
        let selected_ids_vec: Vec<uuid::Uuid> = {
            let selected_ids = ungroup_render_state.borrow();
            if selected_ids.is_empty() {
                tracing::warn!("⚠️  No objects selected for ungrouping");
                return;
            }
            selected_ids.clone()
        };

        // Find frame IDs that are groups (frames with children)
        let frame_ids_to_ungroup: Vec<uuid::Uuid> = ungroup_state
            .with_active_document(|doc| {
                let mut frame_ids = Vec::new();
                if let Some(page) = doc.pages.first() {
                    for element in &page.elements {
                        if selected_ids_vec.contains(&element.id()) {
                            if let testruct_core::document::DocumentElement::Frame(frame) = element {
                                if !frame.children.is_empty() {
                                    frame_ids.push(frame.id);
                                }
                            }
                        }
                    }
                }
                frame_ids
            })
            .unwrap_or_default();

        if frame_ids_to_ungroup.is_empty() {
            tracing::warn!("⚠️  Selected items are not groups");
            return;
        }

        // Create and execute ungroup commands with undo support
        let page_index = ungroup_state.active_page_index();

        for frame_id in &frame_ids_to_ungroup {
            let command = crate::undo_redo::AppUngroupCommand::new(
                ungroup_state.clone(),
                *frame_id,
                page_index,
            );
            ungroup_state.push_command(Box::new(command));
        }

        ungroup_state.mark_as_modified();

        // Clear selection and redraw
        ungroup_render_state.borrow_mut().clear();
        ungroup_drawing_area.queue_draw();
        tracing::info!("✅ Ungrouped {} groups (with undo support)", frame_ids_to_ungroup.len());
    });

    // Wire property panel ungroup button to the same action
    let ungroup_button = property_components.ungroup_btn.clone();
    let window_for_button = window.clone();
    ungroup_button.connect_clicked(move |_| {
        if let Some(action) = window_for_button.lookup_action("ungroup") {
            action.activate(None);
        } else {
            tracing::warn!("⚠️  Ungroup action not found on window");
        }
    });
}
