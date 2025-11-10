//! Property panel signal handlers orchestration
//!
//! Wires up the property panel UI controls to canvas object updates
//! Delegates to specialized handler modules for text and shape properties

#[path = "property_handlers_text.rs"]
mod text_handlers;

#[path = "property_handlers_shape.rs"]
mod shape_handlers;

pub use text_handlers::{
    wire_font_family_signal, wire_font_size_signal, wire_bold_signal, wire_italic_signal,
    wire_underline_signal, wire_strikethrough_signal, wire_text_color_signal,
    wire_text_background_color_signal, wire_alignment_dropdown, wire_line_height_signal,
    wire_text_content_signal, find_string_index,
};
pub use shape_handlers::{
    wire_stroke_color_signal, wire_fill_color_signal, wire_stroke_width_signal, wire_auto_resize_signal,
    color_to_hex,
};

use gtk4::prelude::*;
use testruct_core::document::DocumentElement;
use testruct_core::typography::TextAlignment;

use super::PropertyPanelComponents;
use crate::app::AppState;
use crate::canvas::CanvasView;

/// Wire all property panel signals to update selected objects
pub fn wire_property_signals(
    components: &PropertyPanelComponents,
    app_state: AppState,
    canvas_view: &CanvasView,
) {
    let drawing_area = canvas_view.drawing_area();
    let render_state = canvas_view.render_state().clone();

    // Text properties
    wire_font_family_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_font_size_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_bold_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_italic_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_underline_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_strikethrough_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_text_color_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_text_background_color_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_alignment_dropdown(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_line_height_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_text_content_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Shape properties
    wire_stroke_color_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_fill_color_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_stroke_width_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );
    wire_auto_resize_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    tracing::info!("✅ Property panel signals wired");
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

    let selection_len = selected_ids.len();

    // Update group status section
    let (has_group, group_children) = app_state
        .with_active_document(|doc| {
            if let Some(page) = doc.pages.first() {
                for element in &page.elements {
                    if selected_ids.contains(&element.id()) {
                        if let DocumentElement::Frame(frame) = element {
                            return (true, frame.children.len());
                        }
                    }
                }
            }
            (false, 0)
        })
        .unwrap_or((false, 0));

    if has_group {
        components
            .group_status_label
            .set_text(&format!("{}個のオブジェクトを含むグループ", group_children));
        components.ungroup_btn.set_sensitive(true);
        components.group_name_entry.set_sensitive(false);
    } else {
        let status_text = if selected_ids.is_empty() {
            "グループ化されていません"
        } else {
            "グループ選択なし"
        };
        components.group_status_label.set_text(status_text);
        components.ungroup_btn.set_sensitive(false);
        components.group_name_entry.set_sensitive(false);
        components.group_name_entry.set_text("");
    }

    // Update z-order control availability
    let z_controls_enabled = selection_len >= 1;
    components
        .bring_to_front_btn
        .set_sensitive(z_controls_enabled);
    components
        .bring_forward_btn
        .set_sensitive(z_controls_enabled);
    components
        .send_to_back_btn
        .set_sensitive(z_controls_enabled);
    components
        .send_backward_btn
        .set_sensitive(z_controls_enabled);

    // Alignment buttons require at least two objects
    let align_controls_enabled = selection_len >= 2;
    components
        .align_left_btn
        .set_sensitive(align_controls_enabled);
    components
        .align_center_h_btn
        .set_sensitive(align_controls_enabled);
    components
        .align_right_btn
        .set_sensitive(align_controls_enabled);
    components
        .align_top_btn
        .set_sensitive(align_controls_enabled);
    components
        .align_center_v_btn
        .set_sensitive(align_controls_enabled);
    components
        .align_bottom_btn
        .set_sensitive(align_controls_enabled);

    let mut auto_state: Option<bool> = None;
    let mut auto_mixed = false;
    let mut selected_text: Option<testruct_core::document::TextElement> = None;
    let mut fill_applicable = false;
    let mut fill_state: Option<Option<testruct_core::typography::Color>> = None;
    let mut fill_mixed = false;
    let mut stroke_applicable = false;
    let mut stroke_state: Option<Option<testruct_core::typography::Color>> = None;
    let mut stroke_mixed = false;
    let mut stroke_width_state: Option<f32> = None;
    let mut stroke_width_mixed = false;

    if !selected_ids.is_empty() {
        app_state.with_active_document(|doc| {
            if let Some(page) = doc.pages.first() {
                for element in &page.elements {
                    if selected_ids.contains(&element.id()) {
                        match element {
                            DocumentElement::Shape(shape) => {
                                fill_applicable = true;
                                match &fill_state {
                                    None => fill_state = Some(shape.fill.clone()),
                                    Some(prev) => {
                                        if *prev != shape.fill {
                                            fill_mixed = true;
                                        }
                                    }
                                }

                                stroke_applicable = true;
                                match &stroke_state {
                                    None => stroke_state = Some(shape.stroke.clone()),
                                    Some(prev) => {
                                        if *prev != shape.stroke {
                                            stroke_mixed = true;
                                        }
                                    }
                                }

                                match stroke_width_state {
                                    None => stroke_width_state = Some(shape.stroke_width),
                                    Some(prev) => {
                                        if prev != shape.stroke_width {
                                            stroke_width_mixed = true;
                                        }
                                    }
                                }
                            }
                            DocumentElement::Text(text) => {
                                if selected_text.is_none() {
                                    selected_text = Some(text.clone());
                                }
                                match auto_state {
                                    None => auto_state = Some(text.auto_resize_height),
                                    Some(prev) if prev != text.auto_resize_height => {
                                        auto_mixed = true;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    if !fill_applicable {
        components.fill_color_button.set_sensitive(false);
        components.fill_color_button.set_label("色を選択");
    } else {
        components.fill_color_button.set_sensitive(true);
        let label = if fill_mixed {
            "混在".to_string()
        } else if let Some(Some(color)) = fill_state.as_ref() {
            color_to_hex(color)
        } else {
            "なし".to_string()
        };
        components.fill_color_button.set_label(&label);
    }

    if !stroke_applicable {
        components.stroke_color_button.set_sensitive(false);
        components.stroke_color_button.set_label("色を選択");
    } else {
        components.stroke_color_button.set_sensitive(true);
        let label = if stroke_mixed {
            "混在".to_string()
        } else if let Some(Some(color)) = stroke_state.as_ref() {
            color_to_hex(color)
        } else {
            "なし".to_string()
        };
        components.stroke_color_button.set_label(&label);
    }

    let auto_switch = &components.auto_resize_switch;
    if auto_mixed {
        if auto_switch.state() {
            auto_switch.set_state(false);
        }
        auto_switch.set_sensitive(false);
    } else if let Some(state) = auto_state {
        if auto_switch.state() != state {
            auto_switch.set_state(state);
        }
        auto_switch.set_sensitive(true);
    } else {
        if auto_switch.state() {
            auto_switch.set_state(false);
        }
        auto_switch.set_sensitive(false);
    }

    let text_controls_enabled = selected_text.is_some();
    components
        .text_content_view
        .set_sensitive(text_controls_enabled);
    components
        .font_family_combo
        .set_sensitive(text_controls_enabled);
    components
        .font_size_spin
        .set_sensitive(text_controls_enabled);
    components
        .line_height_scale
        .set_sensitive(text_controls_enabled);
    components
        .text_align_combo
        .set_sensitive(text_controls_enabled);
    components
        .bold_button
        .set_sensitive(text_controls_enabled);
    components
        .italic_button
        .set_sensitive(text_controls_enabled);
    components
        .underline_button
        .set_sensitive(text_controls_enabled);
    components
        .strikethrough_button
        .set_sensitive(text_controls_enabled);
    components
        .text_color_button
        .set_sensitive(text_controls_enabled);
    components
        .text_background_color_button
        .set_sensitive(text_controls_enabled);

    if let Some(text) = selected_text {
        buffer.begin_irreversible_action();
        buffer.set_text(&text.content);
        buffer.end_irreversible_action();
        tracing::debug!("✅ Property panel updated: text content loaded for selected element");

        components
            .font_size_spin
            .set_value(text.style.font_size as f64);

        let line_adjustment = components.line_height_scale.adjustment();
        let clamped_line_height =
            (text.style.line_height as f64).clamp(line_adjustment.lower(), line_adjustment.upper());
        components.line_height_scale.set_value(clamped_line_height);

        if let Some(font_index) =
            find_string_index(&components.font_family_combo, &text.style.font_family)
        {
            if components.font_family_combo.selected() != font_index {
                components.font_family_combo.set_selected(font_index);
            }
        }

        let align_index = match text.style.alignment {
            TextAlignment::Start => 0,
            TextAlignment::Center => 1,
            TextAlignment::End => 2,
            TextAlignment::Justified => 3,
        };
        if components.text_align_combo.selected() != align_index {
            components.text_align_combo.set_selected(align_index);
        }

        // Update text formatting buttons state
        if components.bold_button.is_active() != (text.style.weight == testruct_core::typography::FontWeight::Bold) {
            components.bold_button.set_active(text.style.weight == testruct_core::typography::FontWeight::Bold);
        }
        if components.italic_button.is_active() != text.style.italic {
            components.italic_button.set_active(text.style.italic);
        }
        if components.underline_button.is_active() != text.style.underline {
            components.underline_button.set_active(text.style.underline);
        }
        if components.strikethrough_button.is_active() != text.style.strikethrough {
            components.strikethrough_button.set_active(text.style.strikethrough);
        }

        // Update text background color button label
        let bg_color_label = if let Some(color) = &text.style.background_color {
            crate::panels::property_handlers::color_to_hex(color)
        } else {
            "なし".to_string()
        };
        components.text_background_color_button.set_label(&bg_color_label);
    }

    // Update stroke width spinner
    if !stroke_applicable {
        components.stroke_width_spin.set_sensitive(false);
        components.stroke_width_spin.set_value(2.0); // Default value
    } else {
        components.stroke_width_spin.set_sensitive(true);
        if !stroke_width_mixed {
            if let Some(width) = stroke_width_state {
                components.stroke_width_spin.set_value(width as f64);
            }
        }
    }
}
