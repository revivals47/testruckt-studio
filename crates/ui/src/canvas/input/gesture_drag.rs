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

use crate::app::AppState;
use crate::canvas::mouse::calculate_resize_bounds;
use crate::canvas::rendering::snap_rect_to_grid;
use crate::canvas::tools::{ShapeFactory, ToolMode};
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::{DrawingArea, GestureDrag};
use gtk4::gdk;
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
                "drag update [{:?}]: from ({:.0}, {:.0}) to ({:.0}, {:.0}), offset=({:.1}, {:.1})",
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
                "drag end: tool={:?}, offset=({:.1}, {:.1}), from ({:.0}, {:.0}) to ({:.0}, {:.0})",
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
                                        tracing::info!("Resized shape {} with handle {:?}", object_id, handle);
                                    }
                                    DocumentElement::Image(image) if image.id == object_id => {
                                        let mut new_bounds = calculate_resize_bounds(&image.bounds, handle, delta_x, delta_y);
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        image.bounds = new_bounds;
                                        tracing::info!("Resized image {} with handle {:?}", object_id, handle);
                                    }
                                    DocumentElement::Frame(frame) if frame.id == object_id => {
                                        let mut new_bounds = calculate_resize_bounds(&frame.bounds, handle, delta_x, delta_y);
                                        if snap_enabled {
                                            new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
                                        }
                                        frame.bounds = new_bounds;
                                        tracing::info!("Resized frame {} with handle {:?}", object_id, handle);
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
                            tracing::info!("Moved {} selected object(s)", selected_ids.len());
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
        drop(tool_state);

        *state.drag_box.borrow_mut() = None;
        drawing_area_end.queue_draw();
    });

    drawing_area.add_controller(drag_gesture);
}
