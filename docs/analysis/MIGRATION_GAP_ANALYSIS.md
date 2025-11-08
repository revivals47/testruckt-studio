# 新旧開発環境 Migration Gap Analysis

**Generated**: 2025-11-05
**Status**: 新しいプロジェクトはリファクタリング初期段階（反映率: 4.1%）

---

## Executive Summary

| 項目 | 元のプロジェクト | 新しいプロジェクト | ギャップ |
|------|:---:|:---:|:---:|
| **総Rustファイル数** | 115 | 41 | -74ファイル (64%) |
| **総コード行数** | 31,451 | 1,294 | -30,157行 (95.9%) |
| **Core crate** | 11ファイル, 3,849行 | 16ファイル, ~500行 | ▼ 機能は初期段階 |
| **UI/GTK crate** | 86ファイル, 27,602行 | 20ファイル, ~500行 | ▼ 骨組みのみ |
| **追加crate** | cli, db (2個) | cli のみ (1個) | ▼ db未実装 |

**反映率**: 1,294 / 31,451 = **4.1%** ✗

---

## Part 1: Core Crate の反映状況

### 元のプロジェクト: Core Module 構成
```
crates/core/src/
├─ alignment.rs          (229行) - グリッドスナップ、オブジェクト配置
├─ document.rs           (727行) - Document, Page, Object モデル
├─ error.rs              (41行)  - エラー定義
├─ image_export.rs       (182行) - PNG/JPEG エクスポート
├─ layout.rs             (317行) - レイアウト計算エンジン
├─ paper_size.rs         (154行) - 用紙サイズ定義
├─ pdf.rs                (433行) - PDF エクスポート
├─ svg_export.rs         (394行) - SVG エクスポート
├─ template.rs           (233行) - テンプレート定義
├─ template_manager.rs   (788行) - テンプレート管理システム
└─ typeset.rs            (325行) - テキスト植字エンジン
```

### 新しいプロジェクト: Core Module 構成
```
crates/core/src/
├─ document/
│  ├─ builder.rs         (55行)  - DocumentBuilder パターン ✓
│  ├─ metadata.rs         (33行)  - Document/Page メタデータ
│  ├─ page.rs             (78行)  - Page モデル
│  └─ mod.rs              (66行)  - Document エクスポート
├─ layout/
│  ├─ canvas.rs           (44行)  - CanvasLayout 定義
│  ├─ engine.rs           (37行)  - LayoutEngine スケルトン
│  ├─ geometry.rs         (44行)  - Point, Rect, Size
│  └─ mod.rs              (9行)
├─ template/
│  ├─ definition.rs       (43行)  - Template スケルトン
│  ├─ library.rs          (31行)  - TemplateLibrary
│  ├─ style.rs            (34行)  - TemplateStyle
│  └─ mod.rs              (9行)
├─ typography/
│  ├─ color.rs            (26行)  - Color
│  ├─ font_catalog.rs      (29行)  - FontCatalog
│  ├─ text_style.rs        (41行)  - TextStyle
│  └─ mod.rs              (9行)
├─ workspace/
│  ├─ assets.rs           (47行)  - AssetCatalog
│  ├─ history.rs          (43行)  - CommandHistory スケルトン
│  ├─ project.rs          (66行)  - Project 管理
│  └─ mod.rs              (9行)
└─ lib.rs                 (16行)  - モジュールエクスポート
```

### ✅ 実装済み (Core層)
- ✓ Document/Page ドメインモデル（Uuid ベース）
- ✓ DocumentBuilder パターン
- ✓ Project/ProjectSettings
- ✓ AssetCatalog 基本構造
- ✓ Template/TemplateLibrary スケルトン
- ✓ Typography モジュール（Color, TextStyle, FontCatalog）
- ✓ Layout モジュール（Rect, Point, Size, CanvasLayout）

### ❌ 未実装 (Core層) - 優先度順

