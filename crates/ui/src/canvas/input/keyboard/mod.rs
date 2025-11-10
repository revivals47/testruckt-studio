//! キーボード入力処理モジュール
//!
//! キャンバスのキーボードイベントを統合的に処理し、テキスト編集とショートカット実行を提供します。
//!
//! # モジュール構成
//!
//! このモジュールは以下の専門的なサブモジュールで構成されています：
//!
//! - [`text_editing_keys`]: テキスト編集用キー処理（文字入力、削除、カーソル移動）
//! - [`text_alignment_keys`]: テキスト配置用キー処理（Ctrl+L/R/E/C/J）
//!
//! # 主な機能
//!
//! - **テキスト編集**: 文字入力、削除、カーソル移動（`text_editing_keys` で実装）
//! - **テキスト配置**: 左揃え、右揃え、中央揃え、両端揃え（`text_alignment_keys` で実装）
//! - **ショートカット統合**: `keyboard_shortcuts` モジュールと連携
//! - **オブジェクト移動**: 矢印キーによるオブジェクト移動
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
//! - 文字の挿入・削除（`text_editing_keys` で処理）
//! - カーソル移動（`text_editing_keys` で処理）
//! - テキスト配置の変更（`text_alignment_keys` で処理）
//! - Escape キーで編集終了

pub mod text_alignment_keys;
pub mod text_editing_keys;
// IME module is declared in parent input module

use crate::app::AppState;
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::{DrawingArea, EventControllerKey};
use std::cell::RefCell;
use std::rc::Rc;

// Import keyboard shortcuts and key handlers
use self::text_alignment_keys::handle_text_alignment;
use self::text_editing_keys::handle_text_editing_key;
use super::ime::ImeManager;
use super::keyboard_shortcuts;

