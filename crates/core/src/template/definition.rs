use super::style::TemplateStyle;
use crate::document::PageMetadata;
use crate::layout::CanvasLayout;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemplateId(uuid::Uuid);

impl TemplateId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for TemplateId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Template {
    pub id: TemplateId,
    pub name: String,
    pub description: Option<String>,
    pub pages: Vec<TemplatePage>,
    pub style: TemplateStyle,
}

impl Template {
    pub fn single_page(name: impl Into<String>, layout: CanvasLayout) -> Self {
        Self {
            id: TemplateId::new(),
            name: name.into(),
            description: None,
            pages: vec![TemplatePage {
                metadata: PageMetadata::default(),
                layout,
            }],
            style: TemplateStyle::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplatePage {
    pub metadata: PageMetadata,
    pub layout: CanvasLayout,
}
