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

        let selected_ids = group_render_state.borrow();
        if selected_ids.is_empty() {
            tracing::warn!("⚠️  No objects selected for grouping");
            return;
        }

        if selected_ids.len() < 2 {
            tracing::warn!("⚠️  Need at least 2 objects to group");
            return;
        }

        group_state.with_active_document(|doc| {
            if let Some(page) = doc.pages.get_mut(0) {
                // Calculate bounding box of all selected elements
                let mut min_x = f32::MAX;
                let mut min_y = f32::MAX;
                let mut max_x = f32::MIN;
                let mut max_y = f32::MIN;

                // First pass: calculate bounds
                for element in &page.elements {
                    if selected_ids.contains(&element.id()) {
                        let bounds = match element {
                            testruct_core::document::DocumentElement::Frame(f) => &f.bounds,
                            testruct_core::document::DocumentElement::Text(t) => &t.bounds,
                            testruct_core::document::DocumentElement::Image(i) => &i.bounds,
                            testruct_core::document::DocumentElement::Shape(s) => &s.bounds,
                        };

                        min_x = min_x.min(bounds.origin.x);
                        min_y = min_y.min(bounds.origin.y);
                        max_x = max_x.max(bounds.origin.x + bounds.size.width);
                        max_y = max_y.max(bounds.origin.y + bounds.size.height);
                    }
                }

                // Create group frame with calculated bounds
                let group_bounds = testruct_core::layout::Rect::new(
                    testruct_core::layout::Point::new(min_x, min_y),
                    testruct_core::layout::Size::new(max_x - min_x, max_y - min_y),
                );

                let mut group = testruct_core::document::FrameElement {
                    id: uuid::Uuid::new_v4(),
                    bounds: group_bounds,
                    children: Vec::new(),
                };

                // Second pass: move selected elements to group
                let mut indices_to_remove = Vec::new();
                for (i, element) in page.elements.iter().enumerate() {
                    if selected_ids.contains(&element.id()) {
                        indices_to_remove.push(i);
                    }
                }

                // Remove in reverse order to maintain indices
                for i in indices_to_remove.iter().rev() {
                    let element = page.elements.remove(*i);
                    group.children.push(element);
                }

                // Reverse to maintain original order
                group.children.reverse();

                // Add group to page
                page.add_element(testruct_core::document::DocumentElement::Frame(
                    group.clone(),
                ));

                tracing::info!("✅ Grouped {} objects", group.children.len());
            }
        });

        // Clear selection and redraw
        group_render_state.borrow_mut().clear();
        let _ = group_drawing_area.queue_draw();
    });

    let ungroup_state = state.clone();
    let ungroup_drawing_area = canvas_view.drawing_area();
    let ungroup_render_state = canvas_view.render_state().selected_ids.clone();
    add_window_action(window, "ungroup", move |_| {
        tracing::info!("Action: ungroup selected objects");

        let selected_ids = ungroup_render_state.borrow();
        if selected_ids.is_empty() {
            tracing::warn!("⚠️  No objects selected for ungrouping");
            return;
        }

        ungroup_state.with_active_document(|doc| {
            if let Some(page) = doc.pages.get_mut(0) {
                let mut indices_to_ungroup = Vec::new();
                let mut children_to_add = Vec::new();

                // Find group frames to ungroup
                for (i, element) in page.elements.iter().enumerate() {
                    if selected_ids.contains(&element.id()) {
                        if let testruct_core::document::DocumentElement::Frame(frame) = element {
                            indices_to_ungroup.push(i);
                            children_to_add.extend(frame.children.iter().cloned());
                        }
                    }
                }

                if indices_to_ungroup.is_empty() {
                    tracing::warn!("⚠️  Selected items are not groups");
                    return;
                }

                // Remove groups in reverse order
                for i in indices_to_ungroup.iter().rev() {
                    page.elements.remove(*i);
                }

                // Add children back to page
                page.elements.extend(children_to_add.clone());

                tracing::info!(
                    "✅ Ungrouped {} groups with {} objects",
                    indices_to_ungroup.len(),
                    children_to_add.len()
                );
            }
        });

        // Clear selection and redraw
        ungroup_render_state.borrow_mut().clear();
        let _ = ungroup_drawing_area.queue_draw();
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
