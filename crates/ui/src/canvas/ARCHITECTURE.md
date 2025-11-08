# キャンバスモジュール - アーキテクチャ概要

testruct-studio のキャンバス機能の全体的なアーキテクチャを説明します。

## ディレクトリ構成

```
crates/ui/src/canvas/
├── input/                      # 入力処理（キーボード、マウス、ジェスチャー）
│   ├── keyboard.rs             # テキスト編集、ショートカット処理
│   ├── gesture.rs              # クリック・ドラッグジェスチャー
│   └── mouse.rs                # マウス動作追跡、カーソル管理
│
├── input.rs                    # 入力処理の統合エントリーポイント
├── rendering.rs                # 図形描画、グリッド、ガイド表示
├── shapes_rendering.rs         # 図形（Rectangle、Circle、Line など）の描画
├── grid_rendering.rs           # グリッド表示処理
├── overlays.rs                 # 選択枠、リサイズハンドルなどのオーバーレイ表示
│
├── selection.rs                # オブジェクト選択と Hit test
├── tools.rs                    # ツール定義（Select、Rectangle、Circle など）
├── mouse.rs                    # リサイズハンドル検出、座標計算
│
├── snapping.rs                 # グリッド/ガイドスナップ処理
├── alignment.rs                # オブジェクト配置・分散機能
├── text_editor.rs              # テキスト編集機能（カーソル、選択など）
│
├── keyboard.rs                 # キーボードコマンド定義（列挙型）
├── dirty_region.rs             # 更新領域の追跡（最適化用）
│
├── mod.rs                      # キャンバスモジュール統合
├── INPUT_MODULE.md             # 入力モジュールの詳細ドキュメント
└── ARCHITECTURE.md             # このファイル
```

## 主要モジュールの役割

### 1. 入力処理 (input/)
**責務**: ユーザーのキーボード・マウス入力を処理

```
┌─────────────────────────────┐
│   GTK イベント              │
├─────────────────────────────┤
│ EventControllerKey          │ ──→ keyboard.rs
│ EventControllerMotion       │ ──→ mouse.rs
│ GestureClick               │ ──→ gesture.rs
│ GestureDrag                │ ──→ gesture.rs
└─────────────────────────────┘
        ↓
┌─────────────────────────────┐
│   状態更新                  │
│  (selected_ids,             │
│   tool_state, drag_box)     │
└─────────────────────────────┘
        ↓
┌─────────────────────────────┐
│   ドキュメント更新          │
│  (with_mutable_active_      │
│   document)                 │
└─────────────────────────────┘
```

詳細は [INPUT_MODULE.md](./INPUT_MODULE.md) を参照。

### 2. 描画処理 (rendering.rs, shapes_rendering.rs など)
**責務**: キャンバス描画の実装

```
┌─────────────────────────────┐
│   draw_canvas()             │
│   (Cairo context)           │
├─────────────────────────────┤
│ 1. グリッド描画             │ ──→ grid_rendering.rs
│ 2. ドキュメント要素描画      │ ──→ shapes_rendering.rs
│ 3. テキスト描画             │ ──→ shapes_rendering.rs
│ 4. 画像描画                 │ ──→ shapes_rendering.rs
│ 5. オーバーレイ描画         │ ──→ overlays.rs
│    ├─ 選択枠
│    ├─ リサイズハンドル
│    ├─ ガイド
│    └─ ドラッグボックス
└─────────────────────────────┘
```

### 3. 選択・Hit Test (selection.rs)
**責務**: オブジェクト選択判定

```
マウスクリック位置
        ↓
┌─────────────────┐
│  Hit Test       │
├─────────────────┤
│ 1. ドキュメント座標に変換
│ 2. 全オブジェクトの AABB と比較
│ 3. マウス位置と交差するか判定
└─────────────────┘
        ↓
選択されたオブジェクト ID
```

### 4. ツール定義 (tools.rs)
**責務**: ツール種別と図形生成

