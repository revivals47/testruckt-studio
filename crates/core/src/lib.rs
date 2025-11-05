//! Core domain model for the Testruct desktop rewrite.
//!
//! The new codebase aims for a clear separation of responsibilities by
//! breaking large modules into smaller files. Each module provides a single
//! cohesive feature so that files stay around 500 lines or less.

pub mod document;
pub mod layout;
pub mod template;
pub mod typography;
pub mod workspace;

pub use document::{Document, DocumentId, PageId};
pub use layout::{CanvasLayout, LayoutEngine, Point, Rect, Size};
pub use template::Template;
pub use workspace::{AssetCatalog, AssetRef, CommandHistory, Project, ProjectSettings};