#### Priority 1 (Critical - 機能停止レベル)
| 機能 | 行数 | 状態 | 説明 |
|------|----:|:---:|------|
| PDF Export | 433行 | ✗ | pdf.rs 全体 |
| SVG Export | 394行 | ✗ | svg_export.rs 全体 |
| Image Export | 182行 | ✗ | image_export.rs (PNG/JPEG) |
| Template Manager | 788行 | ✗ | template_manager.rs (全体) |
| Typeset Engine | 325行 | ✗ | typeset.rs (テキスト植字ロジック) |
| Layout Engine | 317行 | ▲ 部分的 | layout.rs の実装ロジック |
| Alignment | 229行 | ✗ | グリッドスナップ、配置ロジック |

#### Priority 2 (Important - 機能制限)
- Command History (Undo/Redo) - CommandHistory はスケルトン
- Paper Size Presets (154行) - 用紙サイズ定義
- Error Handling - 基本的なエラー型のみ
- Page Properties - ガイドライン、グループ名管理など

### 推奨実装順序 (Core)
```
1. Template Manager (788行) → キャンバスに表示可能なテンプレート機能
2. Layout Engine (317行) → オブジェクト配置計算
3. Alignment (229行) → グリッドスナップ、配置命令
4. PDF Export (433行) → ドキュメント出力
5. SVG Export (394行) → ベクトルエクスポート
6. Image Export (182行) → 画像エクスポート
7. Typeset Engine (325行) → テキストレンダリング品質向上
8. Paper Size (154行) → 用紙テンプレート
```

---

## Part 2: UI/GTK Crate の反映状況

### 元のプロジェクト: gtkapp Module 構成
```
crates/gtkapp/src/ (86ファイル, 27,602行)

【Top-level Modules】
├─ app.rs                (619行) - アプリケーションメインロジック
├─ main_window.rs        (956行) - メインウィンドウ設定
├─ window_setup.rs     (2,097行) ⚠ 巨大 - ウィンドウ全体設定
├─ canvas_widget.rs      (649行) - キャンバスウィジェット基盤
├─ main.rs              - エントリーポイント
├─ undo.rs              (931行) - Undo/Redo スタック
├─ rich_text_editor.rs   (761行) - リッチテキスト編集
├─ keyboard_input.rs     (535行) - キーボード入力処理
├─ mouse_input.rs        (353行) - マウス入力処理（古い）
├─ drag_operations.rs    (531行) - ドラッグ操作
├─ drag_state.rs         (351行) - ドラッグ状態管理
├─ item_library.rs       (361行) - アイテムライブラリUI
├─ json_editor.rs        - JSONエディタ
├─ save_template_dialog.rs - テンプレート保存ダイアログ
├─ settings_dialog.rs    - 設定ダイアログ
├─ template_dialog.rs    - テンプレート選択ダイアログ
├─ block_tools.rs        - ブロックツール
├─ canvas_operations.rs  - キャンバス操作
├─ layer_panel.rs        - レイヤーパネル
├─ theme.rs             - テーマ管理
└─ ui/
   ├─ mod.rs
   ├─ actions.rs         - グローバルアクション
   ├─ keyboard_shortcuts.rs - キーボードショートカット
   ├─ menu.rs            - メニューシステム
   ├─ toolbar.rs         (500行) - ツールバー
   ├─ tool_manager.rs     - ツールモード管理
   └─ view_toggles.rs    - ビュー切り替え

【Canvas Sub-modules】 (52ファイル, ~18,000行)
├─ canvas/
│  ├─ mod.rs             (80行) - キャンバスモジュールインターフェース
│  ├─ widget_events_mouse.rs     (1,135行) ⚠⚠⚠ 巨大 - マウス入力処理
│  ├─ widget_events_keyboard.rs   (360行) - キーボード入力
│  ├─ widget_rendering.rs        (421行) - レンダリング
│  ├─ widget_operations.rs        (587行) - 操作API
│  ├─ widget_state.rs            (403行) - 状態管理
│  ├─ render_loop.rs             (826行) - Cairo描画ループ
│  ├─ rendering.rs               (504行) - レンダリング実装
│  ├─ render_rulers.rs           (236行) - ルーラー描画
│  ├─ render_grid_guides.rs       - グリッド/ガイド描画
│  ├─ render_shapes.rs            - シェイプ描画
│  ├─ render_text.rs              - テキスト描画
│  ├─ render_image.rs             - 画像描画
│  ├─ selection.rs               (505行) - 選択管理
│  ├─ selection_manager.rs        (351行) - 選択操作
│  ├─ selection_handler.rs        - 選択ハンドラ
│  ├─ drag_handlers.rs            (641行) - ドラッグハンドラ
│  ├─ properties.rs              (621行) - プロパティAPI
│  ├─ guides.rs                  (338行) - ガイド管理
│  ├─ guide_manager.rs           (440行) - ガイドマネージャ
│  ├─ keyboard_handlers.rs        (499行) - キーボード入力
│  ├─ alignment_impl.rs           - 配置実装
│  ├─ zorder_impl.rs              - Z-order実装
│  ├─ grouping_impl.rs            - グループ実装
│  ├─ locking_impl.rs             - ロック実装
│  ├─ creation_handlers.rs        (365行) - オブジェクト作成
│  ├─ duplication_handler.rs      - 複製ハンドラ
│  ├─ coordinate_converter.rs     (239行) - 座標変換
│  ├─ coordinate_system.rs        (285行) - 座標系管理
│  ├─ shape_creation.rs           (309行) - シェイプ作成
│  └─ ... [他15個のモジュール]

【Property Panel Sub-modules】 (5ファイル, ~1,500行)
├─ property/
│  ├─ mod.rs
│  ├─ panel_signals.rs            (416行) - シグナル処理
│  ├─ panel_ui_setup.rs           (522行) - UI構築
│  └─ panel_state.rs              - 状態管理

【IO Sub-modules】 (2ファイル, ~500行)
├─ io/
│  ├─ mod.rs
│  └─ file_operations.rs          - ファイルI/O

【Database Modules】 (未確認, db crate)
```

