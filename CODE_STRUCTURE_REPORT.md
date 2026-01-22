# Code Structure Check Report

**Date**: 2026-01-22
**Analyst**: Worker3
**Total Files Analyzed**: 28,211 lines across all .rs files

---

## 500+ Lines Files (14 files)

| File | Lines | Category | Recommended Action |
|------|-------|----------|-------------------|
| undo_redo/app_commands.rs | 875 | Logic | Split by command type |
| canvas/input/gesture_drag.rs | 700 | Input | Consider splitting drag modes |
| export/image.rs | 679 | Export | Split PNG/JPEG into separate files |
| canvas/mod.rs | 634 | UI | Extract draw functions to sub-module |
| panels/property_handlers_text.rs | 633 | UI | Group related handlers |
| tests/page_thumbnail.rs | 626 | Test | OK (test file) |
| tests/undo_redo_integration.rs | 578 | Test | OK (test file) |
| panels/properties_groups.rs | 567 | UI | Consider splitting by panel type |
| tests/export.rs | 541 | Test | OK (test file) |
| app/state.rs | 540 | Core | Extract helper methods |
| window/actions/alignment_actions.rs | 530 | Actions | OK (single responsibility) |
| export/svg.rs | 514 | Export | OK (single format) |
| canvas/snapping.rs | 509 | Logic | OK (single responsibility) |
| toolbar/mod.rs | 502 | UI | Consider extracting button builders |

---

## Responsibility Analysis

### High Priority (Multiple Responsibilities)

#### 1. `undo_redo/app_commands.rs` (875 lines)
**Issue**: 8 different command types in single file
- AppDeleteCommand
- AppMoveCommand
- AppCreateCommand
- AppGroupCommand
- AppUngroupCommand
- AppPropertyChangeCommand
- AppStrokeWidthCommand
- AppResizeCommand

**Recommendation**: Split into separate files per command type
```
undo_redo/
├── mod.rs
├── commands/
│   ├── delete.rs
│   ├── move_cmd.rs
│   ├── create.rs
│   ├── group.rs
│   ├── property.rs
│   └── resize.rs
```

#### 2. `panels/property_handlers_text.rs` (633 lines)
**Issue**: 11+ signal handlers in single file
- Font family, size, style handlers
- Color handlers
- Alignment handlers
- Line height handlers

**Recommendation**: Group by functionality
```
panels/
├── property_handlers/
│   ├── text_font.rs (family, size, style)
│   ├── text_color.rs (text color, background)
│   ├── text_layout.rs (alignment, line height)
```

### Medium Priority (Large but Single Responsibility)

#### 3. `canvas/mod.rs` (634 lines)
**Status**: Single struct (CanvasView) with many methods
**Recommendation**: Extract drawing logic to `canvas/canvas_draw.rs`

#### 4. `app/state.rs` (540 lines)
**Status**: Single struct (AppState) - core state management
**Recommendation**: Consider extracting document manipulation methods

#### 5. `export/image.rs` (679 lines)
**Status**: PNG and JPEG export combined
**Recommendation**: Split to `export/png.rs` and `export/jpeg.rs`

### Low Priority (Acceptable)

| File | Reason |
|------|--------|
| alignment_actions.rs | Single responsibility (alignment) |
| svg.rs | Single format export |
| snapping.rs | Single responsibility (snap logic) |
| gesture_drag.rs | Related drag operations |
| Test files | Integration tests naturally large |

---

## Module Structure Overview

```
crates/
├── core/          # 598 lines - Well structured
├── db/            # 24,820 lines - Single module (item_bank)
├── cli/           # 1,251 lines - Minimal
└── ui/            # 27,562 lines - Needs attention
    ├── app/       # state management
    ├── canvas/    # rendering & input (LARGEST)
    ├── export/    # file export
    ├── panels/    # property panels
    ├── undo_redo/ # command pattern
    └── window/    # actions & menu
```

---

## Metrics Summary

| Metric | Value |
|--------|-------|
| Total .rs files | ~80 |
| Files > 500 lines | 14 (17.5%) |
| Files > 300 lines | 25 (31%) |
| Average file size | ~350 lines |
| Largest file | 875 lines |

---

## Next Phase Recommendations

### Phase 1: High Impact (Effort: Medium)
1. **Split app_commands.rs** - Improves maintainability
2. **Organize property_handlers** - Clearer structure

### Phase 2: Code Quality (Effort: Low)
3. **Extract canvas draw functions** - Better testability
4. **Split export/image.rs** - Cleaner separation

### Phase 3: Optional Improvements (Effort: High)
5. **Refactor AppState** - Extract helper traits
6. **Standardize test organization** - Group by feature

---

## Risk Assessment

| Change | Risk | Benefit |
|--------|------|---------|
| Split app_commands | Low | High |
| Split property_handlers | Low | Medium |
| Refactor canvas/mod | Medium | Medium |
| Refactor AppState | High | Medium |

---

## Conclusion

The codebase is generally well-organized with clear module boundaries. The main areas for improvement are:

1. **undo_redo/app_commands.rs** - Primary candidate for splitting
2. **panels/property_handlers_text.rs** - Would benefit from grouping
3. **Export module** - Minor improvement opportunity

All recommendations are non-urgent and can be addressed in future maintenance phases.
