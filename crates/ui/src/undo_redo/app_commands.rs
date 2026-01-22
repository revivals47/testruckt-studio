//! AppState-compatible undo/redo commands
//!
//! These commands work directly with AppState instead of Arc<Mutex<Document>>,
//! making them compatible with the application's architecture.

use super::Command;
use crate::app::AppState;
use testruct_core::document::DocumentElement;
use testruct_core::typography::Color;
use uuid::Uuid;

/// Delete command that works with AppState
pub struct AppDeleteCommand {
    app_state: AppState,
    deleted_elements: Vec<DocumentElement>,
    element_ids: Vec<Uuid>,
    page_index: usize,
}

impl AppDeleteCommand {
    /// Create a new delete command
    ///
    /// This command stores the elements to be deleted so they can be restored on undo.
    pub fn new(app_state: AppState, element_ids: Vec<Uuid>, page_index: usize) -> Self {
        Self {
            app_state,
            deleted_elements: Vec::new(),
            element_ids,
            page_index,
        }
    }
}

impl Command for AppDeleteCommand {
    fn execute(&mut self) -> Result<String, String> {
        let element_ids = self.element_ids.clone();
        let page_index = self.page_index;

        self.app_state.with_mutable_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }

            let page = &mut doc.pages[page_index];

            // Find and remove elements, storing them for undo
            let mut i = 0;
            while i < page.elements.len() {
                let elem_id = page.elements[i].id();
                if element_ids.contains(&elem_id) {
                    self.deleted_elements.push(page.elements.remove(i));
                } else {
                    i += 1;
                }
            }
        });

        if self.deleted_elements.is_empty() {
            Err("No elements to delete".to_string())
        } else {
            Ok(format!("Deleted {} elements", self.deleted_elements.len()))
        }
    }

    fn undo(&mut self) -> Result<String, String> {
        if self.deleted_elements.is_empty() {
            return Err("No elements to restore".to_string());
        }

        let page_index = self.page_index;
        let elements_to_restore: Vec<DocumentElement> = self.deleted_elements.drain(..).collect();
        let count = elements_to_restore.len();

        self.app_state.with_mutable_active_document(|doc| {
            if page_index < doc.pages.len() {
                for elem in elements_to_restore {
                    doc.pages[page_index].add_element(elem);
                }
            }
        });

        Ok(format!("Restored {} elements", count))
    }

    fn description(&self) -> &str {
        "Delete"
    }
}

impl std::fmt::Debug for AppDeleteCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppDeleteCommand")
            .field("element_ids", &self.element_ids)
            .field("page_index", &self.page_index)
            .finish()
    }
}

/// Move command that works with AppState
pub struct AppMoveCommand {
    app_state: AppState,
    element_ids: Vec<Uuid>,
    page_index: usize,
    delta_x: f32,
    delta_y: f32,
}

impl AppMoveCommand {
    /// Create a new move command
    pub fn new(
        app_state: AppState,
        element_ids: Vec<Uuid>,
        page_index: usize,
        delta_x: f32,
        delta_y: f32,
    ) -> Self {
        Self {
            app_state,
            element_ids,
            page_index,
            delta_x,
            delta_y,
        }
    }

    fn apply_delta(&self, dx: f32, dy: f32) -> Result<String, String> {
        let element_ids = self.element_ids.clone();
        let page_index = self.page_index;
        let mut moved_count = 0;

        self.app_state.with_mutable_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }

            let page = &mut doc.pages[page_index];

            for element in &mut page.elements {
                if element_ids.contains(&element.id()) {
                    match element {
                        DocumentElement::Shape(shape) => {
                            shape.bounds.origin.x += dx;
                            shape.bounds.origin.y += dy;
                        }
                        DocumentElement::Text(text) => {
                            text.bounds.origin.x += dx;
                            text.bounds.origin.y += dy;
                        }
                        DocumentElement::Image(image) => {
                            image.bounds.origin.x += dx;
                            image.bounds.origin.y += dy;
                        }
                        DocumentElement::Frame(frame) => {
                            frame.bounds.origin.x += dx;
                            frame.bounds.origin.y += dy;
                        }
                        DocumentElement::Group(group) => {
                            group.bounds.origin.x += dx;
                            group.bounds.origin.y += dy;
                        }
                    }
                    moved_count += 1;
                }
            }
        });

        if moved_count > 0 {
            Ok(format!("Moved {} elements by ({}, {})", moved_count, dx, dy))
        } else {
            Err("No elements moved".to_string())
        }
    }
}

