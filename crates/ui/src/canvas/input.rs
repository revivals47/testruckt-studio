use gtk4::{prelude::*, DrawingArea, EventControllerMotion, GestureClick, GestureDrag, EventControllerKey};
use gtk4::gdk;
use crate::canvas::{CanvasRenderState, tools::{ToolMode, ShapeFactory}, selection::HitTest, mouse::{test_resize_handle, calculate_resize_bounds, CanvasMousePos, ResizeHandle}, rendering::{snap_rect_to_grid, snap_to_guide, GuideOrientation}};
use crate::app::AppState;
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::document::DocumentElement;

/// Initialize pointer events for canvas
pub fn wire_pointer_events(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    _app_state: &AppState,
) {
    drawing_area.set_focusable(true);

    // Keyboard event controller
    let key_controller = EventControllerKey::new();
    let render_state_keyboard = render_state.clone();
    let app_state_keyboard = _app_state.clone();
    let drawing_area_keyboard = drawing_area.clone();

    key_controller.connect_key_pressed(move |_controller, keyval, _keycode, state| {
        let render_state_kbd = render_state_keyboard.clone();
        let tool_state_ref = render_state_kbd.tool_state.borrow();
        let in_text_editing = tool_state_ref.editing_text_id.is_some();
        let editing_text_id = tool_state_ref.editing_text_id;
        let mut cursor_pos = tool_state_ref.editing_cursor_pos;
        drop(tool_state_ref);

        // Determine if shift and control are pressed
        let shift_pressed = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
        let ctrl_pressed = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);

        // Handle Ctrl+Shift+I to insert image
        if ctrl_pressed && shift_pressed && keyval == gtk4::gdk::Key::i {
            app_state_keyboard.with_active_document(|doc| {
                if let Some(page) = doc.pages.first_mut() {
                    let image = DocumentElement::Image(testruct_core::document::ImageElement {
                        id: uuid::Uuid::new_v4(),
                        source: testruct_core::workspace::assets::AssetRef::new(),
                        bounds: testruct_core::layout::Rect {
                            origin: testruct_core::layout::Point { x: 100.0, y: 100.0 },
                            size: testruct_core::layout::Size { width: 200.0, height: 150.0 },
                        },
                    });
                    page.elements.push(image);
                }
            });
            drawing_area_keyboard.queue_draw();
            tracing::info!("✅ Image inserted");
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Ctrl+Shift+S to save as template
        if ctrl_pressed && shift_pressed && keyval == gtk4::gdk::Key::s {
            if let Some(document) = app_state_keyboard.active_document() {
                let template_name = chrono::Local::now().format("template_%Y%m%d_%H%M%S").to_string();
                match crate::templates::save_template(&template_name, &document) {
                    Ok(_) => {
                        tracing::info!("✅ Document saved as template: {}", template_name);
                    }
                    Err(e) => {
                        tracing::error!("Failed to save template: {}", e);
                    }
                }
            }
            return gtk4::glib::Propagation::Stop;
        }

        // Handle text alignment shortcuts (Ctrl+L, Ctrl+E, Ctrl+R, Ctrl+C)
        if ctrl_pressed && in_text_editing {
            if let Some(text_id) = editing_text_id {
                let alignment = match keyval {
                    gtk4::gdk::Key::l => Some(testruct_core::typography::TextAlignment::Start),
                    gtk4::gdk::Key::r => Some(testruct_core::typography::TextAlignment::End),
                    gtk4::gdk::Key::e => Some(testruct_core::typography::TextAlignment::End),
                    gtk4::gdk::Key::c => Some(testruct_core::typography::TextAlignment::Center),
                    gtk4::gdk::Key::j => Some(testruct_core::typography::TextAlignment::Justified),
                    _ => None,
                };

                if let Some(new_alignment) = alignment {
                    app_state_keyboard.with_active_document(|doc| {
                        if let Some(page) = doc.pages.first_mut() {
                            for element in &mut page.elements {
                                if let DocumentElement::Text(text) = element {
                                    if text.id == text_id {
                                        text.style.alignment = new_alignment;
                                    }
                                }
                            }
                        }
                    });
                    drawing_area_keyboard.queue_draw();
                    let align_name = match new_alignment {
                        testruct_core::typography::TextAlignment::Start => "Left",
                        testruct_core::typography::TextAlignment::Center => "Center",
                        testruct_core::typography::TextAlignment::End => "Right",
                        testruct_core::typography::TextAlignment::Justified => "Justified",
                    };
                    tracing::info!("✅ Text alignment changed to: {}", align_name);
                    return gtk4::glib::Propagation::Stop;
                }
            }
        }

        // Handle text editing keys
        if in_text_editing {
            if let Some(text_id) = editing_text_id {
                let app_state_kbd = app_state_keyboard.clone();

                match keyval {
                    gtk4::gdk::Key::Escape => {
                        // Exit text editing mode
                        let mut tool_state = render_state_kbd.tool_state.borrow_mut();
                        tool_state.editing_text_id = None;
                        tool_state.editing_cursor_pos = 0;
                        drop(tool_state);
                        drawing_area_keyboard.queue_draw();
                        tracing::info!("✅ Exited text editing mode");
                        return gtk4::glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::BackSpace => {
                        // Delete character before cursor
                        if cursor_pos > 0 {
                            app_state_kbd.with_active_document(|doc| {
                                if let Some(page) = doc.pages.first_mut() {
                                    for element in &mut page.elements {
                                        if let DocumentElement::Text(text) = element {
                                            if text.id == text_id {
                                                if cursor_pos <= text.content.len() && cursor_pos > 0 {
                                                    text.content.remove(cursor_pos - 1);
                                                    cursor_pos -= 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                            let mut tool_state = render_state_kbd.tool_state.borrow_mut();
                            tool_state.editing_cursor_pos = cursor_pos;
                            drop(tool_state);
                            drawing_area_keyboard.queue_draw();
                            tracing::info!("✅ Deleted character at cursor position {}", cursor_pos);
                        }
                        return gtk4::glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::Delete => {
                        // Delete character at cursor
                        app_state_kbd.with_active_document(|doc| {
                            if let Some(page) = doc.pages.first_mut() {
                                for element in &mut page.elements {
                                    if let DocumentElement::Text(text) = element {
                                        if text.id == text_id {
                                            if cursor_pos < text.content.len() {
                                                text.content.remove(cursor_pos);
                                            }
                                        }
                                    }
                                }
                            }
                        });
                        drawing_area_keyboard.queue_draw();
                        tracing::info!("✅ Deleted character at cursor position {}", cursor_pos);
                        return gtk4::glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::Left => {
                        // Move cursor left
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                            let mut tool_state = render_state_kbd.tool_state.borrow_mut();
                            tool_state.editing_cursor_pos = cursor_pos;
                            drop(tool_state);
                            drawing_area_keyboard.queue_draw();
                            tracing::debug!("Cursor moved to position {}", cursor_pos);
                        }
                        return gtk4::glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::Right => {
                        // Move cursor right
                        if let Some(document) = app_state_kbd.active_document() {
                            if let Some(page) = document.pages.first() {
                                for element in &page.elements {
                                    if let DocumentElement::Text(text) = element {
                                        if text.id == text_id && cursor_pos < text.content.len() {
                                            cursor_pos += 1;
                                        }
                                    }
                                }
                            }
                        }
                        let mut tool_state = render_state_kbd.tool_state.borrow_mut();
                        tool_state.editing_cursor_pos = cursor_pos;
                        drop(tool_state);
                        drawing_area_keyboard.queue_draw();
                        tracing::debug!("Cursor moved to position {}", cursor_pos);
                        return gtk4::glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::Home => {
                        // Move cursor to start
                        cursor_pos = 0;
                        let mut tool_state = render_state_kbd.tool_state.borrow_mut();
                        tool_state.editing_cursor_pos = cursor_pos;
                        drop(tool_state);
                        drawing_area_keyboard.queue_draw();
                        tracing::debug!("Cursor moved to start");
                        return gtk4::glib::Propagation::Stop;
                    }
                    gtk4::gdk::Key::End => {
                        // Move cursor to end
                        if let Some(document) = app_state_kbd.active_document() {
                            if let Some(page) = document.pages.first() {
                                for element in &page.elements {
                                    if let DocumentElement::Text(text) = element {
                                        if text.id == text_id {
                                            cursor_pos = text.content.len();
                                        }
                                    }
                                }
                            }
                        }
                        let mut tool_state = render_state_kbd.tool_state.borrow_mut();
                        tool_state.editing_cursor_pos = cursor_pos;
                        drop(tool_state);
                        drawing_area_keyboard.queue_draw();
                        tracing::debug!("Cursor moved to end");
                        return gtk4::glib::Propagation::Stop;
                    }
                    _ => {
                        // Try to handle as text input
                        if let Some(ch) = keyval.to_unicode() {
                            if ch.is_ascii() && !ch.is_control() {
                                app_state_kbd.with_active_document(|doc| {
                                    if let Some(page) = doc.pages.first_mut() {
                                        for element in &mut page.elements {
                                            if let DocumentElement::Text(text) = element {
                                                if text.id == text_id {
                                                    text.content.insert(cursor_pos, ch);
                                                    cursor_pos += 1;
                                                }
                                            }
                                        }
                                    }
                                });
                                let mut tool_state = render_state_kbd.tool_state.borrow_mut();
                                tool_state.editing_cursor_pos = cursor_pos;
                                drop(tool_state);
                                drawing_area_keyboard.queue_draw();
                                tracing::debug!("✅ Inserted character '{}' at position {}", ch, cursor_pos - 1);
                                return gtk4::glib::Propagation::Stop;
                            }
                        }
                    }
                }
            }
        }

        // Handle object movement when NOT in text editing
        let movement_amount = if shift_pressed { 10.0 } else { 1.0 };

        // Handle arrow keys for object movement
        let handled = match keyval {
            gtk4::gdk::Key::Left => {
                if !in_text_editing {
                    move_selected_objects(&render_state_keyboard, &app_state_keyboard, -movement_amount, 0.0);
                    drawing_area_keyboard.queue_draw();
                    tracing::info!("✅ Move left ({}px)", movement_amount as i32);
                    true
                } else {
                    false
                }
            }
            gtk4::gdk::Key::Right => {
                if !in_text_editing {
                    move_selected_objects(&render_state_keyboard, &app_state_keyboard, movement_amount, 0.0);
                    drawing_area_keyboard.queue_draw();
                    tracing::info!("✅ Move right ({}px)", movement_amount as i32);
                    true
                } else {
                    false
                }
            }
            gtk4::gdk::Key::Up => {
                if !in_text_editing {
                    move_selected_objects(&render_state_keyboard, &app_state_keyboard, 0.0, -movement_amount);
                    drawing_area_keyboard.queue_draw();
                    tracing::info!("✅ Move up ({}px)", movement_amount as i32);
                    true
                } else {
                    false
                }
            }
            gtk4::gdk::Key::Down => {
                if !in_text_editing {
                    move_selected_objects(&render_state_keyboard, &app_state_keyboard, 0.0, movement_amount);
                    drawing_area_keyboard.queue_draw();
                    tracing::info!("✅ Move down ({}px)", movement_amount as i32);
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        if handled {
            gtk4::glib::Propagation::Stop
        } else {
            gtk4::glib::Propagation::Proceed
        }
    });
    drawing_area.add_controller(key_controller);

    // Motion controller for mouse movement
    let motion = EventControllerMotion::new();
    let render_state_motion = render_state.clone();
    let drawing_area_cursor = drawing_area.clone();
    let app_state_motion = _app_state.clone();
    motion.connect_motion(move |_controller, x, y| {
        let state = render_state_motion.clone();
        tracing::debug!("pointer moved: x={}, y={}", x, y);

        // Get canvas position
        let config = state.config.borrow();
        let ruler_config = state.ruler_config.borrow();
        let screen_x = x - (ruler_config.size + config.pan_x);
        let screen_y = y - (ruler_config.size + config.pan_y);
        let doc_x = screen_x / config.zoom;
        let doc_y = screen_y / config.zoom;
        drop(config);
        drop(ruler_config);

        let canvas_mouse_pos = CanvasMousePos {
            x: doc_x,
            y: doc_y,
        };

        // Check if cursor is over a resize handle of selected objects
        let selected = state.selected_ids.borrow();
        let mut cursor_name = "default";

        if let Some(document) = app_state_motion.active_document() {
            if let Some(page) = document.pages.first() {
                for selected_id in selected.iter() {
                    // Find the selected element
                    for element in &page.elements {
                        let (elem_id, bounds) = match element {
                            testruct_core::document::DocumentElement::Shape(shape) => (shape.id, &shape.bounds),
                            testruct_core::document::DocumentElement::Text(text) => (text.id, &text.bounds),
                            testruct_core::document::DocumentElement::Image(image) => (image.id, &image.bounds),
                            testruct_core::document::DocumentElement::Frame(frame) => (frame.id, &frame.bounds),
                        };

                        if elem_id == *selected_id {
                            // Test for resize handle hit
                            if let Some(handle) = test_resize_handle(canvas_mouse_pos, bounds, 8.0) {
                                cursor_name = match handle {
                                    ResizeHandle::TopLeft | ResizeHandle::BottomRight => "nwse-resize",
                                    ResizeHandle::TopRight | ResizeHandle::BottomLeft => "nesw-resize",
                                    ResizeHandle::Top | ResizeHandle::Bottom => "ns-resize",
                                    ResizeHandle::Left | ResizeHandle::Right => "ew-resize",
                                };
                                break;
                            }
                        }
                    }
                    if cursor_name != "default" {
                        break;
                    }
                }
            }
        }

        // Set cursor
        let cursor = gdk::Cursor::from_name(cursor_name, None);
        drawing_area_cursor.set_cursor(cursor.as_ref());
    });
    drawing_area.add_controller(motion);

    // Click gesture for object selection
    let click_gesture = GestureClick::new();
    click_gesture.set_button(gdk::BUTTON_PRIMARY);
    let render_state_click = render_state.clone();
    let app_state_click = _app_state.clone();
    let drawing_area_click = drawing_area.clone();

    click_gesture.connect_pressed(move |gesture, n_press, x, y| {
        let state = render_state_click.clone();
        let tool_state = state.tool_state.borrow();
        let current_tool = tool_state.current_tool;
        drop(tool_state);

        tracing::debug!("canvas click at ({}, {}), tool: {:?}, n_press: {}", x, y, current_tool, n_press);

        if current_tool == ToolMode::Select {
            // Get modifier key state
            let modifier_state = match gesture.last_event(None) {
                Some(event) => event.modifier_state(),
                None => gdk::ModifierType::empty(),
            };
            let shift_pressed = modifier_state.contains(gdk::ModifierType::SHIFT_MASK);
            let ctrl_pressed = modifier_state.contains(gdk::ModifierType::CONTROL_MASK);

            // Check for double-click (n_press == 2) for text editing
            if n_press == 2 {
                // Try to find a text element at this position
                if let Some(document) = app_state_click.active_document() {
                    if let Some(page) = document.pages.first() {
                        let config = state.config.borrow();
                        let ruler_config = state.ruler_config.borrow();
                        let screen_x = x - (ruler_config.size + config.pan_x);
                        let screen_y = y - (ruler_config.size + config.pan_y);
                        let doc_x = screen_x / config.zoom;
                        let doc_y = screen_y / config.zoom;
                        drop(config);
                        drop(ruler_config);

                        // Check if double-click is on a text element
                        for element in &page.elements {
                            if let DocumentElement::Text(text) = element {
                                let bounds = &text.bounds;
                                // Check if click is within bounds
                                if doc_x >= bounds.origin.x as f64 && doc_x <= (bounds.origin.x + bounds.size.width) as f64
                                    && doc_y >= bounds.origin.y as f64 && doc_y <= (bounds.origin.y + bounds.size.height) as f64 {
                                    // Enter text editing mode
                                    let mut tool_state = state.tool_state.borrow_mut();
                                    tool_state.editing_text_id = Some(text.id);
                                    tool_state.editing_cursor_pos = text.content.len();
                                    drop(tool_state);
                                    drawing_area_click.queue_draw();
                                    tracing::info!("✅ Entered text editing mode for text element: {}", text.id);
                                    return;
                                }
                            }
                        }
                    }
                }
            }

            // Get the active document
            if let Some(document) = app_state_click.active_document() {
                // Transform screen coordinates to document coordinates
                let config = state.config.borrow();
                let ruler_config = state.ruler_config.borrow();
                let screen_x = x - (ruler_config.size + config.pan_x);
                let screen_y = y - (ruler_config.size + config.pan_y);
                let doc_x = screen_x / config.zoom;
                let doc_y = screen_y / config.zoom;
                let canvas_mouse_pos = CanvasMousePos::new(doc_x, doc_y);
                drop(config);
                drop(ruler_config);

                // Check if clicking on a resize handle of a selected object
                let selected_ids = state.selected_ids.borrow();
                let mut resize_detected = false;

                if let Some(page) = document.pages.first() {
                    for element in &page.elements {
                        let element_id = match element {
                            DocumentElement::Shape(shape) => shape.id,
                            DocumentElement::Text(text) => text.id,
                            DocumentElement::Image(image) => image.id,
                            DocumentElement::Frame(frame) => frame.id,
                        };

                        // Only check resize handles for selected objects
                        if !selected_ids.contains(&element_id) {
                            continue;
                        }

                        let bounds = match element {
                            DocumentElement::Shape(shape) => &shape.bounds,
                            DocumentElement::Text(text) => &text.bounds,
                            DocumentElement::Image(image) => &image.bounds,
                            DocumentElement::Frame(frame) => &frame.bounds,
                        };

                        // Test for resize handle hit
                        if let Some(handle) = test_resize_handle(canvas_mouse_pos, bounds, 8.0) {
                            // Store resize state
                            let mut tool_state = state.tool_state.borrow_mut();
                            tool_state.resizing_object_id = Some(element_id);
                            tool_state.resize_handle = Some(handle);
                            tool_state.resize_original_bounds = Some(canvas_mouse_pos);
                            tool_state.drag_start = Some((x, y));
                            drop(tool_state);

                            tracing::info!("✅ Started resizing object {} with handle {:?}", element_id, handle);
                            resize_detected = true;
                            break;
                        }
                    }
                }

                drop(selected_ids);

                // If resize was detected, don't do selection handling
                if resize_detected {
                    drawing_area_click.queue_draw();
                    return;
                }
                if let Some(page) = document.pages.first() {
                    // Build list of objects with their bounds for hit testing
                    let mut objects: Vec<(uuid::Uuid, Rect)> = Vec::new();

                    for element in &page.elements {
                        match element {
                            DocumentElement::Shape(shape) => {
                                objects.push((shape.id, shape.bounds.clone()));
                            }
                            DocumentElement::Text(text) => {
                                objects.push((text.id, text.bounds.clone()));
                            }
                            DocumentElement::Image(image) => {
                                objects.push((image.id, image.bounds.clone()));
                            }
                            DocumentElement::Frame(frame) => {
                                objects.push((frame.id, frame.bounds.clone()));
                            }
                        }
                    }

                    // Convert to references for hit testing
                    let object_refs: Vec<(uuid::Uuid, &Rect)> = objects.iter().map(|(id, bounds)| (*id, bounds)).collect();

                    // Transform screen coordinates to document coordinates
                    let config = state.config.borrow();
                    let ruler_config = state.ruler_config.borrow();

                    let screen_x = x - (ruler_config.size + config.pan_x);
                    let screen_y = y - (ruler_config.size + config.pan_y);
                    let doc_x = screen_x / config.zoom;
                    let doc_y = screen_y / config.zoom;

                    drop(config);
                    drop(ruler_config);

                    // Perform hit test
                    if let Some(clicked_id) = HitTest::hit_test(&object_refs, doc_x, doc_y) {
                        tracing::info!("✅ Hit test: selected object {}", clicked_id);

                        // Update selection based on modifier keys
                        let mut selected = state.selected_ids.borrow_mut();

                        if shift_pressed {
                            // Shift+click: add to selection
                            if !selected.contains(&clicked_id) {
                                selected.push(clicked_id);
                                tracing::debug!("Added object to selection (Shift+click)");
                            }
                        } else if ctrl_pressed {
                            // Ctrl+click: toggle selection
                            if let Some(pos) = selected.iter().position(|&id| id == clicked_id) {
                                selected.remove(pos);
                                tracing::debug!("Removed object from selection (Ctrl+click)");
                            } else {
                                selected.push(clicked_id);
                                tracing::debug!("Toggled object selection (Ctrl+click)");
                            }
                        } else {
                            // Plain click: single select
                            selected.clear();
                            selected.push(clicked_id);
                            tracing::info!("Selected object: {}", clicked_id);
                        }
                        drop(selected);
                        drawing_area_click.queue_draw();
                    } else {
                        // Clicked on empty space: clear selection
                        let mut selected = state.selected_ids.borrow_mut();
                        selected.clear();
                        drop(selected);
                        drawing_area_click.queue_draw();
                        tracing::debug!("Cleared selection (empty space click)");
                    }
                }
            }
        }
    });
    drawing_area.add_controller(click_gesture);

    // Drag gesture for shape creation and object movement
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(gdk::BUTTON_PRIMARY);
    let render_state_drag = render_state.clone();
    let drawing_area_drag = drawing_area.clone();

    drag_gesture.connect_drag_begin(move |_gesture, x, y| {
        let state = render_state_drag.clone();
        let tool_state = state.tool_state.borrow();
        let current_tool = tool_state.current_tool;
        drop(tool_state);

        tracing::debug!("drag start at ({}, {}), tool: {:?}", x, y, current_tool);

        // Store drag start position
        let mut tool_state = state.tool_state.borrow_mut();
        tool_state.drag_start = Some((x, y));
    });

    let render_state_update = render_state.clone();
    drag_gesture.connect_drag_update(move |_gesture, offset_x, offset_y| {
        let state = render_state_update.clone();
        let tool_state = state.tool_state.borrow();
        if let Some((start_x, start_y)) = tool_state.drag_start {
            let current_x = start_x + offset_x;
            let current_y = start_y + offset_y;

            tracing::debug!(
                "drag update: from ({}, {}) to ({}, {})",
                start_x, start_y, current_x, current_y
            );

            // Update drag box for preview rendering
            let (x1, y1, x2, y2) = (start_x, start_y, current_x, current_y);
            let min_x = x1.min(x2);
            let min_y = y1.min(y2);
            let max_x = x1.max(x2);
            let max_y = y1.max(y2);

            let drag_rect = Rect {
                origin: Point {
                    x: min_x as f32,
                    y: min_y as f32,
                },
                size: Size {
                    width: (max_x - min_x) as f32,
                    height: (max_y - min_y) as f32,
                },
            };

            *state.drag_box.borrow_mut() = Some(drag_rect);
        }

        drawing_area_drag.queue_draw();
    });

    let render_state_end = render_state.clone();
    let drawing_area_end = drawing_area.clone();
    let app_state_drag_end = _app_state.clone();

    drag_gesture.connect_drag_end(move |_gesture, offset_x, offset_y| {
        let state = render_state_end.clone();
        let tool_state = state.tool_state.borrow();

        if let Some((start_x, start_y)) = tool_state.drag_start {
            let current_x = start_x + offset_x;
            let current_y = start_y + offset_y;
            let current_tool = tool_state.current_tool;
            let is_resizing = tool_state.resizing_object_id.is_some();
            let resizing_object_id = tool_state.resizing_object_id;
            let resize_handle = tool_state.resize_handle;
            let resize_original_bounds = tool_state.resize_original_bounds;

            tracing::debug!(
                "drag end: shape creation/move from ({}, {}) to ({}, {})",
                start_x, start_y, current_x, current_y
            );

            if is_resizing && (offset_x.abs() > 2.0 || offset_y.abs() > 2.0) {
                // Apply resize
                if let (Some(object_id), Some(handle), Some(_mouse_pos)) = (resizing_object_id, resize_handle, resize_original_bounds) {
                    // Calculate document-space delta
                    let config = state.config.borrow();
                    let delta_x = offset_x / config.zoom;
                    let delta_y = offset_y / config.zoom;
                    let snap_enabled = config.snap_to_grid;
                    let grid_spacing = config.grid_spacing;
                    drop(config);

                    // Get the original bounds from the document
                    if let Some(mut document) = app_state_drag_end.active_document() {
                        if let Some(page) = document.pages.first_mut() {
                            for element in page.elements.iter_mut() {
                                match element {
                                    DocumentElement::Shape(shape) if shape.id == object_id => {
                                        let mut new_bounds = calculate_resize_bounds(&shape.bounds, handle, delta_x, delta_y);
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        shape.bounds = new_bounds;
                                        tracing::info!("✅ Resized shape {} with handle {:?}", object_id, handle);
                                    }
                                    DocumentElement::Image(image) if image.id == object_id => {
                                        let mut new_bounds = calculate_resize_bounds(&image.bounds, handle, delta_x, delta_y);
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        image.bounds = new_bounds;
                                        tracing::info!("✅ Resized image {} with handle {:?}", object_id, handle);
                                    }
                                    DocumentElement::Frame(frame) if frame.id == object_id => {
                                        let mut new_bounds = calculate_resize_bounds(&frame.bounds, handle, delta_x, delta_y);
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        frame.bounds = new_bounds;
                                        tracing::info!("✅ Resized frame {} with handle {:?}", object_id, handle);
                                    }
                                    _ => {}
                                }
                            }
                            // Update the document in app state
                            app_state_drag_end.with_active_document(|doc| {
                                *doc = document;
                            });
                        }
                    }
                }
            } else if current_tool == ToolMode::Select && (offset_x.abs() > 5.0 || offset_y.abs() > 5.0) {
                // Move selected objects
                let selected = state.selected_ids.borrow();
                if !selected.is_empty() {
                    // Transform screen offset to document offset
                    let config = state.config.borrow();
                    let delta_x = offset_x / config.zoom;
                    let delta_y = offset_y / config.zoom;
                    let snap_enabled = config.snap_to_grid;
                    let grid_spacing = config.grid_spacing;
                    drop(config);

                    let selected_ids: Vec<uuid::Uuid> = selected.clone();
                    drop(selected);

                    // Move each selected object
                    if let Some(mut document) = app_state_drag_end.active_document() {
                        if let Some(page) = document.pages.first_mut() {
                            for element in page.elements.iter_mut() {
                                match element {
                                    DocumentElement::Shape(shape) if selected_ids.contains(&shape.id) => {
                                        let mut new_bounds = shape.bounds.clone();
                                        new_bounds.origin.x += delta_x as f32;
                                        new_bounds.origin.y += delta_y as f32;
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        shape.bounds = new_bounds;
                                    }
                                    DocumentElement::Text(text) if selected_ids.contains(&text.id) => {
                                        let mut new_bounds = text.bounds.clone();
                                        new_bounds.origin.x += delta_x as f32;
                                        new_bounds.origin.y += delta_y as f32;
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        text.bounds = new_bounds;
                                    }
                                    DocumentElement::Image(image) if selected_ids.contains(&image.id) => {
                                        let mut new_bounds = image.bounds.clone();
                                        new_bounds.origin.x += delta_x as f32;
                                        new_bounds.origin.y += delta_y as f32;
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        image.bounds = new_bounds;
                                    }
                                    DocumentElement::Frame(frame) if selected_ids.contains(&frame.id) => {
                                        let mut new_bounds = frame.bounds.clone();
                                        new_bounds.origin.x += delta_x as f32;
                                        new_bounds.origin.y += delta_y as f32;
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        frame.bounds = new_bounds;
                                    }
                                    _ => {}
                                }
                            }
                            // Update the document in app state
                            app_state_drag_end.with_active_document(|doc| {
                                *doc = document;
                            });
                            tracing::info!("✅ Moved {} selected object(s)", selected_ids.len());
                        }
                    }
                }
            } else if current_tool != ToolMode::Select && (offset_x.abs() > 5.0 || offset_y.abs() > 5.0) {
                // Shape creation based on tool
                let element = match current_tool {
                    ToolMode::Rectangle => ShapeFactory::create_rectangle(
                        start_x.min(current_x),
                        start_y.min(current_y),
                        (start_x - current_x).abs(),
                        (start_y - current_y).abs(),
                    ),
                    ToolMode::Circle => ShapeFactory::create_circle(
                        start_x.min(current_x),
                        start_y.min(current_y),
                        (start_x - current_x).abs(),
                        (start_y - current_y).abs(),
                    ),
                    ToolMode::Line => ShapeFactory::create_line(
                        start_x,
                        start_y,
                        current_x,
                        current_y,
                    ),
                    ToolMode::Arrow => ShapeFactory::create_arrow(
                        start_x,
                        start_y,
                        current_x,
                        current_y,
                    ),
                    ToolMode::Image => ShapeFactory::create_image(
                        start_x.min(current_x),
                        start_y.min(current_y),
                        (start_x - current_x).abs(),
                        (start_y - current_y).abs(),
                    ),
                    ToolMode::Text => ShapeFactory::create_text(
                        start_x,
                        start_y,
                        (start_x - current_x).abs(),
                        (start_y - current_y).abs(),
                        "New Text".to_string(),
                    ),
                    _ => return,
                };

                // Add element to document
                if let Err(e) = app_state_drag_end.add_element_to_active_page(element) {
                    tracing::warn!("Failed to add element: {}", e);
                } else {
                    tracing::info!("✅ {} element added to document", current_tool.name());
                }
            }
        }

        // Clear drag state
        drop(tool_state);
        let mut tool_state = state.tool_state.borrow_mut();
        tool_state.drag_start = None;
        tool_state.resizing_object_id = None;
        tool_state.resize_handle = None;
        tool_state.resize_original_bounds = None;
        *state.drag_box.borrow_mut() = None;

        drawing_area_end.queue_draw();
    });

    drawing_area.add_controller(drag_gesture);
}

/// Apply guide snapping to bounds
fn apply_guide_snapping(bounds: &Rect, config: &crate::canvas::rendering::RenderConfig) -> Rect {
    let mut snapped_bounds = bounds.clone();

    if config.snap_to_guides && !config.guides.is_empty() {
        // Try snapping origin X
        if let Some(snapped_x) = snap_to_guide(
            bounds.origin.x,
            &config.guides,
            GuideOrientation::Vertical,
            config.guide_snap_distance,
        ) {
            snapped_bounds.origin.x = snapped_x;
        }

        // Try snapping origin Y
        if let Some(snapped_y) = snap_to_guide(
            bounds.origin.y,
            &config.guides,
            GuideOrientation::Horizontal,
            config.guide_snap_distance,
        ) {
            snapped_bounds.origin.y = snapped_y;
        }
    }

    snapped_bounds
}

/// Helper function to move selected objects
fn move_selected_objects(
    render_state: &CanvasRenderState,
    app_state: &AppState,
    delta_x: f64,
    delta_y: f64,
) {
    // Get selected object IDs
    let selected_ids = render_state.selected_ids.borrow();
    if selected_ids.is_empty() {
        return;
    }
    let selected_ids_copy = selected_ids.clone();
    drop(selected_ids);

    // Get snap configuration
    let config = render_state.config.borrow();
    let snap_enabled = config.snap_to_grid;
    let grid_spacing = config.grid_spacing;
    let config_clone = config.clone();
    drop(config);

    // Get the active document
    if let Some(mut document) = app_state.active_document() {
        if let Some(page) = document.pages.first_mut() {
            // Move each selected object
            for element in page.elements.iter_mut() {
                match element {
                    DocumentElement::Shape(shape) if selected_ids_copy.contains(&shape.id) => {
                        let mut new_bounds = shape.bounds.clone();
                        new_bounds.origin.x += delta_x as f32;
                        new_bounds.origin.y += delta_y as f32;

                        // Apply grid snapping if enabled
                        if snap_enabled {
                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                        }

                        // Apply guide snapping
                        new_bounds = apply_guide_snapping(&new_bounds, &config_clone);

                        shape.bounds = new_bounds;
                    }
                    DocumentElement::Text(text) if selected_ids_copy.contains(&text.id) => {
                        let mut new_bounds = text.bounds.clone();
                        new_bounds.origin.x += delta_x as f32;
                        new_bounds.origin.y += delta_y as f32;

                        // Apply grid snapping if enabled
                        if snap_enabled {
                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                        }

                        // Apply guide snapping
                        new_bounds = apply_guide_snapping(&new_bounds, &config_clone);

                        text.bounds = new_bounds;
                    }
                    DocumentElement::Image(image) if selected_ids_copy.contains(&image.id) => {
                        let mut new_bounds = image.bounds.clone();
                        new_bounds.origin.x += delta_x as f32;
                        new_bounds.origin.y += delta_y as f32;

                        // Apply grid snapping if enabled
                        if snap_enabled {
                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                        }

                        // Apply guide snapping
                        new_bounds = apply_guide_snapping(&new_bounds, &config_clone);

                        image.bounds = new_bounds;
                    }
                    DocumentElement::Frame(frame) if selected_ids_copy.contains(&frame.id) => {
                        let mut new_bounds = frame.bounds.clone();
                        new_bounds.origin.x += delta_x as f32;
                        new_bounds.origin.y += delta_y as f32;

                        // Apply grid snapping if enabled
                        if snap_enabled {
                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                        }

                        // Apply guide snapping
                        new_bounds = apply_guide_snapping(&new_bounds, &config_clone);

                        frame.bounds = new_bounds;
                    }
                    _ => {}
                }
            }

            // Update the document in app state
            app_state.with_active_document(|doc| {
                *doc = document;
            });

            tracing::debug!("Moved {} selected object(s) by ({}, {})", selected_ids_copy.len(), delta_x, delta_y);
        }
    }
}
