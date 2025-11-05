//! Item bank operations
//!
//! Provides CRUD operations and search functionality for the item database.

use crate::models::{Item, Passage, ItemType, Difficulty};
use crate::schema;
use anyhow::Result;
use chrono::Utc;
use rusqlite::{Connection, params};
use std::path::Path;
use tracing::{debug, info};
use uuid::Uuid;

/// ItemBank manages access to the SQLite database
pub struct ItemBank {
    conn: Connection,
}

impl ItemBank {
    /// Create a new ItemBank at the specified path
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        schema::init_schema(&conn)?;

        Ok(Self { conn })
    }

    /// Create an in-memory ItemBank for testing
    #[allow(dead_code)]
    pub fn memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        schema::init_schema(&conn)?;

        Ok(Self { conn })
    }

    // ========== Item operations ==========

    /// Insert a new item
    pub fn insert_item(&self, item: &Item) -> Result<()> {
        debug!("Inserting item: {}", item.id);

        self.conn.execute(
            "INSERT INTO items (id, title, description, content, item_type, difficulty, passage_id, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                item.id.to_string(),
                &item.title,
                &item.description,
                &item.content,
                item.item_type.as_str(),
                item.difficulty.as_str(),
                item.passage_id.map(|id| id.to_string()),
                item.created_at.to_rfc3339(),
                item.updated_at.to_rfc3339(),
            ],
        )?;

        // Insert associated skills
        for skill_id in &item.skill_ids {
            self.conn.execute(
                "INSERT INTO item_skills (item_id, skill_id) VALUES (?1, ?2)",
                params![item.id.to_string(), skill_id.to_string()],
            )?;
        }

        info!("Item {} inserted", item.id);
        Ok(())
    }

    /// Get an item by ID
    pub fn get_item(&self, id: &Uuid) -> Result<Option<Item>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, content, item_type, difficulty, passage_id, created_at, updated_at
             FROM items WHERE id = ?1"
        )?;

        let item = stmt.query_row(params![id.to_string()], |row| {
            let created_at_str = row.get::<_, String>(7)?;
            let updated_at_str = row.get::<_, String>(8)?;

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("created_at".to_string()))?
                .with_timezone(&Utc);

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("updated_at".to_string()))?
                .with_timezone(&Utc);

            Ok(Item {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                title: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
                item_type: parse_item_type(&row.get::<_, String>(4)?),
                difficulty: parse_difficulty(&row.get::<_, String>(5)?),
                skill_ids: Vec::new(), // TODO: Load from item_skills table
                passage_id: row.get::<_, Option<String>>(6)?.map(|s| Uuid::parse_str(&s).unwrap()),
                created_at,
                updated_at,
            })
        }).ok();

        Ok(item)
    }

    /// Get all items with optional filtering
    pub fn get_all_items(&self, limit: Option<i64>) -> Result<Vec<Item>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, content, item_type, difficulty, passage_id, created_at, updated_at
             FROM items ORDER BY created_at DESC LIMIT ?1"
        )?;

        let items = stmt.query_map(params![limit.unwrap_or(1000)], |row| {
            let created_at_str = row.get::<_, String>(7)?;
            let updated_at_str = row.get::<_, String>(8)?;

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("created_at".to_string()))?
                .with_timezone(&Utc);

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("updated_at".to_string()))?
                .with_timezone(&Utc);

            Ok(Item {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                title: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
                item_type: parse_item_type(&row.get::<_, String>(4)?),
                difficulty: parse_difficulty(&row.get::<_, String>(5)?),
                skill_ids: Vec::new(),
                passage_id: row.get::<_, Option<String>>(6)?.map(|s| Uuid::parse_str(&s).unwrap()),
                created_at,
                updated_at,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    /// Search items by title
    pub fn search_items(&self, query: &str) -> Result<Vec<Item>> {
        debug!("Searching for items: {}", query);

        let search_pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, content, item_type, difficulty, passage_id, created_at, updated_at
             FROM items WHERE title LIKE ?1 OR content LIKE ?1 ORDER BY created_at DESC LIMIT 100"
        )?;

        let items = stmt.query_map(params![&search_pattern], |row| {
            let created_at_str = row.get::<_, String>(7)?;
            let updated_at_str = row.get::<_, String>(8)?;

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("created_at".to_string()))?
                .with_timezone(&Utc);

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("updated_at".to_string()))?
                .with_timezone(&Utc);

            Ok(Item {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                title: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
                item_type: parse_item_type(&row.get::<_, String>(4)?),
                difficulty: parse_difficulty(&row.get::<_, String>(5)?),
                skill_ids: Vec::new(),
                passage_id: row.get::<_, Option<String>>(6)?.map(|s| Uuid::parse_str(&s).unwrap()),
                created_at,
                updated_at,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    /// Filter items by difficulty
    pub fn get_items_by_difficulty(&self, difficulty: Difficulty) -> Result<Vec<Item>> {
        debug!("Getting items by difficulty: {:?}", difficulty);

        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, content, item_type, difficulty, passage_id, created_at, updated_at
             FROM items WHERE difficulty = ?1 ORDER BY created_at DESC"
        )?;

        let items = stmt.query_map(params![difficulty.as_str()], |row| {
            let created_at_str = row.get::<_, String>(7)?;
            let updated_at_str = row.get::<_, String>(8)?;

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("created_at".to_string()))?
                .with_timezone(&Utc);

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("updated_at".to_string()))?
                .with_timezone(&Utc);

            Ok(Item {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                title: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
                item_type: parse_item_type(&row.get::<_, String>(4)?),
                difficulty: parse_difficulty(&row.get::<_, String>(5)?),
                skill_ids: Vec::new(),
                passage_id: row.get::<_, Option<String>>(6)?.map(|s| Uuid::parse_str(&s).unwrap()),
                created_at,
                updated_at,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    /// Delete an item
    pub fn delete_item(&self, id: &Uuid) -> Result<()> {
        debug!("Deleting item: {}", id);

        self.conn.execute(
            "DELETE FROM items WHERE id = ?1",
            params![id.to_string()],
        )?;

        info!("Item {} deleted", id);
        Ok(())
    }

    // ========== Passage operations ==========

    /// Insert a new passage
    pub fn insert_passage(&self, passage: &Passage) -> Result<()> {
        self.conn.execute(
            "INSERT INTO passages (id, title, content, source, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                passage.id.to_string(),
                &passage.title,
                &passage.content,
                &passage.source,
                passage.created_at.to_rfc3339(),
                passage.updated_at.to_rfc3339(),
            ],
        )?;

        info!("Passage {} inserted", passage.id);
        Ok(())
    }

    /// Get a passage by ID
    pub fn get_passage(&self, id: &Uuid) -> Result<Option<Passage>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, source, created_at, updated_at FROM passages WHERE id = ?1"
        )?;

        let passage = stmt.query_row(params![id.to_string()], |row| {
            let created_at_str = row.get::<_, String>(4)?;
            let updated_at_str = row.get::<_, String>(5)?;

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("created_at".to_string()))?
                .with_timezone(&Utc);

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|_| rusqlite::Error::InvalidParameterName("updated_at".to_string()))?
                .with_timezone(&Utc);

            Ok(Passage {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                title: row.get(1)?,
                content: row.get(2)?,
                source: row.get(3)?,
                created_at,
                updated_at,
            })
        }).ok();

        Ok(passage)
    }

    // ========== Statistics ==========

    /// Get total item count
    pub fn count_items(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM items",
            [],
            |row| row.get(0)
        )?;

        Ok(count)
    }

    /// Get count by item type
    pub fn count_items_by_type(&self, item_type: ItemType) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM items WHERE item_type = ?1",
            params![item_type.as_str()],
            |row| row.get(0)
        )?;

        Ok(count)
    }
}

