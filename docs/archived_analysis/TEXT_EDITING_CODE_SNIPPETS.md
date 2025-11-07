# Text Editing - Key Code Snippets

## 1. Double-Click Detection & Text Editor Launch

**File: `canvas/click_handlers.rs` (lines 184-279)**

```rust
pub fn handle_double_click(
    x: f64,
    y: f64,
    document: &Rc<RefCell<Document>>,
    selected_indices: &Rc<RefCell<Vec<usize>>>,
    drawing_area: &DrawingArea,
    on_edit_text: &Rc<RefCell<Option<Box<dyn Fn(usize, RichText)>>>>,
    on_document_modified: &Rc<RefCell<Option<Box<dyn Fn()>>>>,
) {
    tracing::info!("Double-click detected at ({}, {})", x, y);

    // Account for ruler offset
    const RULER_SIZE: f64 = 20.0;
    let adjusted_x = x - RULER_SIZE;
    let adjusted_y = y - RULER_SIZE;

    let doc = document.borrow();
    let indices = selected_indices.borrow().clone();

    // Check if any selected object is at this position
    for &idx in indices.iter() {
        if let Some(page) = doc.pages.first() {
            if let Some(obj) = page.objects.get(idx) {
                if obj.bounds.contains_point(adjusted_x, adjusted_y) {
                    match obj.object_type {
                        ObjectType::TextFrame => {
                            tracing::info!("Double-clicked on text object {}", obj.id);

                            // Prepare text content for editing
                            if let Some(current_rich_text) =
                                TextHandler::prepare_text_for_editing(&document, 0, idx)
                            {
                                // Call edit text callback
                                if let Some(ref callback) = *on_edit_text.borrow() {
                                    drop(doc);
                                    callback(idx, current_rich_text);
                                }
                            }
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    drop(doc);
}
```

---

## 2. Text Editor Dialog Setup

**File: `rich_text_editor.rs` (lines 32-196)**

```rust
pub fn new<W: IsA<gtk4::Window>>(parent: &W, initial_rich_text: Option<&RichText>) -> Self {
    let dialog = Dialog::builder()
        .title("テキストを編集")
        .transient_for(parent)
        .modal(true)
        .default_width(600)
        .default_height(400)
        .build();

    dialog.add_button("キャンセル", ResponseType::Cancel);
    dialog.add_button("OK", ResponseType::Ok);

    let content_area = dialog.content_area();
    content_area.set_orientation(Orientation::Vertical);

    // Formatting toolbar
    let toolbar = GtkBox::new(Orientation::Horizontal, 5);

    // Bold button
    let bold_btn = ToggleButton::with_label("B");
    bold_btn.set_tooltip_text(Some("太字 (Ctrl+B)"));
    toolbar.append(&bold_btn);

    // ... more buttons ...

    // Text view
    let scrolled = ScrolledWindow::new();
    let text_view = TextView::new();
    text_view.set_wrap_mode(gtk4::WrapMode::Word);

    // Initialize with existing RichText if provided
    let initial_alignment = if let Some(rich_text) = initial_rich_text {
        let buffer = text_view.buffer();
        buffer.set_text(&rich_text.text);
        Self::apply_rich_text_to_buffer(&buffer, rich_text);
        rich_text.alignment
    } else {
        testruct_studio_core::TextAlignment::default()
    };

    scrolled.set_child(Some(&text_view));
    content_area.append(&scrolled);

    let current_alignment = Rc::new(RefCell::new(initial_alignment));
    let current_attributes = Rc::new(RefCell::new(TextAttributes::default()));

    let editor = Self {
        dialog,
        text_view,
        bold_btn,
        // ... store other components ...
        current_alignment,
        current_attributes,
    };

    editor.setup_signals();
    editor
}
```

---

## 3. Signal Handler for Bold Button

**File: `rich_text_editor.rs` (lines 206-210)**

```rust
let text_view = self.text_view.clone();
let blocking = Rc::clone(&blocking_signals);
self.bold_btn.connect_toggled(move |btn| {
    if !*blocking.borrow() {
        Self::apply_tag_to_selection(&text_view, "bold", btn.is_active());
    }
});
```

---

## 4. Apply Tag to Selection

**File: `rich_text_editor.rs` (lines 447-476)**

```rust
fn apply_tag_to_selection(text_view: &TextView, tag_name: &str, apply: bool) {
    let buffer = text_view.buffer();
    let tag_table = buffer.tag_table();

    // Get selection bounds
    if let Some((start, end)) = buffer.selection_bounds() {
        // Get or create the tag
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

        // Apply or remove the tag
        if apply {
            buffer.apply_tag(&tag, &start, &end);
        } else {
            buffer.remove_tag(&tag, &start, &end);
        }
    }
}
```

---

