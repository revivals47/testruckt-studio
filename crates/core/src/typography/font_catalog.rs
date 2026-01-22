use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontCatalog {
    fonts: HashMap<String, FontDescriptor>,
}

impl FontCatalog {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }
}

impl Default for FontCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl FontCatalog {
    pub fn register(&mut self, descriptor: FontDescriptor) {
        self.fonts.insert(descriptor.family.clone(), descriptor);
    }

    pub fn find(&self, family: &str) -> Option<&FontDescriptor> {
        self.fonts.get(family)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontDescriptor {
    pub family: String,
    pub weights: Vec<super::text_style::FontWeight>,
}
