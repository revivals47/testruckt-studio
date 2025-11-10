# Phase 2 Feature Analysis - Testruct Desktop Rust

**Analysis Date**: November 10, 2025
**Current Phase Status**: Phase 1 Completion (Keyboard refactoring)
**Next Phase Target**: Phase 2 (Styling & Advanced Features)

---

## 1. DATA MODEL ANALYSIS

### TextStyle Properties (Complete Inventory)

**File**: `/crates/core/src/typography/text_style.rs`

| Property | Type | Data Model | UI Control | Handler Wired | Rendering |
|----------|------|-----------|-----------|---------------|-----------|
| font_family | String | ✅ | ✅ DropDown | ✅ | ✅ |
| font_size | f32 | ✅ | ✅ SpinButton | ✅ | ✅ |
| weight | FontWeight | ✅ | ⚠️ Bold only | ⚠️ Partial | ✅ |
| alignment | TextAlignment | ✅ | ✅ DropDown | ✅ | ✅ |
| color | Color | ✅ | ✅ ColorDialog | ✅ | ✅ |
| italic | bool | ✅ | ✅ ToggleButton | ✅ | ✅ |
| underline | bool | ✅ | ❌ Missing | ❌ | ✅ |
| strikethrough | bool | ✅ | ❌ Missing | ❌ | ✅ |
| background_color | Option<Color> | ✅ | ❌ Missing | ❌ | ✅ |
| line_height | f32 | ✅ | ✅ Scale | ✅ | ✅ |

**FontWeight Enum Coverage**:
- Thin, Light, Regular, Medium, Bold, Black
- Current UI: Only Bold/Regular toggle
- **Gap**: Light, Medium, Black weights not accessible via UI

---

### ShapeElement Properties

**File**: `/crates/core/src/document/page.rs`

| Property | Type | Data Model | UI Control | Handler Wired | Rendering |
|----------|------|-----------|-----------|---------------|-----------|
| kind | ShapeKind | ✅ | ✅ Toolbar | ✅ | ✅ |
| bounds | Rect | ✅ | ✅ Canvas drag | ✅ | ✅ |
| stroke | Option<Color> | ✅ | ✅ ColorDialog | ✅ | ✅ |
| stroke_width | f32 | ✅ | ⚠️ UI exists | ❌ | ✅ |
| fill | Option<Color> | ✅ | ✅ ColorDialog | ✅ | ✅ |

**ShapeKind Enum**:
- Rectangle, Ellipse, Line, Arrow, Polygon
- All have rendering implementations

**Missing Properties**:
- corner_radius (for Rectangle) - not in data model
- dash_pattern (for Line, Arrow) - not in data model
- opacity/alpha - not in data model
- line_cap/line_join - not in data model
- fill_pattern/gradient - not in data model

---

### ImageElement Properties

**File**: `/crates/core/src/document/page.rs`

| Property | Type | Data Model | UI Control | Handler Wired | Rendering |
|----------|------|-----------|-----------|---------------|-----------|
| source | AssetRef | ✅ | ✅ FileDialog | ✅ | ✅ |
| bounds | Rect | ✅ | ✅ Canvas drag | ✅ | ✅ |

**Missing/Configurable Properties**:
- opacity/transparency - not in data model
- scale_mode (cover, contain, stretch, fill) - not in data model
- rotation - not in data model
- border/stroke around image - not in data model
- corner_radius - not in data model
- drop_shadow - not in data model

---

### FrameElement Properties

| Property | Type | Data Model | UI Control | Handler Wired | Rendering |
|----------|------|-----------|-----------|---------------|-----------|
| bounds | Rect | ✅ | ✅ Canvas drag | ✅ | ✅ |
| children | Vec<DocumentElement> | ✅ | ✅ Layers panel | ✅ | ✅ |

---

## 2. CURRENT UI IMPLEMENTATION STATUS

### Implemented UI Controls

**Property Panel Components** (from `/crates/ui/src/panels/properties.rs`):

1. **Text Content Section** ✅
   - TextBuffer for editing
   - Handler: `wire_text_content_signal()`