## 5. Keyboard Shortcuts in Editor

**File: `rich_text_editor.rs` (lines 320-351)**

```rust
let bold_btn = self.bold_btn.clone();
let italic_btn = self.italic_btn.clone();
let underline_btn = self.underline_btn.clone();

let key_controller = gtk4::EventControllerKey::new();
key_controller.connect_key_pressed(move |_, key, _, modifier| {
    let ctrl = modifier.contains(gdk::ModifierType::CONTROL_MASK);

    if ctrl {
        match key {
            gdk::Key::b | gdk::Key::B => {
                bold_btn.set_active(!bold_btn.is_active());
                return glib::Propagation::Stop;
            }
            gdk::Key::i | gdk::Key::I => {
                italic_btn.set_active(!italic_btn.is_active());
                return glib::Propagation::Stop;
            }
            gdk::Key::u | gdk::Key::U => {
                underline_btn.set_active(!underline_btn.is_active());
                return glib::Propagation::Stop;
            }
            _ => {}
        }
    }

    glib::Propagation::Proceed
});

self.text_view.add_controller(key_controller);
```

---

## 6. Extract RichText from TextBuffer

**File: `rich_text_editor.rs` (lines 534-645)**

```rust
fn extract_rich_text_from_buffer(buffer: &gtk4::TextBuffer, text: &str) -> RichText {
    use testruct_studio_core::TextStyleRange;

    let mut rich_text = RichText::new(text.to_string());
    let tag_table = buffer.tag_table();

    let mut iter = buffer.start_iter();
    let end_iter = buffer.end_iter();

    while iter.offset() < end_iter.offset() {
        let start_char_offset = iter.offset() as usize;
        let tags = iter.tags();

        // Check which formatting is active
        let mut attributes = TextAttributes::default();
        let bold_tag = tag_table.lookup("bold");
        let italic_tag = tag_table.lookup("italic");
        let underline_tag = tag_table.lookup("underline");
        let strikethrough_tag = tag_table.lookup("strikethrough");

        for tag in &tags {
            if let Some(ref bt) = bold_tag {
                if tag == bt {
                    attributes.bold = true;
                }
            }
            if let Some(ref it) = italic_tag {
                if tag == it {
                    attributes.italic = true;
                }
            }
            if let Some(ref ut) = underline_tag {
                if tag == ut {
                    attributes.underline = true;
                }
            }
            if let Some(ref st) = strikethrough_tag {
                if tag == st {
                    attributes.strikethrough = true;
                }
            }

            // Check for color tags
            if let Some(tag_name) = tag.name() {
                let tag_str = tag_name.as_str();
                if tag_str.starts_with("fgcolor_") {
                    let color_hex = tag_str.trim_start_matches("fgcolor_");
                    attributes.color = Some(format!("#{}", color_hex));
                } else if tag_str.starts_with("bgcolor_") {
                    let color_hex = tag_str.trim_start_matches("bgcolor_");
                    attributes.background_color = Some(format!("#{}", color_hex));
                } else if tag_str.starts_with("fontsize_") {
                    if let Ok(size) = tag_str.trim_start_matches("fontsize_").parse::<f64>() {
                        attributes.font_size = Some(size);
                    }
                }
            }
        }

        if attributes.has_any_formatting() {
            let mut end_iter = iter.clone();
            end_iter.forward_to_tag_toggle(Option::<&gtk4::TextTag>::None);

            let end_char_offset = end_iter.offset() as usize;
            
            // Convert character offsets to byte offsets for text string
            let start_byte_offset = text
                .chars()
                .take(start_char_offset)
                .collect::<String>()
                .len();
            let end_byte_offset = text
                .chars()
                .take(end_char_offset)
                .collect::<String>()
                .len();

            if start_byte_offset < end_byte_offset && end_byte_offset <= text.len() {
                rich_text.style_ranges.push(TextStyleRange {
                    start: start_byte_offset,
                    end: end_byte_offset,
                    attributes,
                });
            }

            iter = end_iter;
        } else {
            if !iter.forward_to_tag_toggle(Option::<&gtk4::TextTag>::None) {
                if !iter.forward_char() {
                    break;
                }
            }
        }
    }

    rich_text
}
```

---

## 7. Apply RichText to TextBuffer

**File: `rich_text_editor.rs` (lines 648-760)**

