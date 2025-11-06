//! Typography and color utilities.

mod color;
mod font_catalog;
pub mod rich_text;
mod text_style;

pub use color::{Color, Palette};
pub use font_catalog::{FontCatalog, FontDescriptor};
pub use rich_text::{RichText, TextRun};
pub use text_style::{FontWeight, TextAlignment, TextStyle};
