# Japanese IME Implementation Guide for Testruct (GTK4/Rust)

## Executive Summary

**Status**: IME input is NOT currently implemented in Testruct
**Current Limitation**: `keyval.to_unicode()` only handles direct keyboard input
**Effort**: Medium (2-3 days for full implementation)
**Complexity**: Medium - requires integrating GTK4's IMMulticontext with existing keyboard event handler

---

## 1. CURRENT CODEBASE ANALYSIS

### 1.1 Existing Keyboard Implementation

**Location**: `/crates/ui/src/canvas/input/keyboard/`

**Current Architecture**:
- `keyboard/mod.rs`: Main event handler using `EventControllerKey`
- `keyboard/text_editing_keys.rs`: Character insertion via `keyval.to_unicode()`
- `keyboard/text_alignment_keys.rs`: Text alignment shortcuts
- `gesture_click.rs`: Double-click to enter text editing mode

**Key Finding - CRITICAL LIMITATION**:
```rust
// Line 218 in text_editing_keys.rs
if let Some(ch) = keyval.to_unicode() {
    // Accept any printable character (not just ASCII)
    if !ch.is_control() {
        // Insert character
    }
}
```

**Problem**: `keyval.to_unicode()` ONLY works with:
- Direct keyboard input (ASCII, Latin, accented characters)
- DOES NOT receive preedit text from IME composition
- DOES NOT work with multi-key compositions (Japanese, Chinese, Korean)

### 1.2 Focus Management

**Location**: `/crates/ui/src/canvas/input/gesture_click.rs` (line 148)
```rust
drawing_area_click.grab_focus();
```

**Status**: Already sets focus on DrawingArea for text editing mode
**IME Requirement**: Focus is prerequisite for IME - already satisfied

### 1.3 GTK Dependencies

**Current** (`crates/ui/Cargo.toml`):
```toml
gtk4 = { version = "0.7", features = ["v4_10"] }
glib = "0.18"
gio = "0.18"
```

**Status**: All required dependencies already present
- `gtk4` includes `IMContext`, `IMMulticontext`, `IMContextExt`
- `glib` provides signal handling and closures

---

## 2. GTK4 IMContext TECHNICAL DETAILS

### 2.1 Key Classes and Traits

| Component | Type | Purpose |
|-----------|------|---------|
| `IMMulticontext` | Class | Main context supporting multiple input methods (use this, not abstract `IMContext`) |
| `IMContextExt` | Trait | Provides methods for IMContext instances |
| `EventControllerKey` | Class | Already used in Testruct (must be extended) |

### 2.2 IMContext Signals

| Signal | When Fired | Data Passed | Usage |
|--------|-----------|-----------|-------|
| `commit` | User completes composition | `&str` - final text | **REQUIRED** - receive composed Japanese |
| `preedit-changed` | Composition text changes | (via `preedit_string()`) | **OPTIONAL** - display intermediate text |
| `preedit-start` | Composition begins | (none) | **OPTIONAL** - mark composition start |
| `preedit-end` | Composition ends | (none) | **OPTIONAL** - clear preedit display |
| `retrieve-surrounding` | IME needs context | (none) | **OPTIONAL** - provide surrounding text |
| `delete-surrounding` | IME requests deletion | (start, length) | **OPTIONAL** - delete around cursor |

### 2.3 IMContext Lifecycle

```
1. Create IMMulticontext
2. Set client widget (drawing_area)
3. Connect to EventControllerKey via set_im_context()
4. Connect signal handlers (commit, preedit-changed)
5. Call focus_in() when text editing starts
6. Process keyboard events through IMContext
7. Call focus_out() when text editing ends
8. Call reset() on cursor position changes
```

### 2.4 Key Methods (IMContextExt Trait)

```rust
// Lifecycle
fn focus_in(&self)                          // Notify IME of input focus
fn focus_out(&self)                         // Notify IME of lost focus
fn reset(&self)                             // Reset IME state after cursor change

// Event handling
fn filter_keypress(&self, event: &EventKey) -> bool   // Let IME handle key (GTK3)
fn filter_key(&self, keyval: gdk::Key, modifiers: ModifierType) -> bool  // GTK4 version

// Preedit management
fn preedit_string(&self) -> (GString, AttrList, i32)   // Get current composition text
fn set_cursor_location(&self, area: &Rectangle)        // Update cursor position for IME

// Configuration
fn set_client_widget(&self, widget: impl IsA<Widget>)  // Associate with widget
fn set_surrounding_with_selection(&self, text: &str, cursor_index: i32, anchor_index: i32)
fn set_input_hints(&self, hints: InputHints)           // Configure IME behavior
fn set_input_purpose(&self, purpose: InputPurpose)     // Specify input type
```

