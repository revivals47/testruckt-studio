use super::{Point, Rect, Size};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanvasLayout {
    pub page_size: Size,
    pub margin: f32,
    pub sections: Vec<LayoutSection>,
}

impl CanvasLayout {
    pub fn new(page_size: Size) -> Self {
        Self {
            page_size,
            margin: 16.0,
            sections: Vec::new(),
        }
    }

    pub fn add_section(&mut self, section: LayoutSection) {
        self.sections.push(section);
    }

    pub fn section_at(&self, point: Point) -> Option<&LayoutSection> {
        self.sections
            .iter()
            .find(|section| section.bounds.contains(point))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutSection {
    pub id: uuid::Uuid,
    pub name: String,
    pub bounds: Rect,
}

impl LayoutSection {
    pub fn new(name: impl Into<String>, bounds: Rect) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            bounds,
        }
    }
}
