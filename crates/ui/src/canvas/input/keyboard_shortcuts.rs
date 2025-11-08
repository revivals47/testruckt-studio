//! キーボードショートカット処理モジュール
//!
//! コピー、カット、ペースト、複製、オブジェクト移動などのショートカット機能を提供します。
//!
//! # 主な機能
//!
//! - **コピー/カット/ペースト**: Ctrl+C/X/V
//! - **複製**: Ctrl+D
//! - **オブジェクト移動**: 矢印キー（Shift: 10px、通常: 1px）
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
//! | Ctrl+Shift+I | 画像挿入 |
//! | Ctrl+Shift+S | テンプレートとして保存 |
//! | ←→↑↓ | オブジェクト移動（Shift: 10px、通常: 1px） |

use crate::app::AppState;
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::DrawingArea;
use testruct_core::document::DocumentElement;
use uuid::Uuid;

/// 画像挿入処理（Ctrl+Shift+I）
///
/// 新しい画像要素を現在のページに追加します。
///
/// # 引数
///
/// - `app_state`: アプリケーション状態
/// - `drawing_area`: 描画エリア（再描画用）
pub fn handle_insert_image(app_state: &AppState, drawing_area: &DrawingArea) {
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

/// テンプレート保存処理（Ctrl+Shift+S）
///
/// 現在のドキュメントをテンプレートとして保存します。
///
/// # 引数
///
/// - `app_state`: アプリケーション状態
pub fn handle_save_template(app_state: &AppState) {
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

/// コピー処理（Ctrl+C）
///
/// 選択されたオブジェクトをクリップボードにコピーします。
///
/// # 引数
///
/// - `render_state`: キャンバス描画状態
/// - `app_state`: アプリケーション状態
/// - `drawing_area`: 描画エリア（再描画用）
pub fn handle_copy(
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

/// カット処理（Ctrl+X）
///
/// 選択されたオブジェクトをクリップボードにコピーした後、削除します。
///
/// # 引数
///
/// - `render_state`: キャンバス描画状態
/// - `app_state`: アプリケーション状態
/// - `drawing_area`: 描画エリア（再描画用）
pub fn handle_cut(
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

/// ペースト処理（Ctrl+V）
///
/// クリップボードからオブジェクトをペーストします。
///
/// # 引数
///
/// - `app_state`: アプリケーション状態
/// - `drawing_area`: 描画エリア（再描画用）
pub fn handle_paste(app_state: &AppState, drawing_area: &DrawingArea) {
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

/// 複製処理（Ctrl+D）
///
/// 選択されたオブジェクトを複製し、少しオフセットして配置します。
///
/// # 引数
///
/// - `render_state`: キャンバス描画状態
/// - `app_state`: アプリケーション状態
/// - `drawing_area`: 描画エリア（再描画用）
pub fn handle_duplicate(
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

/// 選択オブジェクトの移動
///
/// 矢印キーによるオブジェクトの移動を処理します。
///
/// # 引数
///
/// - `render_state`: キャンバス描画状態
/// - `app_state`: アプリケーション状態
/// - `delta_x`: X方向の移動量
/// - `delta_y`: Y方向の移動量
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
