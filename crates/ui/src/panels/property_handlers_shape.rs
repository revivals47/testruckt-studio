//! Shape property signal handlers
//!
//! Handles shape-specific property panel controls (stroke color, fill color, auto-resize)

use gtk4::{gdk, gio};
use gtk4::{prelude::*, ColorDialog};
use testruct_core::document::DocumentElement;

use super::PropertyPanelComponents;
use crate::app::AppState;

/// Wire stroke color button
pub fn wire_stroke_color_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let button = components.stroke_color_button.clone();
    let selected_ids_ref = render_state.selected_ids.clone();
    let app_state_clone = app_state.clone();
    let drawing_area_clone = drawing_area.clone();
    let panel_clone = components.clone();

    button.connect_clicked(move |widget| {
        let selected_ids: Vec<uuid::Uuid> = {
            let selected = selected_ids_ref.borrow();
            if selected.is_empty() {
                tracing::warn!("⚠️ 線色を変更するオブジェクトが選択されていません");
                return;
            }
            selected.clone()
        };

        let parent_window = widget
            .root()
            .and_then(|root| root.downcast::<gtk4::Window>().ok());

        let initial_color = app_state_clone
            .with_active_document(|doc| {
                if let Some(page) = doc.pages.first() {
                    for element in &page.elements {
                        if selected_ids.contains(&element.id()) {
                            if let DocumentElement::Shape(shape) = element {
                                if let Some(color) = &shape.stroke {
                                    return Some(color_to_rgba(color));
                                }
                            }
                        }
                    }
                }
                None
            })
            .flatten();

        let dialog = ColorDialog::builder()
            .modal(true)
            .title("枠線色を選択")
            .with_alpha(false)
            .build();

        let selected_ids_for_cb = selected_ids.clone();
        let app_state_for_cb = app_state_clone.clone();
        let drawing_area_for_cb = drawing_area_clone.clone();
        let panel_for_cb = panel_clone.clone();

        dialog.choose_rgba(
            parent_window.as_ref(),
            initial_color.as_ref(),
            None::<&gio::Cancellable>,
            move |result| {
                if let Ok(rgba) = result {
                    let stroke_color = rgba_to_color(&rgba);
                    let updated = app_state_for_cb.with_mutable_active_document(|doc| {
                        let mut changed = false;
                        if let Some(page) = doc.pages.first_mut() {
                            for element in &mut page.elements {
                                if selected_ids_for_cb.contains(&element.id()) {
                                    if let DocumentElement::Shape(shape) = element {
                                        shape.stroke = Some(stroke_color.clone());
                                        changed = true;
                                    }
                                }
                            }
                        }
                        changed
                    });

                    if updated.unwrap_or(false) {
                        drawing_area_for_cb.queue_draw();
                        crate::panels::property_handlers::update_property_panel_on_selection(
                            &panel_for_cb,
                            &app_state_for_cb,
                            &selected_ids_for_cb,
                        );
                        tracing::debug!("✅ Stroke color updated via dialog");
                    }
                }
            },
        );
    });
}

