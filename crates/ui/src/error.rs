//! Error handling and user-friendly error messages
//!
//! Provides comprehensive error types and messages for the application

use std::fmt;

/// Application error types
#[derive(Debug)]
pub enum AppError {
    /// Document operation errors
    DocumentError(String),
    /// File I/O errors
    FileError(String),
    /// JSON serialization errors
    JsonError(String),
    /// Database operation errors
    DatabaseError(String),
    /// Canvas rendering errors
    RenderError(String),
    /// Invalid state or operation
    InvalidOperation(String),
    /// Element not found
    ElementNotFound(String),
    /// Page not found
    PageNotFound(String),
    /// Selection error
    SelectionError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DocumentError(msg) => write!(f, "ドキュメント エラー: {}", msg),
            AppError::FileError(msg) => write!(f, "ファイル エラー: {}", msg),
            AppError::JsonError(msg) => write!(f, "JSON エラー: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "データベース エラー: {}", msg),
            AppError::RenderError(msg) => write!(f, "描画 エラー: {}", msg),
            AppError::InvalidOperation(msg) => write!(f, "無効な操作: {}", msg),
            AppError::ElementNotFound(id) => write!(f, "要素 '{}' が見つかりません", id),
            AppError::PageNotFound(idx) => write!(f, "ページ {} が見つかりません", idx),
            AppError::SelectionError(msg) => write!(f, "選択 エラー: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// Result type for application errors
pub type AppResult<T> = Result<T, AppError>;

/// Validation error with suggestion
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub suggestion: Option<String>,
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }

    pub fn full_message(&self) -> String {
        let mut msg = format!("{}: {}", self.field, self.message);
        if let Some(suggestion) = &self.suggestion {
            msg.push_str(&format!("\n💡 提案: {}", suggestion));
        }
        msg
    }
}

/// Validation result type
pub type ValidationResult<T> = Result<T, ValidationError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::ElementNotFound("abc123".to_string());
        assert_eq!(err.to_string(), "要素 'abc123' が見つかりません");
    }

    #[test]
    fn test_validation_error_with_suggestion() {
        let err = ValidationError::new("font_size", "値が範囲外です")
            .with_suggestion("6～72の間の値を指定してください");

        assert!(err.full_message().contains("💡 提案"));
        assert!(err.full_message().contains("6～72"));
    }
}
