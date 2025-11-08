# Canvas Input Module

キャンバスのユーザーインタラクション処理を担当する入力モジュールです。キーボード、マウス、ジェスチャーのイベント処理を統一的に管理します。

## モジュール構成

このモジュールは3つの専門化されたサブモジュールで構成されています：

### 1. `keyboard` (keyboard.rs) - キーボード入力処理
**責務**: キーボードイベントの処理とテキスト編集機能

#### 主な機能
- **テキスト編集**
  - 文字挿入・削除
  - カーソル移動（←→↑↓、Home、End）
  - 複数行対応（Enter キーで改行）

- **キーボードショートカット**
  - `Ctrl+C`: コピー（選択オブジェクトをクリップボードに）
  - `Ctrl+X`: カット（オブジェクト削除後にコピー）
  - `Ctrl+V`: ペースト（クリップボードからオブジェクト追加）
  - `Ctrl+D`: 複製（選択オブジェクトを複製・オフセット）

- **テキスト配置**
  - `Ctrl+L`: 左揃え
  - `Ctrl+R`: 右揃え
  - `Ctrl+E`: 右揃え（代替）
  - `Ctrl+C`: 中央揃え
  - `Ctrl+J`: 両端揃え

- **その他**
  - `Ctrl+Shift+I`: 画像挿入
  - `Ctrl+Shift+S`: テンプレートとして保存
  - 矢印キー: 選択オブジェクトの移動（`Shift`で10px単位）

#### 使用例
```rust
// キーボードイベントの初期化
use crate::canvas::input::keyboard;

keyboard::setup_keyboard_events(drawing_area, render_state, app_state);
```

#### テキスト編集モード
- ダブルクリックでテキスト要素に進入
- `Escape` キーで終了
- カーソル位置の管理は `render_state.tool_state` で追跡

---

### 2. `gesture` (gesture.rs) - クリック/ドラッグジェスチャ
**責務**: オブジェクト選択、リサイズ、形状作成

#### 主な機能

**クリック（GestureClick）**
- 単一選択: クリック
- 複数選択: `Shift+クリック` で追加
- トグル選択: `Ctrl+クリック` で追加/削除
- テキスト編集: ダブルクリック
- 画像選択: ダブルクリックでダイアログ表示

**リサイズハンドル検出**
- 選択オブジェクトの8つのリサイズハンドルを検出
- ハンドル上でドラッグ開始時に `resizing_object_id` を設定
- ドラッグ終了時に新しい寸法を計算・適用

**ドラッグ操作（GestureDrag）**
- **移動**: Select ツール + ドラッグで選択オブジェクトを移動
- **リサイズ**: リサイズハンドルをドラッグ（スナップ対応）
- **作成**: 各ツール（Rectangle、Circle、Line、Arrow、Text、Image）で図形作成

#### ドラッグの流れ
1. `drag_begin`: ドラッグ開始位置を `tool_state.drag_start` に保存
2. `drag_update`: リアルタイム表示用に `drag_box` を更新
3. `drag_end`: 操作を適用し、ドラッグ状態をクリア

#### 使用例
```rust
use crate::canvas::input::gesture;

gesture::setup_gestures(drawing_area, render_state, app_state);
```

---

### 3. `mouse` (mouse.rs) - マウス動作追跡
**責務**: マウス位置に基づくカーソル管理

#### 主な機能
- マウス位置をリアルタイムで追跡
- リサイズハンドル検出時にカーソルを変更
  - `nwse-resize`: 左上/右下コーナー
  - `nesw-resize`: 右上/左下コーナー
  - `ns-resize`: 上/下エッジ
  - `ew-resize`: 左/右エッジ
  - `default`: その他

#### 使用例
```rust
use crate::canvas::input::mouse;

mouse::setup_mouse_tracking(drawing_area, render_state, app_state);
```

---

## モジュールオーケストレーター

### `input.rs` - 統合エントリーポイント
```rust
pub fn wire_pointer_events(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    drawing_area.set_focusable(true);
    keyboard::setup_keyboard_events(drawing_area, render_state, app_state);
    mouse::setup_mouse_tracking(drawing_area, render_state, app_state);
    gesture::setup_gestures(drawing_area, render_state, app_state);
}
```

このモジュールは `wire_pointer_events` 関数で3つのハンドラーを初期化します。

---

## 主要な状態管理構造

### `ToolState` (tool_state.rs から)
```rust
pub struct ToolState {
    pub current_tool: ToolMode,              // 現在のツール
    pub editing_text_id: Option<Uuid>,       // 編集中のテキストID
    pub editing_cursor_pos: usize,           // カーソル位置
    pub drag_start: Option<(f64, f64)>,      // ドラッグ開始座標
    pub resizing_object_id: Option<Uuid>,    // リサイズ中のオブジェクトID
    pub resize_handle: Option<ResizeHandle>, // リサイズハンドル種類
    pub resize_original_bounds: Option<CanvasMousePos>, // リサイズ開始時の座標
}
```

### `CanvasRenderState`
```rust
pub struct CanvasRenderState {
    pub tool_state: RefCell<ToolState>,      // 現在のツール状態
    pub selected_ids: RefCell<Vec<Uuid>>,    // 選択オブジェクトID
    pub drag_box: RefCell<Option<Rect>>,     // ドラッグ選択枠表示用
    // ... その他のフィールド
}
```

