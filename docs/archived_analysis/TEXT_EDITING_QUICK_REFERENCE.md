# Text Editing - Quick Reference Guide

## File Locations & Key Functions

### Core Text Editing Flow

| Component | File | Key Function | Lines |
|-----------|------|--------------|-------|
| **Double-Click Handler** | `canvas/click_handlers.rs` | `DoubleClickHandler::handle_double_click()` | 184-279 |
| **Text Handler** | `canvas/text_handler.rs` | `TextHandler::prepare_text_for_editing()` | 56-89 |
| **Rich Text Editor** | `rich_text_editor.rs` | `RichTextEditor::new()` | 32-196 |
| **Document Update** | `canvas/widget_operations.rs` | `CanvasWidget::update_object_rich_text()` | 106-131 |
| **Callback Setup** | `window_setup.rs` | `canvas.set_on_edit_text()` | 520-544 |

---

## Data Flow Sequence

```
Double-Click
    ↓
DoubleClickHandler detects text object + selection
    ↓
TextHandler::prepare_text_for_editing() → RichText
    ↓
on_edit_text callback invoked
    ↓
RichTextEditor::new() creates dialog
    ↓
User edits + clicks OK
    ↓
RichTextEditor extracts RichText from buffer
    ↓
canvas.update_object_rich_text() saves to document
    ↓
on_document_modified triggers
    ↓
Canvas redraws
```

---

## Rich Text Editor Components

### Toolbar Elements

| Element | Type | Signal | Handler |
|---------|------|--------|---------|
| **Bold/Italic/Underline/Strike** | ToggleButton | `connect_toggled` | `apply_tag_to_selection()` |
| **Text Color** | ColorButton | `connect_color_set` | `apply_color_to_selection(..., false)` |
| **Background Color** | ColorButton | `connect_color_set` | `apply_color_to_selection(..., true)` |
| **Font Size** | SpinButton (6-72) | `connect_value_changed` | `apply_font_size_to_selection()` |
| **Align Left/Center/Right** | ToggleButton (mutual) | `connect_toggled` | Update `current_alignment` state |

### Keyboard Shortcuts in Editor

- **Ctrl+B** - Toggle bold on selection
- **Ctrl+I** - Toggle italic on selection
- **Ctrl+U** - Toggle underline on selection

---

## Property Panel Integration

### Selection Update Flow

```
User clicks object on canvas
    ↓
on_selection_changed callback
    ↓
PropertyPanel::update_from_selection()
    ↓
Panel widgets updated:
  - font_family_combo.set_active_id()
  - font_size_spin.set_value()
  - line_height_scale.set_value()
  - text_align_combo.set_active_id()
  - border_style_combo.set_active_id()
  - auto_resize_switch.set_active()
  - Color buttons updated with CSS
```

### Property Changes → Canvas

```
User changes font size in property panel
    ↓
font_size_spin signal: connect_value_changed
    ↓
setup_font_size_signal() handler
    ↓
obj.style.font_size = new_value
    ↓
on_style_changed callback
    ↓
Canvas redraws with new font size
```

---

## Rich Text Tag System

### Tag Creation & Management

```
Tag Names and Rules:
├── "bold"                    → weight=700
├── "italic"                  → style=Italic
├── "underline"               → underline=Single
├── "strikethrough"           → strikethrough=true
├── "fgcolor_{RRGGBB}"        → foreground color
├── "bgcolor_{RRGGBB}"        → background color
└── "fontsize_{size}"         → size in points
```

### Tag Application Process

1. Get selection bounds from buffer
2. Check tag_table for existing tag
3. Create tag if not found (and configure properties)
4. Add tag to tag_table
5. Apply tag to selection range
6. On removal: `buffer.remove_tag()`

---

## Text Content Extraction

### Buffer → RichText Conversion

```
Iterate TextBuffer from start to end:
  For each character position:
    Get tags at position
    Check for: bold, italic, underline, strikethrough
    Extract color tags (fgcolor_*, bgcolor_*)
    Extract font size (fontsize_*)
    
When tag group ends:
  Calculate byte offsets (important: convert char→byte)
  Create TextStyleRange { start, end, attributes }
  Store in rich_text.style_ranges[]
  
Result: RichText {
  text: "plain text content",
  style_ranges: [
    { start: 0, end: 4, attributes: { bold: true, ... } },
    ...
  ],
  alignment: TextAlignment::Left
}
```

---

## State Management

### RichTextEditor State Variables

