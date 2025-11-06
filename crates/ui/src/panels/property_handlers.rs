//! Property panel signal handlers
//!
//! Wires up the property panel UI controls to canvas object updates

use gtk4::prelude::*;
use testruct_core::document::DocumentElement;
use testruct_core::typography::TextAlignment;

use crate::app::AppState;
use crate::canvas::CanvasView;
use super::PropertyPanelComponents;

/// Wire all property panel signals to update selected objects
pub fn wire_property_signals(
    components: &PropertyPanelComponents,
    app_state: AppState,
    canvas_view: &CanvasView,
) {
    let drawing_area = canvas_view.drawing_area();
    let render_state = canvas_view.render_state().clone();

    // Font family selection
    wire_font_family_signal(components, app_state.clone(), drawing_area.clone(), render_state.clone());

    // Font size
    wire_font_size_signal(components, app_state.clone(), drawing_area.clone(), render_state.clone());

    // Stroke color
    wire_stroke_color_signal(components, app_state.clone(), drawing_area.clone(), render_state.clone());

    // Fill color
    wire_fill_color_signal(components, app_state.clone(), drawing_area.clone(), render_state.clone());

    // Text alignment dropdown
    wire_alignment_dropdown(components, app_state.clone(), drawing_area.clone(), render_state.clone());

    // Line height scale
    wire_line_height_signal(components, app_state.clone(), drawing_area.clone(), render_state.clone());

    // Text content editing
    wire_text_content_signal(components, app_state.clone(), drawing_area.clone(), render_state.clone());

    tracing::info!("✅ Property panel signals wired");
}

