# 既知の問題と未実装機能一覧

**作成日**: 2025年11月8日
**最終更新**: 2025年11月8日
**対象**: Testruct Studio Rust版 v0.3

---

## 🐛 既知のバグ

### 優先度: 🔴 高（すぐに対応すべき）

#### 1. Save/Load が未実装
**ステータス**: ❌ 未実装
**影響範囲**: 全ユーザー（データ永続化が不可能）
**詳細**:
```
- ドキュメントをファイルに保存できない
- 保存されたファイルを開くことができない
- アプリ終了時にデータが失われる
```

**対応予定**: Phase 3（2025年12月20日）

**簡易対応**:
```bash
# 一時的な対応として、JSON export は可能かもしれない
# 確認が必要
```

---

#### 2. Undo/Redo が完全に動作していない
**ステータス**: ⚠️ 部分的（インフラは整備済み、統合が不完全）
**影響範囲**: 編集ユーザー
**詳細**:
```
- UndoRedoStack クラスは実装済み
- しかし各操作がコマンドパターンに統合されていない
- メニューの Undo/Redo が無効（グレーアウト）状態かもしれない
- テストケースが不足
```

**対応予定**: Phase 4（2025年12月末）

**技術負債**:
- `canvas/input.rs` の各操作に `apply_command()` 呼び出しを追加
- コマンドファクトリー関数の実装
- スタック管理の統合

---

#### 3. Copy/Paste/Duplicate が未検証
**ステータス**: ⚠️ 実装済みだが検証不足
**影響範囲**: 編集機能
**詳細**:
```
- Ctrl+C (Copy) - 実装済み
- Ctrl+X (Cut) - 実装済み
- Ctrl+V (Paste) - 実装済み
- Ctrl+D (Duplicate) - 実装済み

- クリップボード管理システムは実装済み
- しかし実際に動作するか不確定
- 複数オブジェクト選択時の動作未確認
- テストケースが不足
```

**対応予定**: 検証・テスト（1-2日）

**確認項目**:
- [ ] 単一オブジェクトのコピー・ペースト
- [ ] 複数オブジェクトのコピー・ペースト
- [ ] ペースト後のオブジェクト位置
- [ ] カット後のコマンド実行履歴

---

#### 4. Fill/Stroke カラー編集が UI に反映されていない
**ステータス**: ⚠️ UI は実装、ドキュメント同期が不完全
**影響範囲**: スタイル編集
**詳細**:
```
- プロパティパネルに Fill/Stroke カラーピッカーがある
- ピッカーで色を選択可能
- しかし Canvas の描画に反映されない可能性
- ドキュメント永続化の確認が必要
```

**対応予定**: Phase 1（2025年11月22日）

**確認項目**:
- [ ] ShapeStyle への色情報の保存
- [ ] Canvas 描画時に ShapeStyle 参照
- [ ] リアルタイムプレビュー
- [ ] ドキュメント JSON への保存

---

### 優先度: 🟡 中（対応すべき、ただし緊急ではない）

#### 5. Layers パネルが機能していない
**ステータス**: ❌ UI のみ存在（機能が空実装）
**影響範囲**: 高度な編集
**詳細**:
```
- Layer List UI は作成されている
- しかしドラッグ&ドロップが未実装
- レイヤー順序変更ができない
- 表示/非表示トグルが未実装
- レイヤー名編集が未実装
```

**対応予定**: Phase 2（2025年12月13日）

**実装が必要な機能**:
- [ ] Drag & Drop でレイヤー順序変更
- [ ] CheckButton で表示/非表示トグル
- [ ] レイヤー名の編集・入力
- [ ] キャンバス選択との双方向同期

---

#### 6. テキスト太字・斜体が未実装
**ステータス**: ⚠️ UI ボタンはあるが機能が空実装
**影響範囲**: テキスト編集
**詳細**:
```
- プロパティパネルに太字・斜体ボタンがある
- ボタンをクリックしても効果なし
- Pango 属性マッピングが不完全
```

**対応予定**: Phase 1（2025年11月22日）

**実装が必要**:
- [ ] FontWeight enum の Pango マッピング
- [ ] FontStyle enum の Pango マッピング
- [ ] テキスト描画時に属性を適用
- [ ] UI ボタン → TextStyle への同期

