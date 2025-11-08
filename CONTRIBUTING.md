# 貢献ガイド

Testruct Studio プロジェクトへの貢献をお考えいただき、ありがとうございます！

---

## 🎯 貢献できる方法

### 1. 新機能の実装
[ROADMAP_2025.md](./docs/planning/ROADMAP_2025.md) の Phase 1-6 から選択して実装してください。

### 2. バグ修正
GitHub Issues で報告されたバグを修正してください。

### 3. ドキュメント改善
- README の改善
- API ドキュメント充実
- ガイド・チュートリアル作成

### 4. テスト追加
ユニットテスト・統合テストの追加でカバレッジを向上させてください。

### 5. パフォーマンス最適化
プロファイリングして、ボトルネックを改善してください。

---

## 📋 貢献前に確認すること

1. **プロジェクト構造を理解する**
   - [README.md](./README.md) を読む
   - [docs/planning/ROADMAP_2025.md](./docs/planning/ROADMAP_2025.md) で次のステップを確認
   - [crates/ui/src/canvas/ARCHITECTURE.md](./crates/ui/src/canvas/ARCHITECTURE.md) で全体設計を把握

2. **開発環境を準備する**
   ```bash
   git clone https://github.com/revivals47/testruckt-studio.git
   cd testruct-desktop-Rust
   cargo build
   cargo test
   ```

3. **コーディング規約を確認する**
   - Rust 公式スタイルガイドに従う
   - Clippy の警告をなくす
   - フォーマットを `cargo fmt` で統一

---

## 🔄 貢献ワークフロー

### ステップ 1: Issue を作成/確認

バグ修正や新機能の場合、まず Issue を作成してください：

```markdown
## 🐛 バグ報告 / 📝 機能リクエスト

### 説明
[説明を書く]

### 関連する ROADMAP
[該当ドキュメントへのリンク]

### 実装の概要
[実装方法を書く]
```

### ステップ 2: ブランチを作成

```bash
# main から新しいブランチを作成
git checkout -b feature/your-feature-name

# または
git checkout -b fix/bug-name
```

**ブランチ名の規約**:
- 新機能: `feature/lowercase-with-hyphens`
- バグ修正: `fix/bug-name`
- ドキュメント: `docs/topic-name`
- リファクタリング: `refactor/module-name`

### ステップ 3: コードを実装

```bash
# 実装・テスト
cargo build
cargo test
cargo clippy
cargo fmt
```

**実装時のチェックリスト**:
- [ ] テストを追加した
- [ ] Clippy 警告がない
- [ ] `cargo fmt` を実行した
- [ ] ドキュメント（コメント）を追加した
- [ ] 関連する .md ファイルを更新した

### ステップ 4: コミットメッセージを書く

```
<type>: <subject>

<body>

<footer>
```

**例**:
```
feat: Implement Fill & Stroke color editing

- Add ColorPicker integration to ShapeStyle
- Implement real-time preview in canvas
- Update properties panel with style controls
- Add unit tests for color serialization

Closes #123
Related-to: ROADMAP_2025.md Phase 1.1
```

**タイプ**:
- `feat`: 新機能
- `fix`: バグ修正
- `docs`: ドキュメント
- `test`: テスト追加
- `refactor`: リファクタリング
- `perf`: パフォーマンス改善
- `ci`: CI/CD 変更
- `chore`: その他の変更

### ステップ 5: Pull Request を作成

```bash
# ブランチをプッシュ
git push origin feature/your-feature-name
```

**PR テンプレート**:
```markdown
## 📝 説明
[変更の説明]

## 🎯 関連 Issue
Closes #123

## 📋 チェックリスト
- [ ] テストが合格している
- [ ] ドキュメントを更新した
- [ ] Clippy 警告がない
- [ ] コミットメッセージが適切

## 🚀 影響範囲
[影響を受ける機能・モジュール]

## 📊 テスト結果
```

### ステップ 6: レビュー & マージ

1. コードレビューを受ける
2. 指摘事項を修正
3. マージ前に 1 回テストを実行
4. Squash & Merge でマージ

---

## 🔍 コーディング規約

### Rust スタイル

```rust
// ✅ 良い例
pub struct DocumentElement {
    id: Uuid,
    bounds: Rect,
}

impl DocumentElement {
    /// ドキュメント要素の ID を取得
    pub fn id(&self) -> Uuid {
        self.id
    }
}

// ❌ 悪い例
pub struct DocumentElement {
    id: Uuid,
    bounds: Rect,
}

impl DocumentElement {
    pub fn get_id(&self) -> Uuid {  // get_ プレフィックス不要
        self.id
    }
}
```

### ドキュメント

```rust
/// 簡潔な説明（1行）
///
/// 詳細な説明（複数行可）
///
/// # 例
/// ```
/// let element = DocumentElement::new(bounds);
/// ```
///
/// # パニック
/// [パニック条件があれば記述]
pub fn new(bounds: Rect) -> Self { }
```

### テスト

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Arrange（準備）
        let input = vec![1, 2, 3];

        // Act（実行）
        let result = process(input);

        // Assert（確認）
        assert_eq!(result, expected);
    }
}
```