impl Command for AppMoveCommand {
    fn execute(&mut self) -> Result<String, String> {
        self.apply_delta(self.delta_x, self.delta_y)
    }

    fn undo(&mut self) -> Result<String, String> {
        self.apply_delta(-self.delta_x, -self.delta_y)
    }

    fn description(&self) -> &str {
        "Move"
    }
}

impl std::fmt::Debug for AppMoveCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppMoveCommand")
            .field("element_ids", &self.element_ids)
            .field("delta", &(self.delta_x, self.delta_y))
            .finish()
    }
}

/// Create command that works with AppState
pub struct AppCreateCommand {
    app_state: AppState,
    element: Option<DocumentElement>,
    element_id: Option<Uuid>,
    page_index: usize,
}

impl AppCreateCommand {
    /// Create a new create command
    pub fn new(app_state: AppState, element: DocumentElement, page_index: usize) -> Self {
        let element_id = Some(element.id());
        Self {
            app_state,
            element: Some(element),
            element_id,
            page_index,
        }
    }
}

impl Command for AppCreateCommand {
    fn execute(&mut self) -> Result<String, String> {
        if let Some(element) = self.element.take() {
            let element_id = element.id();
            let page_index = self.page_index;

            self.app_state.with_mutable_active_document(|doc| {
                if page_index < doc.pages.len() {
                    doc.pages[page_index].add_element(element);
                }
            });

            Ok(format!("Created element {}", element_id))
        } else {
            Err("No element to create".to_string())
        }
    }

    fn undo(&mut self) -> Result<String, String> {
        if let Some(element_id) = self.element_id {
            let page_index = self.page_index;
            let mut removed_element: Option<DocumentElement> = None;

            self.app_state.with_mutable_active_document(|doc| {
                if page_index < doc.pages.len() {
                    let page = &mut doc.pages[page_index];
                    if let Some(pos) = page.elements.iter().position(|e| e.id() == element_id) {
                        removed_element = Some(page.elements.remove(pos));
                    }
                }
            });

            if let Some(elem) = removed_element {
                self.element = Some(elem);
                Ok(format!("Removed element {}", element_id))
            } else {
                Err(format!("Element {} not found", element_id))
            }
        } else {
            Err("No element ID stored".to_string())
        }
    }

    fn description(&self) -> &str {
        "Create"
    }
}

impl std::fmt::Debug for AppCreateCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppCreateCommand")
            .field("element_id", &self.element_id)
            .field("page_index", &self.page_index)
            .finish()
    }
}

/// Group command that works with AppState
pub struct AppGroupCommand {
    app_state: AppState,
    group_id: Uuid,
    element_ids: Vec<Uuid>,
    page_index: usize,
    grouped_elements: Vec<DocumentElement>,
}

impl AppGroupCommand {
    /// Create a new group command
    pub fn new(app_state: AppState, element_ids: Vec<Uuid>, page_index: usize) -> Self {
        Self {
            app_state,
            group_id: Uuid::new_v4(),
            element_ids,
            page_index,
            grouped_elements: Vec::new(),
        }
    }
}

impl Command for AppGroupCommand {
    fn execute(&mut self) -> Result<String, String> {
        use testruct_core::document::FrameElement;
        use testruct_core::layout::{Point, Rect, Size};

        let element_ids = self.element_ids.clone();
        let page_index = self.page_index;
        let group_id = self.group_id;
        let mut bounds = Rect {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size { width: 0.0, height: 0.0 },
        };
        let mut first = true;

        self.app_state.with_mutable_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }

            let page = &mut doc.pages[page_index];

            // Calculate combined bounds
            for element in page.elements.iter() {
                if element_ids.contains(&element.id()) {
                    let elem_bounds = match element {
                        DocumentElement::Shape(s) => &s.bounds,
                        DocumentElement::Text(t) => &t.bounds,
                        DocumentElement::Image(i) => &i.bounds,
                        DocumentElement::Frame(f) => &f.bounds,
                        DocumentElement::Group(g) => &g.bounds,
                    };
                    if first {
                        bounds = *elem_bounds;
                        first = false;
                    } else {
                        let min_x = bounds.origin.x.min(elem_bounds.origin.x);
                        let min_y = bounds.origin.y.min(elem_bounds.origin.y);
                        let max_x = (bounds.origin.x + bounds.size.width)
                            .max(elem_bounds.origin.x + elem_bounds.size.width);
                        let max_y = (bounds.origin.y + bounds.size.height)
                            .max(elem_bounds.origin.y + elem_bounds.size.height);
                        bounds = Rect {
                            origin: Point { x: min_x, y: min_y },
                            size: Size { width: max_x - min_x, height: max_y - min_y },
                        };
                    }
                }
            }