---

#### 7. Grouping が完全に未実装
**ステータス**: ❌ データ構造なし
**影響範囲**: 高度な編集
**詳細**:
```
- DocumentElement に Group 型がない
- グループコマンドがない
- グループ内の要素操作ができない
```

**対応予定**: Phase 2（2025年12月13日）

---

#### 8. マルチページ対応が未実装
**ステータス**: ⚠️ PageManager API は存在
**影響範囲**: 複数ページドキュメント
**詳細**:
```
- Document 構造に pages: Vec<Page> がある
- しかし UI でページを追加/削除できない
- ページナビゲーション UI がない
- 複数ページ表示・切り替えができない
```

**対応予定**: Phase 6（2026年以降）

---

### 優先度: 🟢 低（nice-to-have、後で対応OK）

#### 9. SVG Export が未実装
**ステータス**: ❌ 未実装
**詳細**: SVG 形式での出力が不可能

**対応予定**: Phase 3（2025年12月20日）

---

#### 10. Template システムが基本的のみ
**ステータス**: ⚠️ 保存・読み込みは可能だが機能が限定
**詳細**:
```
- Ctrl+Shift+S でテンプレート保存可能
- UI パネルが不完全
- テンプレートプレビュー未実装
- カテゴリ分類未実装
```

**対応予定**: Phase 2（2025年12月13日）

---

#### 11. Grid/Guide カスタマイズが未実装
**ステータス**: ⚠️ インフラは整備済み、UI がない
**詳細**:
```
- グリッド間隔は固定（10px）
- ガイド線は表示されるが管理 UI がない
- グリッド on/off 切り替え UI がない
- ガイド追加/削除ができない
```

**対応予定**: Phase 4（2025年12月末）

---

## ⚠️ 警告と注意事項

### GTK4 非推奨 API の使用
**レベル**: ℹ️ 情報（機能に影響なし）
```
警告: use of deprecated struct `gtk4::FileChooserNative`
警告: use of deprecated struct `gtk4::Dialog`
警告: use of deprecated method `gtk4::prelude::WidgetExt::show`

原因: GTK 4.10 以降で非推奨化
影響: なし（コンパイル・動作に影響なし）
対応: 将来的に新 API に置き換え
```

---

### 未使用のインポート
**レベル**: ℹ️ 情報
```
警告: unused import: `ToggleButton`
警告: unused variable: `render_state`
警告: unused variable: `item_list_clone` (複数)

影響: なし
対応: Clippy 警告の解決（優先度低）
```

---

## 📋 未実装機能（優先度順）

### ✅ 完成した機能（15/30）

#### Canvas & Rendering (4/4 完成)
- [x] Canvas Rendering (Cairo)
- [x] Shape Rendering (Rectangle, Circle, Line, Arrow, Polygon)
- [x] Text Rendering (Pango統合)
- [x] Image Rendering (PNG/JPEG/GIF/WebP)

#### Input & Interaction (4/4 完成)
- [x] Mouse Input (クリック、ドラッグ)
- [x] Keyboard Input (基本ショートカット)
- [x] Object Selection (単一・複数・マーキー)
- [x] Object Manipulation (移動・リサイズ)

#### Text Editing (3/3 完成)
- [x] Text Box Placement
- [x] Text Editing (ダブルクリック編集)
- [x] Property Panel Sync

#### Image Loading (4/4 完成)
- [x] Image Block Placement
- [x] Image Selection Dialog
- [x] Asset Management
- [x] Image Rendering

---

### ⏳ 進行中の機能（0/30）

#### Document Management (0/2)
- [ ] **Save/Load** - JSON形式でのシリアライズ
- [ ] **Undo/Redo** - スタック統合（インフラ整備済み）

#### Editing Capabilities (0/4)
- [ ] **Alignment Tools** - 配置・分散機能
- [ ] **Grouping** - グループ化・アングループ
- [ ] **Layers Panel** - ドラッグ&ドロップ機能
- [ ] **Copy/Paste/Duplicate** - 検証・テスト必要

#### Style & Appearance (0/3)
- [ ] **Fill Color** - カラー編集・グラデーション
- [ ] **Stroke Properties** - 幅・色・スタイル
- [ ] **Typography Controls** - 太字・斜体・下線

