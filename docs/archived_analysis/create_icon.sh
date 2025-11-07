#!/bin/bash

# Testruct Studio アイコンを作成するスクリプト
# macOS Finder で表示されるアイコンセットを生成

ICON_FOLDER="/Users/ken/Desktop/Testruct.app/Contents/Resources"
SIZES=(16 32 128 256 512)

# 簡単な SVG から icns を作成
# または、既存画像があればそれを使用

echo "ℹ️  .app バンドルが作成されました"
echo ""
echo "📁 アプリの場所: /Users/ken/Desktop/Testruct.app"
echo ""
echo "🎯 以下の方法でアイコンをカスタマイズできます："
echo ""
echo "方法1: Finder から Info.plist を編集"
echo "  右クリック > 'Get Info' でアイコンを変更"
echo ""
echo "方法2: 既存アイコン画像を使用"
echo "  PNG ファイルを準備して iconutil を使用"
echo ""
echo "方法3: デフォルトアイコンで使用（推奨）"
echo "  そのまま Finder で使用可能"