### 新しいプロジェクト: ui Module 構成
```
crates/ui/src/ (20ファイル, ~500行)

├─ app/
│  ├─ mod.rs             (42行)  - AppConfig, TestructApplication
│  ├─ state.rs           (27行)  - AppState スケルトン
│  └─ actions.rs         (38行)  - グローバルアクション（空）
│
├─ window/
│  ├─ mod.rs             (44行)  - MainWindow 構造
│  ├─ layout.rs          (53行)  - ウィンドウレイアウト
│  └─ bindings.rs        (16行)  - イベントバインディング（スケルトン）
│
├─ canvas/
│  ├─ mod.rs             (50行)  - キャンバスモジュール
│  ├─ input.rs           (35行)  - マウス/キーボード入力（スケルトン）
│  └─ overlays.rs        (8行)   - オーバーレイ（空）
│
├─ panels/
│  ├─ mod.rs             (5行)   - パネルモジュール
│  ├─ layers.rs          (20行)  - レイヤーパネル（空）
│  └─ properties.rs      (21行)  - プロパティパネル（空）
│
├─ dialogs/
│  ├─ mod.rs             (5行)
│  ├─ project_settings.rs (12行)  - プロジェクト設定（空）
│  └─ template_browser.rs (28行)  - テンプレートブラウザ（空）
│
├─ menu/
│  └─ mod.rs             (27行)  - メニューシステム（空）
│
├─ toolbar/
│  └─ mod.rs             (24行)  - ツールバー（空）
│
├─ theme/
│  └─ mod.rs             (9行)   - テーマ管理（スケルトン）
│
└─ lib.rs                (16行)  - モジュールエクスポート
```

### ✅ 実装済み (UI層)
- ✓ ApplicationConfig/TestructApplication 構造
- ✓ MainWindow 骨組み
- ✓ UI コンポーネント構造定義（WindowComponents）
- ✓ Glib イベントループ統合

### ❌ 未実装 (UI層) - 優先度順

#### Priority 1 (Critical - UI非動作)
| 機能 | 元のコード行数 | 状態 | 説明 |
|------|:---:|:---:|------|
| Canvas Rendering | 826行 | ✗ | render_loop.rs (Cairo描画パイプライン全体) |
| Canvas Widget Events - Mouse | 1,135行 | ✗ | widget_events_mouse.rs (マウス入力処理) |
| Canvas Widget Events - Keyboard | 360行 | ✗ | widget_events_keyboard.rs (キーボード入力) |
| Canvas Selection | 505行 | ✗ | selection.rs (オブジェクト選択) |
| Canvas Drag Handlers | 641行 | ✗ | drag_handlers.rs (ドラッグ操作) |
| Window Setup | 2,097行 | ✗ | window_setup.rs (ウィンドウ全体設定) |
| Main Window Logic | 956行 | ✗ | main_window.rs (メインウィンドウロジック) |

