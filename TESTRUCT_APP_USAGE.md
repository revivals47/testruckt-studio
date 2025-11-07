# Testruct Studio macOS App - 使用方法

## 📁 アプリケーション情報

**アプリの場所**: `/Users/ken/Desktop/Testruct.app`

**バージョン**: 0.9.1

**対応環境**: macOS 10.13 以上

---

## 🚀 起動方法

### 方法1: Finder からアイコンをダブルクリック（推奨）

1. Finder を開く
2. Desktop フォルダに移動
3. `Testruct Studio` アイコンをダブルクリック
4. アプリが起動します

### 方法2: 右クリックから開く

1. Desktop 上の `Testruct Studio` を右クリック
2. 「開く」を選択

### 方法3: ターミナルから起動

```bash
open /Users/ken/Desktop/Testruct.app
```

---

## 📋 アプリバンドルの構成

```
Testruct.app/
├── Contents/
│   ├── Info.plist          # アプリメタデータ
│   ├── MacOS/
│   │   ├── testruct        # ランチャースクリプト
│   │   └── testruct-cli    # 実行バイナリ
│   └── Resources/          # リソース（アイコンなど）
```

---

## 🔧 アイコンのカスタマイズ方法

### 簡易方法: Finder から変更

1. `Testruct.app` を右クリック
2. 「情報を見る」を選択
3. 左上のアイコンをコピー＆ペーストで変更可能

### 高度な方法: 独自アイコンセットを作成

```bash
# icns ファイルを Info.plist に指定
# 詳細: Apple Developer Documentation 参照
```

---

## 🛠️ トラブルシューティング

### アプリが起動しない場合

**エラーメッセージ**:
```
"Testruct Studio" は破損しているため開けません
```

**対応方法**:
```bash
# セキュリティ設定をリセット
xattr -d com.apple.quarantine /Users/ken/Desktop/Testruct.app

# または再作成
cd /Users/ken/Desktop/testruct-desktop-Rust
cargo build --release --features ui
rm -rf /Users/ken/Desktop/Testruct.app
# 上記セットアップを再実行
```

### アプリが重複起動する場合

```bash
# プロセスをすべて終了
pkill -f testruct-cli

# 再度起動
open /Users/ken/Desktop/Testruct.app
```

---

## 📚 関連ファイル

- `STARTUP_OPTIMIZATION.md` - スタートアップ最適化ガイド
- `/Users/ken/Desktop/testruct-desktop-Rust/target/release/testruct-cli` - バイナリ本体

---

## 🔄 バイナリを更新する場合

新しいバイナリをビルド後、以下で更新：

```bash
cd /Users/ken/Desktop/testruct-desktop-Rust
cargo build --release --features ui

# アプリ内のバイナリを置き換え
cp target/release/testruct-cli /Users/ken/Desktop/Testruct.app/Contents/MacOS/testruct-cli
```

その後、通常通り Finder からアイコンをダブルクリックで起動します。

---

**Last Updated**: 2024-11-06  
**Status**: ✅ Ready for use