            // Remove elements (from end to start) and preserve original order
            let mut indices: Vec<usize> = page.elements.iter()
                .enumerate()
                .filter_map(|(i, e)| if element_ids.contains(&e.id()) { Some(i) } else { None })
                .collect();
            indices.sort_by(|a, b| b.cmp(a));

            for idx in indices {
                self.grouped_elements.push(page.elements.remove(idx));
            }
            // Reverse to maintain original order
            self.grouped_elements.reverse();

            // Create and add frame (used as group container)
            let frame = FrameElement {
                id: group_id,
                bounds,
                children: self.grouped_elements.clone(),
                visible: true,
                locked: false,
            };
            page.add_element(DocumentElement::Frame(frame));
        });

        Ok(format!("Grouped {} elements", self.element_ids.len()))
    }

    fn undo(&mut self) -> Result<String, String> {
        let page_index = self.page_index;
        let group_id = self.group_id;

        self.app_state.with_mutable_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }

            let page = &mut doc.pages[page_index];

            // Find and remove the frame (group container)
            if let Some(pos) = page.elements.iter().position(|e| e.id() == group_id) {
                if let DocumentElement::Frame(frame) = page.elements.remove(pos) {
                    // Restore children
                    for child in frame.children {
                        page.add_element(child);
                    }
                }
            }
        });

        self.grouped_elements.clear();
        Ok("Ungrouped elements".to_string())
    }

    fn description(&self) -> &str {
        "Group"
    }
}

impl std::fmt::Debug for AppGroupCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppGroupCommand")
            .field("group_id", &self.group_id)
            .field("element_ids", &self.element_ids)
            .finish()
    }
}

/// Ungroup command that works with AppState
pub struct AppUngroupCommand {
    app_state: AppState,
    group_id: Uuid,
    page_index: usize,
    group_element: Option<DocumentElement>,
    child_ids: Vec<Uuid>,
}

impl AppUngroupCommand {
    /// Create a new ungroup command
    pub fn new(app_state: AppState, group_id: Uuid, page_index: usize) -> Self {
        Self {
            app_state,
            group_id,
            page_index,
            group_element: None,
            child_ids: Vec::new(),
        }
    }
}

impl Command for AppUngroupCommand {
    fn execute(&mut self) -> Result<String, String> {
        let page_index = self.page_index;
        let group_id = self.group_id;

        self.app_state.with_mutable_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }

            let page = &mut doc.pages[page_index];

            // Find and remove the frame (group container)
            if let Some(pos) = page.elements.iter().position(|e| e.id() == group_id) {
                let group_elem = page.elements.remove(pos);
                if let DocumentElement::Frame(frame) = &group_elem {
                    // Store child IDs for undo
                    self.child_ids = frame.children.iter().map(|c| c.id()).collect();

                    // Add children back to page
                    for child in frame.children.clone() {
                        page.add_element(child);
                    }
                }
                self.group_element = Some(group_elem);
            }
        });

        Ok(format!("Ungrouped {} elements", self.child_ids.len()))
    }

    fn undo(&mut self) -> Result<String, String> {
        let page_index = self.page_index;
        let child_ids = self.child_ids.clone();

        if let Some(group_elem) = self.group_element.take() {
            self.app_state.with_mutable_active_document(|doc| {
                if page_index >= doc.pages.len() {
                    return;
                }

                let page = &mut doc.pages[page_index];

                // Remove children
                page.elements.retain(|e| !child_ids.contains(&e.id()));

                // Add group back
                page.add_element(group_elem);
            });

            Ok("Re-grouped elements".to_string())
        } else {
            Err("No group to restore".to_string())
        }
    }

    fn description(&self) -> &str {
        "Ungroup"
    }
}

impl std::fmt::Debug for AppUngroupCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppUngroupCommand")
            .field("group_id", &self.group_id)
            .finish()
    }
}

/// Property value that can be changed and undone
#[derive(Clone, Debug)]
pub enum PropertyValue {
    StrokeColor(Option<Color>),
    FillColor(Option<Color>),
    StrokeWidth(f32),
    AutoResizeHeight(bool),
}

/// Command for changing shape/text properties with undo support
pub struct AppPropertyChangeCommand {
    app_state: AppState,
    element_ids: Vec<Uuid>,
    page_index: usize,
    old_values: Vec<(Uuid, PropertyValue)>,
    new_value: PropertyValue,
    description_text: String,
}