/// Wire fill color button
pub fn wire_fill_color_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let button = components.fill_color_button.clone();
    let selected_ids_ref = render_state.selected_ids.clone();
    let app_state_clone = app_state.clone();
    let drawing_area_clone = drawing_area.clone();
    let panel_clone = components.clone();

    button.connect_clicked(move |widget| {
        let selected_ids: Vec<uuid::Uuid> = {
            let selected = selected_ids_ref.borrow();
            if selected.is_empty() {
                tracing::warn!("⚠️ 塗りつぶし色を変更するオブジェクトが選択されていません");
                return;
            }
            selected.clone()
        };

        let parent_window = widget
            .root()
            .and_then(|root| root.downcast::<gtk4::Window>().ok());

        let initial_color = app_state_clone
            .with_active_document(|doc| {
                if let Some(page) = doc.pages.first() {
                    for element in &page.elements {
                        if selected_ids.contains(&element.id()) {
                            if let DocumentElement::Shape(shape) = element {
                                if let Some(color) = &shape.fill {
                                    return Some(color_to_rgba(color));
                                }
                            }
                        }
                    }
                }
                None
            })
            .flatten();

        let dialog = ColorDialog::builder()
            .modal(true)
            .title("塗りつぶし色を選択")
            .with_alpha(false)
            .build();

        let selected_ids_for_cb = selected_ids.clone();
        let app_state_for_cb = app_state_clone.clone();
        let drawing_area_for_cb = drawing_area_clone.clone();
        let panel_for_cb = panel_clone.clone();

        dialog.choose_rgba(
            parent_window.as_ref(),
            initial_color.as_ref(),
            None::<&gio::Cancellable>,
            move |result| {
                if let Ok(rgba) = result {
                    let fill_color = rgba_to_color(&rgba);
                    let updated = app_state_for_cb.with_mutable_active_document(|doc| {
                        let mut changed = false;
                        if let Some(page) = doc.pages.first_mut() {
                            for element in &mut page.elements {
                                if selected_ids_for_cb.contains(&element.id()) {
                                    if let DocumentElement::Shape(shape) = element {
                                        shape.fill = Some(fill_color.clone());
                                        changed = true;
                                    }
                                }
                            }
                        }
                        changed
                    });

                    if updated.unwrap_or(false) {
                        drawing_area_for_cb.queue_draw();
                        crate::panels::property_handlers::update_property_panel_on_selection(
                            &panel_for_cb,
                            &app_state_for_cb,
                            &selected_ids_for_cb,
                        );
                        tracing::debug!("✅ Fill color updated via dialog");
                    }
                }
            },
        );
    });
}

/// Wire auto-resize switch to update text element bounds
pub fn wire_auto_resize_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let switch = components.auto_resize_switch.clone();

    switch.connect_state_set(move |_widget, state| {
        let selected: Vec<uuid::Uuid> = {
            let selected_ids = render_state.selected_ids.borrow();
            if selected_ids.is_empty() {
                tracing::warn!("⚠️  Auto-resize toggled with no selection");
                return gtk4::glib::Propagation::Proceed;
            }
            selected_ids.clone()
        };

        let state_changed = app_state.with_mutable_active_document(|doc| {
            let mut changed = false;
            if let Some(page) = doc.pages.first_mut() {
                for element in &mut page.elements {
                    if selected.contains(&element.id()) {
                        if let DocumentElement::Text(text) = element {
                            text.auto_resize_height = state;
                            if state {
                                recompute_auto_height(text);
                            }
                            changed = true;
                        }
                    }
                }
            }
            changed
        });

        if state_changed.unwrap_or(false) {
            drawing_area.queue_draw();
        }

        gtk4::glib::Propagation::Proceed
    });
}

fn recompute_auto_height(text: &mut testruct_core::document::TextElement) {
    if !text.auto_resize_height {
        return;
    }

    let width = text.bounds.size.width.max(1.0);
    let new_height =
        crate::canvas::rendering::measure_text_height(&text.content, &text.style, width);
    text.bounds.size.height = new_height.max(1.0);
}

fn color_to_rgba(color: &testruct_core::typography::Color) -> gdk::RGBA {
    gdk::RGBA::new(color.r, color.g, color.b, color.a)
}

fn rgba_to_color(rgba: &gdk::RGBA) -> testruct_core::typography::Color {
    testruct_core::typography::Color {
        r: rgba.red() as f32,
        g: rgba.green() as f32,
        b: rgba.blue() as f32,
        a: rgba.alpha() as f32,
    }
}

pub fn color_to_hex(color: &testruct_core::typography::Color) -> String {
    let clamp = |v: f32| -> u8 { (v.clamp(0.0, 1.0) * 255.0).round() as u8 };
    format!(
        "#{:02X}{:02X}{:02X}",
        clamp(color.r),
        clamp(color.g),
        clamp(color.b)
    )
}

/// Wire stroke width spinner to update shape stroke width
pub fn wire_stroke_width_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let spinner = components.stroke_width_spin.clone();

    spinner.connect_value_changed(move |spin| {
        let stroke_width = spin.value() as f32;

        app_state.with_mutable_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        match element {
                            DocumentElement::Shape(shape) if selected.contains(&shape.id) => {
                                shape.stroke_width = stroke_width;
                                tracing::debug!("✅ Stroke width changed to: {}pt", stroke_width);
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
