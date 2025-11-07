# Text Editing System Analysis - Original testruct-desktop Application

## Overview

The text editing system in the original application follows a modal dialog-based approach with rich text formatting support. The flow integrates deeply between canvas interaction, dialog editing, and property panel management.

---

## 1. Text Editing Flow Architecture

### Entry Point: Double-Click on Canvas

**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/canvas/click_handlers.rs`

```
User double-clicks text object
         ↓
DoubleClickHandler::handle_double_click() [lines 184-279]
         ↓
TextHandler::prepare_text_for_editing() [prepares RichText from document]
         ↓
on_edit_text callback triggered [in window_setup.rs line 520]
         ↓
RichTextEditor dialog opened with rich text content
         ↓
User edits and clicks OK
         ↓
RichTextEditor returns RichText with formatting
         ↓
canvas.update_object_rich_text() updates document
         ↓
on_document_modified callback notifies UI
```

### Key Files in Text Editing Pipeline:

1. **Canvas Click Handler** - `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/canvas/click_handlers.rs`
   - Detects double-click on text objects
   - Validates text object is selected
   - Calls callback with object index and RichText

2. **Rich Text Editor Dialog** - `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/rich_text_editor.rs`
   - Full-featured modal dialog with formatting toolbar
   - Supports bold, italic, underline, strikethrough
   - Text color and background color selection
   - Font size adjustment
   - Text alignment (left, center, right)
   - Rich text extraction and application using GTK TextBuffer tags

3. **Text Handler** - `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/canvas/text_handler.rs`
   - Utility functions for text object lifecycle
   - Validates text drag operations (minimum 20px size)
   - Creates new text objects
   - Prepares existing text for editing (converts to RichText)
   - Updates text content in document

4. **Window Setup** - `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/window_setup.rs` (lines 516-544)
   - Sets up the text editing callback
   - Creates RichTextEditor dialog instance
   - Updates canvas document on successful edit
   - Triggers auto-resize if enabled

---

## 2. Rich Text Editor Implementation Details

### Dialog Structure
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/rich_text_editor.rs` (lines 13-28)

```rust
pub struct RichTextEditor {
    dialog: Dialog,
    text_view: TextView,
    
    // Format buttons
    bold_btn: ToggleButton,
    italic_btn: ToggleButton,
    underline_btn: ToggleButton,
    strikethrough_btn: ToggleButton,
    
    // Color controls
    color_btn: ColorButton,      // Text color
    bg_color_btn: ColorButton,   // Background color
    
    // Typography
    font_size_spin: SpinButton,  // 6-72pt range
    
    // Alignment (mutual exclusivity)
    align_left_btn: ToggleButton,
    align_center_btn: ToggleButton,
    align_right_btn: ToggleButton,
    
    // State tracking
    current_alignment: Rc<RefCell<TextAlignment>>,
    current_attributes: Rc<RefCell<TextAttributes>>,
}
```

### Signal Handlers
**Lines 199-351:**

1. **Bold/Italic/Underline/Strikethrough** (lines 206-237)
   - Signal: `connect_toggled`
   - Handler: `apply_tag_to_selection()`
   - Action: Applies or removes text tag from selected text

2. **Text Color** (lines 241-250)
   - Signal: `connect_color_set`
   - Handler: `apply_color_to_selection(..., false)`
   - Creates unique tag: `fgcolor_{hex}`

3. **Background Color** (lines 254-265)
   - Signal: `connect_color_set`
   - Handler: `apply_color_to_selection(..., true)`
   - Creates unique tag: `bgcolor_{hex}`

4. **Font Size** (lines 268-275)
   - Signal: `connect_value_changed`
   - Handler: `apply_font_size_to_selection()`
   - Creates unique tag: `fontsize_{size}`

5. **Alignment Buttons** (lines 277-318)
   - Mutual exclusivity logic
   - Updates `current_alignment` state
   - Stored in buffer but not applied as tag (applied on render)