```rust
fn apply_rich_text_to_buffer(buffer: &gtk4::TextBuffer, rich_text: &RichText) {
    let tag_table = buffer.tag_table();

    for range in &rich_text.style_ranges {
        // Convert byte offsets to character offsets
        let start_char_offset = rich_text.text[..range.start].chars().count();
        let end_char_offset = rich_text.text[..range.end].chars().count();

        let start_iter = buffer.iter_at_offset(start_char_offset as i32);
        let end_iter = buffer.iter_at_offset(end_char_offset as i32);

        let attrs = &range.attributes;

        // Apply bold
        if attrs.bold {
            let tag = if let Some(existing) = tag_table.lookup("bold") {
                existing
            } else {
                let new_tag = gtk4::TextTag::new(Some("bold"));
                new_tag.set_weight(700);
                tag_table.add(&new_tag);
                new_tag
            };
            buffer.apply_tag(&tag, &start_iter, &end_iter);
        }

        // Apply italic
        if attrs.italic {
            let tag = if let Some(existing) = tag_table.lookup("italic") {
                existing
            } else {
                let new_tag = gtk4::TextTag::new(Some("italic"));
                new_tag.set_style(gtk4::pango::Style::Italic);
                tag_table.add(&new_tag);
                new_tag
            };
            buffer.apply_tag(&tag, &start_iter, &end_iter);
        }

        // ... similar for other attributes ...

        // Apply text color
        if let Some(ref color) = attrs.color {
            let tag_name = format!("fgcolor_{}", color.trim_start_matches('#'));
            let tag = if let Some(existing) = tag_table.lookup(&tag_name) {
                existing
            } else {
                let new_tag = gtk4::TextTag::new(Some(&tag_name));
                if let Ok(rgba) = gdk::RGBA::parse(color) {
                    new_tag.set_foreground_rgba(Some(&rgba));
                }
                tag_table.add(&new_tag);
                new_tag
            };
            buffer.apply_tag(&tag, &start_iter, &end_iter);
        }
    }
}
```

---

## 8. Window Setup - Text Editor Callback

**File: `window_setup.rs` (lines 520-544)**

```rust
let window_for_edit = window.clone();
let canvas_for_edit = canvas.clone();

canvas.set_on_edit_text(move |idx, current_rich_text| {
    use crate::rich_text_editor::RichTextEditor;

    tracing::info!("Opening rich text editor for text object");

    let editor = RichTextEditor::new(&window_for_edit, Some(&current_rich_text));

    let canvas_clone = canvas_for_edit.clone();

    editor.show(move |rich_text_opt| {
        if let Some(rich_text) = rich_text_opt {
            tracing::info!(
                "Text editor returned rich text with {} style ranges",
                rich_text.style_ranges.len()
            );

            // Update the rich text via canvas
            canvas_clone.update_object_rich_text(idx, rich_text);
            
            // Trigger auto-resize if enabled
            canvas_clone.update_object_auto_resize(idx);
        } else {
            tracing::info!("Text editor cancelled");
        }
    });
});
```

---

## 9. Update Document with New RichText

**File: `canvas/widget_operations.rs` (lines 106-131)**

```rust
pub fn update_object_rich_text(
    &self,
    object_index: usize,
    rich_text: testruct_studio_core::RichText,
) {
    use testruct_studio_core::document::ObjectContent;

    let page_idx = *self.current_page_index.borrow();

    let mut doc = self.document.borrow_mut();
    if let Some(page) = doc.pages.get_mut(page_idx) {
        if let Some(obj) = page.objects.get_mut(object_index) {
            obj.content = ObjectContent::RichText(rich_text);
            tracing::info!("Updated rich text for object {}", object_index);
        }
    }
    drop(doc);

    // Redraw canvas
    self.drawing_area.queue_draw();

    // Notify that document was modified
    if let Some(callback) = self.on_document_modified.borrow().as_ref() {
        callback();
    }
}
```

---

## 10. Property Panel - Font Size Changed Signal

**File: `property/panel_signals.rs` (lines 130-147)**

```rust
fn setup_font_size_signal(
    current_object: &Rc<RefCell<Option<Object>>>,
    on_style_changed: &Rc<RefCell<OnStyleChanged>>,
    font_size_spin: &SpinButton,
) {
    let current_object = current_object.clone();
    let on_style_changed = on_style_changed.clone();
    font_size_spin.connect_value_changed(move |spin| {
        let size = spin.value();
        if let Some(ref mut obj) = *current_object.borrow_mut() {
            obj.style.font_size = size;
            tracing::info!("Font size changed to: {}", size);
            if let Some(callback) = on_style_changed.borrow().as_ref() {
                callback(obj.style.clone());
            }
        }
    });
}
```

---

## 11. Property Panel - Update from Selection

**File: `property/panel_state.rs` (lines 13-98)**

