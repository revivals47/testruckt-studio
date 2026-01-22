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
                    let page_index = app_state_for_cb.active_page_index();

                    // Create undo command for stroke color change
                    let command = crate::undo_redo::AppPropertyChangeCommand::new(
                        app_state_for_cb.clone(),
                        selected_ids_for_cb.clone(),
                        page_index,
                        crate::undo_redo::PropertyValue::StrokeColor(Some(stroke_color)),
                    );

                    // Push command (this executes the change and adds to undo stack)
                    app_state_for_cb.push_command(Box::new(command));

                    // Mark document as modified
                    app_state_for_cb.mark_as_modified();

                    drawing_area_for_cb.queue_draw();
                    crate::panels::property_handlers::update_property_panel_on_selection(
                        &panel_for_cb,
                        &app_state_for_cb,
                        &selected_ids_for_cb,
                    );
                    tracing::debug!("✅ Stroke color updated via dialog (with undo support)");
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
                    let page_index = app_state_for_cb.active_page_index();

                    // Create undo command for fill color change
                    let command = crate::undo_redo::AppPropertyChangeCommand::new(
                        app_state_for_cb.clone(),
                        selected_ids_for_cb.clone(),
                        page_index,
                        crate::undo_redo::PropertyValue::FillColor(Some(fill_color)),
                    );

                    // Push command (this executes the change and adds to undo stack)
                    app_state_for_cb.push_command(Box::new(command));

                    // Mark document as modified
                    app_state_for_cb.mark_as_modified();

                    drawing_area_for_cb.queue_draw();
                    crate::panels::property_handlers::update_property_panel_on_selection(
                        &panel_for_cb,
                        &app_state_for_cb,
                        &selected_ids_for_cb,
                    );
                    tracing::debug!("✅ Fill color updated via dialog (with undo support)");
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
            // Mark document as modified
            app_state.mark_as_modified();

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
        r: rgba.red(),
        g: rgba.green(),
        b: rgba.blue(),
        a: rgba.alpha(),
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

/// State for debounced stroke width undo
struct StrokeWidthUndoState {
    /// Original stroke widths when user started changing
    original_widths: Vec<(uuid::Uuid, f32)>,
    /// Selected element IDs when change started
    selected_ids: Vec<uuid::Uuid>,
    /// Pending timeout source ID
    timeout_source: Option<gtk4::glib::SourceId>,
    /// Whether we're in the middle of a change sequence
    is_changing: bool,
}

/// Wire stroke width spinner to update shape stroke width with debounced undo
pub fn wire_stroke_width_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    use std::cell::RefCell;
    use std::rc::Rc;

    let spinner = components.stroke_width_spin.clone();

    // Shared state for debounce tracking
    let undo_state = Rc::new(RefCell::new(StrokeWidthUndoState {
        original_widths: Vec::new(),
        selected_ids: Vec::new(),
        timeout_source: None,
        is_changing: false,
    }));

    let undo_state_clone = undo_state.clone();
    let app_state_clone = app_state.clone();
    let render_state_clone = render_state.clone();

    spinner.connect_value_changed(move |spin| {
        let stroke_width = spin.value() as f32;
        let mut state = undo_state_clone.borrow_mut();

        // Get current selection
        let selected: Vec<uuid::Uuid> = render_state_clone.selected_ids.borrow().clone();
        if selected.is_empty() {
            return;
        }

        // If this is the first change in a sequence, capture original values
        if !state.is_changing {
            state.is_changing = true;
            state.selected_ids = selected.clone();
            state.original_widths.clear();

            // Capture original stroke widths
            app_state_clone.with_active_document(|doc| {
                if let Some(page) = doc.pages.first() {
                    for element in &page.elements {
                        if let DocumentElement::Shape(shape) = element {
                            if selected.contains(&shape.id) {
                                state.original_widths.push((shape.id, shape.stroke_width));
                            }
                        }
                    }
                }
            });
        }

        // Cancel any existing timeout
        if let Some(source_id) = state.timeout_source.take() {
            source_id.remove();
        }

        drop(state); // Release borrow before document mutation

        // Apply the change immediately for visual feedback
        let changed = app_state_clone.with_mutable_active_document(|doc| {
            let mut modified = false;
            if let Some(page) = doc.pages.first_mut() {
                for element in &mut page.elements {
                    if let DocumentElement::Shape(shape) = element {
                        if selected.contains(&shape.id) {
                            shape.stroke_width = stroke_width;
                            modified = true;
                        }
                    }
                }
            }
            modified
        });

        if changed.unwrap_or(false) {
            app_state_clone.mark_as_modified();
        }

        drawing_area.queue_draw();

        // Set up debounce timeout (500ms)
        let undo_state_timeout = undo_state_clone.clone();
        let app_state_timeout = app_state_clone.clone();
        let final_stroke_width = stroke_width;

        let source_id = gtk4::glib::timeout_add_local_once(
            std::time::Duration::from_millis(500),
            move || {
                let mut state = undo_state_timeout.borrow_mut();

                if !state.is_changing || state.original_widths.is_empty() {
                    state.is_changing = false;
                    return;
                }

                // Create undo command with original values and final value
                let page_index = app_state_timeout.active_page_index();

                // Only create undo if value actually changed
                let value_changed = state.original_widths.iter().any(|(_, orig)| {
                    (*orig - final_stroke_width).abs() > 0.001
                });

                if value_changed {
                    // Create command with pre-captured original values
                    let command = crate::undo_redo::AppStrokeWidthCommand::new(
                        app_state_timeout.clone(),
                        page_index,
                        state.original_widths.clone(),
                        final_stroke_width,
                    );

                    // Push command to undo stack (execute() is a no-op since value is already applied)
                    app_state_timeout.push_command(Box::new(command));

                    tracing::debug!("✅ Stroke width undo command created (debounced)");
                }

                // Reset state
                state.is_changing = false;
                state.original_widths.clear();
                state.selected_ids.clear();
                state.timeout_source = None;
            },
        );

        // Store the source ID
        undo_state_clone.borrow_mut().timeout_source = Some(source_id);
    });
}
