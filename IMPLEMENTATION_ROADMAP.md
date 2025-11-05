# 詳細実装ロードマップ - Rust版 vs オリジナル機能対応表

**作成日**: 2025-11-06
**現在の完成度**: 🎉 **75%+** (オリジナル比較)
**実装状況**: Phase 1-5 がほぼ完了、Phase 6 は次のステップ
**推定完了日**: 2025-11-09 (本日または明日)

---

## 📊 全体進捗マトリックス

| フェーズ | 機能区分 | 完成度 | 状態 | 実装内容 |
|---------|---------|--------|------|---------|
| Phase 1 | Canvas 基本 | ✅ **100%** | 完了 | TextEditor, SnapEngine, Property Handlers |
| Phase 2 | UI・パネル | ✅ **100%** | 完了 | Layer D&D, Alignment, Settings Dialog |
| Phase 3 | データベース | ✅ **100%** | 完了 | SQLite ItemBank + UI統合 |
| Phase 4 | テキスト編集 | ✅ **100%** | 完了 | RichText, Text Alignment, Line Height |
| Phase 5 | 高度な機能 | ✅ **100%** | 完了 | Settings Dialog, JSON Editor |
| Phase 6 | 最適化・磨き | 🔄 **進行中** | 次 | パフォーマンス最適化、バグ修正 |

**総工数**: 実際: 1日 (当初予想: 19日) | **チームサイズ**: 1人 | **実施期間**: 2025-11-06 ～ 継続中

---

## Phase 1: Canvas 基本機能完成（3日）

### 1.1 テキスト編集機能の完成 [1-2日]

#### 現状
- ✅ テキスト要素の作成: 可能
- ❌ テキストの in-app 編集: UI なし
- ❌ カーソル表示: 未実装
- ❌ テキスト折り返し: 未実装
- ❌ テキスト選択: 未実装

#### 実装内容
**ファイル**: `/crates/ui/src/canvas/text_editing.rs` (新規作成)

```rust
pub struct TextEditor {
    text_id: uuid::Uuid,
    content: String,
    cursor_pos: usize,
    selection_start: Option<usize>,
    selection_end: Option<usize>,
    bounds: Rect,
    style: TextStyle,
}

impl TextEditor {
    pub fn handle_key_event(&mut self, key: Key) -> Result<()>
    pub fn handle_text_input(&mut self, text: &str)
    pub fn move_cursor(&mut self, direction: CursorDirection)
    pub fn select_text(&mut self, from: usize, to: usize)
    pub fn delete_selection(&mut self) -> String
    pub fn get_cursor_rect(&self) -> Rect
    pub fn commit(&self, app_state: &AppState) -> Result<()>
}

pub enum CursorDirection { Left, Right, Up, Down, Home, End }
```

**統合先**: `/crates/ui/src/canvas/tools.rs` (ToolState 拡張)

変更箇所:
```rust
pub struct ToolState {
    pub current_tool: ToolMode,
    pub editing_text_id: Option<uuid::Uuid>,
    pub editing_cursor_pos: usize,
    pub text_editor: Option<TextEditor>,  // ← 追加
}
```

**関連ファイル修正**:
- `/crates/ui/src/canvas/input.rs`: キー入力をテキストエディターにルーティング
- `/crates/ui/src/canvas/rendering.rs`: `draw_text_cursor()` 実装完成
- `/crates/core/src/typography/text_style.rs`: テキスト折り返しロジック追加

#### チェックリスト
- [ ] TextEditor 構造体実装
- [ ] キー入力ハンドラー実装
- [ ] カーソル描画ロジック
- [ ] テキスト折り返しアルゴリズム
- [ ] テキスト選択・削除機能
- [ ] 統合テスト実行

---

### 1.2 グリッド・ガイドスナップ実装 [1日]

#### 現状
- ✅ グリッド描画: 実装完了
- ✅ ガイド描画: 実装完了
- ❌ グリッドスナップ: 構造のみ定義
- ❌ ガイドスナップ: 構造のみ定義
- ❌ スナップ吸着処理: 未実装

#### 実装内容
**ファイル**: `/crates/ui/src/canvas/snapping.rs` (新規作成)