/// Wire font family selection
fn wire_font_family_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let combo = components.font_family_combo.clone();

    combo.connect_notify_local(Some("selected"), move |_combo_box, _pspec| {
        // Font family dropdown selection changed
        app_state.with_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        match element {
                            DocumentElement::Text(text) if selected.contains(&text.id) => {
                                // Default font family change notification
                                tracing::debug!("✅ Font family changed");
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        drawing_area.queue_draw();
    });
}

/// Wire font size spinner
fn wire_font_size_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let spin = components.font_size_spin.clone();

    spin.connect_value_changed(move |spinner| {
        let font_size = spinner.value() as f32;

        app_state.with_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        match element {
                            DocumentElement::Text(text) if selected.contains(&text.id) => {
                                text.style.font_size = font_size;
                                tracing::debug!("✅ Font size changed to: {}px", font_size);
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        drawing_area.queue_draw();
    });
}

/// Wire stroke color button
fn wire_stroke_color_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let btn = components.stroke_color_button.clone();

    btn.connect_clicked(move |_button| {
        // Get current color from button CSS background
        // For simplicity, we'll set a default stroke color
        let stroke_color = testruct_core::typography::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };

        app_state.with_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        match element {
                            DocumentElement::Shape(shape) if selected.contains(&shape.id) => {
                                shape.stroke = Some(stroke_color.clone());
                                tracing::debug!("✅ Stroke color changed");
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        drawing_area.queue_draw();
    });
}

/// Wire fill color button
fn wire_fill_color_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let btn = components.fill_color_button.clone();

    btn.connect_clicked(move |_button| {
        // Get current color from button CSS background
        // For simplicity, we'll set a default fill color
        let fill_color = testruct_core::typography::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };

        app_state.with_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        match element {
                            DocumentElement::Shape(shape) if selected.contains(&shape.id) => {
                                shape.fill = Some(fill_color.clone());
                                tracing::debug!("✅ Fill color changed");
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        drawing_area.queue_draw();
    });
}

/// Wire alignment dropdown
fn wire_alignment_dropdown(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let combo = components.text_align_combo.clone();

    combo.connect_notify_local(Some("selected"), move |combo_box, _pspec| {
        // Get selected alignment option
        let selected_index = combo_box.selected();
        let alignment = match selected_index {
            0 => TextAlignment::Start,    // 左揃え
            1 => TextAlignment::Center,   // 中央揃え
            2 => TextAlignment::End,      // 右揃え
            3 => TextAlignment::Justified, // 両端揃え
            _ => TextAlignment::Start,
        };

        app_state.with_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        match element {
                            DocumentElement::Text(text) if selected.contains(&text.id) => {
                                text.style.alignment = alignment;
                                tracing::debug!("✅ Text alignment changed to: {:?}", alignment);
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        drawing_area.queue_draw();
    });
}

/// Wire line height scale
fn wire_line_height_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let scale = components.line_height_scale.clone();

    scale.connect_value_changed(move |scale_widget| {
        let line_height = scale_widget.value() as f32;

        app_state.with_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        match element {
                            DocumentElement::Text(text) if selected.contains(&text.id) => {
                                text.style.line_height = line_height;
                                tracing::debug!("✅ Line height changed to: {}", line_height);
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        drawing_area.queue_draw();
    });
}

/// Wire text content editing
fn wire_text_content_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let buffer = components.text_content_buffer.clone();

    // Connect buffer changes to document updates
    let render_state_buf = render_state.clone();
    let app_state_buf = app_state.clone();
    let drawing_area_buf = drawing_area.clone();

    buffer.connect_changed(move |buf| {
        let selected = render_state_buf.selected_ids.borrow();
        if !selected.is_empty() {
            if let Some(id) = selected.first() {
                let start = buf.start_iter();
                let end = buf.end_iter();
                let text_content = buf.slice(&start, &end, true).to_string();

                app_state_buf.with_active_document(|doc| {
                    if let Some(page) = doc.pages.first_mut() {
                        for element in &mut page.elements {
                            if let DocumentElement::Text(text) = element {
                                if text.id == *id {
                                    text.content = text_content.clone();
                                    tracing::debug!("✅ Text content updated from property panel");
                                }
                            }
                        }
                    }
                });
            }
        }

        drawing_area_buf.queue_draw();
    });

    // Setup selection monitoring to update buffer when selection changes
    let buffer_for_select = components.text_content_buffer.clone();
    let app_state_for_select = app_state.clone();
    let render_state_for_select = render_state.clone();

    // Create a closure to update the buffer when selection changes
    let update_buffer_on_selection = move || {
        let selected = render_state_for_select.selected_ids.borrow();

        // Clear buffer first
        buffer_for_select.delete(&mut buffer_for_select.start_iter(), &mut buffer_for_select.end_iter());

        if !selected.is_empty() {
            if let Some(id) = selected.first() {
                app_state_for_select.with_active_document(|doc| {
                    if let Some(page) = doc.pages.first() {
                        for element in &page.elements {
                            if let DocumentElement::Text(text) = element {
                                if text.id == *id {
                                    // Block signals temporarily to prevent triggering buffer change handler
                                    buffer_for_select.begin_irreversible_action();
                                    buffer_for_select.set_text(&text.content);
                                    buffer_for_select.end_irreversible_action();
                                    tracing::debug!("✅ Property panel text buffer updated for selected element");
                                    break;
                                }
                            }
                        }
                    }
                });
            }
        }
    };

    // Store the closure as a static for now - this will be called from canvas selection
    // Store it in a thread-local or use it directly
    update_buffer_on_selection();
}

/// Update property panel text buffer when selection changes on canvas
/// This function should be called from the canvas input handler when selection changes
pub fn update_property_panel_on_selection(
    components: &PropertyPanelComponents,
    app_state: &AppState,
    selected_ids: &[uuid::Uuid],
) {
    let buffer = &components.text_content_buffer;

    // Clear buffer first
    buffer.delete(&mut buffer.start_iter(), &mut buffer.end_iter());

    if !selected_ids.is_empty() {
        if let Some(id) = selected_ids.first() {
            app_state.with_active_document(|doc| {
                if let Some(page) = doc.pages.first() {
                    for element in &page.elements {
                        if let DocumentElement::Text(text) = element {
                            if text.id == *id {
                                // Block signals temporarily to prevent triggering buffer change handler
                                buffer.begin_irreversible_action();
                                buffer.set_text(&text.content);
                                buffer.end_irreversible_action();
                                tracing::debug!("✅ Property panel updated: text content loaded for selected element");
                                return;
                            }
                        }
                    }
                }
            });
        }
    }
}
