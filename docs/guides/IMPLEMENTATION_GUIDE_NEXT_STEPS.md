# Testruct Desktop Rust - Next Implementation Steps

**Created**: November 7, 2025
**Target Features**: Copy/Paste, Duplicate, Undo/Redo Integration

---

## 🎯 Current Status Summary

### ✅ Already Implemented
1. **Save/Load** - Fully implemented in `crates/ui/src/io/`
   - `save_document()` - JSON serialization
   - `load_document()` - JSON deserialization
   - File dialogs (Open/Save/Export)
   - Menu actions integrated

2. **Undo/Redo Infrastructure** - Fully implemented in `crates/ui/src/undo_redo.rs`
   - `UndoRedoStack` - Command pattern implementation
   - `Command` trait - For custom commands
   - `DeleteCommand` - For element deletion
   - `CreateCommand` - For element creation
   - Batch operations support
   - Tests included

3. **Keyboard Shortcuts** - Partially implemented
   - Ctrl+Z, Ctrl+Y (not yet wired to undo/redo)
   - Ctrl+C, Ctrl+X, Ctrl+V (not yet implemented)
   - Ctrl+D (duplicate - not yet implemented)

### ⏳ Needs Implementation
1. **Copy/Paste/Duplicate** - Core editing functionality
2. **Undo/Redo Integration** - Wire existing system into operations
3. **Alignment Tools** - Multi-object alignment
4. **Fill & Stroke** - Style properties

---

## 📋 Phase 4: Copy/Paste/Duplicate Implementation

### Step 1: Create Clipboard Management System
**File**: `crates/ui/src/clipboard.rs` (NEW)

```rust
// Clipboard data structure
pub struct ClipboardData {
    elements: Vec<DocumentElement>,
    offset: (f32, f32),  // For pasting at a slight offset
}

impl ClipboardData {
    pub fn new(elements: Vec<DocumentElement>) -> Self {
        Self {
            elements,
            offset: (10.0, 10.0),
        }
    }
}

// Global clipboard state (using lazy_static or once_cell)
lazy_static::lazy_static! {
    static ref CLIPBOARD: Mutex<Option<ClipboardData>> = Mutex::new(None);
}

pub fn copy_to_clipboard(elements: Vec<DocumentElement>) {
    let mut clipboard = CLIPBOARD.lock().unwrap();
    *clipboard = Some(ClipboardData::new(elements));
}

pub fn paste_from_clipboard() -> Option<Vec<DocumentElement>> {
    let clipboard = CLIPBOARD.lock().unwrap();
    clipboard.as_ref().map(|data| {
        data.elements.iter().map(|elem| {
            // Deep clone with ID regeneration
            match elem {
                DocumentElement::Text(t) => {
                    let mut new_text = t.clone();
                    new_text.id = uuid::Uuid::new_v4();
                    // Offset bounds
                    new_text.bounds.origin.x += data.offset.0;
                    new_text.bounds.origin.y += data.offset.1;
                    DocumentElement::Text(new_text)
                },
                // Similar for Image, Shape, Frame
                _ => elem.clone()
            }
        }).collect()
    })
}

pub fn clear_clipboard() {
    let mut clipboard = CLIPBOARD.lock().unwrap();
    *clipboard = None;
}
```

### Step 2: Implement Copy/Cut/Paste Commands
**File**: `crates/ui/src/undo_redo.rs` (EXTEND)