impl AppPropertyChangeCommand {
    /// Create a new property change command
    pub fn new(
        app_state: AppState,
        element_ids: Vec<Uuid>,
        page_index: usize,
        new_value: PropertyValue,
    ) -> Self {
        let description_text = match &new_value {
            PropertyValue::StrokeColor(_) => "Change Stroke Color".to_string(),
            PropertyValue::FillColor(_) => "Change Fill Color".to_string(),
            PropertyValue::StrokeWidth(_) => "Change Stroke Width".to_string(),
            PropertyValue::AutoResizeHeight(_) => "Change Auto Resize".to_string(),
        };

        Self {
            app_state,
            element_ids,
            page_index,
            old_values: Vec::new(),
            new_value,
            description_text,
        }
    }

    fn capture_old_values(&mut self) {
        let element_ids = self.element_ids.clone();
        let page_index = self.page_index;
        let new_value = self.new_value.clone();

        self.app_state.with_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }

            let page = &doc.pages[page_index];

            for element in &page.elements {
                if element_ids.contains(&element.id()) {
                    let old_value = match (&new_value, element) {
                        (PropertyValue::StrokeColor(_), DocumentElement::Shape(shape)) => {
                            Some(PropertyValue::StrokeColor(shape.stroke))
                        }
                        (PropertyValue::FillColor(_), DocumentElement::Shape(shape)) => {
                            Some(PropertyValue::FillColor(shape.fill))
                        }
                        (PropertyValue::StrokeWidth(_), DocumentElement::Shape(shape)) => {
                            Some(PropertyValue::StrokeWidth(shape.stroke_width))
                        }
                        (PropertyValue::AutoResizeHeight(_), DocumentElement::Text(text)) => {
                            Some(PropertyValue::AutoResizeHeight(text.auto_resize_height))
                        }
                        _ => None,
                    };

                    if let Some(value) = old_value {
                        self.old_values.push((element.id(), value));
                    }
                }
            }
        });
    }

    fn apply_value(&self, value: &PropertyValue, element_ids: &[Uuid]) -> bool {
        let page_index = self.page_index;
        let value = value.clone();
        let element_ids = element_ids.to_vec();
        let mut changed = false;

        self.app_state.with_mutable_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }

            let page = &mut doc.pages[page_index];

            for element in &mut page.elements {
                if element_ids.contains(&element.id()) {
                    match (&value, element) {
                        (PropertyValue::StrokeColor(color), DocumentElement::Shape(shape)) => {
                            shape.stroke = *color;
                            changed = true;
                        }
                        (PropertyValue::FillColor(color), DocumentElement::Shape(shape)) => {
                            shape.fill = *color;
                            changed = true;
                        }
                        (PropertyValue::StrokeWidth(width), DocumentElement::Shape(shape)) => {
                            shape.stroke_width = *width;
                            changed = true;
                        }
                        (PropertyValue::AutoResizeHeight(auto), DocumentElement::Text(text)) => {
                            text.auto_resize_height = *auto;
                            changed = true;
                        }
                        _ => {}
                    }
                }
            }
        });

        changed
    }
}

impl Command for AppPropertyChangeCommand {
    fn execute(&mut self) -> Result<String, String> {
        // Capture old values before changing
        if self.old_values.is_empty() {
            self.capture_old_values();
        }

        if self.apply_value(&self.new_value.clone(), &self.element_ids.clone()) {
            Ok(self.description_text.clone())
        } else {
            Err("No properties changed".to_string())
        }
    }

    fn undo(&mut self) -> Result<String, String> {
        let mut success = false;
        for (element_id, old_value) in &self.old_values {
            if self.apply_value(old_value, &[*element_id]) {
                success = true;
            }
        }

        if success {
            Ok(format!("Undo: {}", self.description_text))
        } else {
            Err("Failed to restore properties".to_string())
        }
    }

    fn description(&self) -> &str {
        &self.description_text
    }
}

impl std::fmt::Debug for AppPropertyChangeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppPropertyChangeCommand")
            .field("element_ids", &self.element_ids)
            .field("new_value", &self.new_value)
            .finish()
    }
}

/// Command for stroke width change with pre-captured original values
/// Used for debounced undo where values are already applied to the document
pub struct AppStrokeWidthCommand {
    app_state: AppState,
    page_index: usize,
    /// Original stroke widths (element_id, old_width)
    original_values: Vec<(Uuid, f32)>,
    /// New stroke width
    new_width: f32,
    /// Whether the command has been executed (to handle re-execution after undo)
    executed: bool,
}

