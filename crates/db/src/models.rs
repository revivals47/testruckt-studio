//! Data models for the item bank
//!
//! Defines structures for items, passages, choices, and tags.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An item in the item bank (test question)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub item_type: ItemType,
    pub difficulty: Difficulty,
    pub skill_ids: Vec<Uuid>,
    pub passage_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Type of item (question format)
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemType {
    MultipleChoice,
    ShortAnswer,
    Essay,
    FillInTheBlank,
    Matching,
    TrueFalse,
}

impl ItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemType::MultipleChoice => "multiple_choice",
            ItemType::ShortAnswer => "short_answer",
            ItemType::Essay => "essay",
            ItemType::FillInTheBlank => "fill_in_the_blank",
            ItemType::Matching => "matching",
            ItemType::TrueFalse => "true_false",
        }
    }
}

/// Difficulty level of an item
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Easy = 1,
    Medium = 2,
    Hard = 3,
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        }
    }
}

/// A passage/text that items can reference
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Passage {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A choice option for multiple choice items
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub id: Uuid,
    pub item_id: Uuid,
    pub text: String,
    pub is_correct: bool,
    pub order: i32,
}

/// A tag for organizing items
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub category: String,
}

/// A skill/standard that items can address
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_type_display() {
        assert_eq!(ItemType::MultipleChoice.as_str(), "multiple_choice");
    }

    #[test]
    fn test_difficulty_ordering() {
        assert!(Difficulty::Easy < Difficulty::Medium);
        assert!(Difficulty::Medium < Difficulty::Hard);
    }
}
