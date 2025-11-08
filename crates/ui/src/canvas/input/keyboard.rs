//! キーボード入力処理モジュール
//!
//! キャンバスのキーボードイベントを処理し、テキスト編集、ショートカット実行、
//! オブジェクト移動などの機能を提供します。
//!
//! # 主な機能
//!
//! - **テキスト編集**: 文字入力、削除、カーソル移動
//! - **ショートカット**: コピー/カット/ペースト/複製
//! - **テキスト配置**: 左揃え、右揃え、中央揃え、両端揃え
//! - **オブジェクト移動**: 矢印キーでの選択オブジェクト移動
//! - **画像挿入**: Ctrl+Shift+I
//! - **テンプレート保存**: Ctrl+Shift+S
//!
//! # キーボード操作一覧
//!
//! | キー | 説明 |
//! |------|------|
//! | Ctrl+C | 選択オブジェクトをコピー |
//! | Ctrl+X | 選択オブジェクトをカット（削除後にコピー） |
//! | Ctrl+V | クリップボードからペースト |
//! | Ctrl+D | 選択オブジェクトを複製 |
//! | Ctrl+L | テキスト左揃え |
//! | Ctrl+R | テキスト右揃え |
//! | Ctrl+E | テキスト右揃え（代替） |
//! | Ctrl+C | テキスト中央揃え |
//! | Ctrl+J | テキスト両端揃え |
//! | Ctrl+Shift+I | 画像挿入 |
//! | Ctrl+Shift+S | テンプレートとして保存 |
//! | ←→↑↓ | オブジェクト移動（Shift: 10px、通常: 1px） |
//! | Escape | テキスト編集終了 |
//! | BackSpace | 前の文字削除 |
//! | Delete | カーソル位置の文字削除 |
//! | Left/Right | カーソル左右移動 |
//! | Home/End | カーソル行頭/行末移動 |
//! | Return | 改行挿入 |
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
            handle_insert_image(&app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Ctrl+Shift+S to save as template
        if ctrl_pressed && shift_pressed && keyval == gtk4::gdk::Key::s {
            handle_save_template(&app_state_keyboard);
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
            handle_copy(&render_state_kbd, &app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Cut: Ctrl+X
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::x {
            handle_cut(&render_state_kbd, &app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Paste: Ctrl+V
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::v {
            handle_paste(&app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle Duplicate: Ctrl+D
        if ctrl_pressed && !in_text_editing && keyval == gtk4::gdk::Key::d {
            handle_duplicate(&render_state_kbd, &app_state_keyboard, &drawing_area_keyboard);
            return gtk4::glib::Propagation::Stop;
        }

        // Handle object movement when NOT in text editing
        let movement_amount = if shift_pressed { 10.0 } else { 1.0 };

        // Handle arrow keys for object movement
        let handled = match keyval {
            gtk4::gdk::Key::Left => {
                if !in_text_editing {
                    move_selected_objects(
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
                    move_selected_objects(
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
                    move_selected_objects(
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
                    move_selected_objects(
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

fn handle_insert_image(app_state: &AppState, drawing_area: &DrawingArea) {
    app_state.with_active_document(|doc| {
        if let Some(page) = doc.pages.first_mut() {
            let image = DocumentElement::Image(testruct_core::document::ImageElement {
                id: Uuid::new_v4(),
                source: testruct_core::workspace::assets::AssetRef::new(),
                bounds: testruct_core::layout::Rect {
                    origin: testruct_core::layout::Point { x: 100.0, y: 100.0 },
                    size: testruct_core::layout::Size {
                        width: 200.0,
                        height: 150.0,
                    },
                },
            });
            page.elements.push(image);
        }
    });
    drawing_area.queue_draw();
    tracing::info!("✅ Image inserted");
}

fn handle_save_template(app_state: &AppState) {
    if let Some(document) = app_state.active_document() {
        let template_name = chrono::Local::now()
            .format("template_%Y%m%d_%H%M%S")
            .to_string();
        match crate::templates::save_template(&template_name, &document) {
            Ok(_) => {
                tracing::info!("✅ Document saved as template: {}", template_name);
            }
            Err(e) => {
                tracing::error!("Failed to save template: {}", e);
            }
        }
    }
}

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

fn handle_copy(
    render_state: &CanvasRenderState,
    app_state: &AppState,
    drawing_area: &DrawingArea,
) {
    let selected = render_state.selected_ids.borrow();
    if !selected.is_empty() {
        if let Some(document) = app_state.active_document() {
            if let Some(page) = document.pages.first() {
                let elements: Vec<_> = page
                    .elements
                    .iter()
                    .filter(|e| selected.contains(&e.id()))
                    .cloned()
                    .collect();

                crate::clipboard::copy_to_clipboard(elements);
                tracing::info!("✅ Copied {} objects to clipboard", selected.len());
                drawing_area.queue_draw();
            }
        }
    }
}

fn handle_cut(
    render_state: &CanvasRenderState,
    app_state: &AppState,
    drawing_area: &DrawingArea,
) {
    let selected = render_state.selected_ids.borrow().clone();
    let selected_count = selected.len();

    if !selected.is_empty() {
        app_state.with_mutable_active_document(|doc| {
            if let Some(page) = doc.pages.first_mut() {
                // Copy selected elements to clipboard
                let elements: Vec<_> = page
                    .elements
                    .iter()
                    .filter(|e| selected.contains(&e.id()))
                    .cloned()
                    .collect();

                crate::clipboard::copy_to_clipboard(elements);

                // Delete the selected elements
                page.elements.retain(|e| !selected.contains(&e.id()));
            }
        });

        // Clear selection
        render_state.selected_ids.borrow_mut().clear();

        tracing::info!("✅ Cut {} objects", selected_count);
        drawing_area.queue_draw();
    }
}

fn handle_paste(app_state: &AppState, drawing_area: &DrawingArea) {
    if crate::clipboard::has_clipboard_content() {
        if let Some(pasted_elements) = crate::clipboard::paste_from_clipboard() {
            if !pasted_elements.is_empty() {
                let paste_count = pasted_elements.len();
                app_state.with_mutable_active_document(|doc| {
                    if let Some(page) = doc.pages.first_mut() {
                        for elem in pasted_elements {
                            page.add_element(elem);
                        }
                    }
                });

                tracing::info!("✅ Pasted {} elements", paste_count);
                drawing_area.queue_draw();
            }
        }
    }
}

fn handle_duplicate(
    render_state: &CanvasRenderState,
    app_state: &AppState,
    drawing_area: &DrawingArea,
) {
    let selected = render_state.selected_ids.borrow().clone();
    if !selected.is_empty() {
        app_state.with_mutable_active_document(|doc| {
            if let Some(page) = doc.pages.first_mut() {
                let mut new_elements = Vec::new();

                for orig_elem in page.elements.iter().filter(|e| selected.contains(&e.id())) {
                    let mut new_elem = orig_elem.clone();
                    let new_id = Uuid::new_v4();

                    // Update ID and offset position
                    match &mut new_elem {
                        DocumentElement::Text(t) => {
                            t.id = new_id;
                            t.bounds.origin.x += 20.0;
                            t.bounds.origin.y += 20.0;
                        }
                        DocumentElement::Image(img) => {
                            img.id = new_id;
                            img.bounds.origin.x += 20.0;
                            img.bounds.origin.y += 20.0;
                        }
                        DocumentElement::Shape(shape) => {
                            shape.id = new_id;
                            shape.bounds.origin.x += 20.0;
                            shape.bounds.origin.y += 20.0;
                        }
                        DocumentElement::Frame(frame) => {
                            frame.id = new_id;
                            frame.bounds.origin.x += 20.0;
                            frame.bounds.origin.y += 20.0;
                        }
                        DocumentElement::Group(group) => {
                            group.id = new_id;
                            group.bounds.origin.x += 20.0;
                            group.bounds.origin.y += 20.0;
                        }
                    }

                    new_elements.push(new_elem);
                }

                for elem in new_elements {
                    page.add_element(elem);
                }
            }
        });

        tracing::info!("✅ Duplicated {} objects", selected.len());
        drawing_area.queue_draw();
    }
}

pub fn move_selected_objects(
    render_state: &CanvasRenderState,
    app_state: &AppState,
    delta_x: f32,
    delta_y: f32,
) {
    let selected = render_state.selected_ids.borrow();

    if !selected.is_empty() {
        app_state.with_mutable_active_document(|doc| {
            if let Some(page) = doc.pages.first_mut() {
                for element in &mut page.elements {
                    if selected.contains(&element.id()) {
                        match element {
                            DocumentElement::Text(text) => {
                                text.bounds.origin.x += delta_x;
                                text.bounds.origin.y += delta_y;
                            }
                            DocumentElement::Image(image) => {
                                image.bounds.origin.x += delta_x;
                                image.bounds.origin.y += delta_y;
                            }
                            DocumentElement::Shape(shape) => {
                                shape.bounds.origin.x += delta_x;
                                shape.bounds.origin.y += delta_y;
                            }
                            DocumentElement::Frame(frame) => {
                                frame.bounds.origin.x += delta_x;
                                frame.bounds.origin.y += delta_y;
                            }
                            DocumentElement::Group(group) => {
                                group.bounds.origin.x += delta_x;
                                group.bounds.origin.y += delta_y;
                            }
                        }
                    }
                }
            }
        });
    }
}