#### Priority 2 (Important - 機能制限)
| 機能 | 元のコード行数 | 状態 | 説明 |
|------|:---:|:---:|------|
| Toolbar Setup | 500行 | ✗ | toolbar.rs |
| Property Panel UI | 522行 | ✗ | property/panel_ui_setup.rs |
| Rich Text Editor | 761行 | ✗ | rich_text_editor.rs |
| Undo/Redo | 931行 | ✗ | undo.rs (スタック管理) |
| Canvas Properties API | 621行 | ✗ | canvas/properties.rs |
| Canvas Operations | 587行 | ✗ | canvas/widget_operations.rs |

#### Priority 3 (Lower - 拡張機能)
- Guide Management (440行) - ガイドラインシステム
- File Operations - ファイルI/O実装
- Item Library (361行) - アイテムライブラリUI
- Theme Management - テーマシステム
- Dialogs (Settings, Templates, Save) - 各種ダイアログ

### 推奨実装順序 (UI)
```
【Phase 1: Canvas Core】
1. Canvas Rendering (826行) → Cairo描画パイプライン
2. Canvas Mouse Events (1,135行) → マウス入力とドラッグ
3. Canvas Keyboard Events (360行) → キーボード入力
4. Canvas Selection (505行) → オブジェクト選択

【Phase 2: Window/Controls】
5. Window Setup (2,097行) → メインウィンドウUI構築
6. Toolbar (500行) → ツールバーUI
7. Property Panel UI (522行) → プロパティパネル
8. Main Window Logic (956行) → ウィンドウロジック

【Phase 3: User Features】
9. Undo/Redo (931行) → コマンド履歴
10. Rich Text Editor (761行) → テキスト編集
11. Drag Handlers (641行) → ドラッグ操作

【Phase 4: Polish】
12. Guide Management (440行) → ガイドライン
13. Item Library (361行) → アイテムライブラリ
14. Various Dialogs
```

---

## Part 3: Database/Storage Crate

### 元のプロジェクト
- ✓ `crates/db/` crate 存在（SQLite）
- Likely contains: Project persistence, document storage

### 新しいプロジェクト
- ✗ db crate 不存在
- リファクタリング方針: 後回しまたは core に統合予定

---

## Part 4: Feature Checklist

### Document Model
- ✓ Basic Document/Page structure
- ✓ Object representation
- ❌ RichText support (元では詳細)
- ❌ TextAttributes/TextStyleRange (スタイル指定)
- ❌ Table objects
- ❌ Slot types for templates

### Canvas Interaction
- ❌ Mouse event handling (マウス操作全般)
- ❌ Keyboard event handling (キーボード操作全般)
- ❌ Drag and drop (オブジェクトドラッグ)
- ❌ Object selection (オブジェクト選択)
- ❌ Multi-selection (複数選択)
- ❌ Guide snapping (ガイドへのスナップ)
- ❌ Grid snapping (グリッドへのスナップ)

### Object Operations
- ❌ Object alignment (配置)
- ❌ Z-order management (奥行き順)
- ❌ Object grouping/ungrouping (グループ化)
- ❌ Object locking/unlocking (ロック)
- ❌ Object resizing/moving (移動とリサイズ)
- ❌ Clipboard operations (コピペ)

### Text & Typography
- ✓ Basic Typography module (Color, TextStyle)
- ❌ Rich text formatting (太字、斜体など)
- ❌ Text alignment (左寄せ、中央など)
- ❌ Font selection and management
- ❌ Text wrapping and overflow
- ❌ Rich text editor UI

### Tools & Drawing
- ❌ Tool mode system (Select, Text, Image, Shapes)
- ❌ Shape drawing (Rectangle, Circle, Line, etc)
- ❌ Text insertion
- ❌ Image insertion
- ❌ Guide creation/editing

### Export & Import
- ❌ PDF export (433行のロジック)
- ❌ SVG export (394行のロジック)
- ❌ Image export (182行のロジック)
- ❌ File save/load
- ❌ Template save/load