```rust
// Copy command (doesn't modify document)
pub struct CopyCommand {
    elements: Vec<DocumentElement>,
}

impl Command for CopyCommand {
    fn execute(&mut self) -> Result<String, String> {
        crate::clipboard::copy_to_clipboard(self.elements.clone());
        Ok("Copied to clipboard".to_string())
    }

    fn undo(&mut self) -> Result<String, String> {
        crate::clipboard::clear_clipboard();
        Ok("Cleared clipboard".to_string())
    }

    fn description(&self) -> &str {
        "Copy"
    }
}

// Paste command
pub struct PasteCommand {
    document: Arc<Mutex<Document>>,
    page_index: usize,
    pasted_elements: Vec<DocumentElement>,
}

impl Command for PasteCommand {
    fn execute(&mut self) -> Result<String, String> {
        if let Some(mut elements) = crate::clipboard::paste_from_clipboard() {
            let mut doc = self.document.lock().expect("document");
            if self.page_index >= doc.pages.len() {
                return Err("Page index out of bounds".to_string());
            }

            for elem in &elements {
                doc.pages[self.page_index].add_element(elem.clone());
            }
            self.pasted_elements = elements;
            Ok("Pasted from clipboard".to_string())
        } else {
            Err("Clipboard is empty".to_string())
        }
    }

    fn undo(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document");
        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        // Remove pasted elements
        let page = &mut doc.pages[self.page_index];
        for elem in &self.pasted_elements {
            let id = elem_id(elem);
            page.elements.retain(|e| elem_id(e) != id);
        }
        Ok("Removed pasted elements".to_string())
    }

    fn description(&self) -> &str {
        "Paste"
    }
}

// Duplicate command
pub struct DuplicateCommand {
    document: Arc<Mutex<Document>>,
    page_index: usize,
    element_ids: Vec<uuid::Uuid>,
    duplicated_ids: Vec<uuid::Uuid>,
}

impl Command for DuplicateCommand {
    fn execute(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document");
        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];

        for orig_id in &self.element_ids {
            if let Some(element) = page.elements.iter().find(|e| elem_id(e) == *orig_id) {
                let mut new_elem = element.clone();
                let new_id = uuid::Uuid::new_v4();

                // Update ID and offset position
                match &mut new_elem {
                    DocumentElement::Text(t) => {
                        t.id = new_id;
                        t.bounds.origin.x += 20.0;
                        t.bounds.origin.y += 20.0;
                    },
                    DocumentElement::Image(img) => {
                        img.id = new_id;
                        img.bounds.origin.x += 20.0;
                        img.bounds.origin.y += 20.0;
                    },
                    DocumentElement::Shape(shape) => {
                        shape.id = new_id;
                        shape.bounds.origin.x += 20.0;
                        shape.bounds.origin.y += 20.0;
                    },
                    DocumentElement::Frame(frame) => {
                        frame.id = new_id;
                        frame.bounds.origin.x += 20.0;
                        frame.bounds.origin.y += 20.0;
                    },
                }

                page.add_element(new_elem);
                self.duplicated_ids.push(new_id);
            }
        }

        Ok(format!("Duplicated {} objects", self.element_ids.len()))
    }

    fn undo(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document");
        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];
        for dup_id in &self.duplicated_ids {
            page.elements.retain(|e| elem_id(e) != *dup_id);
        }

        Ok("Removed duplicated objects".to_string())
    }

    fn description(&self) -> &str {
        "Duplicate"
    }
}
```

### Step 3: Wire Keyboard Shortcuts to Canvas Input
**File**: `crates/ui/src/canvas/input.rs` (EXTEND)

```rust
// In keyboard event handler:
match keyval {
    // Copy: Ctrl+C
    c if c == 'c' as u32 && state.contains(ModifierType::CONTROL_MASK) => {
        let selected = state.selected_ids.borrow();
        if !selected.is_empty() {
            if let Some(document) = app_state_keyboard.active_document() {
                if let Some(page) = document.pages.first() {
                    let elements: Vec<_> = page.elements.iter()
                        .filter(|e| selected.contains(&elem_id(e)))
                        .cloned()
                        .collect();

                    let copy_cmd = Box::new(CopyCommand {
                        elements,
                    });
                    app_state_keyboard.push_command(copy_cmd);
                    tracing::info!("✅ Copied {} objects", selected.len());
                }
            }
        }
    },

    // Paste: Ctrl+V
    v if v == 'v' as u32 && state.contains(ModifierType::CONTROL_MASK) => {
        if let Some(document) = app_state_keyboard.active_document() {
            let paste_cmd = Box::new(PasteCommand {
                document: Arc::new(Mutex::new(document)),
                page_index: 0,
                pasted_elements: Vec::new(),
            });
            app_state_keyboard.push_command(paste_cmd);
            drawing_area_keyboard.queue_draw();
            tracing::info!("✅ Pasted from clipboard");
        }
    },

    // Duplicate: Ctrl+D
    d if d == 'd' as u32 && state.contains(ModifierType::CONTROL_MASK) => {
        let selected = state.selected_ids.borrow();
        if !selected.is_empty() {
            if let Some(document) = app_state_keyboard.active_document() {
                let dup_cmd = Box::new(DuplicateCommand {
                    document: Arc::new(Mutex::new(document)),
                    page_index: 0,
                    element_ids: selected.clone(),
                    duplicated_ids: Vec::new(),
                });
                app_state_keyboard.push_command(dup_cmd);
                drawing_area_keyboard.queue_draw();
                tracing::info!("✅ Duplicated {} objects", selected.len());
            }
        }
    },

    _ => {}
}
```

### Step 4: Update Dependencies
**File**: `crates/ui/Cargo.toml`

```toml
[dependencies]
lazy_static = "1.4"  # For global clipboard state
```

### Step 5: Update Module Exports
**File**: `crates/ui/src/lib.rs`

```rust
pub mod clipboard;
```

---

## 📋 Phase 5: Undo/Redo Integration

### Quick Integration Checklist

1. **Delete Operations** (in `canvas/input.rs`)
   ```rust
   // When Delete key is pressed:
   let delete_cmd = Box::new(undo_redo::DeleteCommand::new(
       document_arc,
       element_id,
       0  // page_index
   ));
   app_state.push_command(delete_cmd);
   ```

2. **Create Operations** (in tool handlers)
   ```rust
   // When shape/text/image is created:
   let create_cmd = Box::new(undo_redo::CreateCommand::new(
       document_arc,
       element,
       0  // page_index
   ));
   app_state.push_command(create_cmd);
   ```

