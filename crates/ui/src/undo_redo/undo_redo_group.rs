//! Group-related undo/redo commands
//!
//! Provides commands for grouping and ungrouping document elements.

use super::Command;
use super::undo_redo_shape::elem_id;
use std::sync::{Arc, Mutex};
use testruct_core::document::{Document, DocumentElement, GroupElement};
use testruct_core::layout::{Point, Rect, Size};
use uuid::Uuid;

/// Group command for grouping selected elements
#[derive(Debug)]
pub struct GroupCommand {
    document: Arc<Mutex<Document>>,
    group_id: Uuid,
    element_ids: Vec<Uuid>,
    page_index: usize,
    grouped_elements: Vec<DocumentElement>,
}

impl GroupCommand {
    /// Create a new group command
    pub fn new(
        document: Arc<Mutex<Document>>,
        element_ids: Vec<Uuid>,
        page_index: usize,
        _group_name: String,
    ) -> Self {
        Self {
            document,
            group_id: Uuid::new_v4(),
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
        let mut bounds = Rect {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
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
                    DocumentElement::Text(t) => &t.bounds,
                    DocumentElement::Image(i) => &i.bounds,
                    DocumentElement::Shape(s) => &s.bounds,
                    DocumentElement::Frame(f) => &f.bounds,
                    DocumentElement::Group(g) => &g.bounds,
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
        let group = GroupElement {
            id: self.group_id,
            name: "Group".to_string(),
            bounds,
            children: self.grouped_elements.clone(),
        };

        // Add group to page
        page.add_element(DocumentElement::Group(group));

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
            if let DocumentElement::Group(group) = page.elements.remove(pos) {
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
