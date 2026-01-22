//! ドラッグジェスチャー処理モジュール
//!
//! ドラッグ操作によるオブジェクト移動、リサイズ、図形作成を処理します。
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

use super::coordinates::widget_to_document;
use crate::app::AppState;
use crate::canvas::mouse::calculate_resize_bounds;
use crate::canvas::rendering::snap_rect_to_grid;
use crate::canvas::selection::HitTest;
use crate::canvas::snapping::{ObjectAlignmentPoints, SmartGuideEngine};
use crate::canvas::tools::{ShapeFactory, ToolMode};
use crate::canvas::CanvasRenderState;
use gtk4::gdk;
use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureDrag};
use testruct_core::document::DocumentElement;
use testruct_core::layout::{Point, Rect, Size};

/// ドラッグジェスチャーを設定
pub fn setup_drag_gesture(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(gdk::BUTTON_PRIMARY);
    drag_gesture.set_propagation_phase(gtk4::PropagationPhase::Capture);

    let render_state_drag = render_state.clone();
    let drawing_area_drag = drawing_area.clone();
    let app_state_drag_begin = app_state.clone();

    drag_gesture.connect_drag_begin(move |_gesture, x, y| {
        let state = render_state_drag.clone();
        let tool_state = state.tool_state.borrow();
        let current_tool = tool_state.current_tool;
        let is_resizing = tool_state.resizing_object_id.is_some();
        drop(tool_state);

        // Store drag start position as RAW window coordinates
        // offset_x and offset_y from drag_update are relative to this position
        // So we keep them in the same coordinate system (window coordinates)
        // Conversion to canvas coordinates happens only in drag_end
        let mut tool_state = state.tool_state.borrow_mut();
        tool_state.drag_start = Some((x, y));
        tool_state.last_drag_offset = Some((0.0, 0.0)); // Reset offset tracking
        tool_state.marquee_selecting = false; // Reset marquee state

        // Check if we should start marquee selection
        // Condition: Select mode + not resizing + no object under cursor
        if current_tool == ToolMode::Select && !is_resizing {
            // Convert to document coordinates
            let doc_coords = widget_to_document(x, y, &state);

            // Check if clicking on an object
            let clicked_on_object = app_state_drag_begin.with_active_document(|doc| {
                if let Some(page) = doc.pages.first() {
                    let objects: Vec<(uuid::Uuid, &Rect)> = page.elements.iter()
                        .map(|e| match e {
                            DocumentElement::Shape(s) => (s.id, &s.bounds),
                            DocumentElement::Text(t) => (t.id, &t.bounds),
                            DocumentElement::Image(i) => (i.id, &i.bounds),
                            DocumentElement::Frame(f) => (f.id, &f.bounds),
                            DocumentElement::Group(g) => (g.id, &g.bounds),
                        })
                        .collect();
                    HitTest::hit_test(&objects, doc_coords.x, doc_coords.y).is_some()
                } else {
                    false
                }
            }).unwrap_or(false);

            if !clicked_on_object {
                // Start marquee selection
                tool_state.marquee_selecting = true;
                eprintln!("📦 Starting marquee selection at ({:.1}, {:.1})", x, y);
            }
        }
        drop(tool_state);
    });

    let render_state_update = render_state.clone();
    let app_state_drag_update = app_state.clone();
    drag_gesture.connect_drag_update(move |_gesture, offset_x, offset_y| {
        let state = render_state_update.clone();

        // Extract all values we need from tool_state first, then drop the borrow
        let (start_x, start_y, current_tool, is_resizing, resizing_object_id, resize_handle, marquee_selecting) = {
            let tool_state = state.tool_state.borrow();
            if let Some((start_x, start_y)) = tool_state.drag_start {
                (
                    start_x,
                    start_y,
                    tool_state.current_tool,
                    tool_state.resizing_object_id.is_some(),
                    tool_state.resizing_object_id,
                    tool_state.resize_handle,
                    tool_state.marquee_selecting,
                )
            } else {
                return;
            }
        }; // tool_state borrow is dropped here

        let current_x = start_x + offset_x;
        let current_y = start_y + offset_y;

        eprintln!("🔵 Drag Update:");
        eprintln!("  Start: ({:.1}, {:.1})", start_x, start_y);
        eprintln!("  Offset: ({:.1}, {:.1})", offset_x, offset_y);
        eprintln!("  Current: ({:.1}, {:.1})", current_x, current_y);

        tracing::info!(
            "drag update [{:?}]: from ({:.0}, {:.0}) to ({:.0}, {:.0}), offset=({:.1}, {:.1})",
            current_tool,
            start_x,
            start_y,
            current_x,
            current_y,
            offset_x,
            offset_y
        );

        if is_resizing {
            // REAL-TIME RESIZE with delta calculation
            // Calculate only the delta from last frame to avoid cumulative growth
            if let (Some(object_id), Some(handle)) = (resizing_object_id, resize_handle) {
                let (last_offset_x, last_offset_y) = {
                    let tool_state = state.tool_state.borrow();
                    tool_state.last_drag_offset.unwrap_or((0.0, 0.0))
                };

                // Delta is the change from last frame
                let delta_x_pixels = offset_x - last_offset_x;
                let delta_y_pixels = offset_y - last_offset_y;

                let config = state.config.borrow();
                let delta_x = delta_x_pixels / config.zoom;
                let delta_y = delta_y_pixels / config.zoom;
                let snap_enabled = config.snap_to_grid;
                let grid_spacing = config.grid_spacing();
                drop(config);

                // Update document with delta
                let _ = app_state_drag_update.with_mutable_active_document(|document| {
                    if let Some(page) = document.pages.first_mut() {
                        for element in page.elements.iter_mut() {
                            match element {
                                DocumentElement::Text(text) if text.id == object_id => {
                                    let mut new_bounds = calculate_resize_bounds(
                                        &text.bounds,
                                        handle,
                                        delta_x,
                                        delta_y,
                                    );
                                    if snap_enabled {
                                        new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                    }
                                    text.bounds = new_bounds;
                                    return true;
                                }
                                DocumentElement::Shape(shape) if shape.id == object_id => {
                                    let mut new_bounds = calculate_resize_bounds(
                                        &shape.bounds,
                                        handle,
                                        delta_x,
                                        delta_y,
                                    );
                                    if snap_enabled {
                                        new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                    }
                                    shape.bounds = new_bounds;
                                    return true;
                                }
                                DocumentElement::Image(image) if image.id == object_id => {
                                    let mut new_bounds = calculate_resize_bounds(
                                        &image.bounds,
                                        handle,
                                        delta_x,
                                        delta_y,
                                    );
                                    if snap_enabled {
                                        new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                    }
                                    image.bounds = new_bounds;
                                    return true;
                                }
                                _ => {}
                            }
                        }
                    }
                    false
                });

                // Update last offset for next frame
                let mut tool_state = state.tool_state.borrow_mut();
                tool_state.last_drag_offset = Some((offset_x, offset_y));
                drop(tool_state);
            }

            // Don't show drag_box preview during resize
            *state.drag_box.borrow_mut() = None;
        } else if current_tool == ToolMode::Select {
            // SMART GUIDES: Calculate and show smart guides during Select tool dragging
            let selected = state.selected_ids.borrow();
            if !selected.is_empty() && (offset_x.abs() > 2.0 || offset_y.abs() > 2.0) {
                let selected_ids: Vec<uuid::Uuid> = selected.clone();
                drop(selected);

                let config = state.config.borrow();
                let delta_x = offset_x / config.zoom;
                let delta_y = offset_y / config.zoom;
                drop(config);

                // Get bounds of first selected element for smart guide calculation
                if let Some(first_id) = selected_ids.first() {
                    let first_id = *first_id;
                    let snap_data: Option<(Rect, Vec<ObjectAlignmentPoints>)> = app_state_drag_update.with_active_document(|doc| {
                        let page = doc.pages.first()?;
                        let mut dragging_bounds: Option<Rect> = None;
                        let mut others: Vec<ObjectAlignmentPoints> = Vec::new();

                        for element in &page.elements {
                            let (id, bounds) = match element {
                                DocumentElement::Shape(s) => (s.id, &s.bounds),
                                DocumentElement::Text(t) => (t.id, &t.bounds),
                                DocumentElement::Image(i) => (i.id, &i.bounds),
                                DocumentElement::Frame(f) => (f.id, &f.bounds),
                                DocumentElement::Group(g) => (g.id, &g.bounds),
                            };

                            if id == first_id {
                                // Calculate the preview position
                                let preview_bounds = Rect {
                                    origin: Point {
                                        x: bounds.origin.x + delta_x as f32,
                                        y: bounds.origin.y + delta_y as f32,
                                    },
                                    size: bounds.size.clone(),
                                };
                                dragging_bounds = Some(preview_bounds);
                            } else if !selected_ids.contains(&id) {
                                others.push(ObjectAlignmentPoints::from_rect(id, bounds));
                            }
                        }

                        Some((dragging_bounds?, others))
                    }).flatten();

                    if let Some((bounds, others)) = snap_data {
                        // Calculate smart guides
                        let smart_engine = SmartGuideEngine::default();
                        let result = smart_engine.calculate_snap(
                            &bounds,
                            &others,
                            1200.0, // canvas width
                            1600.0, // canvas height
                        );

                        // Update snap lines for rendering
                        *state.snap_lines.borrow_mut() = result.snap_lines;
                    }
                }
            } else {
                drop(selected);
                // Clear snap lines if not dragging
                state.snap_lines.borrow_mut().clear();
            }

            // Show marquee selection rectangle if marquee_selecting is active
            if marquee_selecting {
                // Convert widget coordinates to document coordinates
                let start_doc = widget_to_document(start_x, start_y, &state);
                let current_doc = widget_to_document(current_x, current_y, &state);

                let min_x = start_doc.x.min(current_doc.x);
                let min_y = start_doc.y.min(current_doc.y);
                let max_x = start_doc.x.max(current_doc.x);
                let max_y = start_doc.y.max(current_doc.y);

                let marquee_rect = Rect {
                    origin: Point {
                        x: min_x as f32,
                        y: min_y as f32,
                    },
                    size: Size {
                        width: (max_x - min_x) as f32,
                        height: (max_y - min_y) as f32,
                    },
                };

                *state.drag_box.borrow_mut() = Some(marquee_rect);
            } else {
                *state.drag_box.borrow_mut() = None;
            }
        } else {
            // SHAPE CREATION: Show drag_box preview for new shapes
            // Convert widget coordinates to document coordinates using unified helper
            // Note: GTK4 GestureDrag provides widget-relative coordinates
            let start_doc = widget_to_document(start_x, start_y, &state);
            let current_doc = widget_to_document(current_x, current_y, &state);

            let doc_x1 = start_doc.x;
            let doc_y1 = start_doc.y;
            let doc_x2 = current_doc.x;
            let doc_y2 = current_doc.y;

            // Update drag box for preview rendering
            let (x1, y1, x2, y2) = (doc_x1, doc_y1, doc_x2, doc_y2);
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
            // Clear snap lines for shape creation
            state.snap_lines.borrow_mut().clear();
        }

        drawing_area_drag.queue_draw();
    });

    let render_state_end = render_state.clone();
    let drawing_area_end = drawing_area.clone();
    let app_state_drag_end = app_state.clone();

    drag_gesture.connect_drag_end(move |_gesture, offset_x, offset_y| {
        let state = render_state_end.clone();

        // Extract all values we need from tool_state, then drop the borrow immediately
        let (start_x, start_y, current_tool, is_resizing, resizing_object_id, resize_handle, resize_original_bounds, resize_element_bounds, marquee_selecting) = {
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
                    tool_state.resize_element_bounds.clone(),
                    tool_state.marquee_selecting,
                )
            } else {
                return;
            }
        }; // tool_state borrow is dropped here

        {
            let current_x = start_x + offset_x;
            let current_y = start_y + offset_y;

            eprintln!(
                "drag end: tool={:?}, offset=({:.1}, {:.1}), from ({:.0}, {:.0}) to ({:.0}, {:.0})",
                current_tool, offset_x, offset_y, start_x, start_y, current_x, current_y
            );

            if is_resizing && (offset_x.abs() > 2.0 || offset_y.abs() > 2.0) {
                // Apply resize
                eprintln!("🔄 RESIZE DETECTED: is_resizing={}, resizing_object_id={:?}, resize_handle={:?}",
                    is_resizing, resizing_object_id, resize_handle);

                if let (Some(object_id), Some(handle), Some(_mouse_pos)) = (resizing_object_id, resize_handle, resize_original_bounds) {
                    // Calculate document-space delta
                    let config = state.config.borrow();
                    let delta_x = offset_x / config.zoom;
                    let delta_y = offset_y / config.zoom;
                    let snap_enabled = config.snap_to_grid;
                    let grid_spacing = config.grid_spacing();
                    drop(config);

                    eprintln!("✏️ Applying resize: delta=({:.2}, {:.2}), handle={:?}", delta_x, delta_y, handle);

                    // Apply resize directly to the document
                    let resize_applied = app_state_drag_end.with_mutable_active_document(|document| {
                        if let Some(page) = document.pages.first_mut() {
                            for element in page.elements.iter_mut() {
                                match element {
                                    DocumentElement::Text(text) if text.id == object_id => {
                                        let old_bounds = text.bounds.clone();
                                        let mut new_bounds = calculate_resize_bounds(&text.bounds, handle, delta_x, delta_y);
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        text.bounds = new_bounds;
                                        eprintln!("✅ Resized TEXT {} with handle {:?}: {:?} -> {:?}",
                                            object_id, handle, old_bounds, text.bounds);
                                        return true;
                                    }
                                    DocumentElement::Shape(shape) if shape.id == object_id => {
                                        let old_bounds = shape.bounds.clone();
                                        let mut new_bounds = calculate_resize_bounds(&shape.bounds, handle, delta_x, delta_y);
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        shape.bounds = new_bounds;
                                        eprintln!("✅ Resized SHAPE {} with handle {:?}: {:?} -> {:?}",
                                            object_id, handle, old_bounds, shape.bounds);
                                        return true;
                                    }
                                    DocumentElement::Image(image) if image.id == object_id => {
                                        let old_bounds = image.bounds.clone();
                                        let mut new_bounds = calculate_resize_bounds(&image.bounds, handle, delta_x, delta_y);
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        image.bounds = new_bounds;
                                        eprintln!("✅ Resized IMAGE {} with handle {:?}: {:?} -> {:?}",
                                            object_id, handle, old_bounds, image.bounds);
                                        return true;
                                    }
                                    DocumentElement::Frame(frame) if frame.id == object_id => {
                                        let old_bounds = frame.bounds.clone();
                                        let mut new_bounds = calculate_resize_bounds(&frame.bounds, handle, delta_x, delta_y);
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        frame.bounds = new_bounds;
                                        eprintln!("✅ Resized FRAME {} with handle {:?}: {:?} -> {:?}",
                                            object_id, handle, old_bounds, frame.bounds);
                                        return true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        false
                    });

                    if resize_applied.unwrap_or(false) {
                        // Create undo command for resize operation
                        if let Some(old_bounds) = resize_element_bounds {
                            // Get the new bounds from the document
                            let new_bounds: Option<Rect> = app_state_drag_end.with_active_document(|doc| {
                                if let Some(page) = doc.pages.first() {
                                    for element in &page.elements {
                                        let (id, bounds) = match element {
                                            DocumentElement::Shape(s) => (s.id, &s.bounds),
                                            DocumentElement::Text(t) => (t.id, &t.bounds),
                                            DocumentElement::Image(i) => (i.id, &i.bounds),
                                            DocumentElement::Frame(f) => (f.id, &f.bounds),
                                            DocumentElement::Group(g) => (g.id, &g.bounds),
                                        };
                                        if id == object_id {
                                            return Some(bounds.clone());
                                        }
                                    }
                                }
                                None
                            }).flatten();

                            if let Some(new_bounds) = new_bounds {
                                let page_index = app_state_drag_end.active_page_index();
                                let command = crate::undo_redo::AppResizeCommand::new(
                                    app_state_drag_end.clone(),
                                    object_id,
                                    page_index,
                                    old_bounds,
                                    new_bounds,
                                );
                                app_state_drag_end.push_command(Box::new(command));
                                tracing::info!("✅ Resize undo command created for object {}", object_id);
                            }
                        }

                        // Mark document as modified after resize
                        app_state_drag_end.mark_as_modified();
                    } else {
                        eprintln!("❌ WARNING: Resize was not applied to document!");
                    }
                } else {
                    eprintln!("❌ ERROR: Missing resize state - object_id={:?}, handle={:?}, bounds={:?}",
                        resizing_object_id, resize_handle, resize_original_bounds);
                }
            } else if current_tool == ToolMode::Select && marquee_selecting && (offset_x.abs() > 5.0 || offset_y.abs() > 5.0) {
                // MARQUEE SELECTION: Select all objects within the marquee rectangle
                let start_doc = widget_to_document(start_x, start_y, &state);
                let current_doc = widget_to_document(current_x, current_y, &state);

                // Find all objects in the marquee selection area
                let selected_by_marquee: Vec<uuid::Uuid> = app_state_drag_end.with_active_document(|doc| {
                    if let Some(page) = doc.pages.first() {
                        let objects: Vec<(uuid::Uuid, &Rect)> = page.elements.iter()
                            .map(|e| match e {
                                DocumentElement::Shape(s) => (s.id, &s.bounds),
                                DocumentElement::Text(t) => (t.id, &t.bounds),
                                DocumentElement::Image(i) => (i.id, &i.bounds),
                                DocumentElement::Frame(f) => (f.id, &f.bounds),
                                DocumentElement::Group(g) => (g.id, &g.bounds),
                            })
                            .collect();
                        HitTest::hit_test_rect(&objects, start_doc.x, start_doc.y, current_doc.x, current_doc.y)
                    } else {
                        Vec::new()
                    }
                }).unwrap_or_default();

                // Update selection with marquee-selected objects
                if !selected_by_marquee.is_empty() {
                    let mut selected = state.selected_ids.borrow_mut();
                    selected.clear();
                    selected.extend(selected_by_marquee.iter().copied());
                    drop(selected);
                    eprintln!("📦 Marquee selection complete: {} object(s) selected", selected_by_marquee.len());
                    tracing::info!("Marquee selected {} object(s)", selected_by_marquee.len());
                } else {
                    // Clear selection if nothing was selected
                    let mut selected = state.selected_ids.borrow_mut();
                    selected.clear();
                    drop(selected);
                    eprintln!("📦 Marquee selection complete: no objects in selection area");
                }
            } else if current_tool == ToolMode::Select && !marquee_selecting && (offset_x.abs() > 5.0 || offset_y.abs() > 5.0) {
                // Move selected objects with Undo support
                let selected = state.selected_ids.borrow();
                if !selected.is_empty() {
                    // Transform screen offset to document offset
                    let config = state.config.borrow();
                    let delta_x = (offset_x / config.zoom) as f32;
                    let delta_y = (offset_y / config.zoom) as f32;
                    drop(config);

                    let selected_ids: Vec<uuid::Uuid> = selected.clone();
                    drop(selected);

                    // Create and execute move command with undo support
                    let page_index = app_state_drag_end.active_page_index();
                    let command = crate::undo_redo::AppMoveCommand::new(
                        app_state_drag_end.clone(),
                        selected_ids.clone(),
                        page_index,
                        delta_x,
                        delta_y,
                    );

                    // Push command (this executes the move and adds to undo stack)
                    app_state_drag_end.push_command(Box::new(command));
                    app_state_drag_end.mark_as_modified();

                    tracing::info!("Moved {} selected object(s) (with undo support)", selected_ids.len());
                }
            } else if current_tool != ToolMode::Select && (offset_x.abs() > 5.0 || offset_y.abs() > 5.0) {
                // Shape/Text creation based on tool
                // Convert widget coordinates to document coordinates using unified helper
                // Note: GTK4 GestureDrag provides widget-relative coordinates
                let start_doc = widget_to_document(start_x, start_y, &state);
                let current_doc = widget_to_document(current_x, current_y, &state);

                let doc_start_x = start_doc.x;
                let doc_start_y = start_doc.y;
                let doc_current_x = current_doc.x;
                let doc_current_y = current_doc.y;

                eprintln!("📐 Shape creation coordinate transformation:");
                eprintln!("  Widget: start=({:.1}, {:.1}), current=({:.1}, {:.1})", start_x, start_y, current_x, current_y);
                eprintln!("  Document: start=({:.2}, {:.2}), current=({:.2}, {:.2})", doc_start_x, doc_start_y, doc_current_x, doc_current_y);

                tracing::info!("Creating {} element with drag offset ({:.1}, {:.1})", current_tool.name(), offset_x, offset_y);

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
                        tracing::info!("Creating text box at ({:.0}, {:.0}) size ({:.0}x{:.0}) (document coords)",
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
                        tracing::warn!("Tool {:?} is not supported for creation", current_tool);
                        return;
                    }
                };

                // Add element to document
                if let Err(e) = app_state_drag_end.add_element_to_active_page(element) {
                    tracing::warn!("Failed to add element: {}", e);
                } else {
                    tracing::info!("{} element added to document", current_tool.name());

                    // Mark document as modified after element creation
                    app_state_drag_end.mark_as_modified();

                    // Auto-switch back to Select tool after creating an element
                    let mut tool_state_auto = state.tool_state.borrow_mut();
                    tool_state_auto.current_tool = ToolMode::Select;
                    tracing::info!("Tool auto-switched to Select");
                    drop(tool_state_auto);

                    // Trigger redraw to update UI
                    drawing_area_end.queue_draw();
                }
            } else {
                tracing::debug!("Drag ignored: tool={:?}, offset=({:.1}, {:.1}), threshold=5.0px",
                    current_tool, offset_x, offset_y);
            }
        } // End of scope block

        // Clear drag state (now safe - all borrows from above are dropped)
        let mut tool_state = state.tool_state.borrow_mut();
        tool_state.drag_start = None;
        tool_state.resizing_object_id = None;
        tool_state.resize_handle = None;
        tool_state.resize_original_bounds = None;
        tool_state.resize_element_bounds = None;
        tool_state.marquee_selecting = false;
        drop(tool_state);

        *state.drag_box.borrow_mut() = None;
        // Clear smart guide snap lines
        state.snap_lines.borrow_mut().clear();
        drawing_area_end.queue_draw();
    });

    drawing_area.add_controller(drag_gesture);
}
