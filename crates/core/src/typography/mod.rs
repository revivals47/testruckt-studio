//! Typography and color utilities.

mod color;
mod text_style;
mod font_catalog;
pub mod rich_text;

pub use color::{Color, Palette};
pub use font_catalog::{FontCatalog, FontDescriptor};
pub use text_style::{TextAlignment, TextStyle, FontWeight};
pub use rich_text::{RichText, TextRun};