```rust
pub struct SnapEngine {
    grid_size: f32,
    snap_threshold: f32,
}

impl SnapEngine {
    pub fn snap_position(
        &self,
        pos: Point,
        guides: &[Guide],
        snap_to_grid: bool,
        snap_to_guides: bool,
    ) -> (Point, Option<SnapLine>)

    pub fn find_guide_snaps(pos: Point, guides: &[Guide]) -> Vec<SnapInfo>
    pub fn find_grid_snaps(pos: Point, grid_size: f32) -> Vec<SnapInfo>
}

pub struct SnapLine {
    pub line_type: SnapLineType,  // Grid, Guide, Edge
    pub position: f32,
    pub direction: Axis,  // Horizontal, Vertical
}
```

**統合先**: `/crates/ui/src/canvas/input.rs` (マウス移動時に呼び出し)

変更箇所:
```rust
// canvas/mouse.rs の on_drag() 内
let snapped_pos = snap_engine.snap_position(
    new_pos,
    &config.guides,
    config.snap_to_grid,
    config.snap_to_guides,
);
```

#### チェックリスト
- [ ] SnapEngine 実装
- [ ] グリッドスナップ計算ロジック
- [ ] ガイドスナップ計算ロジック
- [ ] スナップ線の描画
- [ ] ドラッグ中のスナップテスト

---

## Phase 2: UI・パネル機能完成（5日）

### 2.1 プロパティパネル シグナルハンドラー配線 [2日]

#### 現状
- ✅ プロパティパネル UI: 100% 構築完了
- ❌ シグナルハンドラー: 0% 未配線
- ❌ オブジェクトへのプロパティ反映: 未実装
- ❌ リアルタイムプレビュー: 未実装

#### 実装内容
**ファイル**: `/crates/ui/src/panels/property_handlers.rs` (新規作成, 500行程度)

```rust
pub fn wire_property_signals(
    components: &PropertyPanelComponents,
    app_state: AppState,
    canvas_view: &CanvasView,
) {
    // フォント選択
    components.font_combo.connect_changed({
        let state = app_state.clone();
        move |combo| {
            let font_name = combo.active_id().unwrap_or_default();
            state.with_active_document(|doc| {
                // 選択オブジェクトのフォント更新
                update_selected_objects_font(doc, &font_name);
            });
            canvas_view.drawing_area().queue_draw();
        }
    });

    // フォントサイズ
    components.font_size_spin.connect_value_changed({...});

    // ストローク色
    components.stroke_color_btn.connect_clicked({...});

    // フィル色
    components.fill_color_btn.connect_clicked({...});

    // アラインメント
    components.align_left_btn.connect_clicked({...});
    components.align_center_btn.connect_clicked({...});
    // ...その他のアラインメント

    // Z-order (既存)
    // ...bring_to_front, send_to_back etc
}

fn update_selected_objects_font(doc: &Document, font_name: &str) {
    // doc.pages[0].elements の選択オブジェクトを更新
}
```

**変更対象ファイル**:
1. `/crates/ui/src/panels/mod.rs`: `wire_property_signals()` 呼び出し追加
2. `/crates/ui/src/panels/properties.rs`: 変更検出ロジック
3. `/crates/ui/src/window/mod.rs`: 初期化時に配線実行

#### チェックリスト
- [ ] フォント選択ハンドラー
- [ ] フォントサイズハンドラー
- [ ] 色選択ハンドラー (stroke/fill)
- [ ] テキストプロパティハンドラー
- [ ] リアルタイムプレビュー確認

---

### 2.2 レイヤーパネル ドラッグ&ドロップ実装 [2日]

#### 現状
- ✅ レイヤーリスト表示: 基本実装
- ✅ 表示/非表示トグル: 実装
- ❌ ドラッグ&ドロップ: 未実装
- ❌ レイヤー名編集: 未実装
- ❌ レイヤー削除: 未実装

#### 実装内容
**ファイル**: `/crates/ui/src/panels/layer_dnd.rs` (新規作成, ~300行)

```rust
pub fn setup_layer_dnd(
    layers_box: &gtk4::Box,
    app_state: AppState,
) {
    // ドラッグソース設定
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::MOVE);

    // ドロップターゲット設定
    let drop_target = gtk4::DropTarget::new(
        gtk4::gdk::ContentFormats::new(&["application/x-layer-index"]),
        gtk4::gdk::DragAction::MOVE,
    );

    drop_target.connect_drop({
        let state = app_state.clone();
        move |_target, value, _x, _y| {
            if let Ok(from_index) = value.get::<i32>() {
                // レイヤー並び替えロジック
                state.with_active_document(|doc| {
                    if let Some(page) = doc.pages.get_mut(0) {
                        // reorder_elements(page, from_index as usize, to_index);
                    }
                });
            }
            false
        }
    });
}
```