#### Export & Output (0/3)
- [ ] **Export to PDF** - PDF形式出力
- [ ] **Export to Image** - PNG/JPEG出力
- [ ] **Export to SVG** - SVG形式出力

#### Advanced Features (0/4)
- [ ] **Guides** - ガイド管理 UI
- [ ] **Grid Customization** - グリッド設定
- [ ] **Templates** - テンプレート拡張機能
- [ ] **Multi-page Support** - ページ管理 UI

#### Infrastructure (0/5)
- [ ] **Zoom Levels** - 固定ズーム率
- [ ] **Auto-Save** - 自動保存機能
- [ ] **Performance Optimization** - Dirty region、キャッシング
- [ ] **Error Handling** - エラーダイアログ、リカバリー
- [ ] **Testing** - テストカバレッジ拡充

---

## 🔍 バグ修正の優先順位マトリックス

| # | バグ | 影響度 | 難易度 | 優先度 | 対応予定 |
|---|------|--------|--------|--------|---------|
| 1 | Save/Load 未実装 | 🔴 致命的 | 🟡 中 | 🔴 最高 | Phase 3 |
| 2 | Undo/Redo 未統合 | 🔴 高 | 🟢 低 | 🔴 高 | Phase 4 |
| 3 | Copy/Paste 未検証 | 🟡 中 | 🟢 低 | 🟡 中 | 近日 |
| 4 | Fill/Stroke UI 未反映 | 🟡 中 | 🟡 中 | 🟡 中 | Phase 1 |
| 5 | Layers パネル未実装 | 🟡 中 | 🟡 中 | 🟡 中 | Phase 2 |
| 6 | テキスト装飾未実装 | 🟢 低 | 🟡 中 | 🟡 中 | Phase 1 |
| 7 | Grouping 未実装 | 🟢 低 | 🔴 高 | 🟢 低 | Phase 2 |
| 8 | マルチページ UI 未実装 | 🟢 低 | 🔴 高 | 🟢 低 | Phase 6 |
| 9 | SVG Export 未実装 | 🟢 低 | 🟡 中 | 🟢 低 | Phase 3 |
| 10 | Template 機能限定 | 🟢 低 | 🟢 低 | 🟢 低 | Phase 2 |

---

## 📝 詳細実装ガイド

### バグ #1: Save/Load 未実装（最優先）

**実装ステップ**:
1. DocumentSaver 実装
2. DocumentLoader 実装
3. ファイルダイアログ統合
4. 最近使用したファイル機能
5. Auto-save 実装

**見積り**: 5-7日

---

### バグ #2: Undo/Redo 統合

**実装ステップ**:
1. 各操作を Command に包装
2. AppState に execute_command() メソッド追加
3. メニュー handlers を Command 実行に変更
4. UI 状態更新（Undo/Redo ボタンの enabled/disabled）
5. テスト追加

**見積り**: 2-3日

---

### バグ #3: Copy/Paste 検証

**テスト項目**:
1. 単一オブジェクトのコピー・ペースト
2. 複数オブジェクトのコピー・ペースト
3. ペースト後の位置（オフセット確認）
4. カット後の削除確認
5. 複製の ID 変更確認

**見積り**: 1日

---

## 🚀 推奨実装順序

1. **Save/Load** (Phase 3) - データ永続化が最優先
2. **Undo/Redo 統合** (Phase 4) - ユーザー体験向上
3. **Copy/Paste 検証** (近日) - 既存機能の確認
4. **Fill/Stroke** (Phase 1) - スタイル編集
5. **Layers パネル** (Phase 2) - 高度な編集
6. その他（優先度順）

---

## 📞 今後の対応

このドキュメントは定期的に更新されます：
- **週1回**: 新規バグ追加・優先度調整
- **Phase 完了時**: 完了状態に更新
- **ユーザー報告時**: Issue を追加

---

## 🔗 関連ドキュメント

- [ROADMAP_2025.md](../planning/ROADMAP_2025.md) - 実装計画
- [FEATURE_LIST.md](../analysis/FEATURE_LIST.md) - 機能一覧
- [GitHub Issues](https://github.com/revivals47/testruckt-studio/issues) - Issue トラッキング

---

**最終更新**: 2025年11月8日
**作成者**: Claude Code