2. **Typography Section** ✅
   - Font Family: DropDown (13 fonts)
   - Font Size: SpinButton (6-72px)
   - Line Height: Scale (1.0-3.0)
   - Text Alignment: DropDown (4 options)
   - Handlers: All wired

3. **Text Formatting Buttons** ✅
   - Bold: ToggleButton → FontWeight::Bold
   - Italic: ToggleButton → bool
   - Handlers: Both wired
   - **Missing**: Underline, Strikethrough, Background color

4. **Text Color** ✅
   - ColorDialog button
   - Handler: `wire_text_color_signal()`

5. **Shape Styling Section** ✅
   - Fill Color: ColorDialog button
   - Stroke Color: ColorDialog button
   - Stroke Width: SpinButton
   - Handlers: Fill & Stroke wired, **Stroke Width NOT wired**

6. **Layer Controls** ✅
   - Bring to Front, Forward, Back, Backward buttons
   - Handlers: All wired

7. **Alignment Controls** ✅
   - 6 alignment buttons (L/C/R + T/M/B)
   - Handlers: All wired

8. **Grouping Controls** ⚠️ Partial
   - Group status label
   - Ungroup button (handler exists)
   - **Missing**: Group creation handler, Group name editing handler

---

### UI Controls NOT Wired (High Priority)

| Feature | Component | Status | Complexity |
|---------|-----------|--------|-----------|
| Stroke Width | SpinButton exists | UI only, no handler | 15 min |
| Underline | Missing | Need button + handler | 30 min |
| Strikethrough | Missing | Need button + handler | 30 min |
| Background Color | Missing | Need color picker + handler | 45 min |
| Light Weight | Missing | Need dropdown or weight selector | 1 hour |
| Medium Weight | Missing | Need dropdown or weight selector | 1 hour |
| Black Weight | Missing | Need dropdown or weight selector | 1 hour |
| Font Weight Full | Current only Bold/Regular | Need enhancement to dropdown | 1 hour |

---

## 3. RENDERING STATUS

### Text Rendering (rendering_text.rs) ✅ Complete

**Implemented**:
- Font family selection
- Font size scaling
- Font weight application (Thin-Black mapping)
- Italic style
- Text alignment (L/C/R, Justified fallback)
- Underline (via Pango AttrList)
- Strikethrough (via Pango AttrList)
- Background color fill
- Line height (via Pango layout)
- Text wrapping and clipping

**Gaps**:
- None identified - rendering is feature-complete

---

### Shape Rendering (shapes_rendering.rs) ✅ Complete

**Implemented**:
- Rectangle with fill and stroke
- Ellipse with fill and stroke
- Line with stroke color and width
- Arrow with stroke color and width (hardcoded size 12px)
- Polygon (pentagon) with stroke

**Gaps**:
- No fill gradient support
- No dash patterns
- No corner radius on rectangles
- No opacity/transparency
- Arrow size hardcoded (12px)

---

### Image Rendering (rendering_images.rs) ✅ Complete

**Implemented**:
- Image placeholder with icon
- Actual image loading from asset catalog
- Image scaling to fit bounds (maintains aspect ratio)
- Center positioning within bounds
- RGBA to RGB24 conversion

**Gaps**:
- No opacity/transparency
- No rotation
- No border/stroke option
- No corner radius
- No drop shadow

---

## 4. IDENTIFIED PHASE 2 FEATURES

### Quick Wins (30 min - 1 hour)

#### 1. Wire Stroke Width Handler
- **Status**: UI control exists, handler missing
- **Data Model**: ✅ `stroke_width: f32` in ShapeElement
- **UI**: ✅ `stroke_width_spin: SpinButton` exists
- **Rendering**: ✅ Already supports stroke_width in draw functions
- **Implementation**: Add signal handler to update selected shapes
- **Time**: 15-20 minutes
- **File**: `/crates/ui/src/panels/property_handlers_shape.rs`
- **Dependency**: None

#### 2. Implement Underline Text Property
- **Status**: Data model ✅, Rendering ✅, UI ❌, Handler ❌
- **Data Model**: ✅ `underline: bool` in TextStyle
- **UI**: Add ToggleButton to typography section
- **Rendering**: ✅ Already renders with Pango::Underline::Single
- **Implementation**: 
  1. Add underline toggle button to property panel
  2. Create handler function (copy italic handler pattern)
  3. Wire signal
