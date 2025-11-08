//! Property panel signal handlers
//!
//! Wires up the property panel UI controls to canvas object updates

use gtk4::{gdk, gio};
use gtk4::{prelude::*, ColorDialog, StringList};
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

    // Font family selection
    wire_font_family_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Font size
    wire_font_size_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Bold button
    wire_bold_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Italic button
    wire_italic_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Stroke color
    wire_stroke_color_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Fill color
    wire_fill_color_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Auto-resize switch
    wire_auto_resize_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Text alignment dropdown
    wire_alignment_dropdown(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Line height scale
    wire_line_height_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

    // Text content editing
    wire_text_content_signal(
        components,
        app_state.clone(),
        drawing_area.clone(),
        render_state.clone(),
    );

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
fn wire_font_size_signal(
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
fn wire_bold_signal(
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
fn wire_italic_signal(
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

/// Wire stroke color button
fn wire_stroke_color_signal(
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
                        update_property_panel_on_selection(
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
fn wire_fill_color_signal(
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
                        update_property_panel_on_selection(
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
fn wire_auto_resize_signal(
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
fn wire_line_height_signal(
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

    // Setup selection monitoring to update buffer when selection changes
    let buffer_for_select = components.text_content_buffer.clone();
    let app_state_for_select = app_state.clone();
    let render_state_for_select = render_state.clone();

    // Create a closure to update the buffer when selection changes
    let update_buffer_on_selection = move || {
        let selected = render_state_for_select.selected_ids.borrow();

        // Clear buffer first
        buffer_for_select.delete(
            &mut buffer_for_select.start_iter(),
            &mut buffer_for_select.end_iter(),
        );

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

fn color_to_hex(color: &testruct_core::typography::Color) -> String {
    let clamp = |v: f32| -> u8 { (v.clamp(0.0, 1.0) * 255.0).round() as u8 };
    format!(
        "#{:02X}{:02X}{:02X}",
        clamp(color.r),
        clamp(color.g),
        clamp(color.b)
    )
}

fn dropdown_string(dropdown: &gtk4::DropDown, index: u32) -> Option<String> {
    if index == gtk4::INVALID_LIST_POSITION {
        return None;
    }
    let model = dropdown.model()?;
    let string_list = model.downcast::<StringList>().ok()?;
    string_list.string(index).map(|s| s.to_string())
}

fn find_string_index(dropdown: &gtk4::DropDown, value: &str) -> Option<u32> {
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
    }
}