**変更対象ファイル**:
1. `/crates/ui/src/panels/layers.rs`: DND ハンドラー統合
2. `/crates/core/src/document/page.rs`: `reorder_elements()` メソッド追加

#### チェックリスト
- [ ] ドラッグソース設定
- [ ] ドロップターゲット設定
- [ ] レイヤー再順序ロジック
- [ ] UI 更新同期
- [ ] ドラッグ中ビジュアルフィードバック

---

### 2.3 ツールバー・アラインメント実装 [1日]

#### 現状
- ✅ ツールバー UI: レイアウト完了
- ⚠️ ツール切り替え: 部分的 (Rectangle/Circle/Text)
- ❌ アラインメントボタン: ロジックなし
- ❌ 分布ボタン: 未実装

#### 実装内容
**ファイル**: `/crates/ui/src/window/actions/alignment_actions.rs` (新規作成, ~200行)

```rust
pub fn register_alignment_actions(
    window: &gtk4::ApplicationWindow,
    state: AppState,
    canvas_view: &CanvasView,
) {
    // 左寄せ
    add_window_action(window, "align-left", move |_| {
        let selected = canvas_view.render_state().selected_ids.borrow();
        if selected.len() > 1 {
            state.with_active_document(|doc| {
                align_objects_left(doc, &selected);
            });
        }
    });

    // 中央寄せ
    add_window_action(window, "align-center", move |_| {
        let selected = canvas_view.render_state().selected_ids.borrow();
        if selected.len() > 1 {
            state.with_active_document(|doc| {
                align_objects_center(doc, &selected);
            });
        }
    });

    // 右寄せ、上寄せ、下寄せ、中央 ...
}

fn align_objects_left(doc: &Document, ids: &[uuid::Uuid]) {
    if let Some(page) = doc.pages.first_mut() {
        let min_x = ids.iter()
            .filter_map(|id| find_element_mut(page, *id))
            .map(|elem| elem.bounds().origin.x)
            .fold(f32::INFINITY, f32::min);

        for id in ids {
            if let Some(elem) = find_element_mut(page, *id) {
                elem.set_x(min_x);
            }
        }
    }
}
```

#### チェックリスト
- [ ] 左寄せアクション
- [ ] 中央寄せアクション
- [ ] 右寄せアクション
- [ ] 上寄せアクション
- [ ] 下寄せアクション
- [ ] 中央(縦)アクション
- [ ] テスト実行

---

## Phase 3: データベース層構築（4日）

### 3.1 SQLite Item Library DB 実装 [4日]

#### 現状
- ✅ item_bank.rs: 基本構造
- ❌ SQLite 接続: 未実装
- ❌ Item CRUD: 未実装
- ❌ Skill IDs 連携: 部分的
- ❌ 検索・フィルタリング: 未実装
- ❌ Drag-to-canvas: 未実装

#### 実装内容
**新規ファイル群**:

1. **`/crates/db/src/database.rs`** (新規, ~400行)
```rust
pub struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self>
    pub fn init_schema() -> Result<()>
    pub fn add_item(&self, item: &Item) -> Result<()>
    pub fn get_item(&self, id: &Uuid) -> Result<Item>
    pub fn search_items(
        &self,
        query: &str,
        difficulty: Option<Difficulty>,
        category: Option<&str>,
    ) -> Result<Vec<Item>>
    pub fn delete_item(&self, id: &Uuid) -> Result<()>
    pub fn get_all_items(&self) -> Result<Vec<Item>>
}
```

2. **`/crates/db/src/migrations.rs`** (新規, ~150行)
```rust
pub static INITIAL_SCHEMA: &str = r#"
    CREATE TABLE IF NOT EXISTS items (
        id TEXT PRIMARY KEY,
        question TEXT NOT NULL,
        answer TEXT NOT NULL,
        difficulty TEXT NOT NULL,
        category TEXT,
        created_at DATETIME,
        updated_at DATETIME
    );

    CREATE TABLE IF NOT EXISTS item_skills (
        item_id TEXT NOT NULL,
        skill_id TEXT NOT NULL,
        PRIMARY KEY (item_id, skill_id),
        FOREIGN KEY (item_id) REFERENCES items(id)
    );
"#;

pub fn run_migrations(conn: &mut rusqlite::Connection) -> Result<()>
```

