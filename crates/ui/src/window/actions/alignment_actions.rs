//! Alignment and distribution actions for selected objects
//!
//! Provides alignment operations: left, center (horizontal), right, top, center (vertical), bottom

use gtk4::prelude::*;
use testruct_core::document::DocumentElement;
use testruct_core::layout::{Point, Rect};

use super::common::add_window_action;

/// Register alignment actions
pub fn register(
    window: &gtk4::ApplicationWindow,
    state: crate::app::AppState,
    canvas_view: &crate::canvas::CanvasView,
    property_components: &crate::panels::PropertyPanelComponents,
) {
    let drawing_area = canvas_view.drawing_area();
    let render_state = canvas_view.render_state().clone();

    // Align left (leftmost object's x position)
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();

        add_window_action(window, "align-left", move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignLeft,
                "✅ Objects aligned left",
            );
        });
    }
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();
        let button = property_components.align_left_btn.clone();
        button.connect_clicked(move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignLeft,
                "✅ Objects aligned left",
            );
        });
    }

    // Align center horizontal
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();

        add_window_action(window, "align-center-h", move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignCenterH,
                "✅ Objects aligned center (horizontal)",
            );
        });
    }
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();
        let button = property_components.align_center_h_btn.clone();
        button.connect_clicked(move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignCenterH,
                "✅ Objects aligned center (horizontal)",
            );
        });
    }

    // Align right
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();

        add_window_action(window, "align-right", move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignRight,
                "✅ Objects aligned right",
            );
        });
    }
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();
        let button = property_components.align_right_btn.clone();
        button.connect_clicked(move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignRight,
                "✅ Objects aligned right",
            );
        });
    }

    // Align top
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();

        add_window_action(window, "align-top", move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignTop,
                "✅ Objects aligned top",
            );
        });
    }
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();
        let button = property_components.align_top_btn.clone();
        button.connect_clicked(move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignTop,
                "✅ Objects aligned top",
            );
        });
    }

    // Align center vertical
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();

        add_window_action(window, "align-center-v", move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignCenterV,
                "✅ Objects aligned center (vertical)",
            );
        });
    }
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();
        let button = property_components.align_center_v_btn.clone();
        button.connect_clicked(move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignCenterV,
                "✅ Objects aligned center (vertical)",
            );
        });
    }

    // Align bottom
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();

        add_window_action(window, "align-bottom", move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignBottom,
                "✅ Objects aligned bottom",
            );
        });
    }
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();
        let button = property_components.align_bottom_btn.clone();
        button.connect_clicked(move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::AlignBottom,
                "✅ Objects aligned bottom",
            );
        });
    }

    // Distribute horizontally (equal spacing)
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();

        add_window_action(window, "distribute-h", move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::DistributeH,
                "✅ Objects distributed horizontally",
            );
        });
    }

    // Distribute vertically (equal spacing)
    {
        let state_c = state.clone();
        let selection = render_state.selected_ids.clone();
        let drawing_c = drawing_area.clone();

        add_window_action(window, "distribute-v", move |_| {
            execute_alignment(
                &state_c,
                &selection,
                &drawing_c,
                AlignmentType::DistributeV,
                "✅ Objects distributed vertically",
            );
        });
    }
}

fn execute_alignment(
    state: &crate::app::AppState,
    selected_ids: &std::rc::Rc<std::cell::RefCell<Vec<uuid::Uuid>>>,
    drawing_area: &gtk4::DrawingArea,
    alignment_type: AlignmentType,
    success_log: &str,
) {
    let selected = selected_ids.borrow();

    // Distribute requires at least 3 objects
    let min_count = match alignment_type {
        AlignmentType::DistributeH | AlignmentType::DistributeV => 3,
        _ => 2,
    };

    if selected.len() < min_count {
        if selected.is_empty() {
            tracing::warn!("⚠️  No objects selected for alignment");
        } else {
            tracing::warn!("⚠️  Need at least {} objects for this operation", min_count);
        }
        return;
    }

    let ids: Vec<uuid::Uuid> = selected.iter().copied().collect();
    drop(selected);

    apply_alignment(state, &ids, alignment_type);
    drawing_area.queue_draw();
    tracing::info!("{success_log}");
}

