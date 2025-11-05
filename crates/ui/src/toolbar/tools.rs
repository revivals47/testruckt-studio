//! Tool selection controls for the toolbar
//!
//! Provides buttons for selecting canvas tools (Select, Rectangle, Circle, Text).

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Orientation, ToggleButton};
use crate::canvas::tools::ToolMode;

/// Tool selection button group
pub struct ToolButtons {
    pub select_btn: ToggleButton,
    pub rectangle_btn: ToggleButton,
    pub circle_btn: ToggleButton,
    pub text_btn: ToggleButton,
    pub line_btn: ToggleButton,
}

impl ToolButtons {
    /// Create a new tool button group
    pub fn new() -> Self {
        Self {
            select_btn: ToggleButton::with_label("Select"),
            rectangle_btn: ToggleButton::with_label("Rectangle"),
            circle_btn: ToggleButton::with_label("Circle"),
            text_btn: ToggleButton::with_label("Text"),
            line_btn: ToggleButton::with_label("Line"),
        }
    }

    /// Get the currently selected tool based on button state
    pub fn get_selected_tool(&self) -> ToolMode {
        if self.rectangle_btn.is_active() {
            ToolMode::Rectangle
        } else if self.circle_btn.is_active() {
            ToolMode::Circle
        } else if self.text_btn.is_active() {
            ToolMode::Text
        } else if self.line_btn.is_active() {
            ToolMode::Pan  // Placeholder for line tool
        } else {
            ToolMode::Select
        }
    }

    /// Set the currently selected tool
    pub fn set_selected_tool(&self, tool: ToolMode) {
        // Deselect all buttons
        self.select_btn.set_active(false);
        self.rectangle_btn.set_active(false);
        self.circle_btn.set_active(false);
        self.text_btn.set_active(false);
        self.line_btn.set_active(false);

        // Activate the appropriate button
        match tool {
            ToolMode::Select => self.select_btn.set_active(true),
            ToolMode::Rectangle => self.rectangle_btn.set_active(true),
            ToolMode::Circle => self.circle_btn.set_active(true),
            ToolMode::Text => self.text_btn.set_active(true),
            ToolMode::Pan => self.line_btn.set_active(true),
        }
    }
}

/// Build tool selection toolbar section
pub fn build_tool_section(container: &GtkBox) -> ToolButtons {
    let tool_header = GtkBox::new(Orientation::Horizontal, 8);
    tool_header.set_margin_start(12);
    tool_header.set_margin_top(12);

    let tool_icon = gtk4::Label::new(Some("🛠"));
    tool_icon.add_css_class("section-icon");

    let tool_label = gtk4::Label::new(Some("Shape Tools"));
    tool_label.add_css_class("section-heading");
    tool_label.set_halign(gtk4::Align::Start);

    tool_header.append(&tool_icon);
    tool_header.append(&tool_label);
    container.append(&tool_header);

    let tool_buttons_box = GtkBox::new(Orientation::Vertical, 6);
    tool_buttons_box.set_margin_start(12);
    tool_buttons_box.set_margin_end(12);

    let tool_row = GtkBox::new(Orientation::Horizontal, 6);
    tool_row.set_homogeneous(true);

    let buttons = ToolButtons::new();

    buttons.select_btn.set_active(true);  // Default to Select
    buttons.select_btn.add_css_class("flat");
    tool_row.append(&buttons.select_btn);

    buttons.rectangle_btn.add_css_class("flat");
    tool_row.append(&buttons.rectangle_btn);

    buttons.circle_btn.add_css_class("flat");
    tool_row.append(&buttons.circle_btn);

    buttons.text_btn.add_css_class("flat");
    tool_row.append(&buttons.text_btn);

    buttons.line_btn.add_css_class("flat");
    tool_row.append(&buttons.line_btn);

    tool_buttons_box.append(&tool_row);
    container.append(&tool_buttons_box);

    buttons
}
