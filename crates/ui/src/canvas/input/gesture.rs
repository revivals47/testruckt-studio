//! ジェスチャー処理モジュール
//!
//! クリックおよびドラッグジェスチャーを処理し、オブジェクト選択、リサイズ、
//! 図形作成などの操作を実現します。
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
//!
//! # ドラッグジェスチャー (GestureDrag)
//!
//! ドラッグ操作により3つの処理が実行されます：
//!
//! ## 1. オブジェクト移動
//! Select ツール + 選択オブジェクトをドラッグ
//! - `delta_x`, `delta_y` でオブジェクト座標を更新
//! - グリッドスナップ対応
//!
//! ## 2. オブジェクトリサイズ
//! リサイズハンドルをドラッグ
//! - `calculate_resize_bounds()` で新しい寸法を計算
//! - ハンドルタイプ（TopLeft, Top, TopRight など）に基づいて計算
//! - グリッドスナップ対応
//!
//! ## 3. 図形作成
//! Rectangle、Circle、Line、Arrow、Text、Image ツール + ドラッグ
//! - `ShapeFactory` で新規要素を作成
//! - ドラッグ開始・終了座標で図形サイズを決定
//! - 作成後は自動的に Select ツールに切り替え
//!
//! # ドラッグ処理の流れ
//!
//! ```text
//! drag_begin
//!   └─ 開始座標を tool_state.drag_start に保存
//!
//! drag_update (繰り返し)
//!   ├─ オフセット計算
//!   ├─ drag_box を更新（プレビュー用）
//!   └─ キャンバス再描画
//!
//! drag_end
//!   ├─ 操作タイプ判定（リサイズ/移動/作成）
//!   ├─ ドキュメント更新
//!   ├─ グリッドスナップ適用
//!   └─ ドラッグ状態をクリア
//! ```
//!
//! # 使用例
//!
//! ```ignore
//! use crate::canvas::input::gesture;
//!
//! gesture::setup_gestures(drawing_area, render_state, app_state);
//! ```

use crate::app::AppState;
use crate::canvas::mouse::{test_resize_handle, CanvasMousePos, calculate_resize_bounds};
use crate::canvas::rendering::{snap_rect_to_grid};
use crate::canvas::selection::HitTest;
use crate::canvas::tools::{ShapeFactory, ToolMode};
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureClick, GestureDrag};
use gtk4::gdk;
use testruct_core::document::DocumentElement;
use testruct_core::layout::{Point, Rect, Size};

/// クリックおよびドラッグジェスチャーを設定
pub fn setup_gestures(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    setup_click_gesture(drawing_area, render_state, app_state);
    setup_drag_gesture(drawing_area, render_state, app_state);
}

