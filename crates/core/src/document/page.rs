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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DocumentElement {
    Frame(FrameElement),
    Text(TextElement),
    Image(ImageElement),
    Shape(ShapeElement),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ShapeKind {
    Rectangle,
    Ellipse,
    Line,
    Polygon,
}