### 2.5 Signal Connection Signatures (gtk4-rs)

```rust
// Commit signal - most important for text input
im_context.connect_commit(|context, text| {
    // `text` is the final composed string (e.g., "こんにちは")
    // Insert into document immediately
});

// Preedit-changed signal - for showing composition
im_context.connect_preedit_changed(|context| {
    let (preedit_str, attrs, cursor_pos) = context.preedit_string();
    // Display preedit text in underline/highlight
});

// Preedit lifecycle signals
im_context.connect_preedit_start(|context| {
    // Composition started - show preedit indicator
});

im_context.connect_preedit_end(|context| {
    // Composition ended - hide preedit indicator
});

// Surrounding text request (for smart input methods)
im_context.connect_retrieve_surrounding(|context| {
    // Call set_surrounding_with_selection() with text around cursor
    true
});
```

---

## 3. KNOWN GTK4 GOTCHAS & ISSUES

### 3.1 Event Processing Changes (GTK3 -> GTK4)

**Critical Issue**: `filter_keypress(event)` no longer works in GTK4

From Chromium source code findings:
> "In GTK4, clients can no longer create or modify events. This makes using the gtk_im_context_filter_keypress() API impossible."

**Solution**: Use `filter_key(keyval, modifiers)` instead (available in gtk-rs)

### 3.2 DrawingArea vs Text Widget

| Feature | Text Widget | DrawingArea |
|---------|----------|------------|
| Built-in IME | Yes (automatic) | No (manual setup) |
| Focus handling | Automatic | Manual via `grab_focus()` |
| Preedit display | Built-in | Must implement custom |
| Cursor tracking | Automatic | Must call `set_cursor_location()` |
| Surrounding text | Automatic | Must implement |

**For Testruct**: DrawingArea requires FULL manual IME implementation

### 3.3 Key Event Flow

```
User presses key
        ↓
EventControllerKey::key-pressed signal
        ↓
IMMulticontext.filter_key(keyval, modifiers) → bool
        ├─ true: IME handled key (don't process further)
        │   └─ Wait for ::commit signal
        └─ false: Normal key input (process immediately)
```

### 3.4 Preedit Display Challenges

**Issue**: Preedit text is NOT automatically displayed
**Implementation Required**:
1. Connect to `preedit-changed` signal
2. Get preedit string via `im_context.preedit_string()`
3. Render preedit text with underline/highlight in DrawingArea
4. Track cursor position in preedit string
5. Clear preedit when `preedit-end` fires

---

## 4. IMPLEMENTATION PLAN

### Phase 1: Basic IMContext Integration (1-1.5 days)

#### Step 1: Create IME Module Structure
```
crates/ui/src/canvas/input/ime/
├── mod.rs           # Main IME manager
├── context.rs       # IMMulticontext wrapper
└── preedit.rs       # Preedit rendering & management
```

#### Step 2: Wrap IMMulticontext
```rust
// crates/ui/src/canvas/input/ime/context.rs
pub struct ImeContext {
    context: gtk4::IMMulticontext,
    preedit_string: String,
    preedit_cursor: usize,
}

impl ImeContext {
    pub fn new() -> Self {
        let context = gtk4::IMMulticontext::new();
        Self {
            context,
            preedit_string: String::new(),
            preedit_cursor: 0,
        }
    }
    
    pub fn setup_signals(&self, app_state: &AppState, render_state: &CanvasRenderState) {
        // Connect commit signal
        // Connect preedit-changed signal
        // Connect preedit-start/end signals
    }
}
```

#### Step 3: Connect to EventControllerKey
```rust
// In keyboard/mod.rs setup_keyboard_events()
let key_controller = EventControllerKey::new();
let ime_context = ImeContext::new();
ime_context.setup_signals(app_state, render_state);

// NEW: Set IMContext on key controller
key_controller.set_im_context(Some(&ime_context.context));
```

#### Step 4: Update Key Event Handler
```rust
// In keyboard/mod.rs key_pressed handler
let ime_context = ...; // Access IMContext

if in_text_editing {
    // NEW: Let IME try to handle key first
    if ime_context.filter_key(keyval, state) {
        // IME handled it - will send ::commit signal later
        return gtk4::glib::Propagation::Stop;
    }
}

// OLD: Direct key handling (for keys IME doesn't handle)
// ... existing code for arrow keys, backspace, etc.
```