/// Helper function to parse ItemType from string
fn parse_item_type(s: &str) -> ItemType {
    match s {
        "multiple_choice" => ItemType::MultipleChoice,
        "short_answer" => ItemType::ShortAnswer,
        "essay" => ItemType::Essay,
        "fill_in_the_blank" => ItemType::FillInTheBlank,
        "matching" => ItemType::Matching,
        "true_false" => ItemType::TrueFalse,
        _ => ItemType::MultipleChoice,
    }
}

/// Helper function to parse Difficulty from string
fn parse_difficulty(s: &str) -> Difficulty {
    match s {
        "easy" => Difficulty::Easy,
        "medium" => Difficulty::Medium,
        "hard" => Difficulty::Hard,
        _ => Difficulty::Medium,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_bank_creation() -> Result<()> {
        let _bank = ItemBank::memory()?;
        Ok(())
    }

    #[test]
    fn test_insert_and_get_item() -> Result<()> {
        let bank = ItemBank::memory()?;

        let item = Item {
            id: Uuid::new_v4(),
            title: "Test Item".to_string(),
            description: Some("Test description".to_string()),
            content: "What is 2+2?".to_string(),
            item_type: ItemType::MultipleChoice,
            difficulty: Difficulty::Easy,
            skill_ids: vec![],
            passage_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        bank.insert_item(&item)?;
        let retrieved = bank.get_item(&item.id)?;

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Test Item");

        Ok(())
    }

    #[test]
    fn test_count_items() -> Result<()> {
        let bank = ItemBank::memory()?;

        assert_eq!(bank.count_items()?, 0);

        let item = Item {
            id: Uuid::new_v4(),
            title: "Test Item".to_string(),
            description: None,
            content: "Content".to_string(),
            item_type: ItemType::MultipleChoice,
            difficulty: Difficulty::Medium,
            skill_ids: vec![],
            passage_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        bank.insert_item(&item)?;
        assert_eq!(bank.count_items()?, 1);

        Ok(())
    }
}