- **Time**: 25-30 minutes
- **Files**: 
  - `/crates/ui/src/panels/properties_groups.rs` - Add button
  - `/crates/ui/src/panels/property_handlers_text.rs` - Add handler
- **Dependency**: None

#### 3. Implement Strikethrough Text Property
- **Status**: Data model ✅, Rendering ✅, UI ❌, Handler ❌
- **Data Model**: ✅ `strikethrough: bool` in TextStyle
- **UI**: Add ToggleButton to typography section
- **Rendering**: ✅ Already renders with Pango::AttrInt::new_strikethrough
- **Implementation**: Same pattern as Underline
- **Time**: 25-30 minutes
- **Files**: Same as Underline
- **Dependency**: None

#### 4. Implement Background Color for Text
- **Status**: Data model ✅, Rendering ✅, UI ❌, Handler ❌
- **Data Model**: ✅ `background_color: Option<Color>` in TextStyle
- **UI**: Add color button to text section
- **Rendering**: ✅ Already fills background if Some(color)
- **Implementation**:
  1. Add button to properties_groups.rs text color section
  2. Create handler (copy text_color_signal pattern)
  3. Wire signal
- **Time**: 35-40 minutes
- **Files**: 
  - `/crates/ui/src/panels/properties_groups.rs` - Add button
  - `/crates/ui/src/panels/property_handlers_text.rs` - Add handler
- **Dependency**: None

---

### Medium Features (1-2 hours)

#### 5. Full Font Weight Support (6 weights)
- **Current**: Only Bold (FontWeight::Bold/Regular)
- **Target**: All 6 weights (Thin, Light, Regular, Medium, Bold, Black)
- **Data Model**: ✅ FontWeight enum has all 6 variants
- **Rendering**: ✅ Pango weight mapping complete
- **UI Implementation Options**:
  - **Option A**: Replace bold button with 6-button selector (grid)
  - **Option B**: Add dropdown with 6 options
  - **Option C**: Add SegmentedButton (if available in GTK4)
- **Recommended**: Option B (DropDown) - cleaner UI
- **Time**: 50-60 minutes
- **Files**: 
  - `/crates/ui/src/panels/properties_groups.rs` - Replace bold section
  - `/crates/ui/src/panels/property_handlers_text.rs` - New handler
- **Dependency**: Replaces current Bold/Regular toggle

#### 6. Text Alignment Enhanced (add underline style)
- **Current**: 4 alignment options (L/C/R/Justified)
- **Gap**: No visual indication of current alignment
- **Enhancement**: Show text-decoration underline on dropdown button
- **Time**: 20-30 minutes
- **Files**: `/crates/ui/src/panels/properties.rs`
- **Note**: Rendering already supports Justified

#### 7. Stroke Width Real-Time Preview
- **Current**: Handler missing but UI exists
- **Gap**: User can't see stroke width change in real-time
- **Implementation**: Wire the SpinButton signal
- **Time**: 15-20 minutes
- **Files**: `/crates/ui/src/panels/property_handlers_shape.rs`

#### 8. Text Transform Support (case)
- **Feature**: Add CSS-like text-transform property
- **Options**: UPPERCASE, lowercase, Capitalize
- **Data Model**: Need to add `text_transform: Option<TextTransform>` enum
- **UI**: Add dropdown or buttons
- **Rendering**: Apply transform during text drawing
- **Time**: 45-60 minutes
- **Files**: 
  - `/crates/core/src/typography/text_style.rs` - Add enum
  - `/crates/ui/src/canvas/rendering_text.rs` - Apply transform
  - `/crates/ui/src/panels/properties_groups.rs` - UI
  - `/crates/ui/src/panels/property_handlers_text.rs` - Handler

---

### Complex Features (2-4 hours)

