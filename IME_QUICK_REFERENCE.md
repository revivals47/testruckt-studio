# Japanese IME for Testruct - Quick Reference

## File Locations Summary

### Current Implementation Files (Need Modification)

| File | Purpose | Lines | Issue |
|------|---------|-------|-------|
| `/crates/ui/src/canvas/input/keyboard/mod.rs` | Main keyboard event handler | 65-227 | Needs IMContext integration + filter_key() call |
| `/crates/ui/src/canvas/input/keyboard/text_editing_keys.rs` | Character insertion | 216-252 | Uses `keyval.to_unicode()` - insufficient for IME |
| `/crates/ui/src/canvas/input/gesture_click.rs` | Focus management | 148 | Already calls `grab_focus()` - good foundation |
| `/crates/ui/Cargo.toml` | Dependencies | 18-20 | All required deps present - no changes needed |

### Files to Create

```
crates/ui/src/canvas/input/ime/
├── mod.rs              # Main ImeManager struct
├── context.rs          # IMMulticontext wrapper (optional but clean)
└── signals.rs          # Signal handlers (optional but clean)
```

---

## Key Code Locations & Changes

### 1. Keyboard Event Handler (`keyboard/mod.rs` line 65)

**Current**: Raw key handling
```rust
key_controller.connect_key_pressed(move |_controller, keyval, _keycode, state| {
    // ... direct key handling
});
```

**Add**: IMContext filter call
```rust
if in_text_editing {
    if ime_manager.filter_key(keyval, state) {
        return gtk4::glib::Propagation::Stop;
    }
}
```

### 2. Text Insertion (`text_editing_keys.rs` line 216-252)

**Current**: Only handles `keyval.to_unicode()`
```rust
if let Some(ch) = keyval.to_unicode() {
    // Insert single character
}
```

**Replace With**: IME::commit signal handles all insertion
- Remove direct character insertion from key handler
- Let IMContext signal handlers manage all text input

### 3. Focus Management (`gesture_click.rs` line 148)

**Current**: Already has focus
```rust
drawing_area_click.grab_focus();
```

**Add After**: IME focus notification
```rust
drawing_area_click.grab_focus();
ime_manager.focus_in();  // NEW
```

---

## Integration Checklist

- [ ] Create `crates/ui/src/canvas/input/ime/mod.rs`
- [ ] Implement `ImeManager::new()`
- [ ] Implement `ImeManager::filter_key()`
- [ ] Implement `ImeManager::focus_in()` / `focus_out()`
- [ ] Connect `::commit` signal
- [ ] Connect `::preedit-changed` signal (optional)
- [ ] Update `keyboard/mod.rs` to create and use ImeManager
- [ ] Update `gesture_click.rs` to call `ime_manager.focus_in()`
- [ ] Test with Japanese input

---

## Signal Handler Template

```rust
// In IME context.rs
use glib::clone;

context.connect_commit(
    clone!(@strong app_state, @strong render_state, @weak drawing_area => move |_, text: &str| {
        eprintln!("IME Commit: '{}'", text);
        
        // Insert text at cursor position
        if let Some(text_id) = render_state.current_editing_text() {
            app_state.with_active_document(|doc| {
                // Find text element and insert
            });
        }
        
        drawing_area.queue_draw();
    })
);
```

---

## Testing Checklist

### Manual Testing
- [ ] Japanese text input via IBus
- [ ] Preedit display (underline)
- [ ] Multi-character composition
- [ ] Cursor navigation
- [ ] Backspace/Delete in edit mode
- [ ] Cut/Paste with Japanese text
- [ ] Focus loss/regain

### Edge Cases
- [ ] Rapid key presses
- [ ] Multi-line text
- [ ] Switching input methods
- [ ] Undo/Redo with composed text

---

## Build & Dependencies

### No new dependencies needed!

```toml
gtk4 = { version = "0.7", features = ["v4_10"] }  # Already has IMContext/IMMulticontext
glib = "0.18"                                       # Signal handling
```

### Compilation Check
```bash
cd crates/ui
cargo check
cargo build
```

---

## Debugging Tips

### Enable logging
```rust
eprintln!("IME: filter_key({:?}, {:?}) -> {}", keyval, state, handled);
eprintln!("IME: commit('{}')", text);
eprintln!("IME: preedit('{}')", preedit_str);
```

### Check IME setup
```bash
echo $GTK_IM_MODULE  # Should be 'ibus' or 'fcitx'
ibus list-engines    # List available input methods
```

### Check focus state
```rust
eprintln!("DrawingArea can_focus: {}", drawing_area.can_focus());
eprintln!("DrawingArea has_focus: {}", drawing_area.has_focus());
```

---

## Expected Behavior

### Before IME Implementation
- Type "a" → letter "a" appears
- Type "あ" in Japanese → nothing happens (IME not supported)

### After IME Implementation
- Type "a" → letter "a" appears (direct input)
- Type "こんにちは" in Japanese:
  1. "こ" appears as preedit (underlined)
  2. More keys convert composition
  3. Select from candidate list
  4. Final text "こんにちは" committed and inserted

---

## Architecture Diagram

```
User presses key
        ↓
EventControllerKey::key-pressed
        ↓
ImeManager::filter_key(keyval, state)
        ├─ true: IME consumed (wait for ::commit)
        │   └─ IMContext detects composition
        │       └─ Emits preedit-changed (show underline)
        │           └─ Emits commit (insert text)
        └─ false: Direct key
            └─ Handle with existing logic (arrows, backspace)
```

---

## Resources

### Code examples in this guide
1. `ImeManager` struct (minimal implementation)
2. `connect_commit` signal handler
3. Integration into `keyboard/mod.rs`

### External references
- gtk4-rs docs: https://gtk-rs.org/gtk4-rs/git/docs/gtk4/
- GTK4 IMContext: https://docs.gtk.org/gtk4/class.IMContext.html
- GTK4 input handling: https://docs.gtk.org/gtk4/input-handling.html

