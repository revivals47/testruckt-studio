//! Text property signal handlers
//!
//! Handles text-specific property panel controls (font family, size, bold, italic, etc.)

use gtk4::{prelude::*, StringList};
use testruct_core::document::DocumentElement;

use super::PropertyPanelComponents;
use crate::app::AppState;

/// Wire font family selection
pub fn wire_font_family_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let combo = components.font_family_combo.clone();

    combo.connect_notify_local(Some("selected"), move |combo_box, _pspec| {
        if let Some(font_name) = dropdown_string(combo_box, combo_box.selected()) {
            app_state.with_mutable_active_document(|doc| {
                let selected = render_state.selected_ids.borrow();
                if !selected.is_empty() {
                    if let Some(page) = doc.pages.first_mut() {
                        for element in &mut page.elements {
                            if let DocumentElement::Text(text) = element {
                                if selected.contains(&text.id) {
                                    text.style.font_family = font_name.clone();
                                    recompute_auto_height(text);
                                    tracing::debug!("✅ Font family changed to {}", font_name);
                                }
                            }
                        }
                    }
                }
            });
        }

        drawing_area.queue_draw();
    });
}

/// Wire font size spinner
pub fn wire_font_size_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let spin = components.font_size_spin.clone();

    spin.connect_value_changed(move |spinner| {
        let font_size = spinner.value() as f32;

        app_state.with_mutable_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        match element {
                            DocumentElement::Text(text) if selected.contains(&text.id) => {
                                text.style.font_size = font_size;
                                recompute_auto_height(text);
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

/// Wire bold button toggle
pub fn wire_bold_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let button = components.bold_button.clone();

    button.connect_toggled(move |btn| {
        let is_bold = btn.is_active();

        app_state.with_mutable_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        if let DocumentElement::Text(text) = element {
                            if selected.contains(&text.id) {
                                text.style.weight = if is_bold {
                                    testruct_core::typography::FontWeight::Bold
                                } else {
                                    testruct_core::typography::FontWeight::Regular
                                };
                                recompute_auto_height(text);
                                tracing::debug!("✅ Bold: {}", is_bold);
                            }
                        }
                    }
                }
            }
        });

        drawing_area.queue_draw();
    });
}

/// Wire italic button toggle
pub fn wire_italic_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let button = components.italic_button.clone();

    button.connect_toggled(move |btn| {
        let is_italic = btn.is_active();

        app_state.with_mutable_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        if let DocumentElement::Text(text) = element {
                            if selected.contains(&text.id) {
                                text.style.italic = is_italic;
                                recompute_auto_height(text);
                                tracing::debug!("✅ Italic: {}", is_italic);
                            }
                        }
                    }
                }
            }
        });

        drawing_area.queue_draw();
    });
}

/// Wire alignment dropdown
pub fn wire_alignment_dropdown(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    use testruct_core::typography::TextAlignment;
    let combo = components.text_align_combo.clone();

    combo.connect_notify_local(Some("selected"), move |combo_box, _pspec| {
        // Get selected alignment option
        let selected_index = combo_box.selected();
        let alignment = match selected_index {
            0 => TextAlignment::Start,     // 左揃え
            1 => TextAlignment::Center,    // 中央揃え
            2 => TextAlignment::End,       // 右揃え
            3 => TextAlignment::Justified, // 両端揃え
            _ => TextAlignment::Start,
        };

        app_state.with_mutable_active_document(|doc| {
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
pub fn wire_line_height_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let scale = components.line_height_scale.clone();

    scale.connect_value_changed(move |scale_widget| {
        let line_height = scale_widget.value() as f32;

        app_state.with_mutable_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        match element {
                            DocumentElement::Text(text) if selected.contains(&text.id) => {
                                text.style.line_height = line_height;
                                recompute_auto_height(text);
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
pub fn wire_text_content_signal(
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

                app_state_buf.with_mutable_active_document(|doc| {
                    if let Some(page) = doc.pages.first_mut() {
                        for element in &mut page.elements {
                            if let DocumentElement::Text(text) = element {
                                if text.id == *id {
                                    text.content = text_content.clone();
                                    recompute_auto_height(text);
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

fn dropdown_string(dropdown: &gtk4::DropDown, index: u32) -> Option<String> {
    if index == gtk4::INVALID_LIST_POSITION {
        return None;
    }
    let model = dropdown.model()?;
    let string_list = model.downcast::<StringList>().ok()?;
    string_list.string(index).map(|s| s.to_string())
}

pub fn find_string_index(dropdown: &gtk4::DropDown, value: &str) -> Option<u32> {
    let model = dropdown.model()?;
    let string_list = model.downcast::<StringList>().ok()?;
    let total = string_list.n_items();
    for idx in 0..total {
        if let Some(item) = string_list.string(idx) {
            if item.as_str().eq_ignore_ascii_case(value) {
                return Some(idx);
            }
        }
    }
    None
}