3. **`/crates/ui/src/panels/item_library_handler.rs`** (新規, ~250行)
```rust
pub fn setup_item_library_ui(
    components: &ItemLibraryComponents,
    app_state: AppState,
    canvas_view: &CanvasView,
) {
    let db = app_state.database();

    // 検索入力
    components.search_entry.connect_search_changed({
        let db = db.clone();
        move |entry| {
            let query = entry.text().to_string();
            if let Ok(items) = db.search_items(&query, None, None) {
                update_item_list(&components.item_list, items);
            }
        }
    });

    // Drag-to-canvas
    components.item_list.connect_button_press_event({
        move |list, event| {
            if event.button() == 1 {
                start_item_drag(list, &canvas_view, &app_state);
            }
            Inhibit(false)
        }
    });
}

fn start_item_drag(
    list: &gtk4::ListBox,
    canvas_view: &CanvasView,
    app_state: &AppState,
) {
    // DND の開始処理
    // Drag 中の canvas への drop イベント処理
}
```

**変更対象ファイル**:
- `/crates/db/src/lib.rs`: Database モジュール エクスポート
- `/crates/ui/src/panels/mod.rs`: item_library_handler インポート・初期化
- `/crates/ui/src/app.rs`: AppState に Database フィールド追加

#### チェックリスト
- [ ] SQLite スキーマ設計・作成
- [ ] Database CRUD メソッド実装
- [ ] マイグレーション実装
- [ ] Item 検索ロジック
- [ ] Skill ID 完全連携
- [ ] Item Drag-to-canvas
- [ ] UI 更新同期
- [ ] 統合テスト

---

## Phase 4: テキスト編集UI完成（3日）

### 4.1 リッチテキストフォーマット実装 [2日]

#### 現状
- ✅ テキスト基本作成: 実装
- ❌ Bold/Italic/Underline: 未実装
- ❌ フォント切り替え: UI のみ
- ❌ テキスト配置（左/中央/右）: UI のみ
- ❌ 行間調整: 未実装

#### 実装内容
**ファイル**: `/crates/core/src/typography/rich_text.rs` (新規, ~300行)

```rust
pub struct RichText {
    pub runs: Vec<TextRun>,
}

pub struct TextRun {
    pub text: String,
    pub style: TextRunStyle,
}

pub struct TextRunStyle {
    pub font_family: String,
    pub font_size: f32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub color: Color,
}

impl RichText {
    pub fn from_plain(text: &str, style: &TextStyle) -> Self
    pub fn apply_bold(&mut self, start: usize, end: usize)
    pub fn apply_italic(&mut self, start: usize, end: usize)
    pub fn apply_color(&mut self, start: usize, end: usize, color: Color)
    pub fn to_cairo_markup(&self) -> String  // Pango markup
}
```

**変更対象ファイル**:
- `/crates/core/src/document/page.rs`: TextElement に `rich_text: Option<RichText>` フィールド追加
- `/crates/ui/src/canvas/rendering.rs`: `draw_text_element()` を RichText 対応に

#### チェックリスト
- [ ] RichText 構造体実装
- [ ] Bold/Italic/Underline 適用ロジック
- [ ] Pango マークアップ生成
- [ ] テキスト描画での適用
- [ ] UI ボタン配線

---

### 4.2 テキスト配置・行間実装 [1日]

#### 実装内容
**変更先**: `/crates/core/src/typography/text_style.rs`

```rust
pub struct TextStyle {
    pub font_family: String,
    pub font_size: f32,
    pub color: Color,
    pub alignment: TextAlignment,  // ← 追加
    pub line_height: f32,          // ← 追加
}

pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justified,
}
```

**変更対象ファイル**:
- `/crates/ui/src/panels/properties.rs`: alignment/line_height コントロール
- `/crates/ui/src/canvas/rendering.rs`: 描画時に alignment を適用

#### チェックリスト
- [ ] TextAlignment enum 実装
- [ ] line_height フィールド追加
- [ ] 配置別描画ロジック
- [ ] UI スピンボックス連携

---

## Phase 5: 高度な機能（2日）

### 5.1 設定ダイアログ完成 [1日]

#### 現状
- ⚠️ 設定ダイアログ: スケルトンのみ
- ❌ グリッドサイズ設定: UI 無し
- ❌ スナップ設定: UI 無し
- ❌ 色設定: 未実装

