//! Workspace state and asset management for the editor.

pub mod assets;
pub mod history;
pub mod project;

pub use assets::{AssetCatalog, AssetRef};
pub use history::{CommandHistory, HistoryEntry};
pub use project::{Project, ProjectSettings};