```rust
pub fn update_from_selection(
    current_object: &Rc<RefCell<Option<Object>>>,
    object: &Object,
    font_family_combo: &gtk4::ComboBoxText,
    font_size_spin: &gtk4::SpinButton,
    line_height_scale: &gtk4::Scale,
    text_align_combo: &gtk4::ComboBoxText,
    border_style_combo: &gtk4::ComboBoxText,
    auto_resize_switch: &gtk4::Switch,
    stroke_width_spin: &gtk4::SpinButton,
    fill_color_button: &Button,
    stroke_color_button: &Button,
    group_status_label: &Label,
    ungroup_btn: &Button,
) {
    *current_object.borrow_mut() = Some(object.clone());

    let style = &object.style;

    // Font family
    let font_id = match style.font_family.as_str() {
        "Noto Sans JP" => "noto-sans-jp",
        "Noto Serif JP" => "noto-serif-jp",
        // ... map other fonts ...
        _ => "noto-sans-jp",
    };
    font_family_combo.set_active_id(Some(font_id));

    // Font size
    font_size_spin.set_value(style.font_size);

    // Line height
    line_height_scale.set_value(style.line_height);

    // Text alignment
    let align_id = match style.text_align {
        testruct_studio_core::document::TextAlign::Left => "left",
        testruct_studio_core::document::TextAlign::Center => "center",
        testruct_studio_core::document::TextAlign::Right => "right",
        testruct_studio_core::document::TextAlign::Justify => "justify",
    };
    text_align_combo.set_active_id(Some(align_id));

    // Border style
    let border_id = match style.border {
        testruct_studio_core::document::BorderStyle::None => "none",
        testruct_studio_core::document::BorderStyle::Thin => "thin",
        testruct_studio_core::document::BorderStyle::Thick => "thick",
    };
    border_style_combo.set_active_id(Some(border_id));

    // Auto-resize switch
    auto_resize_switch.set_active(object.auto_resize_height);

    // Stroke width
    stroke_width_spin.set_value(style.stroke_width);

    // Update color button appearances
    update_fill_color_button(fill_color_button, style.fill_color);
    update_stroke_color_button(stroke_color_button, style.stroke_color);
}
```

---

## 12. Text Handler - Prepare for Editing

**File: `canvas/text_handler.rs` (lines 56-89)**

```rust
pub fn prepare_text_for_editing(
    document: &Rc<RefCell<Document>>,
    page_idx: usize,
    object_idx: usize,
) -> Option<RichText> {
    let doc = document.borrow();

    if let Some(page) = doc.pages.get(page_idx) {
        if let Some(obj) = page.objects.get(object_idx) {
            // Ensure it's a text object
            if !matches!(obj.object_type, ObjectType::TextFrame) {
                return None;
            }

            // Convert content to RichText
            let rich_text = match &obj.content {
                ObjectContent::RichText(rt) => rt.clone(),
                _ => {
                    // Convert plain text to RichText
                    RichText::new(obj.content.as_str().to_string())
                }
            };

            tracing::info!(
                ">>> Text editing prepared for object {} on page {}",
                object_idx,
                page_idx
            );
            return Some(rich_text);
        }
    }
    None
}
```

---

## 13. Apply Color to Selection

**File: `rich_text_editor.rs` (lines 479-509)**

```rust
fn apply_color_to_selection(text_view: &TextView, color_hex: &str, is_background: bool) {
    let buffer = text_view.buffer();
    let tag_table = buffer.tag_table();

    if let Some((start, end)) = buffer.selection_bounds() {
        // Create unique tag name for this color
        let tag_name = if is_background {
            format!("bgcolor_{}", color_hex.trim_start_matches('#'))
        } else {
            format!("fgcolor_{}", color_hex.trim_start_matches('#'))
        };

        // Get or create tag
        let tag = if let Some(existing_tag) = tag_table.lookup(&tag_name) {
            existing_tag
        } else {
            let new_tag = gtk4::TextTag::new(Some(&tag_name));
            if let Ok(rgba) = gdk::RGBA::parse(color_hex) {
                if is_background {
                    new_tag.set_background_rgba(Some(&rgba));
                } else {
                    new_tag.set_foreground_rgba(Some(&rgba));
                }
            }
            tag_table.add(&new_tag);
            new_tag
        };

        buffer.apply_tag(&tag, &start, &end);
    }
}
```

---

## 14. Apply Font Size to Selection

**File: `rich_text_editor.rs` (lines 512-532)**

```rust
fn apply_font_size_to_selection(text_view: &TextView, font_size: f64) {
    let buffer = text_view.buffer();
    let tag_table = buffer.tag_table();

    if let Some((start, end)) = buffer.selection_bounds() {
        // Create unique tag name for this font size
        let tag_name = format!("fontsize_{}", font_size as i32);

        // Get or create tag
        let tag = if let Some(existing_tag) = tag_table.lookup(&tag_name) {
            existing_tag
        } else {
            let new_tag = gtk4::TextTag::new(Some(&tag_name));
            new_tag.set_size_points(font_size);
            tag_table.add(&new_tag);
            new_tag
        };

        buffer.apply_tag(&tag, &start, &end);
    }
}
```

