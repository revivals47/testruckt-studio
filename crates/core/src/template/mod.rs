//! Template definitions and catalog management.

mod definition;
mod library;
mod style;

pub use definition::{Template, TemplateId, TemplatePage};
pub use library::{TemplateLibrary, TemplateRef};
pub use style::{TemplateStyle, ThemeColors};
