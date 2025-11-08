# Testruct Desktop Rust - スタートアップ最適化ガイド

## 📊 現在の状況

アプリケーション起動時に GTK4 初期化が主な時間を占めています。
これはほぼコントロール不可能な部分ですが、以下の方法で改善できます。

---

## ⚡ 高速起動方法（推奨）

### 方法1：リリースバイナリの直接実行（最速）

**セットアップ（1回だけ）**:

```bash
# ビルド
cargo build --release

# バイナリに実行権限を付与
chmod +x target/release/testruct-cli

# ターミナルから実行
./target/release/testruct-cli ui
```

**メリット**:
- ✅ 最速（Cargo の起動オーバーヘッドなし）
- ✅ 最適化されたバイナリ
- ✅ 即座に起動

---

### 方法2：デスクトップアイコン（macOS）

**macOS でアプリアイコンを作成する場合**:

```bash
mkdir -p Testruct.app/Contents/MacOS
mkdir -p Testruct.app/Contents/Resources

# バイナリをコピー
cp target/release/testruct-cli Testruct.app/Contents/MacOS/testruct

# Info.plist を作成（以下参照）
cat > Testruct.app/Contents/Info.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>testruct</string>
    <key>CFBundleName</key>
    <string>Testruct Studio</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>0.9.1</string>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
</dict>
</plist>
EOF

# 起動スクリプト
cat > Testruct.app/Contents/MacOS/testruct << 'EOF'
#!/bin/bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$DIR/testruct-cli" ui
EOF

chmod +x Testruct.app/Contents/MacOS/testruct
```

---

### 方法3：シェルエイリアス（開発者向け）

`.zshrc` または `.bash_profile` に以下を追加：

```bash
alias testruct="/Users/ken/Desktop/testruct-desktop-Rust/target/release/testruct-cli ui"
```

その後、ターミナルから：

```bash
testruct
```

---

## 🔧 GTK4 初期化時間の最適化

### 既実装：スタートアップタイミングの可視化

以下のメッセージが stderr に表示されます：

```
🚀 Starting GTK application...
📐 Creating window...
⏱️  Window created: XXXms
📋 Building menu bar...
⏱️  Menu bar built: XXXms
🛠️  Building toolbars...
⏱️  Toolbars built: XXXms
🎨 Building main layout...
⏱️  Main layout built: XXXms
✅ Total widget build time: XXXms
🎯 ACTIVATE SIGNAL FIRED!
✅ Window presented
```

### 計測方法

```bash
# ターミナルで実行して、stderr に時間情報を表示
time ./target/release/testruct-cli ui 2>&1 | grep "⏱️"
```

---

## 📈 期待される起動時間

| メソッド | 時間 | 備考 |
|---------|------|------|
| cargo run | 3-5秒 | Cargo のオーバーヘッド含む |
| 直接実行 | 1-2秒 | **推奨** |
| Finder アイコン | 1-2秒 |（方法2 セットアップ後） |

---

## 🎯 将来の最適化案（実装対象外）

### 低優先度の改善
1. **スプラッシュスクリーン** - 視覚的フィードバック提供
2. **Lazy loading** - 初回表示後にパネル初期化
3. **プリロード** - バックグラウンドで初期化

### GTK4 レイテンシの根本原因
- GTK4 フレームワーク初期化
- X11/Wayland システム初期化
- フォント キャッシング
- テーマ読み込み

これらは**言語・フレームワークレベルで改善困難**です。

---

## ✅ 推奨セットアップ

### Step 1: リリースバイナリビルド

```bash
cd /Users/ken/Desktop/testruct-desktop-Rust
cargo build --release
```

### Step 2: 実行

```bash
./target/release/testruct-cli ui
```

### Step 3（オプション）: ターミナルエイリアス設定

```bash
echo 'alias testruct="/Users/ken/Desktop/testruct-desktop-Rust/target/release/testruct-cli ui"' >> ~/.zshrc
source ~/.zshrc
```

その後は：

```bash
testruct
```

---

## 📊 パフォーマンス計測

スタートアップ時間を計測する場合：

```bash
# 1回目（キャッシュなし）
time ./target/release/testruct-cli ui 2>/dev/null

# 2回目（キャッシュあり）
time ./target/release/testruct-cli ui 2>/dev/null
```

---

## 🔍 デバッグ情報表示

詳細なスタートアップ情報を表示する場合：

```bash
# すべてのログを表示
RUST_LOG=debug ./target/release/testruct-cli ui

# GTK ログのみ
RUST_LOG=gtk4 ./target/release/testruct-cli ui
```

---

## まとめ

| 対策 | 効果 | 実施難度 |
|-----|------|--------|
| リリースバイナリ使用 | ★★★ | ☆☆☆ |
| デスクトップアイコン作成 | ★★ | ☆☆ |
| ターミナルエイリアス | ★★ | ☆☆☆ |
| GTK4 最適化 | ★☆ | ★★★ |

**推奨**: リリースバイナリ直接実行が最も効果的です。

---

**Last Updated**: 2024-11-06
**Version**: v0.9.1-tier1-complete
