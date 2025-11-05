use super::{Template, TemplateId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TemplateLibrary {
    templates: HashMap<TemplateId, Template>,
}

impl TemplateLibrary {
    pub fn register(&mut self, template: Template) -> TemplateRef {
        let id = template.id;
        self.templates.insert(id, template);
        TemplateRef { id }
    }

    pub fn get(&self, id: TemplateId) -> Option<Template> {
        self.templates.get(&id).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = Template> + '_ {
        self.templates.values().cloned()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TemplateRef {
    pub id: TemplateId,
}