#### 9. Letter Spacing Support
- **Feature**: Adjust horizontal spacing between characters
- **Data Model**: Add `letter_spacing: f32` to TextStyle
- **UI**: Add SpinButton to typography section
- **Rendering**: Apply spacing in Pango via SetLetterSpacing or manual glyph positioning
- **Time**: 1.5-2 hours
- **Complexity**: Pango integration required
- **Files**: 
  - `/crates/core/src/typography/text_style.rs`
  - `/crates/ui/src/canvas/rendering_text.rs`
  - `/crates/ui/src/panels/properties_groups.rs`
  - `/crates/ui/src/panels/property_handlers_text.rs`

#### 10. Shape Fill Gradients
- **Feature**: Linear or radial gradients for shape fills
- **Data Model**: Extend ShapeElement with gradient support
  ```rust
  pub enum FillStyle {
      Solid(Color),
      LinearGradient { start_color: Color, end_color: Color, angle: f32 },
      RadialGradient { center_color: Color, edge_color: Color, radius: f32 },
  }
  pub fill: Option<FillStyle>
  ```
- **UI**: Add fill style selector + color/gradient pickers
- **Rendering**: Cairo gradient support (available)
- **Time**: 2-3 hours
- **Complexity**: High - new data types, UI selection, rendering
- **Files**: 
  - `/crates/core/src/document/page.rs`
  - `/crates/ui/src/canvas/shapes_rendering.rs`
  - `/crates/ui/src/panels/properties_groups.rs`
  - `/crates/ui/src/panels/property_handlers_shape.rs`

#### 11. Shape Stroke Dashes (patterns)
- **Feature**: Dashed, dotted, dash-dot stroke patterns
- **Data Model**: Add `stroke_dash_pattern: Option<Vec<f32>>` to ShapeElement
- **UI**: Add pattern selector (dropdown with patterns: solid, dashed, dotted, dash-dot)
- **Rendering**: Cairo set_dash support (available)
- **Time**: 1.5-2 hours
- **Files**: 
  - `/crates/core/src/document/page.rs`
  - `/crates/ui/src/canvas/shapes_rendering.rs`
  - `/crates/ui/src/panels/properties_groups.rs`
  - `/crates/ui/src/panels/property_handlers_shape.rs`

#### 12. Corner Radius for Rectangles
- **Feature**: Rounded corners with configurable radius
- **Data Model**: Add `corner_radius: Option<f32>` to ShapeElement
- **UI**: Add SpinButton for radius value (0-50px)
- **Rendering**: Use Cairo rounded_rectangle function
- **Time**: 1-1.5 hours
- **Files**: 
  - `/crates/core/src/document/page.rs`
  - `/crates/ui/src/canvas/shapes_rendering.rs`
  - `/crates/ui/src/panels/properties_groups.rs`
  - `/crates/ui/src/panels/property_handlers_shape.rs`

#### 13. Image Opacity/Transparency
- **Feature**: Adjust image transparency (0-100%)
- **Data Model**: Add `opacity: f32` (0.0-1.0) to ImageElement
- **UI**: Add Scale for opacity slider
- **Rendering**: Apply alpha blending in Cairo
- **Time**: 1-1.5 hours
- **Files**: 
  - `/crates/core/src/document/page.rs`
  - `/crates/ui/src/canvas/rendering_images.rs`
  - `/crates/ui/src/panels/properties_groups.rs`
  - `/crates/ui/src/panels/property_handlers_shape.rs` (or new image handler)

#### 14. Global Opacity for All Elements
- **Feature**: Transparency that applies to any element
- **Data Model**: Add `opacity: f32` to all elements
- **UI**: Add opacity slider to main property section
- **Rendering**: Apply to all draw functions
- **Time**: 2-2.5 hours (requires updates to all rendering functions)
- **Complexity**: High - affects all element types

#### 15. Text Drop Shadow
- **Feature**: Simple drop shadow effect for text
- **Data Model**: Add `shadow: Option<TextShadow>` where TextShadow has offset, blur, color
- **UI**: Add shadow controls section with X/Y offset, blur, color
- **Rendering**: Draw shadow before text in rendering_text.rs
- **Time**: 1.5-2 hours
- **Files**: Multiple rendering and property files

---

### Advanced Features (3+ hours)

