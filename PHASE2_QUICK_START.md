# Phase 2 - Quick Start Guide

## TL;DR - What to Build First

### The 4 Quick Wins (2.5 hours total)

These are the easiest features to implement. All data models and rendering are READY. Just need UI + signal handlers.

1. **Wire Stroke Width Handler** (15 minutes)
   - File: `/crates/ui/src/panels/property_handlers_shape.rs`
   - What: Signal handler for `stroke_width_spin` SpinButton
   - Copy pattern from `wire_fill_color_signal()` but update `stroke_width` field instead
   - Impact: Users can now control line thickness on shapes

2. **Underline Text** (30 minutes)
   - Files: 
     - `/crates/ui/src/panels/properties_groups.rs` - Add toggle button
     - `/crates/ui/src/panels/property_handlers_text.rs` - Add signal handler
   - What: Add underline button next to italic button
   - Copy pattern from `wire_italic_signal()`
   - Impact: Professional text decoration now available

3. **Strikethrough Text** (30 minutes)
   - Same pattern as Underline
   - Just toggle `strikethrough` field instead of `underline`
   - Impact: Another professional text decoration

4. **Background Color** (45 minutes)
   - Files: Same as Underline
   - What: Add color picker button for text background
   - Copy pattern from `wire_text_color_signal()` but update `background_color` field
   - Impact: Highlight text with background color

---

## Code References for Copy-Paste Patterns

### Pattern 1: Boolean Toggle Handler (Underline/Strikethrough)

**Location**: `/crates/ui/src/panels/property_handlers_text.rs`

**Copy from**: `wire_italic_signal()` function (lines 116-146)

**Adapt**:
- Change `italic_button` → `underline_button`
- Change `text.style.italic = is_italic` → `text.style.underline = is_underline`
- Change `text.style.strikethrough = is_strikethrough` for strikethrough version

---

### Pattern 2: Color Dialog Handler (Background Color)

**Location**: `/crates/ui/src/panels/property_handlers_text.rs`

**Copy from**: `wire_text_color_signal()` function (lines 149-237)

