//! Testruct Database
//!
//! Provides SQLite-based storage for item banks, templates, and document metadata.

pub mod item_bank;
pub mod models;
pub mod schema;

pub use item_bank::ItemBank;
pub use models::{Choice, Item, Passage, Tag};

use anyhow::Result;
use std::path::Path;
use tracing::info;

/// Initialize a new database at the specified path
pub fn initialize_database(path: &Path) -> Result<ItemBank> {
    info!("Initializing database at: {}", path.display());
    ItemBank::new(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_loads() {
        // Basic module loading test
        assert!(true);
    }
}