### Undo/Redo
- ❌ Command history stack (931行のロジック)
- ❌ Undo/Redo UI integration
- ❌ Command serialization

### Panels & UI
- ❌ Layers panel (functional)
- ❌ Properties panel (functional)
- ❌ Toolbar (functional)
- ❌ Menu system (functional)
- ❌ Status bar
- ❌ Dialogs (Settings, Templates, Save)

---

## Part 5: Code Quality & Architecture

### 元のプロジェクトの問題点 (REFACTORING_GUIDE.md から)

```
巨大ファイルの存在:
├─ window_setup.rs        (2,097行) → 分割対象
├─ widget_events_mouse.rs (1,135行) → 分割対象
├─ rich_text_editor.rs    (761行)   → 分割対象
└─ undo.rs               (931行)   → 分割対象

パフォーマンス問題:
├─ Heavy RefCell usage (39フィールド)
├─ Document cloning in hot paths
├─ VecDeque ではなく Vec を使用 (Undo)
├─ Guide snapping computation (毎フレーム)
└─ Thumbnail cache の無制限成長

テスト問題:
├─ Panic in tests (recoverable)
├─ Limited test coverage
└─ Integration tests missing
```

### 新しいプロジェクトの設計方針 ✓

```
✓ ファイルサイズ規律 (500行以下)
✓ レイヤードアーキテクチャ (Presentation → Application → Domain)
✓ 責務分離 (1ファイル1機能)
✓ テスト対応性向上
✓ モジュール再利用性確保
```

---

## Part 6: 推奨実装ロードマップ

### Week 1-2: Canvas Core
```
目標: キャンバスが画面に表示されて、基本的なマウス/キー入力に応答
- Canvas rendering (Cairo)
- Mouse event handling
- Keyboard event handling
- Object selection
進捗目安: ~3,000行実装予定
```

### Week 3-4: Window & Controls
```
目標: ツールバー、パネル、メニューが機能
- Window setup
- Toolbar
- Property panel
- Menu system
進捗目安: ~1,500行実装予定
```

### Week 5-6: User Operations
```
目標: オブジェクトの作成・編集・操作が可能
- Undo/Redo
- Rich text editor
- Drag operations
- Object operations (align, z-order, group)
進捗目安: ~2,500行実装予定
```

### Week 7-8: Core Functionality
```
目標: ドキュメント出力が可能
- Template system
- Export (PDF, SVG, Image)
- File I/O
- Guide system
進捗目安: ~2,500行実装予定
```

### Estimated Total
```
Original Project:     31,451行
New Project Target:   ~10,000-12,000行 (モジュール再構成による削減)
リファクタリング効果: -50%以上の削減を目指す
```

---

## Part 7: 移行時の注意事項

### ❌ 単純なコピペはNG
元のコードを直接新プロジェクトにコピーすると：
- ファイルサイズが 500行を超える
- レイヤー依存関係が破壊される
- テスト性が低下する

### ✓ 推奨アプローチ
```
1. 元のコードをロジックを理解する
2. 責務を明確に分割する (SRP)
3. 新規作成 or 既存ファイルに統合
4. レイヤー原則を守る (Core ← UI)
5. 各段階でテストを追加
```

### ⚠️ 優先順位を守る
```
Critical (機能停止) → Important (機能制限) → Nice-to-have (拡張)
```

---

## Summary Table

| Layer | 元の規模 | 新の規模 | 実装率 | 次の優先順位 |
|-------|:------:|:------:|:-----:|:-----------|
| **Core** | 3,849行 | ~500行 | 13% | Template Manager (788行) |
| **UI** | 27,602行 | ~500行 | 2% | Canvas Rendering (826行) |
| **CLI** | 45行 | 45行 | 100% | ✓ 完成 |
| **DB** | 計測不可 | ✗ | 0% | 後回し |
| **TOTAL** | 31,451行 | 1,294行 | 4.1% | **7,000+行の実装が必要** |

---

**最終評価**: 新しいプロジェクトはアーキテクチャの基礎がしっかり設計されており、後は実装の積み重ねです。roadmap.md に沿って段階的に進めることをお勧めします。
