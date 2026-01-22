//! 座標変換ヘルパーモジュール
//!
//! ウィジェット座標とドキュメント座標の変換を統一的に行います。
//!
//! # 座標系
//!
//! ```text
//! ウィジェット座標 (Widget Coordinates)
//!   └─ GTK4イベントから取得される座標
//!   └─ DrawingAreaの左上が原点(0, 0)
//!
//! キャンバス座標 (Canvas Coordinates)
//!   └─ ルーラーとパンを考慮した座標
//!   └─ キャンバス表示領域の左上が原点
//!
//! ドキュメント座標 (Document Coordinates)
//!   └─ ズームを考慮した座標
//!   └─ ドキュメントの左上が原点(0, 0)
//!   └─ 図形の配置に使用
//! ```
//!
//! # 変換式
//!
//! ```text
//! Widget → Document:
//!   canvas_x = widget_x - ruler_size - pan_x
//!   canvas_y = widget_y - ruler_size - pan_y
//!   doc_x = canvas_x / zoom
//!   doc_y = canvas_y / zoom
//! ```

use crate::canvas::CanvasRenderState;

/// ウィジェット座標からドキュメント座標への変換結果
#[derive(Debug, Clone, Copy)]
pub struct DocumentCoords {
    pub x: f64,
    pub y: f64,
}

impl DocumentCoords {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// ウィジェット座標をドキュメント座標に変換
///
/// GTK4のジェスチャーイベントから取得した座標を、
/// ドキュメント内の実際の位置に変換します。
///
/// # 引数
///
/// * `widget_x` - ウィジェット相対X座標
/// * `widget_y` - ウィジェット相対Y座標
/// * `render_state` - キャンバスの描画状態
///
/// # 戻り値
///
/// ドキュメント座標（図形の配置に使用する座標）
pub fn widget_to_document(
    widget_x: f64,
    widget_y: f64,
    render_state: &CanvasRenderState,
) -> DocumentCoords {
    let config = render_state.config.borrow();
    let ruler_config = render_state.ruler_config.borrow();

    let ruler_size = ruler_config.size;
    let pan_x = config.pan_x;
    let pan_y = config.pan_y;
    let zoom = config.zoom;

    // Step 1: ルーラーとパンを引く（キャンバス座標へ）
    let canvas_x = widget_x - ruler_size - pan_x;
    let canvas_y = widget_y - ruler_size - pan_y;

    // Step 2: ズームで割る（ドキュメント座標へ）
    let doc_x = canvas_x / zoom;
    let doc_y = canvas_y / zoom;

    DocumentCoords::new(doc_x, doc_y)
}

/// ウィジェット座標をドキュメント座標に変換（デバッグ出力付き）
///
/// 座標変換の各ステップをeprintln!で出力します。
/// デバッグ時にのみ使用してください。
pub fn widget_to_document_debug(
    widget_x: f64,
    widget_y: f64,
    render_state: &CanvasRenderState,
    label: &str,
) -> DocumentCoords {
    let config = render_state.config.borrow();
    let ruler_config = render_state.ruler_config.borrow();

    let ruler_size = ruler_config.size;
    let pan_x = config.pan_x;
    let pan_y = config.pan_y;
    let zoom = config.zoom;

    eprintln!("\n=== {} Coordinate Transform ===", label);
    eprintln!("Widget: ({:.1}, {:.1})", widget_x, widget_y);
    eprintln!("Config: ruler={:.0}, pan=({:.1}, {:.1}), zoom={:.2}",
              ruler_size, pan_x, pan_y, zoom);

    // Step 1: ルーラーとパンを引く
    let canvas_x = widget_x - ruler_size - pan_x;
    let canvas_y = widget_y - ruler_size - pan_y;
    eprintln!("Canvas (after ruler/pan): ({:.2}, {:.2})", canvas_x, canvas_y);

    // Step 2: ズームで割る
    let doc_x = canvas_x / zoom;
    let doc_y = canvas_y / zoom;
    eprintln!("Document (after zoom): ({:.2}, {:.2})", doc_x, doc_y);
    eprintln!("=== End Transform ===\n");

    DocumentCoords::new(doc_x, doc_y)
}

/// ピクセルオフセットをドキュメント単位に変換
///
/// ドラッグ操作の移動量を変換する際に使用します。
///
/// # 引数
///
/// * `offset_x` - X方向のピクセルオフセット
/// * `offset_y` - Y方向のピクセルオフセット
/// * `render_state` - キャンバスの描画状態
pub fn offset_to_document(
    offset_x: f64,
    offset_y: f64,
    render_state: &CanvasRenderState,
) -> (f64, f64) {
    let config = render_state.config.borrow();
    let zoom = config.zoom;
    (offset_x / zoom, offset_y / zoom)
}

/// ドキュメント座標をウィジェット座標に変換（逆変換）
///
/// # 引数
///
/// * `doc_x` - ドキュメントX座標
/// * `doc_y` - ドキュメントY座標
/// * `render_state` - キャンバスの描画状態
pub fn document_to_widget(
    doc_x: f64,
    doc_y: f64,
    render_state: &CanvasRenderState,
) -> (f64, f64) {
    let config = render_state.config.borrow();
    let ruler_config = render_state.ruler_config.borrow();

    let ruler_size = ruler_config.size;
    let pan_x = config.pan_x;
    let pan_y = config.pan_y;
    let zoom = config.zoom;

    // 逆変換: doc * zoom + ruler + pan = widget
    let widget_x = doc_x * zoom + ruler_size + pan_x;
    let widget_y = doc_y * zoom + ruler_size + pan_y;

    (widget_x, widget_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::CanvasRenderState;

    #[test]
    fn test_widget_to_document_default() {
        let state = CanvasRenderState::default();

        // デフォルト: ruler=20, pan=(0,0), zoom=1.0
        // widget(100, 100) → doc(80, 80)
        let coords = widget_to_document(100.0, 100.0, &state);
        assert!((coords.x - 80.0).abs() < 0.01);
        assert!((coords.y - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_widget_to_document_with_zoom() {
        let state = CanvasRenderState::default();
        state.config.borrow_mut().zoom = 2.0;

        // ruler=20, pan=(0,0), zoom=2.0
        // widget(100, 100) → canvas(80, 80) → doc(40, 40)
        let coords = widget_to_document(100.0, 100.0, &state);
        assert!((coords.x - 40.0).abs() < 0.01);
        assert!((coords.y - 40.0).abs() < 0.01);
    }

    #[test]
    fn test_roundtrip() {
        let state = CanvasRenderState::default();
        state.config.borrow_mut().zoom = 1.5;
        state.config.borrow_mut().pan_x = 50.0;
        state.config.borrow_mut().pan_y = 30.0;

        let original_widget = (200.0, 150.0);
        let doc = widget_to_document(original_widget.0, original_widget.1, &state);
        let back = document_to_widget(doc.x, doc.y, &state);

        assert!((back.0 - original_widget.0).abs() < 0.01);
        assert!((back.1 - original_widget.1).abs() < 0.01);
    }
}
