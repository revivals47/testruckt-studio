//! キーボード入力処理モジュール
//!
//! キャンバスのキーボードイベントを処理し、テキスト編集とショートカット実行を提供します。
//!
//! # 主な機能
//!
//! - **テキスト編集**: 文字入力、削除、カーソル移動
//! - **テキスト配置**: 左揃え、右揃え、中央揃え、両端揃え
//! - **ショートカット統合**: keyboard_shortcuts モジュールと連携
//!
//! # キーボード操作一覧
//!
//! ## テキスト編集
//!
//! | キー | 説明 |
//! |------|------|
//! | Escape | テキスト編集終了 |
//! | BackSpace | 前の文字削除 |
//! | Delete | カーソル位置の文字削除 |
//! | Left/Right | カーソル左右移動 |
//! | Home/End | カーソル行頭/行末移動 |
//! | Return | 改行挿入 |
//!
//! ## テキスト配置（編集中）
//!
//! | キー | 説明 |
//! |------|------|
//! | Ctrl+L | テキスト左揃え |
//! | Ctrl+R | テキスト右揃え |
//! | Ctrl+E | テキスト右揃え（代替） |
//! | Ctrl+C | テキスト中央揃え |
//! | Ctrl+J | テキスト両端揃え |
//!
//! ## ショートカット（詳細は keyboard_shortcuts モジュール参照）
//!
//! | キー | 説明 |
//! |------|------|
//! | Ctrl+C | 選択オブジェクトをコピー |
//! | Ctrl+X | 選択オブジェクトをカット（削除後にコピー） |
//! | Ctrl+V | クリップボードからペースト |
//! | Ctrl+D | 選択オブジェクトを複製 |
//! | Ctrl+Shift+I | 画像挿入 |
//! | Ctrl+Shift+S | テンプレートとして保存 |
//! | ←→↑↓ | オブジェクト移動（Shift: 10px、通常: 1px） |
//!
//! # 使用例
//!
//! ```ignore
//! use crate::canvas::input::keyboard;
//!
//! keyboard::setup_keyboard_events(drawing_area, render_state, app_state);
//! ```
//!
//! # テキスト編集モード
//!
//! テキスト要素をダブルクリックすると編集モードに進入し、以下が可能になります：
//! - 文字の挿入・削除
//! - カーソル移動
//! - テキスト配置の変更
//! - Escape キーで編集終了
//!
//! # 状態管理
//!
//! キーボード処理は以下の状態を参照・更新します：
//! - `render_state.tool_state.editing_text_id`: 編集中のテキスト要素ID
//! - `render_state.tool_state.editing_cursor_pos`: カーソル位置
//! - `render_state.selected_ids`: 選択オブジェクトID一覧

use crate::app::AppState;
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::{DrawingArea, EventControllerKey};
use testruct_core::document::DocumentElement;
use uuid::Uuid;

// Import keyboard shortcuts module
use super::keyboard_shortcuts;