```rust
blocking_signals: Rc<RefCell<bool>>
  └─ Prevents feedback loops during programmatic updates

current_attributes: Rc<RefCell<TextAttributes>>
  └─ Tracks current formatting (declared but underutilized)

current_alignment: Rc<RefCell<TextAlignment>>
  └─ Paragraph alignment state (independent of selection)
```

### PropertyPanel State Variables

```rust
current_object: Rc<RefCell<Option<Object>>>
  └─ Stores selected object with all properties

on_style_changed: Rc<RefCell<OnStyleChanged>>
  └─ Callback when style properties updated

on_layer_command: Rc<RefCell<OnLayerCommand>>
  └─ Z-order operations (bring to front, etc)

on_alignment_command: Rc<RefCell<OnAlignmentCommand>>
  └─ Object alignment commands (left, center, etc)

on_auto_resize_changed: Rc<RefCell<OnAutoResizeChanged>>
  └─ Text auto-resize height toggle
```

---

## Critical Implementation Details

### Text Handler Validation

```rust
// Minimum drag size for text object creation
const MIN_TEXT_SIZE: f64 = 20.0;

// Validates drag rectangle is large enough
TextHandler::validate_text_drag(width, height)
  → width > 20.0 && height > 20.0
```

### Double-Click Coordinate Adjustment

```rust
// Account for ruler offset
const RULER_SIZE: f64 = 20.0;
let adjusted_x = x - RULER_SIZE;
let adjusted_y = y - RULER_SIZE;
```

### Auto-Resize Behavior

```
When auto_resize_height = true:
  After text edit:
    canvas.update_object_auto_resize(idx)
      ↓
    Triggers redraw with text measurement
      ↓
    GTK calculates required height for text width
      ↓
    Object height updated automatically
```

---

## What's Missing from Current Rust Version

### High Priority

- [ ] RichTextEditor modal dialog implementation
- [ ] TextHandler utility module
- [ ] Double-click detection + callback mechanism
- [ ] Text extraction from buffer with tag parsing
- [ ] Tag application when showing existing text

### Medium Priority

- [ ] Cursor-aware formatting button state detection
- [ ] Format painter / copy format functionality
- [ ] Individual text change undo support
- [ ] Text alignment enum with justify support

### Low Priority

- [ ] Rich text preview in property panel
- [ ] Inline text editing (F2 key)
- [ ] Text placeholder support
- [ ] Clipboard integration for styled text

---

## Debugging Checklist

When implementing text editing in Rust:

1. **Double-click not triggering edit?**
   - Check GestureClick n_press == 2
   - Verify selected_indices has text object
   - Check ObjectType::TextFrame check

2. **Text not persisting after edit?**
   - Verify update_object_rich_text updates document
   - Check on_document_modified callback fires
   - Confirm drawing_area.queue_draw() called

3. **Formatting lost after edit?**
   - Check tag_table.lookup() finds existing tags
   - Verify TextStyleRange byte offsets correct
   - Test character→byte offset conversion

4. **Property panel not updating?**
   - Check on_selection_changed callback setup
   - Verify update_from_selection called
   - Check current_object borrow state

5. **Keyboard shortcuts not working?**
   - Verify EventControllerKey added to text_view
   - Check modifier key parsing (CONTROL_MASK)
   - Test key code matching (gdk::Key::b, etc)

---

## Performance Considerations

1. **Tag Reuse**: Tags are looked up/created once and reused
2. **Signal Blocking**: Use blocking_signals to prevent loops
3. **Buffer Iteration**: Character-by-character for extraction could be slow on large texts
4. **Borrow Scope**: Explicit drop() statements ensure borrows released promptly

---

## File Structure Summary

```
testruct-desktop/crates/gtkapp/src/
├── rich_text_editor.rs
│   ├── RichTextEditor struct
│   ├── Dialog setup & toolbar
│   ├── Signal handlers
│   ├── Text extraction logic
│   └── Text application logic
├── canvas/
│   ├── click_handlers.rs
│   │   └── DoubleClickHandler
│   ├── text_handler.rs
│   │   ├── Text validation
│   │   ├── Text creation
│   │   ├── Text preparation for editing
│   │   └── Text content update
│   └── widget_operations.rs
│       └── update_object_rich_text()
├── property/
│   ├── mod.rs
│   │   └── PropertyPanel struct
│   ├── panel_state.rs
│   │   └── Selection update logic
│   └── panel_signals.rs
│       └── Property change handlers
└── window_setup.rs
    └── Text editing callback setup
```