#### 16. Shape Rotation Support
- **Feature**: Rotate shapes by angle
- **Data Model**: Add `rotation: f32` (degrees) to ShapeElement
- **UI**: Add angle spinner or rotation handle on canvas
- **Rendering**: Apply cairo rotate transform
- **Time**: 2-3 hours
- **Note**: Requires handling rotated bounds for selection
- **Files**: Multiple canvas and property files

#### 17. Box Shadow for Shapes
- **Feature**: Drop shadow effect on shapes
- **Data Model**: Add `shadow: Option<Shadow>` struct
- **UI**: Add shadow section with offset, blur, color, spread
- **Rendering**: Draw blurred copy before shape
- **Time**: 2-3 hours
- **Complexity**: High - blur effect implementation

#### 18. Text Custom Text Decoration Colors
- **Feature**: Underline/strikethrough in different color than text
- **Data Model**: Extend TextStyle to track decoration color
- **UI**: Add color picker for decoration
- **Rendering**: Custom Pango attributes
- **Time**: 1.5-2 hours

#### 19. Shape Blend Modes
- **Feature**: Multiply, Screen, Overlay, etc. blend modes
- **Data Model**: Add `blend_mode: BlendMode` enum
- **UI**: Add blend mode dropdown
- **Rendering**: Cairo blend mode operators
- **Time**: 2-2.5 hours

#### 20. Image Rotation and Flip
- **Feature**: Rotate/flip images
- **Data Model**: Add `rotation: f32` and `flip_h/flip_v: bool` to ImageElement
- **UI**: Add rotation spinner and flip buttons
- **Rendering**: Apply transforms in rendering_images.rs
- **Time**: 1.5-2 hours

---

## 5. FEATURE PRIORITY MATRIX

### Quick Wins (Implement First - Minimal Risk)

| # | Feature | Impact | Effort | Risk | Total Score |
|---|---------|--------|--------|------|-------------|
| 1 | Wire Stroke Width | High | Low | Low | **9/10** |
| 2 | Underline Text | High | Low | Low | **9/10** |
| 3 | Strikethrough Text | High | Low | Low | **9/10** |
| 4 | Background Color | High | Low | Low | **9/10** |

**Why First**: Data models and rendering already complete. Just need UI + 1 handler each.

---

### High Value (Medium Effort)

| # | Feature | Impact | Effort | Risk | Total Score |
|---|---------|--------|--------|------|-------------|
| 5 | Full Font Weights | High | Medium | Low | **8/10** |
| 7 | Stroke Width Live Preview | Medium | Low | Low | **7.5/10** |
| 8 | Text Transform | Medium | Medium | Low | **7/10** |
| 9 | Letter Spacing | High | Medium | Medium | **7.5/10** |

**Why Important**: Significantly improve typography capabilities.

---

### Advanced (High Effort, High Value)

| # | Feature | Impact | Effort | Risk | Total Score |
|---|---------|--------|--------|------|-------------|
| 10 | Shape Gradients | High | High | Medium | **7.5/10** |
| 11 | Stroke Dashes | Medium | Medium | Low | **6.5/10** |
| 12 | Corner Radius | Medium | Medium | Low | **7/10** |
| 13 | Image Opacity | Medium | Medium | Low | **7/10** |

---

## 6. RECOMMENDED PHASE 2 IMPLEMENTATION PLAN

### Week 1: Quick Wins (4 features, 2.5 hours total)

1. **Wire Stroke Width Handler** (15 min)
   - File: `/crates/ui/src/panels/property_handlers_shape.rs`
   - Add signal handler similar to fill/stroke color

2. **Implement Underline + Strikethrough** (1 hour)
   - Files: `properties_groups.rs`, `property_handlers_text.rs`
   - Add two toggle buttons and handlers

3. **Implement Background Color** (45 min)
   - Files: Same as above
   - Add color dialog button and handler

**Deliverable**: All text decorations working, stroke width controllable

### Week 2: Typography Enhancements (3 features, 3 hours)

4. **Full Font Weight Support** (1.5 hours)
   - Replace bold button with weight dropdown
   - Map all 6 weights to UI

5. **Letter Spacing** (1.5 hours)
   - Add to data model
   - Create Pango integration
   - Add UI control