/// キーボードイベント処理を初期化
///
/// # 引数
///
/// - `drawing_area`: GTK DrawingArea ウィジェット
/// - `render_state`: キャンバス描画状態
/// - `app_state`: アプリケーション全体の状態
/// - `ime_manager`: Shared IME manager for handling Japanese/Chinese input
pub fn setup_keyboard_events(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
    ime_manager: Rc<RefCell<ImeManager>>,
) {
    let key_controller = EventControllerKey::new();
    let render_state_keyboard = render_state.clone();
    let app_state_keyboard = app_state.clone();
    let drawing_area_keyboard = drawing_area.clone();

    // Setup IME (Input Method Editor) for Japanese and other input methods
    ime_manager.borrow().setup_with_controller(&key_controller);

    // Register callback for IME-composed text insertion
    let render_state_ime = render_state.clone();
    let app_state_ime = app_state.clone();
    let drawing_area_ime = drawing_area.clone();

    ime_manager
        .borrow()
        .set_text_insertion_callback(move |composed_text: String| {
            // When IME delivers composed text (e.g., 日本語), insert it at cursor
            let render_state_ime_cb = render_state_ime.clone();
            let app_state_ime_cb = app_state_ime.clone();
            let drawing_area_ime_cb = drawing_area_ime.clone();

            eprintln!("📱 IME callback invoked with text: '{}'", composed_text);

            let tool_state_ref = render_state_ime_cb.tool_state.borrow();
            if let Some(text_id) = tool_state_ref.editing_text_id {
                let mut cursor_pos = tool_state_ref.editing_cursor_pos;
                eprintln!("📝 Text ID: {:?}, Initial cursor: {}", text_id, cursor_pos);
                drop(tool_state_ref);

                // Insert each composed character at the current cursor position, updating cursor for next char
                for ch in composed_text.chars() {
                    eprintln!("  Inserting '{}' at position {}", ch, cursor_pos);

                    // Use the existing character insertion logic
                    app_state_ime_cb.with_mutable_active_document(|doc| {
                        if let Some(page) = doc.pages.first_mut() {
                            for element in &mut page.elements {
                                if let testruct_core::document::DocumentElement::Text(text) =
                                    element
                                {
                                    if text.id == text_id {
                                        let mut chars: Vec<char> = text.content.chars().collect();
                                        if cursor_pos <= chars.len() {
                                            chars.insert(cursor_pos, ch);
                                            text.content = chars.iter().collect();
                                            tracing::debug!(
                                                "✅ IME inserted character: '{}' at {}",
                                                ch,
                                                cursor_pos
                                            );
                                        } else {
                                            eprintln!(
                                                "⚠️  Cursor pos {} exceeds text length {}",
                                                cursor_pos,
                                                chars.len()
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    });

                    // Increment cursor for next character
                    cursor_pos += 1;
                }

                // Update cursor position in tool state and refresh canvas
                let mut tool_state = render_state_ime_cb.tool_state.borrow_mut();
                tool_state.editing_cursor_pos = cursor_pos;
                drop(tool_state);

                drawing_area_ime_cb.queue_draw();
                eprintln!(
                    "✅ IME commit complete: inserted '{}' ({} chars), cursor now at {}",
                    composed_text,
                    composed_text.chars().count(),
                    cursor_pos
                );
                tracing::debug!(
                    "🎌 IME commit: inserted '{}' ({} chars)",
                    composed_text,
                    composed_text.chars().count()
                );
            } else {
                eprintln!("⚠️  IME callback but no text editing active!");
            }
        });

    key_controller.connect_key_pressed(move |_controller, keyval, _keycode, state| {
        let render_state_kbd = render_state_keyboard.clone();
        let tool_state_ref = render_state_kbd.tool_state.borrow();
        let in_text_editing = tool_state_ref.editing_text_id.is_some();
        let editing_text_id = tool_state_ref.editing_text_id;
        let mut cursor_pos = tool_state_ref.editing_cursor_pos;
        drop(tool_state_ref);

        // Determine if shift and control are pressed (must be before logging)
        // NOTE: On macOS, Command key maps to META_MASK
        let shift_pressed = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
        let ctrl_pressed = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK)
            || state.contains(gtk4::gdk::ModifierType::SUPER_MASK)
            || state.contains(gtk4::gdk::ModifierType::META_MASK);

        eprintln!(
            "🔑 Key pressed: keyval={:?}, in_text_editing={}, ctrl={}, shift={}, state={:?}",
            keyval, in_text_editing, ctrl_pressed, shift_pressed, state
        );

        if in_text_editing {
            eprintln!(
                "📝 In text editing mode - Text ID: {:?}, Cursor pos: {}",
                editing_text_id, cursor_pos
            );
            tracing::debug!(
                "📝 In text editing mode - Text ID: {:?}, Cursor pos: {}",
                editing_text_id,
                cursor_pos
            );
        }

        // NOTE: IME key filtering is handled automatically by GTK4's EventControllerKey
        // when we call set_im_context(). The IME will emit ::commit signal when
        // composition is complete, which we handle in the callback registered above.
        // Direct key handling continues here for non-composition keys (arrows, escape, etc)

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
                    &ime_manager,
                ) {
                    if should_stop {
                        return gtk4::glib::Propagation::Stop;
                    }
                }
            }
        }

        // Handle Copy: Ctrl+C
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::c {
            keyboard_shortcuts::handle_copy(
                &render_state_kbd,
                &app_state_keyboard,
                &drawing_area_keyboard,
            );
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Cut: Ctrl+X
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::x {
            keyboard_shortcuts::handle_cut(
                &render_state_kbd,
                &app_state_keyboard,
                &drawing_area_keyboard,
            );
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Paste: Ctrl+V
        if ctrl_pressed && keyval == gtk4::gdk::Key::v {
            if in_text_editing {
                // Paste text (including Japanese) during text editing mode
                keyboard_shortcuts::handle_paste_text_in_editing(
                    &app_state_keyboard,
                    &render_state_kbd,
                    &drawing_area_keyboard,
                );
                eprintln!("📋 Text paste in editing mode");
            } else {
                // Paste elements when not in text editing
                keyboard_shortcuts::handle_paste(&app_state_keyboard, &drawing_area_keyboard);
            }
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Duplicate: Ctrl+D
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::d {
            keyboard_shortcuts::handle_duplicate(
                &render_state_kbd,
                &app_state_keyboard,
                &drawing_area_keyboard,
            );
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