#### Step 5: Implement Commit Handler
```rust
// In ime/context.rs
im_context.connect_commit(move |_context, text: &str| {
    let app_state_commit = app_state.clone();
    let render_state_commit = render_state.clone();
    let drawing_area_commit = drawing_area.clone();
    
    // Use existing text_editing_keys::handle_text_insertion logic
    app_state_commit.with_active_document(|doc| {
        if let Some(page) = doc.pages.first_mut() {
            for element in &mut page.elements {
                if let DocumentElement::Text(text_elem) = element {
                    if text_elem.id == editing_text_id {
                        // Insert composed Japanese text
                        let mut chars: Vec<char> = text_elem.content.chars().collect();
                        for ch in text.chars() {
                            chars.insert(cursor_pos, ch);
                            cursor_pos += 1;
                        }
                        text_elem.content = chars.iter().collect();
                    }
                }
            }
        }
    });
    
    drawing_area_commit.queue_draw();
});
```

### Phase 2: Preedit Display (0.5-1 day)

#### Step 6: Track Preedit State
```rust
// In ime/context.rs
pub struct PreeditInfo {
    pub string: String,
    pub cursor_pos: usize,
    pub attrs: Vec<PangoAttr>,  // Attributes from IME
}

impl ImeContext {
    pub fn get_preedit(&self) -> PreeditInfo {
        let (gstr, attr_list, cursor) = self.context.preedit_string();
        PreeditInfo {
            string: gstr.to_string(),
            cursor_pos: cursor as usize,
            attrs: attr_list.attrs().collect(),
        }
    }
}
```

#### Step 7: Render Preedit in Canvas
```rust
// In canvas/rendering.rs or rendering_text.rs
// Add preedit rendering logic:

if let Some((editing_text_id, cursor_pos)) = (tool_state.editing_text_id, tool_state.editing_cursor_pos) {
    if let Some(preedit_info) = render_state.ime_context.get_preedit() {
        // Render preedit with red underline at cursor position
        ctx.set_source_rgb(1.0, 0.0, 0.0);  // Red
        ctx.set_line_width(2.0);
        // Draw underline under preedit text
    }
}
```

#### Step 8: Connect Preedit Signals
```rust
// In ime/context.rs
im_context.connect_preedit_changed(move |context| {
    let (preedit_str, _, cursor) = context.preedit_string();
    render_state.update_preedit(preedit_str.to_string(), cursor as usize);
    drawing_area.queue_draw();
});

im_context.connect_preedit_start(move |_context| {
    // Optional: Log start of composition
});

im_context.connect_preedit_end(move |_context| {
    render_state.clear_preedit();
    drawing_area.queue_draw();
});
```

### Phase 3: Advanced Features (0.5-1 day)

#### Step 9: Surrounding Text Support
```rust
// In ime/context.rs
im_context.connect_retrieve_surrounding(move |context| {
    if let Some(document) = app_state.active_document() {
        if let Some(page) = document.pages.first() {
            for element in &page.elements {
                if let DocumentElement::Text(text) = element {
                    if text.id == editing_text_id {
                        let cursor_idx = editing_cursor_pos as i32;
                        context.set_surrounding_with_selection(
                            &text.content,
                            cursor_idx,
                            cursor_idx,
                        );
                        return true;
                    }
                }
            }
        }
    }
    false
});
```

#### Step 10: Cursor Position Updates
```rust
// In text_editing_keys.rs, after cursor moves
pub fn update_ime_cursor_location(
    ime_context: &ImeContext,
    text_bounds: &Rect,
    cursor_pos: usize,
) {
    // Calculate visual cursor position from character position
    // Call ime_context.set_cursor_location(rectangle)
}
```

---

## 5. CODE IMPLEMENTATION EXAMPLE

### 5.1 Minimal Working IME Setup