```
ToolMode
├─ Select       : オブジェクト選択・編集
├─ Rectangle    : 矩形作成
├─ Circle       : 円作成
├─ Line         : 直線作成
├─ Arrow        : 矢印作成
├─ Text         : テキスト要素作成
└─ Image        : 画像要素作成

ShapeFactory
├─ create_rectangle()
├─ create_circle()
├─ create_line()
├─ create_arrow()
├─ create_text()
└─ create_image()
```

### 5. スナップ処理 (snapping.rs)
**責務**: グリッド/ガイドへのスナップ

```
移動/リサイズ操作
        ↓
┌──────────────────┐
│ snap_to_grid()   │ (グリッド有効時)
├──────────────────┤
│ 座標をグリッド間隔で
│ 丸める
└──────────────────┘
        ↓
┌──────────────────┐
│ snap_to_guide()  │ (ガイド有効時)
├──────────────────┤
│ ガイドに近ければ
│ スナップ
└──────────────────┘
        ↓
スナップ後の座標
```

### 6. 配置機能 (alignment.rs)
**責務**: オブジェクト配置と分散

```
選択オブジェクト群
        ↓
┌──────────────────┐
│ 配置機能         │
├──────────────────┤
│ - align_left()
│ - align_right()
│ - align_center_h()
│ - align_top()
│ - align_bottom()
│ - align_middle()
│
│ 分散機能
│ - distribute_h_equal()
│ - distribute_v_equal()
└──────────────────┘
        ↓
配置後の座標
```

## ステートフロー

### キャンバス状態構造

```rust
pub struct CanvasRenderState {
    // ツール状態
    pub tool_state: RefCell<ToolState>,
        ├─ current_tool: ToolMode
        ├─ editing_text_id: Option<Uuid>
        ├─ editing_cursor_pos: usize
        ├─ drag_start: Option<(f64, f64)>
        ├─ resizing_object_id: Option<Uuid>
        ├─ resize_handle: Option<ResizeHandle>
        └─ resize_original_bounds: Option<CanvasMousePos>

    // 選択状態
    pub selected_ids: RefCell<Vec<Uuid>>

    // ドラッグ状態
    pub drag_box: RefCell<Option<Rect>>

    // レンダリング設定
    pub config: RefCell<RenderConfig>
        ├─ zoom: f64
        ├─ pan_x: f64
        ├─ pan_y: f64
        ├─ snap_to_grid: bool
        ├─ grid_spacing: f32
        ├─ snap_to_guides: bool
        └─ guides: Vec<Guide>

    // UI設定
    pub ruler_config: RefCell<RulerConfig>
    pub show_grid: bool
    pub show_guides: bool
}
```

## イベント処理フロー

### キーボード入力フロー

```
EventControllerKey::key_pressed
    ↓
keyboard::setup_keyboard_events の closure
    ├─ 修飾キー判定 (Shift, Ctrl)
    ├─ テキスト編集モード判定
    │
    ├─ [テキスト編集中の場合]
    │   ├─ Escape: 編集終了
    │   ├─ BackSpace/Delete: 文字削除
    │   ├─ 矢印キー: カーソル移動
    │   └─ 通常文字: 文字挿入
    │
    ├─ [ショートカットの場合]
    │   ├─ Ctrl+C: copy_to_clipboard()
    │   ├─ Ctrl+X: cut (削除後にコピー)
    │   ├─ Ctrl+V: paste_from_clipboard()
    │   ├─ Ctrl+D: duplicate
    │   ├─ Ctrl+Shift+I: insert_image()
    │   └─ Ctrl+Shift+S: save_template()
    │
    └─ [オブジェクト操作の場合]
        ├─ 矢印キー: move_selected_objects()
        └─ テキスト配置: Ctrl+[LRCJ]

    ↓

app_state.with_mutable_active_document() で
ドキュメントに変更を書き込み

    ↓

drawing_area.queue_draw() でキャンバス再描画要求
```

### マウスクリックフロー

```
GestureClick::pressed
    ↓
修飾キー判定 (Shift, Ctrl)
    ├─ 通常クリック: 単一選択
    ├─ Shift+クリック: 選択に追加
    └─ Ctrl+クリック: トグル選択

    ↓

Hit test 実行
(マウス位置のオブジェクトを検出)
    ├─ [ヒット]
    │   └─ selected_ids に追加
    │
    └─ [ヒットなし]
        └─ selected_ids をクリア

    ↓

drawing_area.queue_draw()
```

