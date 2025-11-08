//! Undo/Redo system for document operations
//!
//! Provides a command-based undo/redo stack for tracking and reverting
//! document changes, with support for batched operations.

use std::collections::VecDeque;

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

/// Delete command for removing document elements
pub struct DeleteCommand {
    document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
    element_id: uuid::Uuid,
    deleted_element: Option<testruct_core::document::DocumentElement>,
    deleted_page_index: usize,
}

impl DeleteCommand {
    /// Create a new delete command
    pub fn new(
        document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
        element_id: uuid::Uuid,
        page_index: usize,
    ) -> Self {
        Self {
            document,
            element_id,
            deleted_element: None,
            deleted_page_index: page_index,
        }
    }
}

impl Command for DeleteCommand {
    fn execute(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document lock");

        if self.deleted_page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.deleted_page_index];

        // Find and remove the element
        if let Some(position) = page
            .elements
            .iter()
            .position(|elem| elem_id(elem) == self.element_id)
        {
            self.deleted_element = Some(page.elements.remove(position));
            Ok(format!("Deleted element {}", self.element_id))
        } else {
            Err(format!("Element {} not found", self.element_id))
        }
    }

    fn undo(&mut self) -> Result<String, String> {
        if let Some(element) = self.deleted_element.take() {
            let mut doc = self.document.lock().expect("document lock");

            if self.deleted_page_index >= doc.pages.len() {
                return Err("Page index out of bounds".to_string());
            }

            doc.pages[self.deleted_page_index].add_element(element);
            Ok(format!("Restored element {}", self.element_id))
        } else {
            Err("No deleted element to restore".to_string())
        }
    }

    fn description(&self) -> &str {
        "Delete object"
    }
}

impl std::fmt::Debug for DeleteCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteCommand")
            .field("element_id", &self.element_id)
            .field("page_index", &self.deleted_page_index)
            .finish()
    }
}

/// Create command for adding new document elements
#[derive(Debug)]
pub struct CreateCommand {
    document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
    element: Option<testruct_core::document::DocumentElement>,
    element_id: Option<uuid::Uuid>,
    page_index: usize,
}

impl CreateCommand {
    /// Create a new create command
    pub fn new(
        document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
        element: testruct_core::document::DocumentElement,
        page_index: usize,
    ) -> Self {
        let element_id = Some(elem_id(&element));
        Self {
            document,
            element: Some(element),
            element_id,
            page_index,
        }
    }
}

impl Command for CreateCommand {
    fn execute(&mut self) -> Result<String, String> {
        if let Some(element) = self.element.take() {
            let element_id = elem_id(&element);
            let mut doc = self.document.lock().expect("document lock");

            if self.page_index >= doc.pages.len() {
                return Err("Page index out of bounds".to_string());
            }

            doc.pages[self.page_index].add_element(element);
            Ok(format!("Created element {}", element_id))
        } else {
            Err("No element to create".to_string())
        }
    }

    fn undo(&mut self) -> Result<String, String> {
        // Find and remove the element we created by ID
        if let Some(element_id) = self.element_id {
            let mut doc = self.document.lock().expect("document lock");

            if self.page_index >= doc.pages.len() {
                return Err("Page index out of bounds".to_string());
            }

            let page = &mut doc.pages[self.page_index];
            if let Some(position) = page
                .elements
                .iter()
                .position(|elem| elem_id(elem) == element_id)
            {
                self.element = Some(page.elements.remove(position));
                Ok(format!("Removed element {}", element_id))
            } else {
                Err(format!("Element {} not found for undo", element_id))
            }
        } else {
            Err("No element ID stored for undo".to_string())
        }
    }

    fn description(&self) -> &str {
        "Create object"
    }
}