6. **Keyboard Shortcuts** (lines 320-351)
   - Ctrl+B: Toggle bold
   - Ctrl+I: Toggle italic
   - Ctrl+U: Toggle underline
   - EventControllerKey captures shortcuts in text_view

### Rich Text Extraction
**Lines 534-645:**

The editor extracts formatting by:

1. Iterating through TextBuffer from start to end
2. At each position, checking which tags are applied
3. Collecting tag information:
   - Bold: weight=700
   - Italic: style=Italic
   - Underline: underline=Single
   - Strikethrough: strikethrough=true
   - Colors: fgcolor_{hex}, bgcolor_{hex}
   - Font size: fontsize_{size}

4. Building `TextStyleRange` objects with byte offsets
5. Creating `RichText` struct with:
   - Plain text content
   - Array of style ranges with formatting attributes
   - Text alignment

### Rich Text Application
**Lines 648-760:**

When displaying existing RichText in editor:

1. Sets TextBuffer text to plain text content
2. Converts byte offsets to character offsets
3. For each style range, applies corresponding tags
4. Tag table lookup or creation on demand
5. Reuses existing tags to avoid duplication

---

## 3. Keyboard Input Handling for Text Editing

### Double-Click Detection
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/canvas/widget_events_mouse.rs` (lines ~400+)

```rust
let double_click_gesture = GestureClick::new();
double_click_gesture.set_button(gdk::BUTTON_PRIMARY);
double_click_gesture.connect_released(move |_gesture, n_press, x, y| {
    if n_press == 2 {
        DoubleClickHandler::handle_double_click(
            x, y, &document, &selected_indices, 
            &drawing_area, &on_edit_text, 
            &on_document_modified
        );
    }
});
```

### Text Editing Keyboard Shortcuts
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/rich_text_editor.rs` (lines 320-351)

Within the RichTextEditor dialog:
- **Ctrl+B**: Toggle bold on selection
- **Ctrl+I**: Toggle italic on selection  
- **Ctrl+U**: Toggle underline on selection
- **Regular typing**: Inserts text in TextView
- **Selection**: Supported via standard text selection (Shift+Arrows, mouse drag)

### Canvas Keyboard Shortcuts (Non-Text Editing)
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/canvas/widget_events_keyboard.rs`

- **T key**: Switch to Text tool (line 255-262)
- **V key**: Switch to Select tool (line 246-252)
- No direct text editing via keyboard on canvas (must double-click to open editor)

---

## 4. Property Panel Integration

### Property Panel State Management
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/property/panel_state.rs`

#### Selection Update (lines 13-98)
When an object is selected on canvas:

```rust
pub fn update_from_selection(
    current_object: &Rc<RefCell<Option<Object>>>,
    object: &Object,
    font_family_combo, font_size_spin, line_height_scale,
    text_align_combo, border_style_combo, auto_resize_switch, ...
) {
    *current_object.borrow_mut() = Some(object.clone());
    
    // Update all UI controls from object properties:
    font_family_combo.set_active_id(font_id);
    font_size_spin.set_value(object.style.font_size);
    line_height_scale.set_value(object.style.line_height);
    text_align_combo.set_active_id(align_id);
    border_style_combo.set_active_id(border_id);
    auto_resize_switch.set_active(object.auto_resize_height);
    stroke_width_spin.set_value(object.style.stroke_width);
    // Update color buttons with CSS styling
    update_fill_color_button(...);
    update_stroke_color_button(...);
}
```

**Data Flow:**
- Selected object → stored in `current_object` RefCell
- Each property updates corresponding UI widget
- Color buttons get CSS background styling

