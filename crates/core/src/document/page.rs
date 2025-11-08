use super::PageMetadata;
use serde::{Deserialize, Serialize};

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
}

impl DocumentElement {
    /// Get the ID of any document element
    pub fn id(&self) -> uuid::Uuid {
        match self {
            DocumentElement::Frame(f) => f.id,
            DocumentElement::Text(t) => t.id,
            DocumentElement::Image(i) => i.id,
            DocumentElement::Shape(s) => s.id,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrameElement {
    pub id: uuid::Uuid,
    pub bounds: super::super::layout::Rect,
    pub children: Vec<DocumentElement>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextElement {
    pub id: uuid::Uuid,
    pub content: String,
    pub style: crate::typography::TextStyle,
    pub bounds: super::super::layout::Rect,
    #[serde(default)]
    pub auto_resize_height: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageElement {
    pub id: uuid::Uuid,
    pub source: crate::workspace::assets::AssetRef,
    pub bounds: super::super::layout::Rect,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShapeElement {
    pub id: uuid::Uuid,
    pub kind: ShapeKind,
    pub bounds: super::super::layout::Rect,
    pub stroke: Option<crate::typography::Color>,
    pub fill: Option<crate::typography::Color>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeKind {
    Rectangle,
    Ellipse,
    Line,
    Arrow,
    Polygon,
}