---

## 🧪 テストの追加方法

### ユニットテスト

```bash
# テスト実行
cargo test

# 特定のテストのみ実行
cargo test test_name

# テスト + ログ出力
RUST_LOG=debug cargo test test_name -- --nocapture
```

### 統合テスト

`crates/ui/tests/` に統合テストを配置します。

---

## 📖 ドキュメントの更新

### README 更新
- 新機能を追加したら、`完成した機能` セクションを更新
- ロードマップに変更があれば、`進行中の機能` セクションを更新

### Roadmap 更新
- [docs/planning/ROADMAP_2025.md](./docs/planning/ROADMAP_2025.md) を更新
- `📈 期間別マイルストーン` セクションを最新化

### 進捗報告
- 大きな機能実装後は、[docs/progress/](./docs/progress/) に進捗レポートを作成

### Code Comments
- 複雑なロジックには説明コメントを追加
- pub 関数には doc comment を追加

---

## ✅ PR マージ前のチェックリスト

### コード品質
- [ ] `cargo clippy` で警告なし
- [ ] `cargo fmt` でフォーマット済み
- [ ] テスト合格率 100%
- [ ] テストカバレッジ低下なし

### ドキュメント
- [ ] Rust doc comment 追加（pub 関数）
- [ ] README/ROADMAP 更新
- [ ] 複雑なロジックにコメント追加

### テスト
- [ ] ユニットテスト追加
- [ ] 既存テスト全て合格
- [ ] 手動テスト確認

### コミット
- [ ] コミットメッセージが明確
- [ ] コミット数が適切（squash 推奨）
- [ ] 関連 Issue をクローズ

---

## 🚀 大きな機能の実装ガイド

### 例: Fill & Stroke 実装（Phase 1.1）

#### 1. Issue 作成
```markdown
## feat: Implement Fill & Stroke Color Editing

### 説明
プロパティパネルから Fill/Stroke の色を編集できるようにします。

### 実装内容
- [ ] ShapeStyle に fill/stroke フィールド追加
- [ ] カラーピッカー統合
- [ ] リアルタイムプレビュー
- [ ] ドキュメント永続化
- [ ] ユニットテスト

### 関連ドキュメント
[ROADMAP_2025.md Phase 1.1](./docs/planning/ROADMAP_2025.md#11-fill--stroke)
```

#### 2. ブランチ作成
```bash
git checkout -b feature/fill-stroke-editing
```

#### 3. 実装順序
```
1. データ構造 (ShapeStyle)
2. UI (プロパティパネル)
3. 描画ロジック (Canvas)
4. テスト
5. ドキュメント
```

#### 4. PR 作成
```markdown
## feat: Implement Fill & Stroke Color Editing

### 説明
ShapeStyle に fill/stroke 属性を追加し、プロパティパネルから
カラーピッカーで編集できるようにしました。

### チェックリスト
- [x] ShapeStyle 実装
- [x] カラーピッカー統合
- [x] リアルタイムプレビュー
- [x] ユニットテスト (8 cases)
- [x] Clippy 警告なし

### テスト結果
```
running 8 tests
test canvas::shape_style::tests::test_fill_color ... ok
test canvas::shape_style::tests::test_stroke_width ... ok
...

test result: ok. 8 passed
```

Closes #456
Related-to: ROADMAP_2025.md Phase 1.1
```

---

## 🐛 バグ報告のフォーマット

```markdown
## 🐛 [バグ名]

### 環境
- OS: [macOS / Linux / Windows]
- Rust version: [version]
- 実行方法: [cargo run / binary]

### 再現手順
1. ...
2. ...
3. ...

### 期待動作
[期待する結果]

### 実際の動作
[実際の結果]

### ログ出力
```
[ログをここに貼り付け]
```

### スクリーンショット
[可能であれば]
```

---

## 📚 参考資料

### プロジェクト関連
- [README.md](./README.md) - プロジェクト概要
- [ROADMAP_2025.md](./docs/planning/ROADMAP_2025.md) - 開発計画
- [ARCHITECTURE.md](./crates/ui/src/canvas/ARCHITECTURE.md) - 設計図
- [INPUT_MODULE.md](./crates/ui/src/canvas/INPUT_MODULE.md) - 入力処理詳細

### Rust 関連
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

### GTK4 関連
- [GTK4 Documentation](https://docs.gtk.org/gtk4/)
- [GTK-rs Guide](https://gtk-rs.org/)

---

## 💬 質問・相談

わからないことがあれば、遠慮なく以下で相談してください：

- **GitHub Discussions**: https://github.com/revivals47/testruckt-studio/discussions
- **GitHub Issues**: https://github.com/revivals47/testruckt-studio/issues

---

## 🙏 貢献いただきありがとうございます！

コードを良くするためのあなたの貢献は非常に大切です。
楽しい開発ライフを！

**Happy Coding! 🚀**

---

**最終更新**: 2025年11月8日

