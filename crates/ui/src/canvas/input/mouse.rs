//! マウス動作追跡モジュール
//!
//! マウス位置をリアルタイムで追跡し、UI 要素（特にリサイズハンドル）の上にあるか
//! を検出してカーソルを適切に変更します。
//!
//! # 主な機能
//!
//! - **マウス座標追跡**: スクリーン座標をドキュメント座標に変換
//! - **リサイズハンドル検出**: 選択オブジェクトのハンドル範囲を調査
//! - **カーソル変更**: ハンドルタイプに応じてカーソルを更新
//!
//! # カーソルタイプ
//!
//! | ハンドル位置 | カーソル形状 | 説明 |
//! |-------------|-----------|------|
//! | 左上/右下コーナー | nwse-resize | 北西←→南東方向リサイズ |
//! | 右上/左下コーナー | nesw-resize | 北東←→南西方向リサイズ |
//! | 上/下エッジ | ns-resize | 南北方向リサイズ |
//! | 左/右エッジ | ew-resize | 東西方向リサイズ |
//! | その他 | default | デフォルトカーソル |
//!
//! # 座標変換
//!
//! マウス座標はスクリーン座標でイベントから渡されますが、以下の処理で
//! ドキュメント座標に変換されます：
//!
//! ```text
//! スクリーン座標
//!   ├─ ルーラーオフセット減算: screen_x - ruler_config.size
//!   ├─ パン値減算: screen_x - config.pan_x
//!   └─ ズーム値除算: screen_x / config.zoom
//!       └─ ドキュメント座標
//! ```
//!
//! # 使用例
//!
//! ```ignore
//! use crate::canvas::input::mouse;
//!
//! mouse::setup_mouse_tracking(drawing_area, render_state, app_state);
//! ```
//!
//! # パフォーマンス
//!
//! `motion` イベントは高頻度で発生（通常 60Hz 以上）するため、処理は最小限に
//! 留めて実装されています。リサイズハンドル検出のみで、他の処理は行いません。

use crate::app::AppState;
use crate::canvas::mouse::{test_resize_handle, CanvasMousePos, ResizeHandle};
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::{DrawingArea, EventControllerMotion};
use gtk4::gdk;

/// マウス動作追跡を初期化
///
/// # 引数
///
/// - `drawing_area`: GTK DrawingArea ウィジェット
/// - `render_state`: キャンバス描画状態
/// - `app_state`: アプリケーション全体の状態
pub fn setup_mouse_tracking(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    let motion = EventControllerMotion::new();
    let render_state_motion = render_state.clone();
    let drawing_area_cursor = drawing_area.clone();
    let app_state_motion = app_state.clone();

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

        let canvas_mouse_pos = CanvasMousePos { x: doc_x, y: doc_y };

        // Check if cursor is over a resize handle of selected objects
        let selected = state.selected_ids.borrow();
        let mut cursor_name = "default";

        if let Some(document) = app_state_motion.active_document() {
            if let Some(page) = document.pages.first() {
                for selected_id in selected.iter() {
                    // Find the selected element
                    for element in &page.elements {
                        let (elem_id, bounds) = match element {
                            testruct_core::document::DocumentElement::Shape(shape) => {
                                (shape.id, &shape.bounds)
                            }
                            testruct_core::document::DocumentElement::Text(text) => {
                                (text.id, &text.bounds)
                            }
                            testruct_core::document::DocumentElement::Image(image) => {
                                (image.id, &image.bounds)
                            }
                            testruct_core::document::DocumentElement::Frame(frame) => {
                                (frame.id, &frame.bounds)
                            }
                        };

                        if elem_id == *selected_id {
                            // Test for resize handle hit
                            if let Some(handle) = test_resize_handle(canvas_mouse_pos, bounds, 8.0) {
                                cursor_name = match handle {
                                    ResizeHandle::TopLeft | ResizeHandle::BottomRight => {
                                        "nwse-resize"
                                    }
                                    ResizeHandle::TopRight | ResizeHandle::BottomLeft => {
                                        "nesw-resize"
                                    }
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
}