### Property Panel Signal Handlers
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/property/panel_signals.rs`

#### Font Family (lines 91-127)
```rust
fn setup_font_family_signal(...) {
    font_family_combo.connect_changed(move |combo| {
        if let Some(font) = combo.active_id() {
            if let Some(ref mut obj) = *current_object.borrow_mut() {
                obj.style.font_family = font_name;
                if let Some(callback) = on_style_changed.borrow().as_ref() {
                    callback(obj.style.clone());
                }
            }
        }
    });
}
```

**Critical Note:** Changes in property panel directly modify `current_object` and trigger `on_style_changed` callback.

#### Font Size (lines 130-147)
- Updates `obj.style.font_size`
- Triggers immediate callback with style

#### Line Height (lines 150-167)
- Updates `obj.style.line_height`
- Triggers immediate callback

#### Text Alignment (lines 210-224)
- ComboBox with options: left, center, right, justify
- Maps to `TextAlign` enum
- Updated from `object.style.text_align` on selection

#### Auto-Resize Switch (lines 195-212)
```rust
fn setup_auto_resize_signal(...) {
    auto_resize_switch.connect_active_notify(move |switch| {
        let active = switch.is_active();
        if let Some(ref mut obj) = *current_object.borrow_mut() {
            obj.auto_resize_height = active;
            if let Some(callback) = on_auto_resize_changed.borrow().as_ref() {
                callback(active);
            }
        }
    });
}
```

#### Color Buttons (lines 235-326)
- Async dialog using `ColorDialog::choose_rgba_future()`
- Updates style color and triggers callback

### Property Panel Callback Setup
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/window_setup.rs`

Property panel callbacks are set up to:
1. Trigger canvas redraw
2. Mark document as modified
3. Update UI elements

---

## 5. Text State and Cursor Tracking

### State Management in Rich Text Editor
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/rich_text_editor.rs`

#### Blocking Signals (line 201)
```rust
let blocking_signals = Rc::new(RefCell::new(false));
```
Prevents feedback loops when programmatically updating button states based on cursor position.

#### Current Attributes (line 166)
```rust
let current_attributes = Rc::new(RefCell::new(TextAttributes::default()));
```
Tracks formatting attributes but NOT actively used for cursor-aware formatting in this implementation.

#### Current Alignment (line 167)
```rust
let current_alignment = Rc::new(RefCell::new(initial_alignment));
```
Stores paragraph alignment state independent of selected text.

### Cursor Position Tracking
**Missing Feature:** The current implementation does NOT track cursor position for automatic formatting detection. 

Issues:
1. No cursor_position state variable
2. Formatting buttons don't update based on cursor position
3. User must manually select text and apply formatting
4. No "current formatting" indicator in toolbar

### TextBuffer Tag System
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/rich_text_editor.rs` (lines 447-476)

```rust
fn apply_tag_to_selection(text_view: &TextView, tag_name: &str, apply: bool) {
    let buffer = text_view.buffer();
    let tag_table = buffer.tag_table();
    
    if let Some((start, end)) = buffer.selection_bounds() {
        // Get or create tag
        let tag = if let Some(existing_tag) = tag_table.lookup(tag_name) {
            existing_tag
        } else {
            let new_tag = gtk4::TextTag::new(Some(tag_name));
            match tag_name {
                "bold" => new_tag.set_weight(700),
                "italic" => new_tag.set_style(gtk4::pango::Style::Italic),
                "underline" => new_tag.set_underline(gtk4::pango::Underline::Single),
                "strikethrough" => new_tag.set_strikethrough(true),
                _ => {}
            }
            tag_table.add(&new_tag);
            new_tag
        };
        
        // Apply or remove
        if apply {
            buffer.apply_tag(&tag, &start, &end);
        } else {
            buffer.remove_tag(&tag, &start, &end);
        }
    }
}
```

---

## 6. Text Content Propagation

### From Editor to Canvas
**File:** `/Users/ken/Desktop/testruct-desktop/crates/gtkapp/src/window_setup.rs` (lines 520-544)