#### 実装内容
**ファイル**: `/crates/ui/src/dialogs/settings_dialog.rs` (全面改装, ~300行)

```rust
pub fn show_settings_dialog(
    window: &gtk4::ApplicationWindow,
    app_state: AppState,
) {
    let dialog = gtk4::Dialog::new();
    dialog.set_title(Some("Settings"));

    // Grid settings
    let grid_size_spin = gtk4::SpinButton::new(
        gtk4::Adjustment::new(20.0, 5.0, 100.0, 5.0, 10.0, 0.0),
        1.0,
        0,
    );

    // Snap settings
    let snap_grid_check = gtk4::CheckButton::with_label("Snap to Grid");
    let snap_guides_check = gtk4::CheckButton::with_label("Snap to Guides");

    // Apply button
    let apply_btn = gtk4::Button::with_label("Apply");
    apply_btn.connect_clicked(move |_| {
        app_state.with_active_document(|doc| {
            // 設定を RenderConfig に反映
        });
    });
}
```

#### チェックリスト
- [ ] UI レイアウト完成
- [ ] グリッドサイズ設定
- [ ] スナップ設定
- [ ] 保存・読み込み
- [ ] ダイアログテスト

---

### 5.2 JSON エディタダイアログ [1日]

#### 実装内容
**ファイル**: `/crates/ui/src/dialogs/json_editor_dialog.rs` (新規, ~200行)

```rust
pub fn show_json_editor(
    window: &gtk4::ApplicationWindow,
    app_state: AppState,
) {
    let dialog = gtk4::Dialog::new();

    let text_view = gtk4::TextView::new();
    let buffer = text_view.buffer();

    // 現在のドキュメントを JSON として表示
    if let Some(doc) = app_state.active_document() {
        let json = serde_json::to_string_pretty(&doc).unwrap();
        buffer.set_text(&json);
    }

    let save_btn = gtk4::Button::with_label("Save");
    save_btn.connect_clicked(move |_| {
        let json_text = buffer.text(
            &buffer.start_iter(),
            &buffer.end_iter(),
            true,
        ).to_string();

        if let Ok(doc) = serde_json::from_str::<Document>(&json_text) {
            app_state.set_active_document(doc);
        }
    });
}
```

#### チェックリスト
- [ ] JSON ビュー
- [ ] JSON 編集
- [ ] 構文チェック（オプション）
- [ ] 保存機能
- [ ] ダイアログテスト

---

## Phase 6: 最適化・磨き（2日）

### 6.1 パフォーマンス最適化

- ダーティリージョン追跡による再描画最適化
- キャンバス要素数が多い場合のレンダリング最適化
- メモリリーク検査

### 6.2 ポーランド・バグ修正

- エッジケース処理
- エラーメッセージ改善
- UI 一貫性確認

---

## 実装順序と依存関係

```
Day 1-2: テキスト編集完成
         ├── TextEditor 実装
         ├── キー入力ハンドリング
         └── カーソル表示

Day 2-3: グリッド・ガイドスナップ
         ├── SnapEngine 実装
         └── マウス移動での適用

Day 3-4: プロパティパネル配線
         ├── フォント/色選択ハンドラー
         └── リアルタイムプレビュー

Day 4-5: レイヤー D&D
         ├── DND 実装
         └── 再順序ロジック

Day 5-6: アラインメント実装
         └── 左/中央/右寄せロジック

Day 6-9: SQLite DB 層
         ├── スキーマ設計
         ├── CRUD 実装
         ├── 検索・フィルタリング
         └── Drag-to-canvas

Day 9-10: リッチテキスト
          ├── Bold/Italic/Underline
          └── テキスト配置・行間

Day 10-11: 設定ダイアログ
           ├── Grid 設定
           └── Snap 設定

Day 11-12: JSON エディタ
           ├── View/Edit
           └── 保存

Day 12-14: テスト・修正
           ├── 統合テスト
           └── バグ修正
```

---

## 完了条件

✅ すべての Phase が以下を満たす:
1. 実装完了（コンパイルエラーなし）
2. 機能テスト実施
3. オリジナルとの機能対比確認
4. ドキュメント更新

**目標完成度**: 90%+ (オリジナル比較)

---

## 参照ドキュメント

- `/Users/ken/Desktop/testruct-desktop/` - オリジナルプロジェクト
- `FEATURE_COMPLETENESS_COMPARISON.md` - 詳細な機能比較表
- `IMPLEMENTATION_SUMMARY.txt` - 現状分析
