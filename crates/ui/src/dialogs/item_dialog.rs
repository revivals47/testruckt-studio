//! Functions for creating and managing test items

use std::sync::{Arc, Mutex};
use testruct_db::{ItemBank, Item, models::{ItemType, Difficulty}};
use uuid::Uuid;
use chrono::Utc;

/// Create a new sample item and save to database
pub fn create_new_item(item_bank: Arc<Mutex<ItemBank>>, title: String, content: String, description: Option<String>) -> Option<Item> {
    // Create new item
    let item = Item {
        id: Uuid::new_v4(),
        title,
        description,
        content,
        item_type: ItemType::MultipleChoice,
        difficulty: Difficulty::Medium,
        skill_ids: Vec::new(),
        passage_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Save to database
    match item_bank.lock() {
        Ok(bank) => {
            match bank.insert_item(&item) {
                Ok(_) => {
                    tracing::info!("✅ Item created and saved: {}", item.title);
                    Some(item)
                }
                Err(e) => {
                    tracing::error!("❌ Failed to save item: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            tracing::error!("❌ Failed to lock item bank: {}", e);
            None
        }
    }
}

/// Delete an item from the database
pub fn delete_item(item_bank: Arc<Mutex<ItemBank>>, item_id: Uuid) -> Result<(), String> {
    match item_bank.lock() {
        Ok(bank) => {
            bank.delete_item(&item_id).map_err(|e| e.to_string())
        }
        Err(e) => {
            Err(format!("Failed to lock item bank: {}", e))
        }
    }
}
