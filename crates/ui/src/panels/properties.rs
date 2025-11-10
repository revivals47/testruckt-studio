//! Property panel UI for editing object properties
//!
//! Provides a comprehensive property panel with sections for typography,
//! border styles, layer ordering, alignment, grouping, and shape styling.

use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, DropDown, Label, Orientation, Scale, ScrolledWindow, SpinButton, Switch,
};

use super::properties_groups::*;

/// UI components created during property panel setup
#[derive(Clone)]
pub struct PropertyPanelComponents {
    pub scrolled_window: ScrolledWindow,
    pub container: GtkBox,
    pub text_content_buffer: gtk4::TextBuffer,
    pub text_content_view: gtk4::TextView,
    pub font_family_combo: DropDown,
    pub font_size_spin: SpinButton,
    pub line_height_scale: Scale,
    pub text_align_combo: DropDown,
    pub border_style_combo: DropDown,
    pub auto_resize_switch: Switch,
    pub bold_button: gtk4::ToggleButton,
    pub italic_button: gtk4::ToggleButton,
    pub underline_button: gtk4::ToggleButton,
    pub strikethrough_button: gtk4::ToggleButton,
    pub text_color_button: Button,
    pub text_background_color_button: Button,
    pub fill_color_button: Button,
    pub stroke_color_button: Button,
    pub stroke_width_spin: SpinButton,
    pub group_status_label: Label,
    pub group_name_entry: gtk4::Entry,
    pub ungroup_btn: Button,
    pub bring_to_front_btn: Button,
    pub bring_forward_btn: Button,
    pub send_to_back_btn: Button,
    pub send_backward_btn: Button,
    pub align_left_btn: Button,
    pub align_center_h_btn: Button,
    pub align_right_btn: Button,
    pub align_top_btn: Button,
    pub align_center_v_btn: Button,
    pub align_bottom_btn: Button,
}

/// Build the property panel as a GtkBox for compatibility
pub fn build_property_panel() -> GtkBox {
    let components = build_property_panel_components();
    components.container.upcast()
}

/// Build the property panel and return both the box and the button components
pub fn build_property_panel_with_components() -> (GtkBox, PropertyPanelComponents) {
    let components = build_property_panel_components();
    (components.container.clone().upcast(), components)
}

/// Build the complete property panel with all components
fn build_property_panel_components() -> PropertyPanelComponents {
    // Create scrolled window for the panel content
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(false);

    let container = GtkBox::new(Orientation::Vertical, 10);
    container.set_margin_start(10);
    container.set_margin_end(10);
    container.set_margin_top(10);
    container.set_margin_bottom(10);
    container.set_width_request(320);

    // Title section
    build_title_section(&container);

    // Text content editing section
    let (text_content_buffer, text_content_view) = build_text_content_section(&container);

    // Typography section
    let (font_family_combo, font_size_spin, line_height_scale, text_align_combo) =
        build_typography_section(&container);

    // Text options section
    let auto_resize_switch = build_text_options_section(&container);

    // Border section
    let border_style_combo = build_border_section(&container);

    // Layer section
    let (bring_to_front_btn, bring_forward_btn, send_to_back_btn, send_backward_btn) =
        build_layer_section(&container);

    // Alignment section
    let (
        align_left_btn,
        align_center_h_btn,
        align_right_btn,
        align_top_btn,
        align_center_v_btn,
        align_bottom_btn,
    ) = build_alignment_section(&container);

    // Group section
    let (group_status_label, group_name_entry, ungroup_btn) = build_group_section(&container);

    // Text formatting section (bold/italic/underline/strikethrough buttons)
    let (bold_button, italic_button, underline_button, strikethrough_button) =
        build_text_formatting_buttons(&container);

    // Text color section (color picker button)
    let text_color_button = build_text_color_section(&container);

    // Text background color section
    let text_background_color_button = build_text_background_color_section(&container);

    // Shape styling section
    let (fill_color_button, stroke_color_button, stroke_width_spin) =
        build_shape_styling_section(&container);

    // Set the container as the child of scrolled window
    scrolled_window.set_child(Some(&container));

    PropertyPanelComponents {
        scrolled_window,
        container,
        text_content_buffer,
        text_content_view,
        font_family_combo,
        font_size_spin,
        line_height_scale,
        text_align_combo,
        border_style_combo,
        auto_resize_switch,
        bold_button,
        italic_button,
        underline_button,
        strikethrough_button,
        text_color_button,
        text_background_color_button,
        fill_color_button,
        stroke_color_button,
        stroke_width_spin,
        group_status_label,
        group_name_entry,
        ungroup_btn,
        bring_to_front_btn,
        bring_forward_btn,
        send_to_back_btn,
        send_backward_btn,
        align_left_btn,
        align_center_h_btn,
        align_right_btn,
        align_top_btn,
        align_center_v_btn,
        align_bottom_btn,
    }
}