### ドラッグフロー

```
GestureDrag::drag_begin
    ↓
tool_state.drag_start に座標保存

GestureDrag::drag_update (繰り返し)
    ↓
drag_box を更新（プレビュー表示用）
    ↓
drawing_area.queue_draw()

GestureDrag::drag_end
    ↓
操作タイプ判定
├─ リサイズ?
│   └─ calculate_resize_bounds()
│      → apply to document
│
├─ Select ツールで移動?
│   └─ オブジェクト座標更新
│      → apply to document
│
└─ 図形作成?
    └─ ShapeFactory.create_*()
       → add to document

    ↓

ドラッグ状態をクリア
    ↓
drawing_area.queue_draw()
```

## パフォーマンス最適化

### 1. 更新領域管理
- `dirty_region.rs` で再描画が必要な領域を追跡
- 全キャンバスではなく変更部分のみ更新

### 2. 高頻度イベント処理
- マウス動作 (`motion`): 最小限の処理（リサイズハンドル検出のみ）
- ドラッグ更新: `drag_box` キャッシュで再計算を最小化

### 3. 状態管理
- `RefCell` で複数所有権問題を回避
- 各イベントハンドラーで短く状態を保持

## データフロー

```
┌──────────────┐
│ GTK イベント │
└──────┬───────┘
       │
       ↓
┌──────────────────────────────────┐
│   input/ モジュール処理           │
│  (keyboard, gesture, mouse)      │
├──────────────────────────────────┤
│ ・状態更新 (render_state)        │
│ ・ドキュメント更新               │
│ ・キャンバス再描画要求           │
└──────┬───────────────────────────┘
       │
       ↓
┌──────────────────────────────────┐
│   rendering モジュール            │
│  (draw_canvas)                   │
├──────────────────────────────────┤
│ ・グリッド描画                   │
│ ・図形描画                       │
│ ・テキスト描画                   │
│ ・オーバーレイ描画               │
└──────┬───────────────────────────┘
       │
       ↓
┌──────────────────────────────────┐
│   GTK Canvas (Cairo描画)         │
└──────────────────────────────────┘
```

## 拡張パターン

### 新しいキーボードショートカットを追加

1. `input/keyboard.rs` の `setup_keyboard_events` に条件を追加
2. 必要に応じて新しい関数を作成
3. `app_state.with_mutable_active_document()` でドキュメント更新

### 新しいツール（図形タイプ）を追加

1. `ToolMode` に新しい列挙値を追加
2. `tools.rs` に `ShapeFactory::create_*()` メソッドを追加
3. `gesture.rs` の `drag_end` で新しいツール処理を追加

### 新しい描画機能を追加

1. 必要に応じて `shapes_rendering.rs` に描画関数を追加
2. `rendering.rs` の `draw_canvas` で呼び出し
3. 後処理（スナップ計算など）が必要なら別モジュール作成

## テストアプローチ

各モジュールは以下のシナリオでテスト：

- **keyboard**: キー入力、ショートカット実行、文字挿入
- **gesture**: オブジェクト選択、リサイズ、図形作成
- **mouse**: カーソル変更、ハンドル検出
- **selection**: Hit test 精度
- **snapping**: グリッド/ガイドスナップ
- **alignment**: オブジェクト配置結果

## 関連ドキュメント

- [INPUT_MODULE.md](./INPUT_MODULE.md) - 入力モジュール詳細
- `keyboard.rs` の doc comment - キーボード処理詳細
- `gesture.rs` の doc comment - ジェスチャー処理詳細
- `mouse.rs` の doc comment - マウス追跡詳細

## バージョン履歴

### v1.1 (2025-11-08)
- 包括的なドキュメント整備
- アーキテクチャ概要作成
- モジュール間の依存関係を明示

### v1.0 (2025-11-08)
- 入力モジュールのリファクタリング完了
- keyboard.rs: キーボード処理
- gesture.rs: ジェスチャー処理
- mouse.rs: マウス追跡
