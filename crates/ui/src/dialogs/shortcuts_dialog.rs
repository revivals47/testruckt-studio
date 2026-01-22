//! Keyboard shortcuts dialog for the application
//!
//! Displays all available keyboard shortcuts organized by category.

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Dialog, Grid, Label, Orientation, ScrolledWindow, Window};

/// Show the Keyboard Shortcuts dialog
pub fn show_shortcuts_dialog(parent: &Window) {
    let dialog = Dialog::builder()
        .title("キーボードショートカット - Testruct Studio")
        .transient_for(parent)
        .modal(true)
        .default_width(500)
        .default_height(600)
        .build();

    let content_box = GtkBox::new(Orientation::Vertical, 16);
    content_box.set_margin_start(20);
    content_box.set_margin_end(20);
    content_box.set_margin_top(20);
    content_box.set_margin_bottom(20);

    // File operations
    let file_section = create_shortcut_section("ファイル操作", &[
        ("Ctrl+N", "新規作成"),
        ("Ctrl+O", "ファイルを開く"),
        ("Ctrl+S", "保存"),
        ("Ctrl+Shift+S", "テンプレートとして保存"),
    ]);
    content_box.append(&file_section);

    // Edit operations
    let edit_section = create_shortcut_section("編集操作", &[
        ("Ctrl+Z", "取り消し（Undo）"),
        ("Ctrl+Shift+Z", "やり直し（Redo）"),
        ("Ctrl+C", "コピー"),
        ("Ctrl+X", "カット"),
        ("Ctrl+V", "ペースト"),
        ("Ctrl+D", "複製"),
        ("Delete", "削除"),
    ]);
    content_box.append(&edit_section);

    // Selection and movement
    let selection_section = create_shortcut_section("選択・移動", &[
        ("Ctrl+A", "すべて選択"),
        ("↑ ↓ ← →", "オブジェクト移動（1px）"),
        ("Shift+↑↓←→", "オブジェクト移動（10px）"),
        ("Shift+クリック", "複数選択"),
    ]);
    content_box.append(&selection_section);

    // View operations
    let view_section = create_shortcut_section("表示", &[
        ("F6", "ルーラー表示切替"),
        ("F7", "ガイド表示切替"),
        ("F8", "グリッド表示切替"),
    ]);
    content_box.append(&view_section);

    // Tool operations
    let tool_section = create_shortcut_section("ツール", &[
        ("Ctrl+Shift+I", "画像挿入"),
        ("F1", "このダイアログを表示"),
    ]);
    content_box.append(&tool_section);

    // Canvas operations
    let canvas_section = create_shortcut_section("キャンバス操作", &[
        ("ホイールスクロール", "ズーム"),
        ("ドラッグ", "範囲選択/オブジェクト移動"),
        ("ダブルクリック", "テキスト編集開始"),
        ("Escape", "テキスト編集終了/選択解除"),
    ]);
    content_box.append(&canvas_section);

    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&content_box));
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);

    let content_area = dialog.content_area();
    content_area.append(&scrolled);

    // Add close button
    dialog.add_button("閉じる", gtk4::ResponseType::Ok);
    dialog.connect_response(|dialog, _response_id| {
        dialog.close();
    });

    // Show the dialog
    dialog.show();
}

/// Create a section with a title and a grid of shortcuts
fn create_shortcut_section(title: &str, shortcuts: &[(&str, &str)]) -> GtkBox {
    let section_box = GtkBox::new(Orientation::Vertical, 8);

    // Section title
    let title_label = Label::new(Some(title));
    title_label.set_halign(gtk4::Align::Start);
    title_label.add_css_class("heading");
    title_label.set_markup(&format!("<b>{}</b>", title));
    section_box.append(&title_label);

    // Shortcuts grid
    let grid = Grid::new();
    grid.set_row_spacing(4);
    grid.set_column_spacing(16);
    grid.set_margin_start(12);

    for (row, (key, description)) in shortcuts.iter().enumerate() {
        // Key label with monospace styling
        let key_label = Label::new(Some(key));
        key_label.set_halign(gtk4::Align::Start);
        key_label.set_markup(&format!("<tt>{}</tt>", key));
        key_label.set_width_chars(16);
        grid.attach(&key_label, 0, row as i32, 1, 1);

        // Description label
        let desc_label = Label::new(Some(description));
        desc_label.set_halign(gtk4::Align::Start);
        grid.attach(&desc_label, 1, row as i32, 1, 1);
    }

    section_box.append(&grid);
    section_box
}
