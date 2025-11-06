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
}

fn execute_alignment(
    state: &crate::app::AppState,
    selected_ids: &std::rc::Rc<std::cell::RefCell<Vec<uuid::Uuid>>>,
    drawing_area: &gtk4::DrawingArea,
    alignment_type: AlignmentType,
    success_log: &str,
) {
    let selected = selected_ids.borrow();
    if selected.len() < 2 {
        if selected.is_empty() {
            tracing::warn!("⚠️  No objects selected for alignment");
        } else {
            tracing::warn!("⚠️  Need at least 2 objects for alignment");
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
}

/// Helper function to get bounds from a DocumentElement
fn get_element_bounds(element: &DocumentElement) -> Rect {
    match element {
        DocumentElement::Shape(shape) => shape.bounds,
        DocumentElement::Text(text) => text.bounds,
        DocumentElement::Image(image) => image.bounds,
        DocumentElement::Frame(frame) => frame.bounds,
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
            let mut selected_bounds = Vec::new();
            for element in &page.elements {
                if selected_ids.contains(&element.id()) {
                    selected_bounds.push((element.id(), get_element_bounds(element)));
                }
            }

            if selected_bounds.len() < 2 {
                return;
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
                        }
                        break;
                    }
                }
            }
        }
    });
}
