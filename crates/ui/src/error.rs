//! Error handling and user-friendly error messages
//!
//! Provides comprehensive error types with friendly messages for educators.
//! Messages are localized to Japanese for the educational context.

use std::fmt;

/// Error severity level
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Non-blocking warning
    Warning,
    /// Standard error
    Error,
    /// Critical issue
    Critical,
}

/// Application error types with user-friendly messages
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

impl AppError {
    /// Get the severity level of this error
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppError::DocumentError(_) => ErrorSeverity::Error,
            AppError::FileError(_) => ErrorSeverity::Critical,
            AppError::JsonError(_) => ErrorSeverity::Error,
            AppError::DatabaseError(_) => ErrorSeverity::Critical,
            AppError::RenderError(_) => ErrorSeverity::Warning,
            AppError::InvalidOperation(_) => ErrorSeverity::Error,
            AppError::ElementNotFound(_) => ErrorSeverity::Warning,
            AppError::PageNotFound(_) => ErrorSeverity::Error,
            AppError::SelectionError(_) => ErrorSeverity::Warning,
        }
    }

    /// Get the user-friendly message
    pub fn user_message(&self) -> String {
        match self {
            AppError::DocumentError(msg) => format!("ドキュメント処理エラー：{}", msg),
            AppError::FileError(msg) => format!("ファイルを操作できませんでした：{}", msg),
            AppError::JsonError(msg) => format!("ファイル形式エラー：{}", msg),
            AppError::DatabaseError(msg) => format!("データベースエラー：{}", msg),
            AppError::RenderError(msg) => format!("描画エラー：{}", msg),
            AppError::InvalidOperation(msg) => format!("この操作はできません：{}", msg),
            AppError::ElementNotFound(id) => {
                format!("オブジェクト「{}」が見つかりません", id)
            }
            AppError::PageNotFound(idx) => {
                format!("ページ {} は存在しません", idx)
            }
            AppError::SelectionError(msg) => format!("選択エラー：{}", msg),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.user_message())
    }
}

impl std::error::Error for AppError {}

/// Result type for application errors
pub type AppResult<T> = Result<T, AppError>;

/// Validation error with suggestion for user guidance
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub suggestion: Option<String>,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
            suggestion: None,
        }
    }

    /// Add a helpful suggestion
    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }

    /// Get the full formatted message with suggestion
    pub fn full_message(&self) -> String {
        let mut msg = format!("【{}】{}", self.field, self.message);
        if let Some(suggestion) = &self.suggestion {
            msg.push_str(&format!("\n\n💡 ヒント：{}", suggestion));
        }
        msg
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_message())
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
        assert_eq!(err.to_string(), "オブジェクト「abc123」が見つかりません");
    }

    #[test]
    fn test_error_severity() {
        assert_eq!(AppError::FileError("test".to_string()).severity(), ErrorSeverity::Critical);
        assert_eq!(AppError::ElementNotFound("id".to_string()).severity(), ErrorSeverity::Warning);
    }

    #[test]
    fn test_validation_error_with_suggestion() {
        let err = ValidationError::new("font_size", "値が範囲外です")
            .with_suggestion("6～72の間の値を指定してください");

        assert!(err.full_message().contains("💡 ヒント"));
        assert!(err.full_message().contains("6～72"));
        assert!(err.full_message().contains("【font_size】"));
    }
}