fn setup_click_gesture(
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
            eprintln!("🖱️  Click: n_press={}, tool=Select", n_press);
            if n_press == 2 {
                eprintln!("🔍 Double-click detected at ({:.0}, {:.0})", x, y);
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
                                            "✅ Entered text editing mode for text element: {}",
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
                                            "🖼️  Image double-click detected for image: {}",
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
                                                        "📝 Image selected for block {}: {}",
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
                                                                        tracing::info!("✅ Updated image element with asset reference");
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
                                            tracing::warn!("⚠️  Window not available for image dialog");
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

                            tracing::info!(
                                "✅ Started resizing object {} with handle {:?}",
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
}

fn setup_drag_gesture(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(gdk::BUTTON_PRIMARY);
    drag_gesture.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let render_state_drag = render_state.clone();
    let drawing_area_drag = drawing_area.clone();

    drag_gesture.connect_drag_begin(move |_gesture, x, y| {
        let state = render_state_drag.clone();
        let tool_state = state.tool_state.borrow();
        let _current_tool = tool_state.current_tool;
        drop(tool_state);

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
            let current_tool = tool_state.current_tool;

            tracing::info!(
                "📍 drag update [{:?}]: from ({:.0}, {:.0}) to ({:.0}, {:.0}), offset=({:.1}, {:.1})",
                current_tool, start_x, start_y, current_x, current_y, offset_x, offset_y
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
    let app_state_drag_end = app_state.clone();

    drag_gesture.connect_drag_end(move |_gesture, offset_x, offset_y| {
        let state = render_state_end.clone();

        // Extract all values we need from tool_state, then drop the borrow immediately
        let (start_x, start_y, current_tool, is_resizing, resizing_object_id, resize_handle, resize_original_bounds) = {
            let tool_state = state.tool_state.borrow();
            if let Some((start_x, start_y)) = tool_state.drag_start {
                (
                    start_x,
                    start_y,
                    tool_state.current_tool,
                    tool_state.resizing_object_id.is_some(),
                    tool_state.resizing_object_id,
                    tool_state.resize_handle,
                    tool_state.resize_original_bounds,
                )
            } else {
                return;
            }
        }; // tool_state borrow is dropped here

        {
            let current_x = start_x + offset_x;
            let current_y = start_y + offset_y;

            eprintln!(
                "🎯 drag end: tool={:?}, offset=({:.1}, {:.1}), from ({:.0}, {:.0}) to ({:.0}, {:.0})",
                current_tool, offset_x, offset_y, start_x, start_y, current_x, current_y
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

                    // Apply resize directly to the document
                    app_state_drag_end.with_mutable_active_document(|document| {
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
                        }
                    });
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

                    // Move each selected object directly to the document
                    app_state_drag_end.with_mutable_active_document(|document| {
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
                            tracing::info!("✅ Moved {} selected object(s)", selected_ids.len());
                        }
                    });
                }
            } else if current_tool != ToolMode::Select && (offset_x.abs() > 5.0 || offset_y.abs() > 5.0) {
                // Shape/Text creation based on tool
                // Convert screen coordinates to document coordinates
                let config = state.config.borrow();
                let pan_x = config.pan_x;
                let pan_y = config.pan_y;
                let zoom = config.zoom;
                drop(config);

                // Convert to document coordinates (accounting for pan and zoom)
                let doc_start_x = (start_x / zoom) - pan_x;
                let doc_start_y = (start_y / zoom) - pan_y;
                let doc_current_x = (current_x / zoom) - pan_x;
                let doc_current_y = (current_y / zoom) - pan_y;

                tracing::info!("📐 Creating {} element with drag offset ({:.1}, {:.1})", current_tool.name(), offset_x, offset_y);

                let element = match current_tool {
                    ToolMode::Rectangle => ShapeFactory::create_rectangle(
                        doc_start_x.min(doc_current_x),
                        doc_start_y.min(doc_current_y),
                        (doc_start_x - doc_current_x).abs(),
                        (doc_start_y - doc_current_y).abs(),
                    ),
                    ToolMode::Circle => ShapeFactory::create_circle(
                        doc_start_x.min(doc_current_x),
                        doc_start_y.min(doc_current_y),
                        (doc_start_x - doc_current_x).abs(),
                        (doc_start_y - doc_current_y).abs(),
                    ),
                    ToolMode::Line => ShapeFactory::create_line(
                        doc_start_x,
                        doc_start_y,
                        doc_current_x,
                        doc_current_y,
                    ),
                    ToolMode::Arrow => ShapeFactory::create_arrow(
                        doc_start_x,
                        doc_start_y,
                        doc_current_x,
                        doc_current_y,
                    ),
                    ToolMode::Image => ShapeFactory::create_image(
                        doc_start_x.min(doc_current_x),
                        doc_start_y.min(doc_current_y),
                        (doc_start_x - doc_current_x).abs(),
                        (doc_start_y - doc_current_y).abs(),
                    ),
                    ToolMode::Text => {
                        tracing::info!("📝 Creating text box at ({:.0}, {:.0}) size ({:.0}x{:.0}) (document coords)",
                            doc_start_x, doc_start_y, (doc_start_x - doc_current_x).abs(), (doc_start_y - doc_current_y).abs());
                        ShapeFactory::create_text(
                            doc_start_x,
                            doc_start_y,
                            (doc_start_x - doc_current_x).abs(),
                            (doc_start_y - doc_current_y).abs(),
                            "テキストを入力".to_string(),
                        )
                    },
                    _ => {
                        tracing::warn!("⚠️  Tool {:?} is not supported for creation", current_tool);
                        return;
                    }
                };

                // Add element to document
                if let Err(e) = app_state_drag_end.add_element_to_active_page(element) {
                    tracing::warn!("❌ Failed to add element: {}", e);
                } else {
                    tracing::info!("✅ {} element added to document", current_tool.name());

                    // Auto-switch back to Select tool after creating an element
                    let mut tool_state_auto = state.tool_state.borrow_mut();
                    tool_state_auto.current_tool = ToolMode::Select;
                    tracing::info!("🔄 Tool auto-switched to Select");
                    drop(tool_state_auto);

                    // Trigger redraw to update UI
                    drawing_area_end.queue_draw();
                }
            } else {
                tracing::debug!("⚠️  Drag ignored: tool={:?}, offset=({:.1}, {:.1}), threshold=5.0px",
                    current_tool, offset_x, offset_y);
            }
        } // End of scope block

        // Clear drag state (now safe - all borrows from above are dropped)
        let mut tool_state = state.tool_state.borrow_mut();
        tool_state.drag_start = None;
        tool_state.resizing_object_id = None;
        tool_state.resize_handle = None;
        tool_state.resize_original_bounds = None;
        drop(tool_state);

        *state.drag_box.borrow_mut() = None;
        drawing_area_end.queue_draw();
    });

    drawing_area.add_controller(drag_gesture);
}