**Adapt**:
- Change `text_color_button` → `background_color_button`
- Change `text.style.color = text_color` → `text.style.background_color = Some(bg_color)`
- Add check for "Clear" color option (right now there's no way to unset it)

---

### Pattern 3: Spinner Handler (Stroke Width)

**Location**: `/crates/ui/src/panels/property_handlers_shape.rs`

**Copy from**: `/crates/ui/src/panels/property_handlers_text.rs` `wire_font_size_signal()` (lines 45-76)

**Adapt**:
- Change `font_size_spin` → `stroke_width_spin`
- Change type from SpinButton to SpinButton (same)
- Change `text.style.font_size = font_size` → `shape.stroke_width = stroke_width`
- Update debug message

---

## Step-by-Step Implementation

### Step 1: Add UI Button (30 minutes)

**File**: `/crates/ui/src/panels/properties_groups.rs`

**Find**: `build_text_formatting_buttons()` function

**Add after italic button**:
```rust
// Underline button
let underline_button = ToggleButton::with_label("下線");
underline_button.add_css_class("formatting-button");
buttons_box.append(&underline_button);

// Strikethrough button
let strikethrough_button = ToggleButton::with_label("打消し");
strikethrough_button.add_css_class("formatting-button");
buttons_box.append(&strikethrough_button);
```

**Also in `properties.rs`**:
- Add `pub underline_button: gtk4::ToggleButton` to `PropertyPanelComponents` struct
- Add `pub strikethrough_button: gtk4::ToggleButton` to struct
- Capture and return them from `build_text_formatting_buttons()`

---

### Step 2: Add UI Component to Property Panel Builder

**File**: `/crates/ui/src/panels/properties.rs`

**Find**: `fn build_property_panel_components()`

**Update**:
```rust
let (bold_button, italic_button, underline_button, strikethrough_button) = 
    build_text_formatting_buttons(&container);
```

**Add to PropertyPanelComponents struct initialization**:
```rust
PropertyPanelComponents {
    // ... existing fields ...
    bold_button,
    italic_button,
    underline_button,      // ← ADD
    strikethrough_button,  // ← ADD
    // ... rest ...
}
```

---

### Step 3: Wire Signals in property_handlers.rs

**File**: `/crates/ui/src/panels/property_handlers.rs`

**In `wire_property_signals()` function, add**:
```rust
wire_underline_signal(
    components,
    app_state.clone(),
    drawing_area.clone(),
    render_state.clone(),
);

wire_strikethrough_signal(
    components,
    app_state.clone(),
    drawing_area.clone(),
    render_state.clone(),
);

wire_background_color_signal(
    components,
    app_state.clone(),
    drawing_area.clone(),
    render_state.clone(),
);

wire_stroke_width_signal(
    components,
    app_state.clone(),
    drawing_area.clone(),
    render_state.clone(),
);
```

---

### Step 4: Implement Signal Handlers

**File**: `/crates/ui/src/panels/property_handlers_text.rs`

**Add at end of file**:

```rust
/// Wire underline toggle
pub fn wire_underline_signal(
    components: &PropertyPanelComponents,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    let button = components.underline_button.clone();

    button.connect_toggled(move |btn| {
        let is_underline = btn.is_active();

        app_state.with_mutable_active_document(|doc| {
            let selected = render_state.selected_ids.borrow();
            if !selected.is_empty() {
                if let Some(page) = doc.pages.first_mut() {
                    for element in &mut page.elements {
                        if let DocumentElement::Text(text) = element {
                            if selected.contains(&text.id) {
                                text.style.underline = is_underline;
                                recompute_auto_height(text);
                                tracing::debug!("✅ Underline: {}", is_underline);
                            }
                        }
                    }
                }
            }
        });

        drawing_area.queue_draw();
    });
}

// Similar for strikethrough, background_color, stroke_width...
```

---

## Testing Checklist

### Basic Functionality
- [ ] Underline button appears in property panel
- [ ] Clicking underline button applies underline to selected text
- [ ] Underline renders on canvas
- [ ] Same for strikethrough
- [ ] Background color picker opens
- [ ] Background color applies to selected text
- [ ] Stroke width spinner changes line thickness

### Edge Cases
- [ ] Multiple text elements selected → all get underline
- [ ] Mix of underlined/non-underlined selected → button shows mixed state (or just first)
- [ ] Undo/Redo work correctly
- [ ] Save/Load preserves underline/strikethrough/background color
- [ ] Removing background color (set to None)

---

## File Changes Summary

| File | Changes | Lines Added |
|------|---------|------------|
| properties_groups.rs | Add 2 toggle buttons | ~20 |
| properties.rs | Add 4 fields to struct, wire buttons | ~15 |
| property_handlers.rs | Add 4 wire_*_signal calls | ~20 |
| property_handlers_text.rs | Add 4 handler functions | ~200 |
| property_handlers_shape.rs | Add stroke width handler | ~50 |
| **Total** | | **~305 lines** |

---

## Success Metrics After Quick Wins

- All 4 text decorations (underline, strikethrough, bold, italic) working
- Background color for text working
- Stroke width controllable for shapes
- All features save/load correctly
- No clippy warnings
- Tests still passing

---

## What NOT to Do

- Don't add gradients yet (complex Cairo integration)
- Don't add opacity to all elements (affects all render functions)
- Don't refactor existing handlers (just add new ones)
- Don't change the data models (they're already correct)
- Don't add new dependencies

---

## Next Steps After Quick Wins

1. **Full Font Weight Support** (1 hour)
   - Replace bold button with 6-option dropdown
   - Map all FontWeight variants to UI
   - Update `wire_bold_signal()` → `wire_font_weight_signal()`

2. **Letter Spacing** (1.5 hours)
   - Add `letter_spacing: f32` to TextStyle
   - Add SpinButton to typography section
   - Integrate with Pango layout

3. **Corner Radius** (1 hour)
   - Add to ShapeElement
   - Use Cairo rounded rectangle function
   - Add SpinButton to shape properties

---

## Questions About Patterns?

Check these files for similar implementations:
- `wire_font_size_signal()` - SpinButton pattern
- `wire_italic_signal()` - ToggleButton pattern
- `wire_text_color_signal()` - ColorDialog pattern
- All in `/crates/ui/src/panels/property_handlers_text.rs`

