//! テキスト編集キー処理
//!
//! テキスト編集モード中のキーボード入力を処理します。
//!
//! # 対応キー
//!
//! | キー | 説明 |
//! |------|------|
//! | Escape | テキスト編集終了 |
//! | BackSpace | 前の文字削除 |
//! | Delete | カーソル位置の文字削除 |
//! | Left | カーソル左移動 |
//! | Right | カーソル右移動 |
//! | Home | カーソル行頭移動 |
//! | End | カーソル行末移動 |
//! | Return | 改行挿入 |
//! | その他 | 通常文字入力（ASCII、Unicode） |

use super::super::ime::ImeManager;
use crate::app::AppState;
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::DrawingArea;
use std::cell::RefCell;
use std::rc::Rc;
use testruct_core::document::DocumentElement;
use uuid::Uuid;

/// テキスト編集キー処理
///
/// テキスト編集モード中のキーボード入力を処理します。
///
/// # 引数
///
/// - `app_state`: アプリケーション状態
/// - `render_state`: キャンバス描画状態
/// - `drawing_area`: 描画エリア（再描画用）
/// - `text_id`: 編集中のテキスト要素ID
/// - `keyval`: 押されたキーの値
/// - `cursor_pos`: カーソル位置（可変参照）
///
/// # 戻り値
///
/// キーが処理された場合は `Some(true)`、伝播を続ける場合は `Some(false)`、
/// 未処理の場合は `None`
pub fn handle_text_editing_key(
    app_state: &AppState,
    render_state: &CanvasRenderState,
    drawing_area: &DrawingArea,
    text_id: Uuid,
    keyval: gtk4::gdk::Key,
    cursor_pos: &mut usize,
    ime_manager: &Rc<RefCell<ImeManager>>,
) -> Option<bool> {
    match keyval {
        gtk4::gdk::Key::Escape => {
            // Exit text editing mode
            let mut tool_state = render_state.tool_state.borrow_mut();
            tool_state.editing_text_id = None;
            tool_state.editing_cursor_pos = 0;
            drop(tool_state);

            // NOTE: IME focus management is handled automatically by GTK4
            // on macOS with EventControllerKey, so no explicit focus_out/reset needed

            drawing_area.queue_draw();
            tracing::info!("✅ Exited text editing mode");
            Some(true)
        }
        gtk4::gdk::Key::BackSpace => {
            // Delete character before cursor (use char count, not byte count)
            if *cursor_pos > 0 {
                app_state.with_active_document(|doc| {
                    if let Some(page) = doc.pages.first_mut() {
                        for element in &mut page.elements {
                            if let DocumentElement::Text(text) = element {
                                if text.id == text_id {
                                    // Convert cursor position from char count to byte index
                                    let chars: Vec<char> = text.content.chars().collect();
                                    if *cursor_pos <= chars.len() && *cursor_pos > 0 {
                                        // Remove character at position cursor_pos - 1
                                        let new_content: String = chars
                                            .iter()
                                            .enumerate()
                                            .filter(|(i, _)| *i != *cursor_pos - 1)
                                            .map(|(_, c)| c)
                                            .collect();
                                        text.content = new_content;
                                        *cursor_pos -= 1;
                                    }
                                }
                            }
                        }
                    }
                });
                let mut tool_state = render_state.tool_state.borrow_mut();
                tool_state.editing_cursor_pos = *cursor_pos;
                drop(tool_state);
                drawing_area.queue_draw();
                tracing::info!("✅ Deleted character at cursor position {}", cursor_pos);
            }
            Some(true)
        }
        gtk4::gdk::Key::Delete => {
            // Delete character at cursor (use char count, not byte count)
            app_state.with_active_document(|doc| {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        if let DocumentElement::Text(text) = element {
                            if text.id == text_id {
                                let chars: Vec<char> = text.content.chars().collect();
                                if *cursor_pos < chars.len() {
                                    // Remove character at position cursor_pos
                                    let new_content: String = chars
                                        .iter()
                                        .enumerate()
                                        .filter(|(i, _)| *i != *cursor_pos)
                                        .map(|(_, c)| c)
                                        .collect();
                                    text.content = new_content;
                                }
                            }
                        }
                    }
                }
            });
            drawing_area.queue_draw();
            tracing::info!("✅ Deleted character at cursor position {}", cursor_pos);
            Some(true)
        }
        gtk4::gdk::Key::Left => {
            // Move cursor left
            if *cursor_pos > 0 {
                *cursor_pos -= 1;
                let mut tool_state = render_state.tool_state.borrow_mut();
                tool_state.editing_cursor_pos = *cursor_pos;
                drop(tool_state);
                drawing_area.queue_draw();
                tracing::debug!("Cursor moved to position {}", cursor_pos);
            }
            Some(true)
        }
        gtk4::gdk::Key::Right => {
            // Move cursor right
            if let Some(document) = app_state.active_document() {
                if let Some(page) = document.pages.first() {
                    for element in &page.elements {
                        if let DocumentElement::Text(text) = element {
                            if text.id == text_id && *cursor_pos < text.content.len() {
                                *cursor_pos += 1;
                            }
                        }
                    }
                }
            }
            let mut tool_state = render_state.tool_state.borrow_mut();
            tool_state.editing_cursor_pos = *cursor_pos;
            drop(tool_state);
            drawing_area.queue_draw();
            tracing::debug!("Cursor moved to position {}", cursor_pos);
            Some(true)
        }
        gtk4::gdk::Key::Home => {
            // Move cursor to start
            *cursor_pos = 0;
            let mut tool_state = render_state.tool_state.borrow_mut();
            tool_state.editing_cursor_pos = *cursor_pos;
            drop(tool_state);
            drawing_area.queue_draw();
            tracing::debug!("Cursor moved to start");
            Some(true)
        }
        gtk4::gdk::Key::End => {
            // Move cursor to end
            if let Some(document) = app_state.active_document() {
                if let Some(page) = document.pages.first() {
                    for element in &page.elements {
                        if let DocumentElement::Text(text) = element {
                            if text.id == text_id {
                                *cursor_pos = text.content.len();
                            }
                        }
                    }
                }
            }
            let mut tool_state = render_state.tool_state.borrow_mut();
            tool_state.editing_cursor_pos = *cursor_pos;
            drop(tool_state);
            drawing_area.queue_draw();
            tracing::debug!("Cursor moved to end");
            Some(true)
        }
        gtk4::gdk::Key::Return => {
            // Insert newline character for multiline support
            app_state.with_active_document(|doc| {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        if let DocumentElement::Text(text) = element {
                            if text.id == text_id {
                                // Insert newline using char-based indexing
                                let mut chars: Vec<char> = text.content.chars().collect();

                                // Insert newline at cursor position
                                if *cursor_pos <= chars.len() {
                                    chars.insert(*cursor_pos, '\n');
                                    text.content = chars.iter().collect();
                                    *cursor_pos += 1;
                                }
                            }
                        }
                    }
                }
            });
            let mut tool_state = render_state.tool_state.borrow_mut();
            tool_state.editing_cursor_pos = *cursor_pos;
            drop(tool_state);
            drawing_area.queue_draw();
            tracing::info!("✅ Inserted newline at position {}", *cursor_pos - 1);
            Some(true)
        }
        _ => {
            // Try to handle as text input (support both ASCII and Unicode characters)
            if let Some(ch) = keyval.to_unicode() {
                // Accept any printable character (not just ASCII)
                if !ch.is_control() {
                    app_state.with_active_document(|doc| {
                        if let Some(page) = doc.pages.first_mut() {
                            for element in &mut page.elements {
                                if let DocumentElement::Text(text) = element {
                                    if text.id == text_id {
                                        // Insert character using char-based indexing
                                        let mut chars: Vec<char> = text.content.chars().collect();

                                        // Insert character at cursor position
                                        if *cursor_pos <= chars.len() {
                                            chars.insert(*cursor_pos, ch);
                                            text.content = chars.iter().collect();
                                            *cursor_pos += 1;
                                        }
                                    }
                                }
                            }
                        }
                    });
                    let mut tool_state = render_state.tool_state.borrow_mut();
                    tool_state.editing_cursor_pos = *cursor_pos;
                    drop(tool_state);
                    drawing_area.queue_draw();
                    tracing::debug!(
                        "✅ Inserted character '{}' at position {}",
                        ch,
                        *cursor_pos - 1
                    );
                    return Some(true);
                }
            }
            None
        }
    }
}