3. **Menu Actions** (in `window/actions/edit_actions.rs`)
   ```rust
   add_window_action(window, "undo", move |_| {
       if app_state.undo() {
           drawing_area.queue_draw();
       }
   });
   ```

---

## 📋 Phase 6: Alignment Tools

### Simple Alignment Implementation

```rust
// crates/ui/src/canvas/alignment.rs (NEW)

pub struct AlignmentTools;

impl AlignmentTools {
    // Align left edges
    pub fn align_left(bounds: &mut [Rect]) {
        if bounds.is_empty() { return; }
        let min_x = bounds.iter().map(|b| b.origin.x).fold(f32::MAX, f32::min);
        for bound in bounds {
            bound.origin.x = min_x;
        }
    }

    // Align horizontally centered
    pub fn align_center_h(bounds: &mut [Rect]) {
        if bounds.is_empty() { return; }
        let avg_x = bounds.iter().map(|b| b.origin.x + b.size.width / 2.0).sum::<f32>() / bounds.len() as f32;
        for bound in bounds {
            bound.origin.x = avg_x - bound.size.width / 2.0;
        }
    }

    // Similar for: align_right, align_top, align_middle, align_bottom
    // And spacing functions: space_h_equal, space_v_equal
}
```

---

## 🚀 Quick Start Implementation Guide

### Week 1: Copy/Paste/Duplicate
```
Day 1-2: Create clipboard system & commands
Day 3-4: Wire keyboard shortcuts
Day 5: Testing & debugging
```

### Week 2: Undo/Redo Integration
```
Day 1-2: Integrate into delete/create operations
Day 3-4: Wire menu actions
Day 5: Testing
```

### Week 3: Alignment Tools
```
Day 1-2: Implement alignment algorithms
Day 3-4: Add UI menu/buttons
Day 5: Testing & polish
```

---

## 📊 Feature Implementation Map

```
┌─────────────────────────────────────────┐
│      COPY/PASTE/DUPLICATE (Week 1)     │
├─────────────────────────────────────────┤
│  ✅ Clipboard Data Structure             │
│  ✅ Commands (Copy/Cut/Paste/Duplicate) │
│  ✅ Keyboard Wiring                      │
│  ✅ Tests                                │
└─────────────────────────────────────────┘
            ↓
┌─────────────────────────────────────────┐
│    UNDO/REDO INTEGRATION (Week 2)      │
├─────────────────────────────────────────┤
│  ✅ Delete/Create Commands               │
│  ✅ Canvas Operation Integration        │
│  ✅ Menu Actions (Ctrl+Z/Y)             │
│  ✅ Tests                                │
└─────────────────────────────────────────┘
            ↓
┌─────────────────────────────────────────┐
│     ALIGNMENT TOOLS (Week 3)            │
├─────────────────────────────────────────┤
│  ✅ Alignment Functions                  │
│  ✅ Multi-object Selection               │
│  ✅ UI Integration                      │
│  ✅ Tests                                │
└─────────────────────────────────────────┘
```

---

## 🔍 Testing Checklist

### Copy/Paste Tests
- [ ] Copy single object
- [ ] Copy multiple objects
- [ ] Paste in same document
- [ ] Paste offset (not overlapping)
- [ ] Clear clipboard after operations
- [ ] Undo copy/paste

### Duplicate Tests
- [ ] Duplicate single object
- [ ] Duplicate multiple objects
- [ ] New objects get new UUIDs
- [ ] Offset applied correctly
- [ ] Undo duplicate restores original

### Alignment Tests
- [ ] Align left works with 2+ objects
- [ ] Align center works
- [ ] Align right works
- [ ] Vertical alignments work
- [ ] Spacing functions work

---

## 💡 Implementation Tips

1. **Use Arc<Mutex<>> for shared document state**
   - Allows commands to modify document
   - Thread-safe for future async operations

2. **Clone elements with regenerated IDs**
   - Deep clone to avoid sharing state
   - Generate new UUIDs for duplicates
   - Maintain all other properties

3. **Test with visual feedback**
   - Call `drawing_area.queue_draw()` after operations
   - Enable tracing for debugging
   - Test Undo/Redo visually

4. **Handle edge cases**
   - Empty selection for copy/duplicate
   - Empty clipboard for paste
   - Invalid page indices
   - Document lock failures

---

## 📝 Notes for Implementation

- All infrastructure is already in place
- Just need to wire operations together
- Test thoroughly with existing test suite
- Build incrementally, test frequently
- Commit after each feature is working

---

## 🔗 Related Files

- `crates/ui/src/undo_redo.rs` - Undo/Redo system (existing)
- `crates/ui/src/io/file_io.rs` - File operations (existing)
- `crates/ui/src/canvas/input.rs` - Input handling (needs extension)
- `crates/ui/src/canvas/tools.rs` - Tool management (reference)
- `crates/ui/src/window/actions/file_actions.rs` - Action patterns (reference)
