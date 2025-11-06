//! Tool action handlers (image insertion, templates, z-order)

use super::common::add_window_action;
use gtk4::prelude::*;

/// Register tool menu actions and z-order button handlers
pub fn register(
    window: &gtk4::ApplicationWindow,
    state: crate::app::AppState,
    canvas_view: &crate::canvas::CanvasView,
    property_components: &crate::panels::PropertyPanelComponents,
) {
    // Image insertion
    let insert_image_state = state.clone();
    let insert_image_window = window.clone();
    let insert_image_drawing_area = canvas_view.drawing_area();
    add_window_action(window, "insert-image", move |_| {
        tracing::info!("Action: insert image");

        let window_ref = insert_image_window.clone();
        let state_ref = insert_image_state.clone();
        let drawing_area = insert_image_drawing_area.clone();

        let window_as_base = window_ref.upcast::<gtk4::Window>();
        crate::dialogs::show_image_chooser_async(
            &window_as_base,
            Box::new(move |path| {
                tracing::info!("Selected image file: {}", path.display());

                let asset_catalog = state_ref.asset_catalog();
                let asset_ref = {
                    let mut catalog = asset_catalog.lock().expect("asset catalog");
                    catalog.register(&path)
                };
                tracing::info!("✅ Registered image asset: {:?}", asset_ref);

                let image_element = testruct_core::document::ImageElement {
                    id: uuid::Uuid::new_v4(),
                    source: asset_ref,
                    bounds: testruct_core::layout::Rect {
                        origin: testruct_core::layout::Point { x: 100.0, y: 100.0 },
                        size: testruct_core::layout::Size {
                            width: 200.0,
                            height: 200.0,
                        },
                    },
                };

                match state_ref.add_element_to_active_page(
                    testruct_core::document::DocumentElement::Image(image_element),
                ) {
                    Ok(_) => {
                        tracing::info!("✅ Image inserted: {}", path.display());
                        let _ = drawing_area.queue_draw();
                    }
                    Err(e) => {
                        tracing::error!("❌ Failed to insert image: {}", e);
                    }
                }
            }),
        );
    });

    // Template browser
    let templates_state = state.clone();
    let templates_window = window.clone();
    add_window_action(window, "templates", move |_| {
        tracing::info!("Action: show templates");

        let window_ref = templates_window.clone();
        let state_ref = templates_state.clone();

        let window_as_base = window_ref.clone().upcast::<gtk4::Window>();

        let project = state_ref.project();
        let templates: Vec<_> = project.templates.iter().collect();

        if templates.is_empty() {
            tracing::warn!("⚠️  No templates available");
            return;
        }

        crate::dialogs::show_template_browser_async(
            &window_as_base,
            templates,
            Box::new(move |selected_template| {
                if let Some(template) = selected_template {
                    tracing::info!("✅ Template selected: {}", template.name);

                    let project = state_ref.project();
                    if let Some(_new_doc) = project
                        .apply_template(testruct_core::template::TemplateRef { id: template.id })
                    {
                        tracing::info!("✅ New document created from template");
                    } else {
                        tracing::error!("❌ Failed to create document from template");
                    }
                } else {
                    tracing::info!("⚠️  Template selection cancelled");
                }
            }),
        );
    });

    // Z-order button handlers
    register_zorder_actions(state, canvas_view, property_components);
}

/// Register z-order button handlers
fn register_zorder_actions(
    state: crate::app::AppState,
    canvas_view: &crate::canvas::CanvasView,
    property_components: &crate::panels::PropertyPanelComponents,
) {
    let bring_to_front_state = state.clone();
    let bring_to_front_drawing_area = canvas_view.drawing_area();
    let bring_to_front_render_state = canvas_view.render_state().selected_ids.clone();
    property_components
        .bring_to_front_btn
        .connect_clicked(move |_| {
            tracing::info!("Action: bring to front");
            let selected_ids = bring_to_front_render_state.borrow();
            if let Some(element_id) = selected_ids.first().copied() {
                bring_to_front_state.with_active_document(|doc| {
                    if let Some(page) = doc.pages.get_mut(0) {
                        if page.bring_to_front(element_id) {
                            tracing::info!("✅ Element brought to front");
                            let _ = bring_to_front_drawing_area.queue_draw();
                        } else {
                            tracing::warn!("⚠️  Element not found");
                        }
                    } else {
                        tracing::warn!("⚠️  No active page");
                    }
                });
            } else {
                tracing::warn!("⚠️  No element selected");
            }
        });

    let bring_forward_state = state.clone();
    let bring_forward_drawing_area = canvas_view.drawing_area();
    let bring_forward_render_state = canvas_view.render_state().selected_ids.clone();
    property_components
        .bring_forward_btn
        .connect_clicked(move |_| {
            tracing::info!("Action: bring forward");
            let selected_ids = bring_forward_render_state.borrow();
            if let Some(element_id) = selected_ids.first().copied() {
                bring_forward_state.with_active_document(|doc| {
                    if let Some(page) = doc.pages.get_mut(0) {
                        if page.bring_forward(element_id) {
                            tracing::info!("✅ Element brought forward");
                            let _ = bring_forward_drawing_area.queue_draw();
                        } else {
                            tracing::warn!("⚠️  Element already at front or not found");
                        }
                    } else {
                        tracing::warn!("⚠️  No active page");
                    }
                });
            } else {
                tracing::warn!("⚠️  No element selected");
            }
        });

    let send_to_back_state = state.clone();
    let send_to_back_drawing_area = canvas_view.drawing_area();
    let send_to_back_render_state = canvas_view.render_state().selected_ids.clone();
    property_components
        .send_to_back_btn
        .connect_clicked(move |_| {
            tracing::info!("Action: send to back");
            let selected_ids = send_to_back_render_state.borrow();
            if let Some(element_id) = selected_ids.first().copied() {
                send_to_back_state.with_active_document(|doc| {
                    if let Some(page) = doc.pages.get_mut(0) {
                        if page.send_to_back(element_id) {
                            tracing::info!("✅ Element sent to back");
                            let _ = send_to_back_drawing_area.queue_draw();
                        } else {
                            tracing::warn!("⚠️  Element not found");
                        }
                    } else {
                        tracing::warn!("⚠️  No active page");
                    }
                });
            } else {
                tracing::warn!("⚠️  No element selected");
            }
        });

    let send_backward_state = state.clone();
    let send_backward_drawing_area = canvas_view.drawing_area();
    let send_backward_render_state = canvas_view.render_state().selected_ids.clone();
    property_components
        .send_backward_btn
        .connect_clicked(move |_| {
            tracing::info!("Action: send backward");
            let selected_ids = send_backward_render_state.borrow();
            if let Some(element_id) = selected_ids.first().copied() {
                send_backward_state.with_active_document(|doc| {
                    if let Some(page) = doc.pages.get_mut(0) {
                        if page.send_backward(element_id) {
                            tracing::info!("✅ Element sent backward");
                            let _ = send_backward_drawing_area.queue_draw();
                        } else {
                            tracing::warn!("⚠️  Element already at back or not found");
                        }
                    } else {
                        tracing::warn!("⚠️  No active page");
                    }
                });
            } else {
                tracing::warn!("⚠️  No element selected");
            }
        });
}