```rust
// crates/ui/src/canvas/input/ime/mod.rs
use gtk4::prelude::*;
use gtk4::{IMMulticontext, DrawingArea};
use glib::clone;
use uuid::Uuid;

pub struct ImeManager {
    context: IMMulticontext,
    preedit_string: String,
    editing_text_id: Option<Uuid>,
}

impl ImeManager {
    pub fn new() -> Self {
        let context = IMMulticontext::new();
        Self {
            context,
            preedit_string: String::new(),
            editing_text_id: None,
        }
    }
    
    pub fn connect_to_drawing_area(
        &self,
        key_controller: &gtk4::EventControllerKey,
        drawing_area: &DrawingArea,
    ) {
        key_controller.set_im_context(Some(&self.context));
        self.context.set_client_widget(drawing_area);
    }
    
    pub fn connect_signals(
        &self,
        app_state: &crate::app::AppState,
        render_state: &crate::canvas::CanvasRenderState,
        drawing_area: &DrawingArea,
    ) {
        let context = self.context.clone();
        
        // Commit signal - most important!
        context.connect_commit(
            clone!(@weak drawing_area, @strong app_state, @strong render_state => move |_, text| {
                eprintln!("IME commit: '{}'", text);
                
                let tool_state = render_state.tool_state.borrow();
                if let (Some(text_id), cursor_pos) = 
                    (tool_state.editing_text_id, tool_state.editing_cursor_pos) {
                    drop(tool_state);
                    
                    // Insert IME-composed text
                    app_state.with_active_document(|doc| {
                        if let Some(page) = doc.pages.first_mut() {
                            for element in &mut page.elements {
                                if let testruct_core::document::DocumentElement::Text(text_elem) = element {
                                    if text_elem.id == text_id {
                                        let mut chars: Vec<char> = text_elem.content.chars().collect();
                                        let mut pos = cursor_pos;
                                        for ch in text.chars() {
                                            if pos <= chars.len() {
                                                chars.insert(pos, ch);
                                                pos += 1;
                                            }
                                        }
                                        text_elem.content = chars.iter().collect();
                                        
                                        let mut tool_state = render_state.tool_state.borrow_mut();
                                        tool_state.editing_cursor_pos = pos;
                                    }
                                }
                            }
                        }
                    });
                    
                    drawing_area.queue_draw();
                }
            })
        );
        
        // Preedit-changed signal (optional but recommended)
        context.connect_preedit_changed(
            clone!(@weak drawing_area => move |context| {
                let (preedit_str, _, cursor_pos) = context.preedit_string();
                eprintln!("IME preedit: '{}' (cursor={})", preedit_str, cursor_pos);
                drawing_area.queue_draw();
            })
        );
    }
    
    pub fn focus_in(&self) {
        self.context.focus_in();
    }
    
    pub fn focus_out(&self) {
        self.context.focus_out();
    }
    
    pub fn reset(&self) {
        self.context.reset();
    }
    
    pub fn filter_key(&self, keyval: gtk4::gdk::Key, modifiers: gtk4::gdk::ModifierType) -> bool {
        self.context.filter_key(keyval, modifiers)
    }
    
    pub fn preedit_string(&self) -> (String, Vec<(usize, usize)>) {
        let (gstr, _, cursor) = self.context.preedit_string();
        (gstr.to_string(), vec![])  // Simplified
    }
}
```

### 5.2 Integration into Keyboard Handler

```rust
// crates/ui/src/canvas/input/keyboard/mod.rs
use super::ime::ImeManager;

pub fn setup_keyboard_events(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    let key_controller = EventControllerKey::new();
    
    // NEW: Create IME manager
    let ime_manager = std::rc::Rc::new(ImeManager::new());
    ime_manager.connect_to_drawing_area(&key_controller, drawing_area);
    ime_manager.connect_signals(app_state, render_state, drawing_area);
    
    let render_state_keyboard = render_state.clone();
    let app_state_keyboard = app_state.clone();
    let ime_manager_keyboard = ime_manager.clone();
    
    key_controller.connect_key_pressed(move |_controller, keyval, _keycode, state| {
        let tool_state_ref = render_state_keyboard.tool_state.borrow();
        let in_text_editing = tool_state_ref.editing_text_id.is_some();
        drop(tool_state_ref);
        
        if in_text_editing {
            // NEW: Let IME filter key first
            if ime_manager_keyboard.filter_key(keyval, state) {
                return gtk4::glib::Propagation::Stop;
            }
        }
        
        // OLD: Direct key handling for special keys
        match keyval {
            gtk4::gdk::Key::Escape => {
                // Exit text editing
                let mut tool_state = render_state_keyboard.tool_state.borrow_mut();
                tool_state.editing_text_id = None;
                drop(tool_state);
                ime_manager_keyboard.focus_out();
                gtk4::glib::Propagation::Stop
            }
            gtk4::gdk::Key::BackSpace if in_text_editing => {
                // Handle backspace
                gtk4::glib::Propagation::Stop
            }
            _ => gtk4::glib::Propagation::Proceed
        }
    });
    
    drawing_area.add_controller(key_controller);
}
```

### 5.3 Updated Click Handler for Focus

```rust
// In gesture_click.rs, when entering text editing mode
drawing_area_click.grab_focus();

// NEW: Notify IME of input focus
let ime_manager = ...; // Get IME manager
ime_manager.focus_in();
```

---

## 6. TESTING STRATEGY