```
RichTextEditor dialog shows
    ↓
User edits text and formatting
    ↓
User clicks OK
    ↓
editor.show(move |rich_text_opt| { ... })
    ↓
RichTextEditor extracts RichText from buffer
    ↓
callback invoked with RichText
    ↓
canvas.update_object_rich_text(idx, rich_text)
    ↓
Document updated: obj.content = ObjectContent::RichText(rich_text)
    ↓
canvas.update_object_auto_resize(idx)
    ↓
on_document_modified callback triggered
    ↓
Drawing area queued for redraw
```

### From Property Panel to Canvas
**No direct text editing in property panel.** The property panel:
- Displays object properties (font, size, line height)
- Updates style properties when user changes them
- Does NOT provide text content editor
- Text content always edited via RichTextEditor dialog

---

## 7. Missing Features in Current Version

Based on comparing original and current Rust version:

### 1. Modal Dialog-Based Text Editing
- Original: Uses modal RichTextEditor dialog
- Current (Testruct.app): No RichTextEditor found in Rust codebase
- **Missing:** Rich text formatting toolbar and dialog

### 2. Text Content Display in Properties
- Original: Text content edited only in modal dialog
- Current: No text display in property panel
- **Missing:** Text content field in property panel

### 3. Cursor-Aware Formatting
- Original: Buttons don't reflect current formatting at cursor
- Current: Same limitation
- **Missing:** Automatic formatting button state update

### 4. Text Alignment
- Original: Full support (left, center, right, justify)
- Current: May lack justify option
- **Note:** Check TextAlign enum in core

### 5. Background Color Support
- Original: Full support in RichTextEditor
- Current: Check if implemented
- **Note:** Important for highlighting/annotation use cases

### 6. Keyboard-Driven Text Editing on Canvas
- Original: Double-click only, no inline editing
- Current: Same limitation
- **Missing:** Inline text editing mode (F2 key)

---

## 8. Signal Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    CANVAS LAYER                             │
├─────────────────────────────────────────────────────────────┤
│ Double-click on text object                                 │
│         ↓                                                    │
│ DoubleClickHandler::handle_double_click()                   │
│   - Validates object is TextFrame                           │
│   - Gets object index from selected_indices                 │
│   - Calls TextHandler::prepare_text_for_editing()           │
│         ↓                                                    │
└─────────────────────────────────────────────────────────────┘
         │
         ↓
┌─────────────────────────────────────────────────────────────┐
│              WINDOW SETUP / CALLBACK LAYER                  │
├─────────────────────────────────────────────────────────────┤
│ on_edit_text callback invoked with (idx, RichText)          │
│         ↓                                                    │
│ Creates RichTextEditor dialog                               │
│   - Initializes TextBuffer with RichText                    │
│   - Applies existing formatting tags                        │
│   - Sets alignment buttons from alignment state             │
│         ↓                                                    │
└─────────────────────────────────────────────────────────────┘
         │
         ↓
┌─────────────────────────────────────────────────────────────┐
│             RICH TEXT EDITOR DIALOG LAYER                   │
├─────────────────────────────────────────────────────────────┤
│ User edits text and formatting                              │
│   - Types text into TextView                                │
│   - Selects text (mouse/keyboard)                           │
│   - Clicks format buttons:                                  │
│      • Bold/Italic/Underline/Strikethrough                  │
│      • Text Color / Background Color                        │
│      • Font Size                                            │
│      • Text Alignment buttons                               │
│   - Keyboard shortcuts: Ctrl+B, Ctrl+I, Ctrl+U             │
│         ↓                                                    │
│ User clicks OK button                                       │
│         ↓                                                    │
│ RichTextEditor::show() callback triggered                   │
│   - Extracts TextBuffer content                             │
│   - Iterates through buffer to find formatting tags         │
│   - Builds TextStyleRange array                             │
│   - Creates RichText struct                                 │
│         ↓                                                    │
└─────────────────────────────────────────────────────────────┘
         │
         ↓