**Deliverable**: Professional typography control

### Week 3: Shape & Image Features (4 features, 5 hours)

6. **Corner Radius for Rectangles** (1.5 hours)
   - Add to data model
   - Implement rounded rectangle rendering
   - Add UI spinner

7. **Stroke Dash Patterns** (1.5 hours)
   - Add pattern selector UI
   - Implement Cairo dash pattern

8. **Image Opacity** (1 hour)
   - Add opacity to ImageElement
   - Implement alpha blending

9. **Shape Opacity (Global)** (1 hour)
   - Add opacity to all elements
   - Update all render functions

**Deliverable**: Advanced shape and image styling

---

## 7. RISK ASSESSMENT & MITIGATION

### Technical Risks

**Risk 1**: Pango integration complexity for letter spacing
- **Likelihood**: Medium
- **Impact**: Could delay implementation
- **Mitigation**: Start with simple Cairo-based spacing if Pango insufficient

**Risk 2**: Performance impact of additional properties
- **Likelihood**: Low
- **Impact**: Canvas rendering slowdown
- **Mitigation**: Profile before merging, use dirty region optimization

**Risk 3**: Backwards compatibility with saved documents
- **Likelihood**: High (adding new fields)
- **Impact**: Old documents won't load
- **Mitigation**: Use `#[serde(default)]` on new fields, implement migration

### Implementation Risks

**Risk 4**: Handler complexity increases
- **Likelihood**: Medium
- **Impact**: More bugs in multi-element selection
- **Mitigation**: Add comprehensive tests for property handlers

**Risk 5**: UI layout issues with new controls
- **Likelihood**: Low
- **Impact**: Property panel becomes too crowded
- **Mitigation**: Consider collapsible sections or tabs

---

## 8. SUCCESS METRICS

### Phase 2 Completion Criteria

1. **Quick Wins** (100% must complete)
   - Stroke width handler working
   - Underline rendering
   - Strikethrough rendering
   - Background color rendering
   - Tests: 4 passing

2. **Medium Features** (80% should complete)
   - Font weight selection
   - Letter spacing (or equivalent)
   - Stroke dash patterns
   - Corner radius

3. **Advanced Features** (50% optional)
   - Image opacity
   - Shape opacity
   - Additional effects as time permits

### Quality Metrics

- All new handlers tested with multi-selection
- All properties save/load correctly
- No performance regression on 100+ object documents
- Backwards compatibility with Phase 1 documents
- Zero clippy warnings

---

## 9. DEPENDENCIES & TECHNICAL NOTES

### No External Dependencies Needed

All features can be implemented with existing:
- GTK4 widgets (SpinButton, ToggleButton, ColorDialog, DropDown)
- Cairo graphics library (supports all transformations)
- Pango text layout (supports all text attributes)
- Testruct core data structures

### Backward Compatibility Strategy

For new fields in data model:

```rust
#[derive(Serialize, Deserialize)]
pub struct TextStyle {
    // ... existing fields ...
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub strikethrough: bool,
    #[serde(default)]
    pub background_color: Option<Color>,
}
```

The `#[serde(default)]` attribute ensures old JSON documents without these fields will still load with default values.

---

## 10. ESTIMATED TIMELINE

**Total Phase 2 Duration**: 2-3 weeks

- **Week 1**: Quick wins + basic typography (16-20 hours)
- **Week 2**: Advanced typography + shape features (16-20 hours)
- **Week 3**: Image features + final integration (12-16 hours)
- **Buffer**: 5 days for testing, bug fixes, unforeseen issues

**Total Effort**: 44-56 hours

---

## CONCLUSION

Phase 2 offers significant value with relatively low implementation risk:

1. **Quick wins** require minimal effort but deliver immediately visible improvements
2. **Data models are already in place** - focus is UI + handlers
3. **Rendering implementations are complete** - no graphics engine work needed
4. **Backwards compatibility** can be maintained through serde defaults
5. **Timeline is realistic** - 2-3 weeks for comprehensive styling system

**Recommended Start**: Begin with 4 quick wins (estimated 2.5 hours), which will establish confidence and provide immediate user value.

