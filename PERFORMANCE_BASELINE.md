# Performance Baseline Report

**Date**: 2026-01-22
**Analyst**: Worker3
**Project**: testruct-desktop-Rust

---

## 1. Build Performance

### Clean Build Time
| Metric | Value |
|--------|-------|
| Total Time | 1m 25.76s |
| User CPU | 337.53s |
| System CPU | 16.63s |
| CPU Utilization | 412% (4-core parallel) |
| Files Generated | 8,350 |
| Build Size | 2.2 GiB |

### Incremental Build Time
- Release profile rebuild (no changes): ~0.08s
- Initial release build (cached deps): ~40.32s

---

## 2. Clippy Warnings Analysis

### Summary
| Crate | Warnings | Auto-fixable |
|-------|----------|--------------|
| testruct-core | 7 | 7 |
| testruct-ui | 83 | 65 |
| testruct-db | 1 | 0 |
| Tests | 4 | 3 |
| **Total** | **95** | **75** |

### Critical Issues (Priority: High)

#### 2.1 Arc<T> Send/Sync Problem
**Locations**:
- `crates/ui/src/app/state.rs:25` - `Arc<Mutex<AppShared>>`
- `crates/ui/src/app/state.rs:29` - `Arc<Mutex<UndoRedoStack>>`
- `crates/ui/src/dialogs/template_browser.rs:162` - `Arc<Mutex<Option<Box<dyn Fn>>>>`

**Impact**:
- GTK4 is single-threaded, so using `Arc` for non-Send/Sync types is wasteful
- Memory overhead and unnecessary atomic operations

**Recommendation**:
```rust
// Replace Arc with Rc for single-threaded GTK contexts
// Before:
inner: Arc::new(Mutex::new(AppShared { ... }))
// After:
inner: Rc::new(RefCell::new(AppShared { ... }))
```

#### 2.2 Complex Type Definition
**Location**: `crates/ui/src/canvas/input/ime/mod.rs:22`
```rust
text_insertion_callback: Rc<RefCell<Option<Box<dyn Fn(String)>>>>
```

**Recommendation**:
```rust
// Create type alias for readability
type TextInsertionCallback = Rc<RefCell<Option<Box<dyn Fn(String)>>>>;
```

#### 2.3 Thread-local Initialization
**Location**: `crates/ui/src/window/actions/clipboard_actions.rs:16`
```rust
// Current:
static CLIPBOARD_CONTENT: RefCell<Option<String>> = RefCell::new(None);
// Recommended:
static CLIPBOARD_CONTENT: RefCell<Option<String>> = const { RefCell::new(None) };
```

**Impact**: Slight startup performance improvement

### Medium Priority Issues

#### 2.4 Clone on Copy Types (30+ instances)
**Pattern**: Using `.clone()` on types that implement `Copy`

**Files affected**:
- `crates/ui/src/canvas/input/gesture_click.rs` - 6 instances
- `crates/ui/src/canvas/input/gesture_drag.rs` - 8 instances
- `crates/ui/src/undo_redo/app_commands.rs` - 10 instances
- `crates/ui/src/panels/property_handlers.rs` - 3 instances

**Example fix**:
```rust
// Before:
let old_bounds = text.bounds.clone();
// After:
let old_bounds = text.bounds;
```

#### 2.5 Derivable Default Implementations (5 instances)
**Files**:
- `crates/core/src/document/page_size.rs:25`
- `crates/ui/src/canvas/grid_rendering.rs:204`
- `crates/ui/src/export/mod.rs:56, 103`

**Example**:
```rust
// Before:
impl Default for GridStyle {
    fn default() -> Self { GridStyle::Dots }
}
// After:
#[derive(Default)]
pub enum GridStyle {
    Lines,
    #[default]
    Dots,
}
```

#### 2.6 Manual Clamp Patterns (5 instances)
**Files**:
- `crates/ui/src/canvas/shapes_rendering.rs:149`
- `crates/ui/src/clipboard.rs:142-143`
- `crates/ui/src/window/actions/view_actions.rs:154, 219`

**Example**:
```rust
// Before:
let arrow_length = (stroke_width as f64 * 4.0).max(8.0).min(24.0);
// After:
let arrow_length = (stroke_width as f64 * 4.0).clamp(8.0, 24.0);
```

### Low Priority Issues

#### 2.7 Let Unit Value Bindings (17 instances)
**Pattern**: `let _ = widget.queue_draw();`

**Files**: Mostly in `crates/ui/src/window/actions/*.rs`

**Recommendation**: Remove unnecessary `let _` bindings
```rust
// Before:
let _ = drawing_area.queue_draw();
// After:
drawing_area.queue_draw();
```

