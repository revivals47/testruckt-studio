# 座標システムの根本的な問題分析
## Coordinate System Root Cause Analysis

**日時**: 2025-11-08 21:00 JST
**状態**: 根本原因調査中

---

## 🔍 **問題の再整理**

### ユーザーの観測
- マウスカーソルの視覚的位置とクリック判定位置にズレ
- ズレ量が一定（3グリッド右、4グリッド下）
- ズレが修正されない（+30px, +40px の強制オフセットでも解決せず）

### つまり
前回の修正が機能しなかったということは：
1. オフセット値の計算が間違っている、または
2. オフセットを適用すべき場所が違う、または
3. 根本的な座標系の理解が間違っている

---

## 📊 **座標システムの詳細分析**

### 描画時の座標変換 (crates/ui/src/canvas/mod.rs: 155-159)

```rust
// スクリーン座標系
ctx.translate(
    ruler_config.size + config.pan_x,      // X: 20 + pan_x
    ruler_config.size + config.pan_y,      // Y: 20 + pan_y
);
ctx.scale(config.zoom, config.zoom);       // Default zoom: 1.0

// 以降のすべての描画はドキュメント座標系で実行
```

**変換の意味**:
- Canvas座標 (100, 200) は Screen座標では (100 + 20 + pan_x, 200 + 20 + pan_y) に描画される
- Zoom > 1.0 の場合はさらに拡大される

### クリック時の座標変換 (gesture_click.rs: 現在の実装)

```rust
// 現在のコード（修正前）:
let screen_x = x - (ruler_config.size + config.pan_x);
let canvas_x = screen_x / config.zoom;

// 期待される逆変換:
// canvas_x = (x - ruler_size - pan_x) / zoom
// つまり: screen_position = x - (ruler_size + pan_x)
```

**問題候補**:

イベント座標 `x, y` の座標系が不明:
- **可能性1**: Widget座標（DrawingArea内での座標）
- **可能性2**: Overlay座標（Overlay内での座標）
- **可能性3**: ScrolledWindow座標（スクロール後の座標）
- **可能性4**: Screen座標（ウィンドウ相対）

---

## 🔧 **根本原因の特定方法**

### ステップ1: イベント座標の実際の値を取得

デバッグログで以下を出力することで、イベント座標の実際の値と期待値を比較：

```rust
// イベントで報告された座標
eprintln!("Event coord: ({:.0}, {:.0})", x, y);

// DrawingArea の絶対位置を取得
if let Some(parent) = drawing_area.parent() {
    eprintln!("Parent widget: {:?}", parent);
}
let allocation = drawing_area.allocation();
eprintln!("DrawingArea allocation: x={}, y={}, w={}, h={}",
    allocation.x(), allocation.y(), allocation.width(), allocation.height());

// ScrolledWindow のスクロール位置
if let Some(hadjustment) = container.hadjustment() {
    eprintln!("Horizontal scroll: {:.0}", hadjustment.value());
}
if let Some(vadjustment) = container.vadjustment() {
    eprintln!("Vertical scroll: {:.0}", vadjustment.value());
}
```

### ステップ2: 逆変換の確認

Canvas座標の逆変換を厳密に適用：

```
Screen座標 = Event座標 (この時点で未知)
Canvas座標 = (Screen座標 - Ruler - Pan) / Zoom

問題: Event座標 = Screen座標 か？
     または Event座標 = Widget座標 (= Screen座標 - ウィジェット位置) か？
```

---

## 💡 **最可能性の高い原因**

### GTK4 の座標系

GTK4では、ジェスチャーに送られるイベント座標は**ウィジェット相対座標**です。

**DrawingAreaが正確にWindow左上から配置されている場合:**
```
Event座標 = Widget座標 = 正確な計算
```

**しかし、メニューバー・ツールバーがある場合:**
```
Window座標系:
  (0, 0) ┌─ウィンドウ左上
         │
         ├─ メニューバー (高さ: 0-28px程度)
         │
         ├─ ツールバー (高さ: 28-70px程度)
         │
         └─ DrawingArea領域 (ここから実際の描画が始まる)

Event座標 = DrawingArea内の相対座標（これは正確）
```

つまり、**イベント座標は理論的には正確であるはず**。

---

## 🤔 **ズレが継続する理由**

もし +30px, +40px のオフセット修正でもズレが解決しないなら：

### 可能性1: スクロール位置が考慮されていない
ScrolledWindowのスクロール値をマイナスする必要がある：
```rust
let scroll_x = hadjustment.value();
let scroll_y = vadjustment.value();
let actual_x = x - scroll_x;
let actual_y = y - scroll_y;
```

### 可能性2: Zoom/Pan の値が実際と異なる
```rust
let config = state.config.borrow();
eprintln!("Zoom: {:.2}, Pan: ({:.1}, {:.1})", config.zoom, config.pan_x, config.pan_y);
```
この値が実際のUI表示と一致しているか確認

### 可能性3: ウィジェット階層が複雑
DrawingArea → Overlay → ScrolledWindow の階層で、どのレベルでイベントが送られているか

### 可能性4: Cairo のピクセルグリッド問題
Cairo とGTKのピクセル座標が0.5px ずれている可能性

---

## ✅ **推奨アクション**

修正を戻して（+30px, +40px を削除）、以下の詳細なデバッグログを追加：

```rust
click_gesture.connect_pressed(move |gesture, n_press, x, y| {
    let state = render_state_click.clone();
    let tool_state = state.tool_state.borrow();
    let current_tool = tool_state.current_tool;
    drop(tool_state);

    if current_tool == ToolMode::Select {
        // === デバッグ情報を徹底的に出力 ===
        eprintln!("\n=== Click Event Debug Info ===");
        eprintln!("n_press: {}", n_press);
        eprintln!("Event coord (x, y): ({:.1}, {:.1})", x, y);

        let config = state.config.borrow();
        let ruler_config = state.ruler_config.borrow();
        eprintln!("Ruler size: {:.0}", ruler_config.size);
        eprintln!("Zoom: {:.2}", config.zoom);
        eprintln!("Pan: ({:.1}, {:.1})", config.pan_x, config.pan_y);
        drop(config);
        drop(ruler_config);

        // Canvas座標を計算
        let config = state.config.borrow();
        let ruler_config = state.ruler_config.borrow();
        let ruler_size = ruler_config.size;
        let zoom = config.zoom;
        let pan_x = config.pan_x;
        let pan_y = config.pan_y;

        // 変換ステップを詳細に記録
        let step1_x = x - ruler_size;
        let step1_y = y - ruler_size;
        eprintln!("Step 1 (subtract ruler): ({:.1}, {:.1})", step1_x, step1_y);

        let step2_x = step1_x - pan_x;
        let step2_y = step1_y - pan_y;
        eprintln!("Step 2 (subtract pan): ({:.2}, {:.2})", step2_x, step2_y);

        let canvas_x = step2_x / zoom;
        let canvas_y = step2_y / zoom;
        eprintln!("Step 3 (divide zoom): ({:.2}, {:.2})", canvas_x, canvas_y);

        eprintln!("Final Canvas Coord: ({:.2}, {:.2})", canvas_x, canvas_y);
        eprintln!("=== End Debug ===\n");

        drop(config);
        drop(ruler_config);

        // ... 続きは元のコード
    }
});
```

このログを見ると、どの変換ステップでズレが生じているかが明確になります。

---

**次のステップ**: 修正を一度戻して、詳細なデバッグログを実装し、実際の値を測定します。
