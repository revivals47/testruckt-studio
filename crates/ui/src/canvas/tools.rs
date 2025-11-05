//! Canvas tool modes and creation operations
//!
//! Provides tool modes (Select, Rectangle, Circle, Text) and shape creation functionality.

use testruct_core::document::{DocumentElement, ShapeElement, ShapeKind, TextElement, ImageElement};
use testruct_core::layout::{Rect, Point, Size};
use testruct_core::workspace::assets::AssetRef;
use uuid::Uuid;

/// Available tool modes for canvas operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolMode {
    /// Selection and modification of existing objects
    Select,
    /// Create rectangle shapes
    Rectangle,
    /// Create circle/ellipse shapes
    Circle,
    /// Create line shapes
    Line,
    /// Create arrow shapes
    Arrow,
    /// Insert image objects
    Image,
    /// Create text objects
    Text,
    /// Pan the canvas (space+drag or middle mouse)
    Pan,
}

impl Default for ToolMode {
    fn default() -> Self {
        Self::Select
    }
}

impl ToolMode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Select => "Select",
            Self::Rectangle => "Rectangle",
            Self::Circle => "Circle",
            Self::Line => "Line",
            Self::Arrow => "Arrow",
            Self::Image => "Image",
            Self::Text => "Text",
            Self::Pan => "Pan",
        }
    }

    pub fn cursor_name(&self) -> &'static str {
        match self {
            Self::Select => "default",
            Self::Rectangle => "crosshair",
            Self::Circle => "crosshair",
            Self::Line => "crosshair",
            Self::Arrow => "crosshair",
            Self::Image => "crosshair",
            Self::Text => "text",
            Self::Pan => "grab",
        }
    }
}

/// Factory for creating shape elements
pub struct ShapeFactory;

impl ShapeFactory {
    /// Create a rectangle element
    pub fn create_rectangle(x: f64, y: f64, width: f64, height: f64) -> DocumentElement {
        DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Rectangle,
            bounds: Rect {
                origin: Point {
                    x: x as f32,
                    y: y as f32,
                },
                size: Size {
                    width: width as f32,
                    height: height as f32,
                },
            },
            stroke: None,
            fill: None,
        })
    }

    /// Create a circle/ellipse element
    pub fn create_circle(x: f64, y: f64, width: f64, height: f64) -> DocumentElement {
        DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Ellipse,
            bounds: Rect {
                origin: Point {
                    x: x as f32,
                    y: y as f32,
                },
                size: Size {
                    width: width as f32,
                    height: height as f32,
                },
            },
            stroke: None,
            fill: None,
        })
    }

    /// Create a text element
    pub fn create_text(x: f64, y: f64, width: f64, height: f64, text: String) -> DocumentElement {
        DocumentElement::Text(TextElement {
            id: Uuid::new_v4(),
            content: text,
            style: Default::default(),
            bounds: Rect {
                origin: Point {
                    x: x as f32,
                    y: y as f32,
                },
                size: Size {
                    width: width as f32,
                    height: height as f32,
                },
            },
        })
    }

    /// Create a line element
    pub fn create_line(x1: f64, y1: f64, x2: f64, y2: f64) -> DocumentElement {
        let (x, y) = (x1.min(x2), y1.min(y2));
        let (width, height) = ((x1 - x2).abs(), (y1 - y2).abs());

        DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Line,
            bounds: Rect {
                origin: Point {
                    x: x as f32,
                    y: y as f32,
                },
                size: Size {
                    width: width as f32,
                    height: height as f32,
                },
            },
            stroke: None,
            fill: None,
        })
    }

    /// Create an arrow element
    pub fn create_arrow(x1: f64, y1: f64, x2: f64, y2: f64) -> DocumentElement {
        let (x, y) = (x1.min(x2), y1.min(y2));
        let (width, height) = ((x1 - x2).abs(), (y1 - y2).abs());

        DocumentElement::Shape(ShapeElement {
            id: Uuid::new_v4(),
            kind: ShapeKind::Arrow,
            bounds: Rect {
                origin: Point {
                    x: x as f32,
                    y: y as f32,
                },
                size: Size {
                    width: width as f32,
                    height: height as f32,
                },
            },
            stroke: None,
            fill: None,
        })
    }

    /// Create an image element
    pub fn create_image(x: f64, y: f64, width: f64, height: f64) -> DocumentElement {
        DocumentElement::Image(ImageElement {
            id: Uuid::new_v4(),
            source: AssetRef::new(),
            bounds: Rect {
                origin: Point {
                    x: x as f32,
                    y: y as f32,
                },
                size: Size {
                    width: width as f32,
                    height: height as f32,
                },
            },
        })
    }
}

/// Tool state for shape creation
#[derive(Debug, Clone)]
pub struct ToolState {
    pub current_tool: ToolMode,
    pub is_creating: bool,
    pub create_start_x: f32,
    pub create_start_y: f32,
    /// Drag start position (x, y) for shape creation or movement
    pub drag_start: Option<(f64, f64)>,
    /// Currently resizing object ID
    pub resizing_object_id: Option<uuid::Uuid>,
    /// Resize handle being used
    pub resize_handle: Option<crate::canvas::mouse::ResizeHandle>,
    /// Original bounds before resize started
    pub resize_original_bounds: Option<crate::canvas::mouse::CanvasMousePos>,
    /// ID of text element currently being edited
    pub editing_text_id: Option<uuid::Uuid>,
    /// Cursor position in the edited text
    pub editing_cursor_pos: usize,
    /// Text selection start position (for future multi-click selection)
    pub editing_selection_start: Option<usize>,
}

impl Default for ToolState {
    fn default() -> Self {
        Self {
            current_tool: ToolMode::Select,
            is_creating: false,
            create_start_x: 0.0,
            create_start_y: 0.0,
            drag_start: None,
            resizing_object_id: None,
            resize_handle: None,
            resize_original_bounds: None,
            editing_text_id: None,
            editing_cursor_pos: 0,
            editing_selection_start: None,
        }
    }
}