/// Helper function to get element ID from DocumentElement enum
fn elem_id(element: &testruct_core::document::DocumentElement) -> uuid::Uuid {
    match element {
        testruct_core::document::DocumentElement::Frame(f) => f.id,
        testruct_core::document::DocumentElement::Text(t) => t.id,
        testruct_core::document::DocumentElement::Image(i) => i.id,
        testruct_core::document::DocumentElement::Shape(s) => s.id,
        testruct_core::document::DocumentElement::Group(g) => g.id,
    }
}

/// Paste command for pasting elements from clipboard
#[derive(Debug)]
pub struct PasteCommand {
    document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
    page_index: usize,
    pasted_element_ids: Vec<uuid::Uuid>,
}

impl PasteCommand {
    /// Create a new paste command
    pub fn new(
        document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
        elements: Vec<testruct_core::document::DocumentElement>,
        page_index: usize,
    ) -> Self {
        let pasted_element_ids = elements.iter().map(elem_id).collect();
        {
            let mut doc = document.lock().expect("document");
            if page_index < doc.pages.len() {
                for elem in elements {
                    doc.pages[page_index].add_element(elem);
                }
            }
        }

        Self {
            document,
            page_index,
            pasted_element_ids,
        }
    }
}

impl Command for PasteCommand {
    fn execute(&mut self) -> Result<String, String> {
        Ok(format!("Pasted {} elements", self.pasted_element_ids.len()))
    }

    fn undo(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document");

        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];
        for id in &self.pasted_element_ids {
            page.elements.retain(|e| elem_id(e) != *id);
        }

        Ok(format!("Removed {} pasted elements", self.pasted_element_ids.len()))
    }

    fn description(&self) -> &str {
        "Paste"
    }
}

/// Duplicate command for duplicating selected elements
#[derive(Debug)]
pub struct DuplicateCommand {
    document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
    page_index: usize,
    element_ids_to_duplicate: Vec<uuid::Uuid>,
    duplicated_ids: Vec<uuid::Uuid>,
}

impl DuplicateCommand {
    /// Create a new duplicate command
    pub fn new(
        document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
        element_ids: Vec<uuid::Uuid>,
        page_index: usize,
    ) -> Self {
        Self {
            document,
            page_index,
            element_ids_to_duplicate: element_ids,
            duplicated_ids: Vec::new(),
        }
    }
}

impl Command for DuplicateCommand {
    fn execute(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document");

        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];

        for orig_id in &self.element_ids_to_duplicate {
            if let Some(element) = page.elements.iter().find(|e| elem_id(e) == *orig_id) {
                let mut new_elem = element.clone();
                let new_id = uuid::Uuid::new_v4();

                // Update ID and offset position
                match &mut new_elem {
                    testruct_core::document::DocumentElement::Text(t) => {
                        t.id = new_id;
                        t.bounds.origin.x += 20.0;
                        t.bounds.origin.y += 20.0;
                    }
                    testruct_core::document::DocumentElement::Image(img) => {
                        img.id = new_id;
                        img.bounds.origin.x += 20.0;
                        img.bounds.origin.y += 20.0;
                    }
                    testruct_core::document::DocumentElement::Shape(shape) => {
                        shape.id = new_id;
                        shape.bounds.origin.x += 20.0;
                        shape.bounds.origin.y += 20.0;
                    }
                    testruct_core::document::DocumentElement::Frame(frame) => {
                        frame.id = new_id;
                        frame.bounds.origin.x += 20.0;
                        frame.bounds.origin.y += 20.0;
                    }
                    testruct_core::document::DocumentElement::Group(group) => {
                        group.id = new_id;
                        group.bounds.origin.x += 20.0;
                        group.bounds.origin.y += 20.0;
                    }
                }

                page.add_element(new_elem);
                self.duplicated_ids.push(new_id);
            }
        }

        Ok(format!("Duplicated {} objects", self.element_ids_to_duplicate.len()))
    }

    fn undo(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document");

        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];
        for dup_id in &self.duplicated_ids {
            page.elements.retain(|e| elem_id(e) != *dup_id);
        }

        Ok("Removed duplicated objects".to_string())
    }

    fn description(&self) -> &str {
        "Duplicate"
    }
}