#### 2.8 Assertions on Constants (5 instances)
**Files**:
- `crates/ui/src/export/image.rs:672`
- `crates/ui/src/export/pdf.rs:375`
- `crates/ui/src/export/svg.rs:512`
- `crates/ui/src/export/mod.rs:308`
- `crates/db/src/lib.rs:27`

**Issue**: `assert!(true)` is optimized out by the compiler

---

## 3. Rendering Efficiency Analysis

### Current Architecture
```
CanvasView
├── ScrolledWindow (container)
│   └── Overlay
│       ├── DrawingArea (main canvas)
│       └── Entry (IME input)
└── CanvasRenderState
    ├── RenderConfig
    ├── RulerConfig
    ├── ToolState
    └── DirtyRegionTracker (NOT USED)
```

### Rendering Pipeline Issues

#### 3.1 Full Redraw on Every Change
**Problem**: Every user interaction triggers `queue_draw()` which redraws the entire canvas.

**Evidence** (from mod.rs):
- Line 132: `drawing_area_ime.queue_draw()` - IME text change
- Line 150: `drawing_area_activate.queue_draw()` - Enter key
- Line 191: `drawing_area_key.queue_draw()` - Escape key
- Lines 582, 593, 602, 612, 621, 632: Selection changes

**Impact**: Poor performance with many objects on canvas

#### 3.2 DirtyRegion Not Utilized
**Problem**: `DirtyRegionTracker` exists in `CanvasRenderState` but is never used in the actual draw function.

**Location**: `dirty_region.rs` has full implementation but `draw_canvas()` in `mod.rs` ignores it.

**Current flow**:
1. User action occurs
2. `queue_draw()` called
3. Entire canvas redrawn

**Expected flow**:
1. User action occurs
2. Mark affected region as dirty
3. Only redraw dirty regions

#### 3.3 No Caching for Static Elements
**Problem**: Grid, rulers, and page background are redrawn every frame even when unchanged.

---

## 4. Improvement Recommendations

### High Priority (Performance Impact: High)

| # | Issue | Effort | Impact |
|---|-------|--------|--------|
| 1 | Implement dirty region rendering | 2-3 days | High |
| 2 | Replace Arc with Rc for GTK state | 0.5 day | Medium |
| 3 | Fix clone_on_copy warnings | 0.5 day | Low |

### Medium Priority (Code Quality)

| # | Issue | Effort | Impact |
|---|-------|--------|--------|
| 4 | Add type aliases for complex types | 1 hour | Readability |
| 5 | Fix derivable_impls warnings | 1 hour | Compile time |
| 6 | Use clamp() function | 1 hour | Clarity |

### Low Priority (Polish)

| # | Issue | Effort | Impact |
|---|-------|--------|--------|
| 7 | Remove let_unit_value bindings | 30 min | Code cleanliness |
| 8 | Fix doc link references | 30 min | Documentation |
| 9 | Remove assert!(true) | 15 min | Code cleanliness |

---

## 5. Quick Wins (Auto-fixable)

Run the following to auto-fix 75 warnings:
```bash
cargo clippy --fix --all-targets --allow-dirty
```

**Caution**: Review changes before committing, especially:
- `clone()` removals may affect ownership semantics
- `Default` derive changes require testing

---

## 6. Rendering Optimization Roadmap

### Phase 1: Enable Dirty Region Tracking
1. Modify `draw_canvas()` to check `dirty_region.needs_full_redraw()`
2. If false, use Cairo clip region to limit drawing
3. Mark regions dirty on element changes

### Phase 2: Layer Caching
1. Cache static layers (grid, rulers) to off-screen surface
2. Only redraw when zoom/pan changes
3. Composite cached layers with dynamic content

### Phase 3: Element Caching
1. Cache individual element renders
2. Invalidate cache on element modification
3. Use texture atlases for common shapes

### Expected Performance Gains
| Optimization | Estimated Improvement |
|--------------|----------------------|
| Dirty regions | 30-50% fewer draw calls |
| Layer caching | 40-60% faster grid/ruler |
| Element caching | 20-40% faster complex docs |

---

## 7. Test Results

```bash
cargo test --all-targets
# All tests pass (prerequisite verified)
```

---

## Appendix: Clippy Command Output Summary

```
testruct-core: 7 warnings (all auto-fixable)
testruct-ui: 83 warnings (65 auto-fixable)
testruct-db: 1 warning
tests: 4 warnings (3 auto-fixable)
```

Full clippy output available via:
```bash
cargo clippy --all-targets 2>&1
```