/// キーボードイベント処理を初期化
///
/// # 引数
///
/// - `drawing_area`: GTK DrawingArea ウィジェット
/// - `render_state`: キャンバス描画状態
/// - `app_state`: アプリケーション全体の状態
pub fn setup_keyboard_events(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    let key_controller = EventControllerKey::new();
    let render_state_keyboard = render_state.clone();
    let app_state_keyboard = app_state.clone();
    let drawing_area_keyboard = drawing_area.clone();

    key_controller.connect_key_pressed(move |_controller, keyval, _keycode, state| {
        let render_state_kbd = render_state_keyboard.clone();
        let tool_state_ref = render_state_kbd.tool_state.borrow();
        let in_text_editing = tool_state_ref.editing_text_id.is_some();
        let editing_text_id = tool_state_ref.editing_text_id;
        let mut cursor_pos = tool_state_ref.editing_cursor_pos;
        drop(tool_state_ref);

        // Determine if shift and control are pressed
        let shift_pressed = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
        let ctrl_pressed = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);

        // Handle Ctrl+Shift+I to insert image
        if ctrl_pressed && shift_pressed && keyval == gtk4::gdk::Key::i {
            keyboard_shortcuts::handle_insert_image(&app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Ctrl+Shift+S to save as template
        if ctrl_pressed && shift_pressed && keyval == gtk4::gdk::Key::s {
            keyboard_shortcuts::handle_save_template(&app_state_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle text alignment shortcuts (Ctrl+L, Ctrl+E, Ctrl+R, Ctrl+C)
        if ctrl_pressed && in_text_editing {
            if let Some(text_id) = editing_text_id {
                if handle_text_alignment(
                    &app_state_keyboard,
                    &render_state_kbd,
                    &drawing_area_keyboard,
                    text_id,
                    keyval,
                ) {
                    return gtk4::glib::Propagation::Stop;
                }
            }
        }

        // Handle text editing keys
        if in_text_editing {
            if let Some(text_id) = editing_text_id {
                if let Some(should_stop) = handle_text_editing_key(
                    &app_state_keyboard,
                    &render_state_kbd,
                    &drawing_area_keyboard,
                    text_id,
                    keyval,
                    &mut cursor_pos,
                ) {
                    if should_stop {
                        return gtk4::glib::Propagation::Stop;
                    }
                }
            }
        }

        // Handle Copy: Ctrl+C
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::c {
            keyboard_shortcuts::handle_copy(&render_state_kbd, &app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Cut: Ctrl+X
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::x {
            keyboard_shortcuts::handle_cut(&render_state_kbd, &app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Paste: Ctrl+V
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::v {
            keyboard_shortcuts::handle_paste(&app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Duplicate: Ctrl+D
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::d {
            keyboard_shortcuts::handle_duplicate(&render_state_kbd, &app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle object movement when NOT in text editing
        let movement_amount = if shift_pressed { 10.0 } else { 1.0 };

        // Handle arrow keys for object movement
        let handled = match keyval {
            gtk4::gdk::Key::Left => {
                if !in_text_editing {
                    keyboard_shortcuts::move_selected_objects(
                        &render_state_kbd,
                        &app_state_keyboard,
                        -movement_amount,
                        0.0,
                    );
                    drawing_area_keyboard.queue_draw();
                    tracing::info!("✅ Move left ({}px)", movement_amount as i32);
                    true
                } else {
                    false
                }
            }
            gtk4::gdk::Key::Right => {
                if !in_text_editing {
                    keyboard_shortcuts::move_selected_objects(
                        &render_state_kbd,
                        &app_state_keyboard,
                        movement_amount,
                        0.0,
                    );
                    drawing_area_keyboard.queue_draw();
                    tracing::info!("✅ Move right ({}px)", movement_amount as i32);
                    true
                } else {
                    false
                }
            }
            gtk4::gdk::Key::Up => {
                if !in_text_editing {
                    keyboard_shortcuts::move_selected_objects(
                        &render_state_kbd,
                        &app_state_keyboard,
                        0.0,
                        -movement_amount,
                    );
                    drawing_area_keyboard.queue_draw();
                    tracing::info!("✅ Move up ({}px)", movement_amount as i32);
                    true
                } else {
                    false
                }
            }
            gtk4::gdk::Key::Down => {
                if !in_text_editing {
                    keyboard_shortcuts::move_selected_objects(
                        &render_state_kbd,
                        &app_state_keyboard,
                        0.0,
                        movement_amount,
                    );
                    drawing_area_keyboard.queue_draw();
                    tracing::info!("✅ Move down ({}px)", movement_amount as i32);
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        if handled {
            gtk4::glib::Propagation::Stop
        } else {
            gtk4::glib::Propagation::Proceed
        }
    });
    drawing_area.add_controller(key_controller);
}

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
fn handle_text_alignment(
    app_state: &AppState,
    render_state: &CanvasRenderState,
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
fn handle_text_editing_key(
    app_state: &AppState,
    render_state: &CanvasRenderState,
    drawing_area: &DrawingArea,
    text_id: Uuid,
    keyval: gtk4::gdk::Key,
    cursor_pos: &mut usize,
) -> Option<bool> {
    match keyval {
        gtk4::gdk::Key::Escape => {
            // Exit text editing mode
            let mut tool_state = render_state.tool_state.borrow_mut();
            tool_state.editing_text_id = None;
            tool_state.editing_cursor_pos = 0;
            drop(tool_state);
            drawing_area.queue_draw();
            tracing::info!("✅ Exited text editing mode");
            Some(true)
        }
        gtk4::gdk::Key::BackSpace => {
            // Delete character before cursor
            if *cursor_pos > 0 {
                app_state.with_active_document(|doc| {
                    if let Some(page) = doc.pages.first_mut() {
                        for element in &mut page.elements {
                            if let DocumentElement::Text(text) = element {
                                if text.id == text_id {
                                    if *cursor_pos <= text.content.len() && *cursor_pos > 0 {
                                        text.content.remove(*cursor_pos - 1);
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
                tracing::info!(
                    "✅ Deleted character at cursor position {}",
                    cursor_pos
                );
            }
            Some(true)
        }
        gtk4::gdk::Key::Delete => {
            // Delete character at cursor
            app_state.with_active_document(|doc| {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        if let DocumentElement::Text(text) = element {
                            if text.id == text_id {
                                if *cursor_pos < text.content.len() {
                                    text.content.remove(*cursor_pos);
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
                                text.content.insert(*cursor_pos, '\n');
                                *cursor_pos += 1;
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
                                        text.content.insert(*cursor_pos, ch);
                                        *cursor_pos += 1;
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