### 6.1 Manual Testing

1. **Install Japanese IME** (if not already installed):
   ```bash
   # On Ubuntu/Debian
   sudo apt install ibus ibus-anthy
   export GTK_IM_MODULE=ibus
   
   # On Fedora
   sudo dnf install ibus ibus-anthy
   ```

2. **Test Japanese Input**:
   - Run Testruct
   - Create text element
   - Double-click to edit
   - Switch to Japanese input (Ctrl+Space or equivalent)
   - Type: "konnichiha" → should show preedit + candidates
   - Select character → should commit Japanese text

3. **Test Preedit Display**:
   - Verify underline appears under composition
   - Verify cursor position is shown
   - Verify preedit clears when complete

4. **Test Edge Cases**:
   - Rapid typing
   - Cut/paste with composed text
   - Multi-line with composition
   - Undo/redo with composition

### 6.2 Automated Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ime_context_creation() {
        let ime = ImeManager::new();
        assert!(ime.context.is_some());
    }
    
    #[test]
    fn test_preedit_string() {
        let ime = ImeManager::new();
        let (preedit, _attrs) = ime.preedit_string();
        assert_eq!(preedit, "");  // Initially empty
    }
}
```

---

## 7. POTENTIAL ISSUES & MITIGATION

| Issue | Cause | Mitigation |
|-------|-------|-----------|
| Preedit not showing | No render logic implemented | Implement Phase 2 preedit rendering |
| IME not responding | Wrong IME module | Set `GTK_IM_MODULE=ibus` environment variable |
| Double character input | Both IME and direct key handling | Check `filter_key()` return value before other processing |
| Cursor position wrong | `filter_key()` doesn't update state | Call `ime_context.reset()` after document updates |
| IME loses focus | Unfocused widget receives key | Ensure `grab_focus()` is called on DrawingArea |
| Preedit text invisible | Wrong color/font | Render with contrasting underline/highlight |

---

## 8. PERFORMANCE CONSIDERATIONS

- **No performance impact expected** - IME processing is already optimized in GTK
- Preedit rendering: Minimal (just underline/highlight)
- Signal handling: Lightweight closures via `glib::clone!`
- Memory: IMMulticontext is single instance, minimal overhead

---

## 9. RESOURCES & REFERENCES

### Official Documentation
- [GTK4 IMContext Class](https://docs.gtk.org/gtk4/class.IMContext.html)
- [GTK4 IMMulticontext Class](https://docs.gtk.org/gtk4/class.IMMulticontext.html)
- [GTK4 Input Handling Overview](https://docs.gtk.org/gtk4/input-handling.html)
- [GTK Development Blog: Text Input in GTK4](https://blogs.gnome.org/gtk/2021/08/18/text-input-in-gtk-4/)

### gtk-rs Bindings
- [gtk4-rs IMContextExt](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/prelude/trait.IMContextExt.html)
- [gtk4-rs IMMulticontext](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.IMMulticontext.html)
- [gtk4-rs EventControllerKey](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.EventControllerKey.html)

### Example Code
- [GTK Repository Input Test](https://gitlab.gnome.org/GNOME/gtk/-/blob/master/tests/input.c)
- [GNOME Text Editor](https://gitlab.gnome.org/GNOME/gnome-text-editor)

### Troubleshooting
- [GNOME Discourse: Input Method Editor Issues](https://discourse.gnome.org/t/input-method-editor-not-showing-when-using-gtk-entry/15008)
- [ArchWiki: Input Methods](https://wiki.archlinux.org/title/Input_method)

---

## 10. SUMMARY & NEXT STEPS

### Current Status
- No IMContext implementation in Testruct
- Keyboard handling uses only `keyval.to_unicode()` - insufficient for IME
- Infrastructure (focus, signals) partially ready
- All required GTK dependencies available

### Required Changes
1. Create `ime` module with IMMulticontext wrapper
2. Connect IMContext to EventControllerKey
3. Implement `commit` signal handler for text insertion
4. Implement `preedit-changed` signal for visual feedback
5. Update keyboard handler to call `filter_key()`
6. Add preedit rendering to canvas

### Estimated Timeline
- Phase 1 (IME Integration): 1-1.5 days
- Phase 2 (Preedit Display): 0.5-1 day
- Phase 3 (Polish & Testing): 0.5-1 day
- **Total**: 2-3.5 days for production-ready implementation

### Success Criteria
- Japanese text input works via IME
- Preedit text shows with underline
- Cursor position tracked correctly
- No regression in existing keyboard handling
- Works with ibus, fcitx5, other standard IMEs