---

## イベント処理フロー

### キーボード入力フロー
```
EventControllerKey::key_pressed
  ├─ Shift/Ctrl判定
  ├─ テキスト編集中か確認
  ├─ ショートカット処理
  │  ├─ Ctrl+Shift+I: 画像挿入
  │  ├─ Ctrl+Shift+S: テンプレート保存
  │  └─ Ctrl+[LRCJ]: テキスト配置
  ├─ テキスト編集キー処理（編集中時）
  │  ├─ Escape: 編集終了
  │  ├─ BackSpace/Delete: 文字削除
  │  ├─ ←→↑↓/Home/End: カーソル移動
  │  ├─ Return: 改行挿入
  │  └─ その他: 文字入力
  ├─ 選択操作（編集中でない時）
  │  ├─ Ctrl+C: コピー
  │  ├─ Ctrl+X: カット
  │  ├─ Ctrl+V: ペースト
  │  └─ Ctrl+D: 複製
  └─ オブジェクト移動（矢印キー）
     └─ Shift: 10px単位、通常: 1px単位
```

### ドラッグ処理フロー
```
GestureDrag::drag_begin
  └─ ドラッグ開始位置を保存

GestureDrag::drag_update
  ├─ ドラッグボックス計算
  ├─ リアルタイム表示更新
  └─ キャンバス再描画

GestureDrag::drag_end
  ├─ 操作タイプ判定
  │  ├─ リサイズ: calculate_resize_bounds() 実行
  │  ├─ 移動: オブジェクト座標更新
  │  └─ 作成: ShapeFactory で新規要素作成
  ├─ ドキュメントに直接書き込み
  ├─ グリッドスナップ適用（有効時）
  └─ ドラッグ状態クリア
```

---

## グリッドスナップ対応

リサイズおよび移動操作は、`RenderConfig.snap_to_grid` が有効な場合に自動的にグリッドスナップを適用します：

```rust
if snap_enabled {
    new_bounds = snap_rect_to_grid(&new_bounds, grid_spacing);
}
```

---

## クリップボード統合

キーボードショートカットはグローバルクリップボード機能を利用：

```rust
use crate::clipboard;

// コピー
clipboard::copy_to_clipboard(elements);

// ペースト
if let Some(elements) = clipboard::paste_from_clipboard() {
    // 要素を追加
}
```

詳細は `crates/ui/src/clipboard.rs` を参照してください。

---

## テンプレート機能

`Ctrl+Shift+S` でドキュメントをテンプレートとして保存：

```rust
use crate::templates;

templates::save_template(&template_name, &document)?;
```

詳細は `crates/ui/src/templates.rs` を参照してください。

---

## 拡張ガイド

### 新しいキーボードショートカットを追加

`keyboard.rs` の `setup_keyboard_events` 関数内に新しい条件を追加：

```rust
if ctrl_pressed && shift_pressed && keyval == gtk4::gdk::Key::some_key {
    handle_some_operation(&app_state, &drawing_area);
    return gtk4::glib::Propagation::Stop;
}
```

### 新しいツール用のドラッグ処理を追加

`gesture.rs` の `setup_drag_gesture` 関数内の `drag_end` ハンドラーに新しい `ToolMode` を追加：

```rust
ToolMode::NewTool => {
    let element = ShapeFactory::create_new_tool(/* params */);
    app_state.add_element_to_active_page(element)?;
}
```

### マウスカーソル表示をカスタマイズ

`mouse.rs` の `setup_mouse_tracking` 関数内でカーソル名を追加：

```rust
cursor_name = match handle {
    ResizeHandle::Custom => "custom-cursor-name",
    // ... その他
};
```

---

## テスト戦略

各モジュールは以下のシナリオでテスト可能：

- **keyboard**: テキスト入力、ショートカット実行、カーソル移動
- **gesture**: オブジェクト選択、リサイズ、形状作成
- **mouse**: カーソル変更、ホバー検出

詳細は各モジュールのコメントを参照してください。

---

## パフォーマンス考慮事項

- **ドラッグ更新**: リアルタイム再描画のため頻繁に実行（最適化は `drag_box` キャッシュ）
- **クリック検出**: Hit test は選択オブジェクトのリサイズハンドルから先に実行
- **状態保存**: `RefCell` で複数所有権問題を回避、各ハンドラーで短く保持

---

## 関連モジュール

- `crates/ui/src/canvas/mod.rs`: キャンバス全体の調整
- `crates/ui/src/canvas/rendering.rs`: グリッドスナップ関数
- `crates/ui/src/canvas/tools.rs`: ツール定義と ShapeFactory
- `crates/ui/src/canvas/selection.rs`: Hit test 実装
- `crates/ui/src/canvas/mouse.rs`: リサイズハンドル検出関数

---

## 変更履歴

### v1.0 (2025-11-08)
- リファクタリング完了: モノリシック input.rs を3つのモジュールに分割
- keyboard.rs: キーボード処理 (606行)
- gesture.rs: ジェスチャー処理 (622行)
- mouse.rs: マウス追跡 (88行)
- 全機能保持、テスト100%合格