┌─────────────────────────────────────────────────────────────┐
│               DOCUMENT UPDATE LAYER                         │
├─────────────────────────────────────────────────────────────┤
│ canvas.update_object_rich_text(idx, rich_text)              │
│   - Borrows document mutably                                │
│   - Updates object.content = RichText(rich_text)            │
│   - Releases borrow                                         │
│         ↓                                                    │
│ canvas.update_object_auto_resize(idx)                       │
│   - Checks auto_resize_height flag                          │
│   - Queues redraw if enabled                                │
│         ↓                                                    │
│ on_document_modified callback                               │
│   - Marks document as modified                              │
│   - Updates window title                                    │
│   - Marks file as unsaved                                   │
│         ↓                                                    │
│ drawing_area.queue_draw()                                   │
│   - Canvas will render updated text on next frame           │
│         ↓                                                    │
└─────────────────────────────────────────────────────────────┘
         │
         ↓
┌─────────────────────────────────────────────────────────────┐
│           PROPERTY PANEL (OPTIONAL UPDATE)                  │
├─────────────────────────────────────────────────────────────┤
│ If on_selection_changed callback triggered:                 │
│   - Updates property panel from current_object              │
│   - Reflects any style changes                              │
│         ↓                                                    │
└─────────────────────────────────────────────────────────────┘
```

---

## 9. Key Architectural Observations

### Strengths

1. **Clean Separation of Concerns**
   - Text handler manages lifecycle
   - Editor dialog handles formatting
   - Canvas manages document state
   - Property panel handles display properties

2. **Rich Text Support**
   - Full formatting toolbar
   - Multiple color controls
   - Font size adjustment
   - Text alignment options
   - Tag-based formatting (reusable)

3. **Undo/Redo Potential**
   - Although not shown in editor code, framework supports it
   - Document modifications trigger callbacks

4. **Modal Approach**
   - Prevents accidental lose of editing context
   - Clear on/off state
   - No competing input modes

### Weaknesses

1. **No Inline Text Editing**
   - Must double-click to enter edit mode
   - No F2 or click-to-edit
   - No inline placeholder text

2. **Missing Cursor-Aware Features**
   - Formatting buttons don't reflect cursor position
   - No format painter / copy format
   - No format bar at cursor

3. **No Text-Only Binding**
   - Property panel doesn't show text content
   - Can't edit text from properties
   - Must open full dialog for any text changes

4. **Limited Undo for Text**
   - Rich text extraction discards undo history
   - Can't undo individual formatting changes within editor
   - Only whole text replacement is undoable

5. **No Rich Text Preview**
   - Property panel can't show formatted text preview
   - User must open dialog to see final result

---

## 10. Implementation Checklist for Rust Version

To fully replicate text editing in current Rust version:

- [ ] Implement RichTextEditor modal dialog
- [ ] Add TextHandler utility functions
- [ ] Set up double-click detection on canvas
- [ ] Implement on_edit_text callback mechanism
- [ ] Create text content update path
- [ ] Add property panel text field (optional)
- [ ] Implement keyboard shortcut handling
- [ ] Add rich text tag system
- [ ] Set up formatting buttons with signals
- [ ] Implement text extraction from buffer
- [ ] Add alignment state management
- [ ] Create color selection dialogs
- [ ] Test auto-resize functionality
- [ ] Implement undo support for text changes

---

## Summary

The original application's text editing system is **dialog-modal, GTK-based, and feature-complete**. It provides:

1. Double-click to edit text
2. Full formatting toolbar (bold, italic, underline, strikethrough)
3. Color customization (text and background)
4. Font size adjustment (6-72pt)
5. Text alignment options
6. Auto-resize height support
7. Integration with property panel for style management
8. Document state tracking

The implementation is clean but could benefit from:
- Cursor-aware formatting detection
- Inline text editing mode
- Text preview in properties
- Individual text change undo support
