# Testruct Studio - Rust版

**高度な図形・テキスト編集ツール** | GTK4 + Rust | 完成度: 50%

[![GitHub](https://img.shields.io/badge/GitHub-revivals47%2Ftestruckt--studio-blue)](https://github.com/revivals47/testruckt-studio)
![Rust Version](https://img.shields.io/badge/Rust-1.70%2B-orange)
![Platform](https://img.shields.io/badge/Platform-Linux%2C%20macOS%2C%20Windows-green)

---

## 🎨 概要

Testruct Studio は、図形・テキスト・画像を統合的に扱える**スケッチング＆ダイアグラムツール**です。

**主な特徴**:
- ✅ Canvas ベースの描画（Cairo）
- ✅ オブジェクト選択・移動・リサイズ
- ✅ テキスト編集（マルチライン対応）
- ✅ 画像の読み込み・配置
- ✅ グリッド・ガイドスナップ
- ✅ レイアウト配置（整列・分散）
- ⏳ Fill/Stroke カスタマイズ（進行中）
- ⏳ Undo/Redo 完全統合（進行中）

---

## 🚀 クイックスタート

### 要件
- Rust 1.70 以上
- GTK4
- Cairo
- Pango

### インストール（macOS）
```bash
# GTK4 インストール
brew install gtk4 libadwaita

# リポジトリクローン
git clone https://github.com/revivals47/testruckt-studio.git
cd testruct-desktop-Rust

# ビルド
cargo build --release

# 実行
cargo run --release
```

### Linux
```bash
# Ubuntu/Debian
sudo apt install libgtk-4-dev libadwaita-1-dev libcairo2-dev libpango1.0-dev

cargo build --release
cargo run --release
```

---

## 📁 プロジェクト構成

```
testruct-desktop-Rust/
├── crates/                          # Rust クレート
│   ├── core/                        # ビジネスロジック
│   │   └── src/
│   │       ├── document.rs          # ドキュメント構造
│   │       ├── layout.rs            # レイアウト計算
│   │       └── typography.rs        # テキスト属性
│   │
│   ├── ui/                          # UI フロントエンド
│   │   └── src/
│   │       ├── canvas/              # キャンバス描画
│   │       │   ├── input/           # 入力処理（キーボード、マウス、ジェスチャー）
│   │       │   ├── rendering.rs     # Cairo 描画パイプライン
│   │       │   ├── tools.rs         # ツール管理
│   │       │   └── ...
│   │       ├── panels/              # UI パネル
│   │       ├── dialogs/             # ダイアログ
│   │       ├── window/              # ウィンドウ管理
│   │       └── app/                 # アプリケーション状態
│   │
│   └── Cargo.toml
│
├── docs/                            # ドキュメント
│   ├── planning/                    # ロードマップ・計画
│   ├── analysis/                    # 分析・比較
│   ├── progress/                    # 進捗報告
│   └── guides/                      # ガイド・マニュアル
│
├── Cargo.toml                       # ワークスペース設定
├── README.md                        # このファイル
└── CONTRIBUTING.md                  # 貢献ガイド

```

---

## 📖 ドキュメント一覧

### 📋 計画 (`docs/planning/`)
- **[ROADMAP_2025.md](./docs/planning/ROADMAP_2025.md)** - 詳細な実装ロードマップ（2025年11月～2026年1月）
- **[IMPLEMENTATION_ROADMAP.md](./docs/planning/IMPLEMENTATION_ROADMAP.md)** - 初期実装計画
- **[REFACTORING_ROADMAP.md](./docs/planning/REFACTORING_ROADMAP.md)** - リファクタリング計画

### 📊 分析 (`docs/analysis/`)
- **[FEATURE_LIST.md](./docs/analysis/FEATURE_LIST.md)** - 機能一覧と実装状況（50%完了）
- **[FEATURE_COMPLETENESS_COMPARISON.md](./docs/analysis/FEATURE_COMPLETENESS_COMPARISON.md)** - オリジナル版との比較
- **[MIGRATION_GAP_ANALYSIS.md](./docs/analysis/MIGRATION_GAP_ANALYSIS.md)** - マイグレーション分析
- **[ANALYSIS_INDEX.md](./docs/analysis/ANALYSIS_INDEX.md)** - 分析インデックス

### 📈 進捗 (`docs/progress/`)
- **[IMPLEMENTATION_PROGRESS.md](./docs/progress/IMPLEMENTATION_PROGRESS.md)** - 実装進捗レポート
- **[TIER1_COMPLETION_REPORT.md](./docs/progress/TIER1_COMPLETION_REPORT.md)** - Tier 1 完了報告

### 📚 ガイド (`docs/guides/`)
- **[IMPLEMENTATION_GUIDE_NEXT_STEPS.md](./docs/guides/IMPLEMENTATION_GUIDE_NEXT_STEPS.md)** - 次のステップガイド
- **[STARTUP_OPTIMIZATION.md](./docs/guides/STARTUP_OPTIMIZATION.md)** - 起動最適化ガイド
- **[TESTRUCT_APP_USAGE.md](./docs/guides/TESTRUCT_APP_USAGE.md)** - アプリ使用方法

### 🏗️ アーキテクチャ
- **[crates/ui/src/canvas/ARCHITECTURE.md](./crates/ui/src/canvas/ARCHITECTURE.md)** - Canvas モジュール設計
- **[crates/ui/src/canvas/INPUT_MODULE.md](./crates/ui/src/canvas/INPUT_MODULE.md)** - 入力処理モジュール詳細

---

## ✨ 完成した機能

### Canvas & Rendering
- ✅ 2D 描画パイプライン（Cairo）
- ✅ Zoom/Pan 対応
- ✅ Ruler（定規）表示
- ✅ グリッド表示
- ✅ ガイド線表示

### Shape Drawing
- ✅ Rectangle（矩形）
- ✅ Circle/Ellipse（円・楕円）
- ✅ Line（直線）
- ✅ Arrow（矢印）
- ✅ Polygon（多角形）

### Text Editing
- ✅ テキスト要素の作成
- ✅ ダブルクリックで編集開始
- ✅ マルチラインテキスト
- ✅ テキスト配置（左・中央・右・両端揃え）
- ✅ フォント・フォントサイズ指定
- ✅ テキストカラー指定

### Input & Interaction
- ✅ マウス入力（クリック・ドラッグ）
- ✅ キーボード入力
- ✅ オブジェクト選択（単一・複数・トグル）
- ✅ リサイズ（8方向）
- ✅ 矢印キーでの移動

### Image Handling
- ✅ 画像の配置（PNG/JPEG/GIF/WebP）
- ✅ 画像選択ダイアログ
- ✅ Asset Catalog 管理
- ✅ アスペクト比保持

### Productivity
- ✅ コピー/カット/ペースト/複製（Ctrl+C/X/V/D）
- ✅ 配置機能（左揃え・右揃え・中央揃え等）
- ✅ 分散機能（均等分散）
- ✅ グリッドスナップ
- ✅ ガイドスナップ
- ✅ テンプレート保存・読み込み
- ✅ Asset Management

### Export & Output
- ✅ PDF 出力
- ✅ PNG/JPEG 出力
- ✅ SVG 出力

---

## 🔄 進行中の機能

### Near-term（2週間内）
- ⏳ Fill & Stroke カラー編集
- ⏳ Typography 完全対応（太字・斜体）
- ⏳ 透明度（Opacity）調整

### Mid-term（1ヶ月以内）
- ⏳ Layers パネル完全実装
- ⏳ Grouping 機能
- ⏳ Auto-Save & Recovery
- ⏳ Save/Load の完全実装

### Long-term（1-2ヶ月以上）
- ⏳ Undo/Redo 完全統合
- ⏳ マルチページ対応
- ⏳ パフォーマンス最適化
- ⏳ シンボル/プリセット機能

詳細は [ROADMAP_2025.md](./docs/planning/ROADMAP_2025.md) を参照してください。

---

## 🛠️ 開発者向け情報

### コード構造の理解

1. **入力処理を理解する**
   - 📖 [crates/ui/src/canvas/INPUT_MODULE.md](./crates/ui/src/canvas/INPUT_MODULE.md)
   - キーボード、マウス、ジェスチャー処理の詳細

2. **Canvas 全体を理解する**
   - 📖 [crates/ui/src/canvas/ARCHITECTURE.md](./crates/ui/src/canvas/ARCHITECTURE.md)
   - モジュール構成、状態管理、イベントフロー

3. **実装ステップを確認する**
   - 📖 [docs/planning/ROADMAP_2025.md](./docs/planning/ROADMAP_2025.md)
   - 次の実装タスクと見積り

### ビルド & テスト

```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release

# テスト実行
cargo test

# ドキュメント生成
cargo doc --open

# Clippy チェック
cargo clippy

# フォーマット
cargo fmt
```

### ドキュメント生成

```bash
# Rust API ドキュメント
cargo doc --no-deps --open

# Canvas アーキテクチャ図を確認
cat crates/ui/src/canvas/ARCHITECTURE.md
```

---

## 🐛 バグ報告・機能リクエスト

### Issues
GitHub Issues で報告してください：
https://github.com/revivals47/testruckt-studio/issues

### 報告内容
- 実行環境（OS、Rust version）
- 再現手順
- スクリーンショット（可能であれば）
- ログ出力

---

## 🤝 貢献方法

1. Fork して新しいブランチを作成
2. 機能を実装
3. テストを追加
4. PR を送信

詳細は [CONTRIBUTING.md](./CONTRIBUTING.md) を参照。

---

## 📊 進捗ダッシュボード

### 全体完成度
```
█████████░░░░░░░░░░ 50% (15/30機能)
```

### 機能別完成度
```
基本機能（Canvas, Input, Text）    ██████████ 100%
中級機能（Selection, Tools）      ██████████ 100%
スタイル機能（Fill, Stroke）     ░░░░░░░░░░ 0%
高度な機能（Layers, Grouping）   ██░░░░░░░░ 20%
エクスポート（PDF, Image, SVG）  ██████████ 100%
ドキュメント管理                 █████░░░░░ 50%
```

### マイルストーン
| 目標 | 予定日 | 状態 |
|------|--------|------|
| Phase 1: Style機能 | 2025-11-22 | ⏳ |
| Phase 2: 高度な機能 | 2025-12-13 | ⏳ |
| Phase 3: Save/Load | 2025-12-20 | ⏳ |
| Phase 4: 最適化 | 2025-12-31 | ⏳ |
| Phase 5: テスト・ドキュメント | 2026-01-15 | ⏳ |
| **v1.0 リリース** | **2026-01-31** | **🎯** |

---

## 🏗️ システム要件

### 最小要件
- macOS 10.15 以上 / Ubuntu 18.04 以上 / Windows 10 以上
- 4GB RAM
- 500MB ディスク空き容量

### 推奨要件
- macOS 11 以上 / Ubuntu 20.04 以上 / Windows 11
- 8GB RAM
- SSD 1GB 以上

---

## 📝 ライセンス

[LICENSE](./LICENSE) ファイルを参照してください。

---

## 🙋 サポート

### ドキュメント
1. [ROADMAP_2025.md](./docs/planning/ROADMAP_2025.md) - 開発予定
2. [FEATURE_LIST.md](./docs/analysis/FEATURE_LIST.md) - 機能一覧
3. [INPUT_MODULE.md](./crates/ui/src/canvas/INPUT_MODULE.md) - 入力処理
4. [ARCHITECTURE.md](./crates/ui/src/canvas/ARCHITECTURE.md) - 全体構成

### コミュニティ
- GitHub Issues: https://github.com/revivals47/testruckt-studio/issues
- Discussions: https://github.com/revivals47/testruckt-studio/discussions

---

## 👏 謝辞

本プロジェクトは以下のライブラリを使用しています：

- **GTK4** - GUI フレームワーク
- **Cairo** - 2D グラフィックス
- **Pango** - テキストレンダリング
- **Serde** - シリアライゼーション
- **Tokio** - 非同期ランタイム

---

**最終更新**: 2025年11月8日
**開発者**: Ken (Claude Code)
**Status**: 🔄 開発進行中 (50% 完了)