/// Group command for grouping selected elements
#[derive(Debug)]
pub struct GroupCommand {
    document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
    group_id: uuid::Uuid,
    element_ids: Vec<uuid::Uuid>,
    page_index: usize,
    grouped_elements: Vec<testruct_core::document::DocumentElement>,
}

impl GroupCommand {
    /// Create a new group command
    pub fn new(
        document: std::sync::Arc<std::sync::Mutex<testruct_core::document::Document>>,
        element_ids: Vec<uuid::Uuid>,
        page_index: usize,
        group_name: String,
    ) -> Self {
        Self {
            document,
            group_id: uuid::Uuid::new_v4(),
            element_ids,
            page_index,
            grouped_elements: Vec::new(),
        }
    }
}

impl Command for GroupCommand {
    fn execute(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document lock");

        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];

        // Collect elements to group and calculate bounds
        let mut bounds = testruct_core::layout::Rect {
            origin: testruct_core::layout::Point { x: 0.0, y: 0.0 },
            size: testruct_core::layout::Size {
                width: 0.0,
                height: 0.0,
            },
        };

        let mut first = true;
        for element_id in &self.element_ids {
            if let Some(pos) = page.elements.iter().position(|e| elem_id(e) == *element_id) {
                let element = page.elements.get(pos).expect("element");

                // Get element bounds
                let elem_bounds = match element {
                    testruct_core::document::DocumentElement::Text(t) => &t.bounds,
                    testruct_core::document::DocumentElement::Image(i) => &i.bounds,
                    testruct_core::document::DocumentElement::Shape(s) => &s.bounds,
                    testruct_core::document::DocumentElement::Frame(f) => &f.bounds,
                    testruct_core::document::DocumentElement::Group(g) => &g.bounds,
                };

                if first {
                    bounds = elem_bounds.clone();
                    first = false;
                } else {
                    // Expand bounds to encompass all elements
                    let min_x = bounds.origin.x.min(elem_bounds.origin.x);
                    let min_y = bounds.origin.y.min(elem_bounds.origin.y);
                    let max_x = (bounds.origin.x + bounds.size.width)
                        .max(elem_bounds.origin.x + elem_bounds.size.width);
                    let max_y = (bounds.origin.y + bounds.size.height)
                        .max(elem_bounds.origin.y + elem_bounds.size.height);

                    bounds.origin.x = min_x;
                    bounds.origin.y = min_y;
                    bounds.size.width = max_x - min_x;
                    bounds.size.height = max_y - min_y;
                }
            }
        }

        // Remove elements from page (in reverse order to maintain indices)
        let mut indices_to_remove: Vec<usize> = page
            .elements
            .iter()
            .enumerate()
            .filter_map(|(idx, e)| {
                if self.element_ids.contains(&elem_id(e)) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        indices_to_remove.sort_by(|a, b| b.cmp(a)); // Remove from end to start

        for idx in indices_to_remove {
            self.grouped_elements.push(page.elements.remove(idx));
        }

        // Create group element
        let group = testruct_core::document::GroupElement {
            id: self.group_id,
            name: "Group".to_string(),
            bounds,
            children: self.grouped_elements.clone(),
        };

        // Add group to page
        page.add_element(testruct_core::document::DocumentElement::Group(group));

        Ok(format!("Grouped {} elements", self.element_ids.len()))
    }

    fn undo(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document lock");

        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];

        // Find and remove the group
        if let Some(pos) = page.elements.iter().position(|e| elem_id(e) == self.group_id) {
            if let testruct_core::document::DocumentElement::Group(group) =
                page.elements.remove(pos)
            {
                // Add children back
                for child in group.children {
                    page.add_element(child);
                }
                Ok("Ungrouped elements".to_string())
            } else {
                Err("Element is not a group".to_string())
            }
        } else {
            Err("Group not found".to_string())
        }
    }

    fn description(&self) -> &str {
        "Group objects"
    }
}
