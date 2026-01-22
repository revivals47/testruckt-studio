//! テキスト配置キー処理
//!
//! テキスト編集モード中のアライメント変更キー（Ctrl+L/R/E/C/J）を処理します。
//!
//! # 対応キー（Ctrl+X の形式）
//!
//! | キー | 説明 |
//! |------|------|
//! | Ctrl+L | テキスト左揃え |
//! | Ctrl+R | テキスト右揃え |
//! | Ctrl+E | テキスト右揃え（代替） |
//! | Ctrl+C | テキスト中央揃え |
//! | Ctrl+J | テキスト両端揃え |

use crate::app::AppState;
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::DrawingArea;
use testruct_core::document::DocumentElement;
use uuid::Uuid;

/// テキスト配置変更処理
///
/// Ctrl+L/R/E/C/J によるテキストアライメント変更を処理します。
///
/// # 引数
///
/// - `app_state`: アプリケーション状態
/// - `render_state`: キャンバス描画状態
/// - `drawing_area`: 描画エリア（再描画用）
/// - `text_id`: 編集中のテキスト要素ID
/// - `keyval`: 押されたキーの値
///
/// # 戻り値
///
/// アライメントが変更された場合は `true`、それ以外は `false`
pub fn handle_text_alignment(
    app_state: &AppState,
    _render_state: &CanvasRenderState,
    drawing_area: &DrawingArea,
    text_id: Uuid,
    keyval: gtk4::gdk::Key,
) -> bool {
    let alignment = match keyval {
        gtk4::gdk::Key::l => Some(testruct_core::typography::TextAlignment::Start),
        gtk4::gdk::Key::r => Some(testruct_core::typography::TextAlignment::End),
        gtk4::gdk::Key::e => Some(testruct_core::typography::TextAlignment::End),
        gtk4::gdk::Key::c => Some(testruct_core::typography::TextAlignment::Center),
        gtk4::gdk::Key::j => Some(testruct_core::typography::TextAlignment::Justified),
        _ => None,
    };

    if let Some(new_alignment) = alignment {
        app_state.with_active_document(|doc| {
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
        drawing_area.queue_draw();
        let align_name = match new_alignment {
            testruct_core::typography::TextAlignment::Start => "Left",
            testruct_core::typography::TextAlignment::Center => "Center",
            testruct_core::typography::TextAlignment::End => "Right",
            testruct_core::typography::TextAlignment::Justified => "Justified",
        };
        tracing::info!("✅ Text alignment changed to: {}", align_name);
        true
    } else {
        false
    }
}
