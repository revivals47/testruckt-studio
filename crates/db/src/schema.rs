//! Database schema definitions
//!
//! Defines SQL schema for item bank tables.

use anyhow::Result;
use rusqlite::Connection;
use tracing::debug;

/// Initialize the database schema
pub fn init_schema(conn: &Connection) -> Result<()> {
    debug!("Initializing database schema");

    // Items table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            content TEXT NOT NULL,
            item_type TEXT NOT NULL,
            difficulty TEXT NOT NULL,
            passage_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // Passages table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS passages (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            source TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // Choices table (for multiple choice items)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS choices (
            id TEXT PRIMARY KEY,
            item_id TEXT NOT NULL,
            text TEXT NOT NULL,
            is_correct INTEGER NOT NULL,
            \"order\" INTEGER NOT NULL,
            FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Tags table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL
        )",
        [],
    )?;

    // Item-Tag associations
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_tags (
            item_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            PRIMARY KEY(item_id, tag_id),
            FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE,
            FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Skills table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS skills (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            code TEXT UNIQUE NOT NULL,
            description TEXT
        )",
        [],
    )?;

    // Item-Skill associations
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_skills (
            item_id TEXT NOT NULL,
            skill_id TEXT NOT NULL,
            PRIMARY KEY(item_id, skill_id),
            FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE,
            FOREIGN KEY(skill_id) REFERENCES skills(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Usage history table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS usage_history (
            id TEXT PRIMARY KEY,
            item_id TEXT NOT NULL,
            used_at TEXT NOT NULL,
            paper_id TEXT,
            FOREIGN KEY(item_id) REFERENCES items(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create indices for performance
    create_indices(conn)?;

    debug!("Database schema initialized");
    Ok(())
}

/// Create indices for better query performance
fn create_indices(conn: &Connection) -> Result<()> {
    // Index on item_type for efficient filtering
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_items_type ON items(item_type)",
        [],
    )?;

    // Index on difficulty for efficient filtering
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_items_difficulty ON items(difficulty)",
        [],
    )?;

    // Index on item_id for choices
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_choices_item ON choices(item_id)",
        [],
    )?;

    // Index on item_id for item_tags
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_item_tags_item ON item_tags(item_id)",
        [],
    )?;

    // Index on tag_id for item_tags
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_item_tags_tag ON item_tags(tag_id)",
        [],
    )?;

    // Index on item_id for item_skills
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_item_skills_item ON item_skills(item_id)",
        [],
    )?;

    // Index on item_id for usage_history
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_usage_item ON usage_history(item_id)",
        [],
    )?;

    debug!("Database indices created");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_init() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        init_schema(&conn)?;

        // Verify tables exist
        let mut stmt =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?;

        let tables: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

        assert!(tables.contains(&"items".to_string()));
        assert!(tables.contains(&"passages".to_string()));
        assert!(tables.contains(&"choices".to_string()));
        assert!(tables.contains(&"tags".to_string()));

        Ok(())
    }
}