impl AppStrokeWidthCommand {
    /// Create a new stroke width command with pre-captured original values
    pub fn new(
        app_state: AppState,
        page_index: usize,
        original_values: Vec<(Uuid, f32)>,
        new_width: f32,
    ) -> Self {
        Self {
            app_state,
            page_index,
            original_values,
            new_width,
            executed: false,
        }
    }
}

impl Command for AppStrokeWidthCommand {
    fn execute(&mut self) -> Result<String, String> {
        // If already executed (initial apply), just mark as executed
        // The value is already applied to the document
        if !self.executed {
            self.executed = true;
            return Ok("Change Stroke Width".to_string());
        }

        // Re-execute after undo - apply new width
        let page_index = self.page_index;
        let new_width = self.new_width;
        let element_ids: Vec<Uuid> = self.original_values.iter().map(|(id, _)| *id).collect();

        self.app_state.with_mutable_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }
            let page = &mut doc.pages[page_index];
            for element in &mut page.elements {
                if let DocumentElement::Shape(shape) = element {
                    if element_ids.contains(&shape.id) {
                        shape.stroke_width = new_width;
                    }
                }
            }
        });

        Ok("Change Stroke Width".to_string())
    }

    fn undo(&mut self) -> Result<String, String> {
        let page_index = self.page_index;
        let original_values = self.original_values.clone();

        self.app_state.with_mutable_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }
            let page = &mut doc.pages[page_index];
            for element in &mut page.elements {
                if let DocumentElement::Shape(shape) = element {
                    if let Some((_, orig_width)) = original_values.iter().find(|(id, _)| *id == shape.id) {
                        shape.stroke_width = *orig_width;
                    }
                }
            }
        });

        Ok("Undo: Change Stroke Width".to_string())
    }

    fn description(&self) -> &str {
        "Change Stroke Width"
    }
}

impl std::fmt::Debug for AppStrokeWidthCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStrokeWidthCommand")
            .field("new_width", &self.new_width)
            .field("element_count", &self.original_values.len())
            .finish()
    }
}

/// Command for resize operation with undo support
pub struct AppResizeCommand {
    app_state: AppState,
    element_id: Uuid,
    page_index: usize,
    /// Original bounds before resize
    old_bounds: testruct_core::layout::Rect,
    /// New bounds after resize
    new_bounds: testruct_core::layout::Rect,
}

impl AppResizeCommand {
    /// Create a new resize command
    pub fn new(
        app_state: AppState,
        element_id: Uuid,
        page_index: usize,
        old_bounds: testruct_core::layout::Rect,
        new_bounds: testruct_core::layout::Rect,
    ) -> Self {
        Self {
            app_state,
            element_id,
            page_index,
            old_bounds,
            new_bounds,
        }
    }

    fn apply_bounds(&self, bounds: &testruct_core::layout::Rect) -> bool {
        let page_index = self.page_index;
        let element_id = self.element_id;
        let bounds = *bounds;
        let mut applied = false;

        self.app_state.with_mutable_active_document(|doc| {
            if page_index >= doc.pages.len() {
                return;
            }
            let page = &mut doc.pages[page_index];
            for element in &mut page.elements {
                match element {
                    DocumentElement::Shape(shape) if shape.id == element_id => {
                        shape.bounds = bounds;
                        applied = true;
                        return;
                    }
                    DocumentElement::Text(text) if text.id == element_id => {
                        text.bounds = bounds;
                        applied = true;
                        return;
                    }
                    DocumentElement::Image(image) if image.id == element_id => {
                        image.bounds = bounds;
                        applied = true;
                        return;
                    }
                    DocumentElement::Frame(frame) if frame.id == element_id => {
                        frame.bounds = bounds;
                        applied = true;
                        return;
                    }
                    DocumentElement::Group(group) if group.id == element_id => {
                        group.bounds = bounds;
                        applied = true;
                        return;
                    }
                    _ => {}
                }
            }
        });

        applied
    }
}

impl Command for AppResizeCommand {
    fn execute(&mut self) -> Result<String, String> {
        if self.apply_bounds(&self.new_bounds) {
            Ok("Resize".to_string())
        } else {
            Err("Failed to apply resize".to_string())
        }
    }

    fn undo(&mut self) -> Result<String, String> {
        if self.apply_bounds(&self.old_bounds) {
            Ok("Undo: Resize".to_string())
        } else {
            Err("Failed to undo resize".to_string())
        }
    }

    fn description(&self) -> &str {
        "Resize"
    }
}

impl std::fmt::Debug for AppResizeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppResizeCommand")
            .field("element_id", &self.element_id)
            .field("old_bounds", &self.old_bounds)
            .field("new_bounds", &self.new_bounds)
            .finish()
    }
}
