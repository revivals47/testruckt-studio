//! ジェスチャー処理モジュール
//!
//! クリックおよびドラッグジェスチャーを処理し、オブジェクト選択、リサイズ、
//! 図形作成などの操作を実現します。
//!
//! このモジュールは以下のサブモジュールで構成されています：
//!
//! - [`gesture_click`]: クリック・ダブルクリックによる選択とテキスト編集
//! - [`gesture_drag`]: ドラッグによるオブジェクト移動・リサイズ・図形作成
//!
//! # モジュール構成
//!
//! | モジュール | 責務 |
//! |-----------|------|
//! | gesture.rs | モジュール統合、公開API |
//! | gesture_click.rs | クリック、ダブルクリック、選択処理 |
//! | gesture_drag.rs | ドラッグ、移動、リサイズ、図形作成 |
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
use crate::canvas::input::{gesture_click, gesture_drag, ime::ImeManager};
use crate::canvas::CanvasRenderState;
use gtk4::DrawingArea;
use std::cell::RefCell;
use std::rc::Rc;

/// クリックおよびドラッグジェスチャーを設定
///
/// この関数は以下のジェスチャーハンドラーを初期化します：
///
/// - クリックジェスチャー: オブジェクト選択、テキスト編集、画像選択
/// - ドラッグジェスチャー: オブジェクト移動、リサイズ、図形作成
///
/// # 引数
///
/// - `drawing_area`: GTK DrawingArea ウィジェット
/// - `render_state`: キャンバス描画状態
/// - `app_state`: アプリケーション全体の状態
/// - `ime_manager`: Shared IME manager for focus management
///
/// # 例
///
/// ```ignore
/// use crate::canvas::input::gesture;
///
/// gesture::setup_gestures(&drawing_area, &render_state, &app_state, ime_manager);
/// ```
pub fn setup_gestures(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
    ime_manager: Rc<RefCell<ImeManager>>,
) {
    gesture_click::setup_click_gesture(drawing_area, render_state, app_state, ime_manager);
    gesture_drag::setup_drag_gesture(drawing_area, render_state, app_state);
}
