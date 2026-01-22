//! Undo/Redo system for document operations
//!
//! Provides a command-based undo/redo stack for tracking and reverting
//! document changes, with support for batched operations.

use std::collections::VecDeque;

// Re-export command implementations from submodules
mod app_commands;
mod undo_redo_group;
mod undo_redo_shape;
mod undo_redo_text;

pub use undo_redo_group::GroupCommand;
pub use undo_redo_shape::{CreateCommand, DeleteCommand, MoveCommand};
pub use undo_redo_text::{DuplicateCommand, PasteCommand};

// AppState-compatible commands (recommended for new code)
pub use app_commands::{
    AppCreateCommand, AppDeleteCommand, AppGroupCommand, AppMoveCommand,
    AppPropertyChangeCommand, AppResizeCommand, AppStrokeWidthCommand, AppUngroupCommand,
    PropertyValue,
};

/// Command trait for undo/redo operations
pub trait Command: std::fmt::Debug {
    /// Execute the command and return a description
    fn execute(&mut self) -> Result<String, String>;

    /// Undo the command
    fn undo(&mut self) -> Result<String, String>;

    /// Get a description of this command
    fn description(&self) -> &str;
}

/// Undo/Redo stack manager
#[derive(Debug)]
pub struct UndoRedoStack {
    /// Commands that can be undone (uses VecDeque for O(1) operations)
    undo_stack: VecDeque<Box<dyn Command>>,

    /// Commands that can be redone (uses VecDeque for O(1) operations)
    redo_stack: VecDeque<Box<dyn Command>>,

    /// Maximum number of commands to keep in history
    max_history: usize,

    /// Whether we're currently in the middle of a batch operation
    in_batch: bool,

    /// Batched commands being accumulated
    batch_commands: Vec<Box<dyn Command>>,

    /// Description for the current batch
    batch_description: String,
}

impl UndoRedoStack {
    /// Create a new undo/redo stack with default capacity (100 commands)
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Create a new undo/redo stack with specified capacity
    pub fn with_capacity(max_history: usize) -> Self {
        Self {
            undo_stack: VecDeque::with_capacity(max_history),
            redo_stack: VecDeque::with_capacity(max_history),
            max_history,
            in_batch: false,
            batch_commands: Vec::new(),
            batch_description: String::new(),
        }
    }

    /// Push a command onto the undo stack
    pub fn push(&mut self, mut command: Box<dyn Command>) {
        if let Ok(_desc) = command.execute() {
            if self.in_batch {
                // If in batch mode, accumulate the command
                self.batch_commands.push(command);
            } else {
                // Clear redo stack when new command is executed
                self.redo_stack.clear();

                // Add to undo stack
                self.undo_stack.push_back(command);

                // Limit stack size (O(1) operation with VecDeque)
                if self.undo_stack.len() > self.max_history {
                    self.undo_stack.pop_front();
                }
            }
        }
    }

    /// Start a batch operation
    pub fn begin_batch(&mut self, description: &str) {
        self.in_batch = true;
        self.batch_description = description.to_string();
        self.batch_commands.clear();
    }

    /// End a batch operation and push as a single command
    pub fn end_batch(&mut self) {
        if self.in_batch && !self.batch_commands.is_empty() {
            let commands = std::mem::take(&mut self.batch_commands);
            let batch_cmd = BatchCommand::new(commands, &self.batch_description);
            self.undo_stack.push_back(Box::new(batch_cmd));

            // Limit stack size
            if self.undo_stack.len() > self.max_history {
                self.undo_stack.pop_front();
            }

            // Clear redo stack
            self.redo_stack.clear();
        }
        self.in_batch = false;
        self.batch_commands.clear();
    }

    /// Undo the last command
    pub fn undo(&mut self) -> bool {
        if let Some(mut command) = self.undo_stack.pop_back() {
            if command.undo().is_ok() {
                self.redo_stack.push_back(command);
                return true;
            }
        }
        false
    }

    /// Redo the last undone command
    pub fn redo(&mut self) -> bool {
        if let Some(mut command) = self.redo_stack.pop_back() {
            if command.execute().is_ok() {
                self.undo_stack.push_back(command);
                return true;
            }
        }
        false
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty() && !self.in_batch
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty() && !self.in_batch
    }

    /// Get description of the next undo command
    pub fn undo_description(&self) -> Option<&str> {
        self.undo_stack.back().map(|cmd| cmd.description())
    }

    /// Get description of the next redo command
    pub fn redo_description(&self) -> Option<&str> {
        self.redo_stack.back().map(|cmd| cmd.description())
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of commands in the undo stack
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of commands in the redo stack
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
}

impl Default for UndoRedoStack {
    fn default() -> Self {
        Self::new()
    }
}

/// A batch of multiple commands treated as one
#[derive(Debug)]
struct BatchCommand {
    commands: Vec<Box<dyn Command>>,
    description: String,
}

impl BatchCommand {
    fn new(commands: Vec<Box<dyn Command>>, description: &str) -> Self {
        Self {
            commands,
            description: description.to_string(),
        }
    }
}

impl Command for BatchCommand {
    fn execute(&mut self) -> Result<String, String> {
        for cmd in &mut self.commands {
            cmd.execute()?;
        }
        Ok(self.description.clone())
    }

    fn undo(&mut self) -> Result<String, String> {
        // Undo in reverse order
        for cmd in self.commands.iter_mut().rev() {
            cmd.undo()?;
        }
        Ok(format!("Undo: {}", self.description))
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MockCommand {
        executed: bool,
        description: String,
    }

    impl MockCommand {
        fn new(desc: &str) -> Self {
            Self {
                executed: false,
                description: desc.to_string(),
            }
        }
    }

    impl Command for MockCommand {
        fn execute(&mut self) -> Result<String, String> {
            self.executed = true;
            Ok(self.description.clone())
        }

        fn undo(&mut self) -> Result<String, String> {
            self.executed = false;
            Ok(format!("Undo: {}", self.description))
        }

        fn description(&self) -> &str {
            &self.description
        }
    }

    #[test]
    fn test_undo_redo_stack_creation() {
        let stack = UndoRedoStack::new();
        assert_eq!(stack.undo_count(), 0);
        assert_eq!(stack.redo_count(), 0);
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn test_push_and_undo() {
        let mut stack = UndoRedoStack::new();
        let cmd = Box::new(MockCommand::new("Test command"));
        stack.push(cmd);

        assert!(stack.can_undo());
        assert!(!stack.can_redo());
        assert!(stack.undo());
        assert!(!stack.can_undo());
        assert!(stack.can_redo());
    }

    #[test]
    fn test_batch_commands() {
        let mut stack = UndoRedoStack::new();

        stack.begin_batch("Batch operation");
        stack.push(Box::new(MockCommand::new("Cmd 1")));
        stack.push(Box::new(MockCommand::new("Cmd 2")));
        stack.end_batch();

        assert!(stack.can_undo());
        assert_eq!(stack.undo_count(), 1); // Should be single batched command
    }
}
