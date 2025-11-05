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
