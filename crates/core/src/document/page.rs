use super::PageMetadata;
use serde::{Deserialize, Serialize};

/// Default visibility value for elements (true for backwards compatibility)
fn default_visible() -> bool {
    true
}

/// Default locked value for elements (false for backwards compatibility)
fn default_locked() -> bool {
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PageId(uuid::Uuid);

impl PageId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    pub id: PageId,
    pub metadata: PageMetadata,
    pub elements: Vec<DocumentElement>,
}

impl Page {
    pub fn empty() -> Self {
        Self {
            id: PageId::new(),
            metadata: PageMetadata::default(),
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: DocumentElement) {
        self.elements.push(element);
    }

    /// Bring an element to the front (top of z-order stack)
    pub fn bring_to_front(&mut self, element_id: uuid::Uuid) -> bool {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == element_id) {
            let element = self.elements.remove(pos);
            self.elements.push(element);
            true
        } else {
            false
        }
    }

    /// Send an element to the back (bottom of z-order stack)
    pub fn send_to_back(&mut self, element_id: uuid::Uuid) -> bool {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == element_id) {
            let element = self.elements.remove(pos);
            self.elements.insert(0, element);
            true
        } else {
            false
        }
    }

    /// Move an element forward one position in z-order
    pub fn bring_forward(&mut self, element_id: uuid::Uuid) -> bool {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == element_id) {
            if pos < self.elements.len() - 1 {
                self.elements.swap(pos, pos + 1);
                true
            } else {
                false // Already at front
            }
        } else {
            false
        }
    }

    /// Move an element backward one position in z-order
    pub fn send_backward(&mut self, element_id: uuid::Uuid) -> bool {
        if let Some(pos) = self.elements.iter().position(|e| e.id() == element_id) {
            if pos > 0 {
                self.elements.swap(pos, pos - 1);
                true
            } else {
                false // Already at back
            }
        } else {
            false
        }
    }

    /// Get the z-order index of an element (0 = back, len-1 = front)
    pub fn z_order(&self, element_id: uuid::Uuid) -> Option<usize> {
        self.elements.iter().position(|e| e.id() == element_id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DocumentElement {
    Frame(FrameElement),
    Text(TextElement),
    Image(ImageElement),
    Shape(ShapeElement),
    Group(GroupElement),
}

impl DocumentElement {
    /// Get the ID of any document element
    pub fn id(&self) -> uuid::Uuid {
        match self {
            DocumentElement::Frame(f) => f.id,
            DocumentElement::Text(t) => t.id,
            DocumentElement::Image(i) => i.id,
            DocumentElement::Shape(s) => s.id,
            DocumentElement::Group(g) => g.id,
        }
    }

    /// Check if the element is visible
    pub fn is_visible(&self) -> bool {
        match self {
            DocumentElement::Frame(f) => f.visible,
            DocumentElement::Text(t) => t.visible,
            DocumentElement::Image(i) => i.visible,
            DocumentElement::Shape(s) => s.visible,
            DocumentElement::Group(g) => g.visible,
        }
    }

    /// Set the element's visibility
    pub fn set_visible(&mut self, visible: bool) {
        match self {
            DocumentElement::Frame(f) => f.visible = visible,
            DocumentElement::Text(t) => t.visible = visible,
            DocumentElement::Image(i) => i.visible = visible,
            DocumentElement::Shape(s) => s.visible = visible,
            DocumentElement::Group(g) => g.visible = visible,
        }
    }

    /// Check if the element is locked
    pub fn is_locked(&self) -> bool {
        match self {
            DocumentElement::Frame(f) => f.locked,
            DocumentElement::Text(t) => t.locked,
            DocumentElement::Image(i) => i.locked,
            DocumentElement::Shape(s) => s.locked,
            DocumentElement::Group(g) => g.locked,
        }
    }

    /// Set the element's locked state
    pub fn set_locked(&mut self, locked: bool) {
        match self {
            DocumentElement::Frame(f) => f.locked = locked,
            DocumentElement::Text(t) => t.locked = locked,
            DocumentElement::Image(i) => i.locked = locked,
            DocumentElement::Shape(s) => s.locked = locked,
            DocumentElement::Group(g) => g.locked = locked,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrameElement {
    pub id: uuid::Uuid,
    pub bounds: super::super::layout::Rect,
    pub children: Vec<DocumentElement>,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextElement {
    pub id: uuid::Uuid,
    pub content: String,
    pub style: crate::typography::TextStyle,
    pub bounds: super::super::layout::Rect,
    #[serde(default)]
    pub auto_resize_height: bool,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageElement {
    pub id: uuid::Uuid,
    pub source: crate::workspace::assets::AssetRef,
    pub bounds: super::super::layout::Rect,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShapeElement {
    pub id: uuid::Uuid,
    pub kind: ShapeKind,
    pub bounds: super::super::layout::Rect,
    pub stroke: Option<crate::typography::Color>,
    pub stroke_width: f32, // ← 新規追加（デフォルト: 2.0）
    pub fill: Option<crate::typography::Color>,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupElement {
    pub id: uuid::Uuid,
    pub name: String,
    pub bounds: super::super::layout::Rect,
    pub children: Vec<DocumentElement>,
    #[serde(default = "default_visible")]
    pub visible: bool,
    #[serde(default = "default_locked")]
    pub locked: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeKind {
    Rectangle,
    Ellipse,
    Line,
    Arrow,
    Polygon,
}