/// Alignment type
#[derive(Clone, Copy, Debug)]
pub enum AlignmentType {
    AlignLeft,
    AlignCenterH,
    AlignRight,
    AlignTop,
    AlignCenterV,
    AlignBottom,
    DistributeH,
    DistributeV,
}

/// Helper function to get bounds from a DocumentElement
fn get_element_bounds(element: &DocumentElement) -> Rect {
    match element {
        DocumentElement::Shape(shape) => shape.bounds,
        DocumentElement::Text(text) => text.bounds,
        DocumentElement::Image(image) => image.bounds,
        DocumentElement::Frame(frame) => frame.bounds,
        DocumentElement::Group(group) => group.bounds,
    }
}

/// Apply alignment to selected objects
fn apply_alignment(
    state: &crate::app::AppState,
    selected_ids: &[uuid::Uuid],
    alignment_type: AlignmentType,
) {
    state.with_active_document(|doc| {
        if let Some(page) = doc.pages.first_mut() {
            // Collect bounds for all selected elements
            let mut selected_bounds: Vec<(uuid::Uuid, Rect)> = Vec::new();
            for element in &page.elements {
                if selected_ids.contains(&element.id()) {
                    selected_bounds.push((element.id(), get_element_bounds(element)));
                }
            }

            if selected_bounds.len() < 2 {
                return;
            }

            // Handle distribute operations separately
            match alignment_type {
                AlignmentType::DistributeH => {
                    if selected_bounds.len() < 3 {
                        return;
                    }
                    // Sort by x position
                    selected_bounds.sort_by(|a, b| a.1.origin.x.partial_cmp(&b.1.origin.x).unwrap());

                    let leftmost_x = selected_bounds.first().unwrap().1.origin.x;
                    let rightmost_right = {
                        let last = selected_bounds.last().unwrap();
                        last.1.origin.x + last.1.size.width
                    };
                    let total_span = rightmost_right - leftmost_x;
                    let total_width: f32 = selected_bounds.iter().map(|(_, b)| b.size.width).sum();
                    let gap = (total_span - total_width) / (selected_bounds.len() - 1) as f32;

                    let mut current_x = leftmost_x;
                    for (id, bounds) in &selected_bounds {
                        let new_x = current_x;
                        current_x += bounds.size.width + gap;

                        // Apply new position
                        for element in &mut page.elements {
                            if element.id() == *id {
                                let elem_bounds = element.bounds_mut();
                                elem_bounds.origin.x = new_x;
                                break;
                            }
                        }
                    }
                    return;
                }
                AlignmentType::DistributeV => {
                    if selected_bounds.len() < 3 {
                        return;
                    }
                    // Sort by y position
                    selected_bounds.sort_by(|a, b| a.1.origin.y.partial_cmp(&b.1.origin.y).unwrap());

                    let topmost_y = selected_bounds.first().unwrap().1.origin.y;
                    let bottommost_bottom = {
                        let last = selected_bounds.last().unwrap();
                        last.1.origin.y + last.1.size.height
                    };
                    let total_span = bottommost_bottom - topmost_y;
                    let total_height: f32 = selected_bounds.iter().map(|(_, b)| b.size.height).sum();
                    let gap = (total_span - total_height) / (selected_bounds.len() - 1) as f32;

                    let mut current_y = topmost_y;
                    for (id, bounds) in &selected_bounds {
                        let new_y = current_y;
                        current_y += bounds.size.height + gap;

                        // Apply new position
                        for element in &mut page.elements {
                            if element.id() == *id {
                                let elem_bounds = element.bounds_mut();
                                elem_bounds.origin.y = new_y;
                                break;
                            }
                        }
                    }
                    return;
                }
                _ => {}
            }

            // Calculate reference value based on alignment type
            let reference = match alignment_type {
                AlignmentType::AlignLeft => selected_bounds
                    .iter()
                    .map(|(_, bounds)| bounds.origin.x)
                    .fold(f32::INFINITY, f32::min),
                AlignmentType::AlignRight => selected_bounds
                    .iter()
                    .map(|(_, bounds)| bounds.origin.x + bounds.size.width)
                    .fold(f32::NEG_INFINITY, f32::max),
                AlignmentType::AlignCenterH => {
                    let min_x = selected_bounds
                        .iter()
                        .map(|(_, bounds)| bounds.origin.x)
                        .fold(f32::INFINITY, f32::min);
                    let max_x = selected_bounds
                        .iter()
                        .map(|(_, bounds)| bounds.origin.x + bounds.size.width)
                        .fold(f32::NEG_INFINITY, f32::max);
                    (min_x + max_x) / 2.0
                }
                AlignmentType::AlignTop => selected_bounds
                    .iter()
                    .map(|(_, bounds)| bounds.origin.y)
                    .fold(f32::INFINITY, f32::min),
                AlignmentType::AlignBottom => selected_bounds
                    .iter()
                    .map(|(_, bounds)| bounds.origin.y + bounds.size.height)
                    .fold(f32::NEG_INFINITY, f32::max),
                AlignmentType::AlignCenterV => {
                    let min_y = selected_bounds
                        .iter()
                        .map(|(_, bounds)| bounds.origin.y)
                        .fold(f32::INFINITY, f32::min);
                    let max_y = selected_bounds
                        .iter()
                        .map(|(_, bounds)| bounds.origin.y + bounds.size.height)
                        .fold(f32::NEG_INFINITY, f32::max);
                    (min_y + max_y) / 2.0
                }
                AlignmentType::DistributeH | AlignmentType::DistributeV => 0.0, // Already handled
            };

            // Apply alignment to each selected element
            for (element_id, _bounds) in selected_bounds {
                for element in &mut page.elements {
                    if element.id() == element_id {
                        let new_bounds = match alignment_type {
                            AlignmentType::AlignLeft => {
                                let elem_bounds = get_element_bounds(element);
                                Rect {
                                    origin: Point {
                                        x: reference,
                                        y: elem_bounds.origin.y,
                                    },
                                    size: elem_bounds.size,
                                }
                            }
                            AlignmentType::AlignRight => {
                                let elem_bounds = get_element_bounds(element);
                                Rect {
                                    origin: Point {
                                        x: reference - elem_bounds.size.width,
                                        y: elem_bounds.origin.y,
                                    },
                                    size: elem_bounds.size,
                                }
                            }
                            AlignmentType::AlignCenterH => {
                                let elem_bounds = get_element_bounds(element);
                                Rect {
                                    origin: Point {
                                        x: reference - elem_bounds.size.width / 2.0,
                                        y: elem_bounds.origin.y,
                                    },
                                    size: elem_bounds.size,
                                }
                            }
                            AlignmentType::AlignTop => {
                                let elem_bounds = get_element_bounds(element);
                                Rect {
                                    origin: Point {
                                        x: elem_bounds.origin.x,
                                        y: reference,
                                    },
                                    size: elem_bounds.size,
                                }
                            }
                            AlignmentType::AlignBottom => {
                                let elem_bounds = get_element_bounds(element);
                                Rect {
                                    origin: Point {
                                        x: elem_bounds.origin.x,
                                        y: reference - elem_bounds.size.height,
                                    },
                                    size: elem_bounds.size,
                                }
                            }
                            AlignmentType::AlignCenterV => {
                                let elem_bounds = get_element_bounds(element);
                                Rect {
                                    origin: Point {
                                        x: elem_bounds.origin.x,
                                        y: reference - elem_bounds.size.height / 2.0,
                                    },
                                    size: elem_bounds.size,
                                }
                            }
                            // Distribute cases are handled above and return early
                            AlignmentType::DistributeH | AlignmentType::DistributeV => {
                                continue;
                            }
                        };

                        match element {
                            DocumentElement::Shape(shape) => {
                                shape.bounds = new_bounds;
                            }
                            DocumentElement::Text(text) => {
                                text.bounds = new_bounds;
                            }
                            DocumentElement::Image(image) => {
                                image.bounds = new_bounds;
                            }
                            DocumentElement::Frame(frame) => {
                                frame.bounds = new_bounds;
                            }
                            DocumentElement::Group(group) => {
                                group.bounds = new_bounds;
                            }
                        }
                        break;
                    }
                }
            }
        }
    });
}
