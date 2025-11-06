//! User manual dialog for the application

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Dialog, Label, Orientation, ScrolledWindow, Window};

/// Show the User Manual dialog
pub fn show_user_manual_dialog(parent: &Window) {
    let dialog = Dialog::builder()
        .title("ユーザーマニュアル - Testruct Studio")
        .transient_for(parent)
        .modal(true)
        .default_width(800)
        .default_height(600)
        .build();

    let content_box = GtkBox::new(Orientation::Vertical, 12);
    content_box.set_margin_start(16);
    content_box.set_margin_end(16);
    content_box.set_margin_top(16);
    content_box.set_margin_bottom(16);

    // Manual content
    let manual_text = r#"Testruct Studio ユーザーマニュアル

【基本操作】

1. キャンバス操作
   - ドラッグでズーム範囲の移動
   - スクロールホイールでズーム
   - 右クリックでコンテキストメニュー

2. 図形の作成
   - 選択: ツールバーから「選択」を選択
   - 矩形: 「矩形」ボタンをクリック後、ドラッグ
   - 円: 「円」ボタンをクリック後、ドラッグ
   - 直線: 「直線」ボタンをクリック後、ドラッグ
   - 矢印: 「矢印」ボタンをクリック後、ドラッグ
   - テキスト: 「テキスト」ボタンをクリック後、クリック

3. オブジェクト操作
   - クリック: オブジェクトを選択
   - ドラッグ: 選択オブジェクトを移動
   - ハンドルドラッグ: オブジェクトをリサイズ
   - Shift + クリック: 複数選択
   - Ctrl + A: すべて選択

4. ビューメニュー
   - F8: グリッド表示/非表示
   - F7: ガイド表示/非表示
   - F6: ルーラー表示/非表示

5. ファイル操作
   - Ctrl + N: 新規作成
   - Ctrl + O: ファイルを開く
   - Ctrl + S: 保存
   - Ctrl + Shift + S: 別名保存

6. 編集操作
   - Ctrl + Z: 取り消し
   - Ctrl + Shift + Z: やり直し
   - Ctrl + I: 画像を挿入

【詳細機能の使用方法は、メニューのヘルプを参照してください】"#;

    let label = Label::new(Some(manual_text));
    label.set_wrap(true);
    label.set_selectable(true);
    label.set_halign(gtk4::Align::Start);
    label.set_valign(gtk4::Align::Start);

    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&label));
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);

    content_box.append(&scrolled);

    let content_area = dialog.content_area();
    content_area.append(&content_box);

    // Add close button
    dialog.add_button("閉じる", gtk4::ResponseType::Ok);
    dialog.connect_response(|dialog, _response_id| {
        dialog.close();
    });

    // Show the dialog
    dialog.show();
}
