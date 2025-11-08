//! クリックジェスチャー処理モジュール
//!
//! 単一クリックおよびダブルクリックでオブジェクト選択、テキスト編集、
//! 画像選択などを処理します。
//!
//! # クリックジェスチャー (GestureClick)
//!
//! 単一クリックまたは複数クリックでオブジェクトを選択します。
//!
//! ## 選択モード
//!
//! | 操作 | 動作 |
//! |------|------|
//! | クリック | 単一選択（既存選択をクリア） |
//! | Shift+クリック | 選択に追加 |
//! | Ctrl+クリック | トグル選択（選択/解除） |
//! | ダブルクリック（テキスト） | テキスト編集モードに進入 |
//! | ダブルクリック（画像） | 画像ファイル選択ダイアログを表示 |
//! | 空白クリック | 選択をクリア |
//!
//! ## リサイズハンドル
//!
//! 選択オブジェクトのリサイズハンドル（8方向）を検出し、クリック時に
//! リサイズ操作の開始位置として設定します。

use crate::app::AppState;
use crate::canvas::mouse::{test_resize_handle, CanvasMousePos};
use crate::canvas::selection::HitTest;
use crate::canvas::tools::ToolMode;
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureClick};
use gtk4::gdk;
use testruct_core::document::DocumentElement;
use testruct_core::layout::Rect;

/// クリックジェスチャーを設定
pub fn setup_click_gesture(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    let click_gesture = GestureClick::new();
    click_gesture.set_button(gdk::BUTTON_PRIMARY);
    click_gesture.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let render_state_click = render_state.clone();
    let app_state_click = app_state.clone();
    let drawing_area_click = drawing_area.clone();

    click_gesture.connect_pressed(move |gesture, n_press, x, y| {
        let state = render_state_click.clone();
        let tool_state = state.tool_state.borrow();
        let current_tool = tool_state.current_tool;
        drop(tool_state);

        if current_tool == ToolMode::Select {
            // Get modifier key state
            let modifier_state = match gesture.last_event(None) {
                Some(event) => event.modifier_state(),
                None => gdk::ModifierType::empty(),
            };
            let shift_pressed = modifier_state.contains(gdk::ModifierType::SHIFT_MASK);
            let ctrl_pressed = modifier_state.contains(gdk::ModifierType::CONTROL_MASK);

            // Check for double-click (n_press == 2) for text editing or image selection
            eprintln!("Click: n_press={}, tool=Select", n_press);
            if n_press == 2 {
                eprintln!("Double-click detected at ({:.0}, {:.0})", x, y);
                // Try to find a text or image element at this position
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

                        // Check if double-click is on a text or image element
                        for element in &page.elements {
                            match element {
                                DocumentElement::Text(text) => {
                                    let bounds = &text.bounds;
                                    // Check if click is within bounds
                                    if doc_x >= bounds.origin.x as f64
                                        && doc_x <= (bounds.origin.x + bounds.size.width) as f64
                                        && doc_y >= bounds.origin.y as f64
                                        && doc_y <= (bounds.origin.y + bounds.size.height) as f64
                                    {
                                        // Enter text editing mode
                                        let mut tool_state = state.tool_state.borrow_mut();
                                        tool_state.editing_text_id = Some(text.id);
                                        tool_state.editing_cursor_pos = text.content.len();
                                        drop(tool_state);
                                        drawing_area_click.queue_draw();
                                        tracing::info!(
                                            "Entered text editing mode for text element: {}",
                                            text.id
                                        );
                                        return;
                                    }
                                }
                                DocumentElement::Image(image) => {
                                    let bounds = &image.bounds;
                                    // Check if click is within bounds
                                    if doc_x >= bounds.origin.x as f64
                                        && doc_x <= (bounds.origin.x + bounds.size.width) as f64
                                        && doc_y >= bounds.origin.y as f64
                                        && doc_y <= (bounds.origin.y + bounds.size.height) as f64
                                    {
                                        // Image double-click detected
                                        tracing::info!(
                                            "Image double-click detected for image: {}",
                                            image.id
                                        );

                                        // Select the image on the canvas
                                        let mut selected = state.selected_ids.borrow_mut();
                                        selected.clear();
                                        selected.push(image.id);
                                        drop(selected);
                                        drawing_area_click.queue_draw();

                                        // Show image selection dialog
                                        if let Some(window) = app_state_click.window() {
                                            let image_id = image.id;
                                            let app_state_dialog = app_state_click.clone();
                                            let drawing_area_dialog = drawing_area_click.clone();

                                            // Cast ApplicationWindow to Window for the dialog
                                            use gtk4::glib::Cast;
                                            let window_ref = window.upcast::<gtk4::Window>();

                                            crate::dialogs::image_dialog::show_image_chooser_async(
                                                &window_ref,
                                                Box::new(move |path| {
                                                    tracing::info!(
                                                        "Image selected for block {}: {}",
                                                        image_id,
                                                        path.display()
                                                    );

                                                    // Update the asset catalog with the selected image
                                                    let asset_ref = {
                                                        let catalog = app_state_dialog.asset_catalog();
                                                        let mut cat = catalog.lock().expect("asset catalog");
                                                        cat.register(&path)
                                                    };

                                                    // Store the image path in the document
                                                    let _ = app_state_dialog.with_active_document(|doc| {
                                                        if let Some(page) = doc.pages.first_mut() {
                                                            for element in &mut page.elements {
                                                                if let DocumentElement::Image(img) = element {
                                                                    if img.id == image_id {
                                                                        img.source = asset_ref;
                                                                        tracing::info!("Updated image element with asset reference");
                                                                        break;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    });

                                                    // Redraw the canvas
                                                    drawing_area_dialog.queue_draw();
                                                }),
                                            );
                                        } else {
                                            tracing::warn!("Window not available for image dialog");
                                        }

                                        return;
                                    }
                                }
                                _ => {}
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

                // IMPORTANT: Check if clicking on a resize handle FIRST
                // This must happen BEFORE double-click text editing check
                // because users should be able to resize text/image boxes
                let selected_ids = state.selected_ids.borrow();
                let mut resize_detected = false;

                if let Some(page) = document.pages.first() {
                    for element in &page.elements {
                        let element_id = match element {
                            DocumentElement::Shape(shape) => shape.id,
                            DocumentElement::Text(text) => text.id,
                            DocumentElement::Image(image) => image.id,
                            DocumentElement::Frame(frame) => frame.id,
                            DocumentElement::Group(group) => group.id,
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
                            DocumentElement::Group(group) => &group.bounds,
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

                            eprintln!("✏️ RESIZE HANDLE DETECTED: object={:?}, handle={:?}", element_id, handle);
                            tracing::info!(
                                "Started resizing object {} with handle {:?}",
                                element_id,
                                handle
                            );
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
                            DocumentElement::Group(group) => {
                                objects.push((group.id, group.bounds.clone()));
                            }
                        }
                    }

                    // Convert to references for hit testing
                    let object_refs: Vec<(uuid::Uuid, &Rect)> =
                        objects.iter().map(|(id, bounds)| (*id, bounds)).collect();

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
                        tracing::info!("Hit test: selected object {}", clicked_id);

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
}
