//! キャンバス入力処理モジュール
//!
//! キーボード、マウス、ジェスチャーのイベント処理を統一的に管理します。
//!
//! このモジュールは3つの専門化されたサブモジュールで構成されています：
//!
//! - [`keyboard`]: キーボード入力、テキスト編集、ショートカット処理
//! - [`gesture`]: クリック・ドラッグジェスチャー、オブジェクト操作
//! - [`mouse`]: マウス動作追跡、カーソル管理
//!
//! # モジュール概要
//!
//! | モジュール | ファイル | 行数 | 責務 |
//! |-----------|---------|------|------|
//! | keyboard | keyboard.rs | 606 | テキスト編集、ショートカット、オブジェクト移動 |
//! | gesture | gesture.rs | 622 | 選択、リサイズ、図形作成 |
//! | mouse | mouse.rs | 88 | カーソル管理 |
//! | 統合 | input.rs | 22 | イベントハンドラー初期化 |
//!
//! # 使用例
//!
//! ```ignore
//! use crate::canvas::input::wire_pointer_events;
//!
//! wire_pointer_events(drawing_area, render_state, app_state);
//! ```
//!
//! # アーキテクチャ
//!
//! ## 単一責任原則
//!
//! 各モジュールは特定のイベント種別と責務に焦点を当てます：
//!
//! ```text
//! ユーザー入力
//!   ├─ キーボード入力
//!   │   └─ keyboard モジュール
//!   │       ├─ テキスト編集
//!   │       ├─ ショートカット実行
//!   │       └─ オブジェクト移動
//!   │
//!   ├─ マウスジェスチャー
//!   │   ├─ クリック
//!   │   │   └─ gesture モジュール（click_gesture）
//!   │   │       ├─ オブジェクト選択
//!   │   │       └─ リサイズハンドル検出
//!   │   │
//!   │   └─ ドラッグ
//!   │       └─ gesture モジュール（drag_gesture）
//!   │           ├─ オブジェクト移動
//!   │           ├─ オブジェクトリサイズ
//!   │           └─ 図形作成
//!   │
//!   └─ マウス動作
//!       └─ mouse モジュール
//!           └─ カーソル更新
//! ```
//!
//! # 拡張ガイド
//!
//! 新しい入力処理を追加する場合：
//!
//! 1. **キーボードショートカット追加**: `keyboard::setup_keyboard_events` に条件を追加
//! 2. **ジェスチャー処理追加**: `gesture::setup_click_gesture` または `gesture::setup_drag_gesture` を修正
//! 3. **カーソル表示カスタマイズ**: `mouse::setup_mouse_tracking` でカーソル名を追加
//!
//! 詳細は各モジュールドキュメントを参照してください。

mod keyboard;
mod mouse;
mod gesture;

pub use self::keyboard::move_selected_objects;

use crate::app::AppState;
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::DrawingArea;

/// キャンバスのポインタイベント処理を初期化
///
/// キーボード、マウス、ジェスチャーのイベントハンドラーをセットアップします。
///
/// # 引数
///
/// - `drawing_area`: GTK DrawingArea ウィジェット
/// - `render_state`: キャンバス描画状態
/// - `app_state`: アプリケーション全体の状態
///
/// # 初期化内容
///
/// - キーボードイベント（EventControllerKey）
/// - マウス動作追跡（EventControllerMotion）
/// - クリックジェスチャー（GestureClick）
/// - ドラッグジェスチャー（GestureDrag）
pub fn wire_pointer_events(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    drawing_area.set_focusable(true);
    keyboard::setup_keyboard_events(drawing_area, render_state, app_state);
    mouse::setup_mouse_tracking(drawing_area, render_state, app_state);
    gesture::setup_gestures(drawing_area, render_state, app_state);
}
