//! Individual property group UI builders
//!
//! This module contains functions for building specific property group sections
//! including typography, text formatting, borders, layers, alignment, groups, and shape styling.

use gtk4::prelude::*;
use gtk4::{
    Adjustment, Box as GtkBox, Button, DropDown, Label, Orientation, Scale, SpinButton, Switch,
    ToggleButton,
};

/// Build title section with separator
pub(super) fn build_title_section(container: &GtkBox) {
    let title_box = GtkBox::new(Orientation::Vertical, 0);
    let title = Label::new(Some("プロパティ"));
    title.add_css_class("title-3");
    title.set_halign(gtk4::Align::Start);
    title.set_margin_start(12);
    title.set_margin_top(12);
    title.set_margin_bottom(8);
    title_box.append(&title);

    let separator = gtk4::Separator::new(Orientation::Horizontal);
    title_box.append(&separator);

    container.append(&title_box);
}

/// Build text content editing section
pub(super) fn build_text_content_section(container: &GtkBox) -> (gtk4::TextBuffer, gtk4::TextView) {
    // Header
    let content_header = GtkBox::new(Orientation::Horizontal, 8);
    content_header.set_margin_start(12);
    content_header.set_margin_top(12);

    let content_icon = Label::new(Some("📝"));
    let content_label = Label::new(Some("テキストコンテンツ"));
    content_label.set_hexpand(true);
    content_label.set_halign(gtk4::Align::Start);

    content_header.append(&content_icon);
    content_header.append(&content_label);
    container.append(&content_header);

    // Text editor section
    let content_section = GtkBox::new(Orientation::Vertical, 8);
    content_section.set_margin_start(12);
    content_section.set_margin_end(12);
    content_section.set_margin_bottom(8);

    // Create text buffer and view for editing text content
    let text_buffer = gtk4::TextBuffer::new(None);
    let text_view = gtk4::TextView::with_buffer(&text_buffer);
    text_view.set_height_request(80);
    text_view.set_wrap_mode(gtk4::WrapMode::WordChar);

    content_section.append(&text_view);
    container.append(&content_section);

    (text_buffer, text_view)
}

/// Build typography section (font family, size, line height, text alignment)
pub(super) fn build_typography_section(
    container: &GtkBox,
) -> (DropDown, SpinButton, Scale, DropDown) {
    // Header
    let typo_header = GtkBox::new(Orientation::Horizontal, 8);
    typo_header.set_margin_start(12);
    typo_header.set_margin_top(12);

    let typo_icon = Label::new(Some("✏"));
    typo_icon.add_css_class("section-icon");

    let typo_label = Label::new(Some("タイポグラフィ"));
    typo_label.add_css_class("section-heading");
    typo_label.set_halign(gtk4::Align::Start);

    typo_header.append(&typo_icon);
    typo_header.append(&typo_label);
    container.append(&typo_header);

    // Font Family
    let font_section = GtkBox::new(Orientation::Vertical, 5);
    font_section.set_margin_start(12);
    font_section.set_margin_end(12);

    let font_label = Label::new(Some("フォントファミリー"));
    font_label.set_xalign(0.0);
    font_label.add_css_class("property-label");
    font_section.append(&font_label);

    let font_family_combo = DropDown::from_strings(&[
        "Noto Sans JP",
        "Noto Serif JP",
        "Noto Sans",
        "Noto Serif",
        "Noto Sans Mono",
        "Arial",
        "Times New Roman",
        "Courier New",
        "Georgia",
        "Verdana",
        "Helvetica",
        "Liberation Mono",
        "DejaVu Sans Mono",
    ]);
    font_family_combo.set_selected(0); // Default to Noto Sans JP
    font_section.append(&font_family_combo);
    container.append(&font_section);

    // Font Size
    let size_section = GtkBox::new(Orientation::Vertical, 5);
    size_section.set_margin_start(12);
    size_section.set_margin_end(12);

    let size_label = Label::new(Some("フォントサイズ"));
    size_label.set_xalign(0.0);
    size_label.add_css_class("property-label");
    size_section.append(&size_label);

    let size_adj = Adjustment::new(14.0, 6.0, 72.0, 1.0, 5.0, 0.0);
    let font_size_spin = SpinButton::new(Some(&size_adj), 1.0, 1);
    size_section.append(&font_size_spin);
    container.append(&size_section);

    // Line Height
    let line_height_section = GtkBox::new(Orientation::Vertical, 5);
    let line_height_label = Label::new(Some("行間"));
    line_height_label.set_xalign(0.0);
    line_height_label.add_css_class("heading");
    line_height_section.append(&line_height_label);

    let line_height_adj = Adjustment::new(1.4, 1.0, 3.0, 0.1, 0.5, 0.0);
    let line_height_scale = Scale::new(gtk4::Orientation::Horizontal, Some(&line_height_adj));
    line_height_scale.set_digits(1);
    line_height_scale.set_value_pos(gtk4::PositionType::Right);
    line_height_section.append(&line_height_scale);
    container.append(&line_height_section);

    // Text Alignment
    let align_section = GtkBox::new(Orientation::Vertical, 5);
    let align_label = Label::new(Some("テキスト配置"));
    align_label.set_xalign(0.0);
    align_label.add_css_class("heading");
    align_section.append(&align_label);

    let text_align_combo = DropDown::from_strings(&["左揃え", "中央揃え", "右揃え", "両端揃え"]);
    text_align_combo.set_selected(0); // Default to left alignment
    align_section.append(&text_align_combo);
    container.append(&align_section);

    (
        font_family_combo,
        font_size_spin,
        line_height_scale,
        text_align_combo,
    )
}

/// Build text formatting section with bold/italic/underline/strikethrough buttons
pub(super) fn build_text_formatting_buttons(
    container: &GtkBox,
) -> (ToggleButton, ToggleButton, ToggleButton, ToggleButton) {
    let formatting_section = GtkBox::new(Orientation::Vertical, 5);
    formatting_section.set_margin_start(12);
    formatting_section.set_margin_end(12);

    let formatting_label = Label::new(Some("テキスト書式"));
    formatting_label.set_xalign(0.0);
    formatting_label.add_css_class("property-label");
    formatting_section.append(&formatting_label);

    // First row: Bold, Italic
    let buttons_row1 = GtkBox::new(Orientation::Horizontal, 5);
    buttons_row1.set_homogeneous(true);

    // Bold button
    let bold_button = ToggleButton::with_label("太字");
    bold_button.add_css_class("formatting-button");
    buttons_row1.append(&bold_button);

    // Italic button
    let italic_button = ToggleButton::with_label("斜体");
    italic_button.add_css_class("formatting-button");
    buttons_row1.append(&italic_button);

    formatting_section.append(&buttons_row1);

    // Second row: Underline, Strikethrough
    let buttons_row2 = GtkBox::new(Orientation::Horizontal, 5);
    buttons_row2.set_homogeneous(true);

    // Underline button
    let underline_button = ToggleButton::with_label("下線");
    underline_button.add_css_class("formatting-button");
    buttons_row2.append(&underline_button);

    // Strikethrough button
    let strikethrough_button = ToggleButton::with_label("打消し線");
    strikethrough_button.add_css_class("formatting-button");
    buttons_row2.append(&strikethrough_button);

    formatting_section.append(&buttons_row2);
    container.append(&formatting_section);

    (
        bold_button,
        italic_button,
        underline_button,
        strikethrough_button,
    )
}

/// Build text color section with color picker button
pub(super) fn build_text_color_section(container: &GtkBox) -> Button {
    let text_color_section = GtkBox::new(Orientation::Vertical, 5);
    text_color_section.set_margin_start(12);
    text_color_section.set_margin_end(12);

    let text_color_box = GtkBox::new(Orientation::Horizontal, 8);
    let text_color_label = Label::new(Some("テキスト色"));
    text_color_label.set_hexpand(true);
    text_color_label.set_xalign(0.0);
    text_color_box.append(&text_color_label);

    let text_color_button = Button::with_label("色を選択");
    text_color_button.set_halign(gtk4::Align::End);
    text_color_box.append(&text_color_button);

    text_color_section.append(&text_color_box);
    container.append(&text_color_section);

    text_color_button
}

/// Build text background color section with color picker button
pub(super) fn build_text_background_color_section(container: &GtkBox) -> Button {
    let bg_color_section = GtkBox::new(Orientation::Vertical, 5);
    bg_color_section.set_margin_start(12);
    bg_color_section.set_margin_end(12);

    let bg_color_box = GtkBox::new(Orientation::Horizontal, 8);
    let bg_color_label = Label::new(Some("背景色"));
    bg_color_label.set_hexpand(true);
    bg_color_label.set_xalign(0.0);
    bg_color_box.append(&bg_color_label);

    let bg_color_button = Button::with_label("色を選択");
    bg_color_button.set_halign(gtk4::Align::End);
    bg_color_box.append(&bg_color_button);

    bg_color_section.append(&bg_color_box);
    container.append(&bg_color_section);

    bg_color_button
}

/// Build text options section (auto-resize)
pub(super) fn build_text_options_section(container: &GtkBox) -> Switch {
    let text_options_header = GtkBox::new(Orientation::Horizontal, 8);
    text_options_header.set_margin_start(12);
    text_options_header.set_margin_top(12);

    let text_options_icon = Label::new(Some("⚙"));
    text_options_icon.add_css_class("section-icon");

    let text_options_label = Label::new(Some("テキストオプション"));
    text_options_label.add_css_class("section-heading");
    text_options_label.set_halign(gtk4::Align::Start);

    text_options_header.append(&text_options_icon);
    text_options_header.append(&text_options_label);
    container.append(&text_options_header);

    let auto_resize_section = GtkBox::new(Orientation::Vertical, 5);
    auto_resize_section.set_margin_start(12);
    auto_resize_section.set_margin_end(12);

    let auto_resize_box = GtkBox::new(Orientation::Horizontal, 8);
    let auto_resize_label = Label::new(Some("高さを自動調整"));
    auto_resize_label.set_xalign(0.0);
    auto_resize_label.set_hexpand(true);
    auto_resize_box.append(&auto_resize_label);

    let auto_resize_switch = Switch::new();
    auto_resize_switch.set_active(false);
    auto_resize_switch.set_halign(gtk4::Align::End);
    auto_resize_box.append(&auto_resize_switch);

    auto_resize_section.append(&auto_resize_box);
    container.append(&auto_resize_section);

    auto_resize_switch
}

/// Build border section
pub(super) fn build_border_section(container: &GtkBox) -> DropDown {
    let border_header = GtkBox::new(Orientation::Horizontal, 8);
    border_header.set_margin_start(12);
    border_header.set_margin_top(20);

    let border_icon = Label::new(Some("▭"));
    border_icon.add_css_class("section-icon");

    let border_label = Label::new(Some("枠線"));
    border_label.add_css_class("section-heading");
    border_label.set_halign(gtk4::Align::Start);

    border_header.append(&border_icon);
    border_header.append(&border_label);
    container.append(&border_header);

    let border_section = GtkBox::new(Orientation::Vertical, 5);
    border_section.set_margin_start(12);
    border_section.set_margin_end(12);

    let border_style_combo = DropDown::from_strings(&["なし", "細い枠線", "太い枠線"]);
    border_style_combo.set_selected(0); // Default to none
    border_section.append(&border_style_combo);
    container.append(&border_section);

    border_style_combo
}

/// Build layer section with z-order buttons
pub(super) fn build_layer_section(container: &GtkBox) -> (Button, Button, Button, Button) {
    let layer_header = GtkBox::new(Orientation::Horizontal, 8);
    layer_header.set_margin_start(12);
    layer_header.set_margin_top(20);

    let layer_icon = Label::new(Some("⧉"));
    layer_icon.add_css_class("section-icon");

    let layer_label = Label::new(Some("レイヤー順序"));
    layer_label.add_css_class("section-heading");
    layer_label.set_halign(gtk4::Align::Start);

    layer_header.append(&layer_icon);
    layer_header.append(&layer_label);
    container.append(&layer_header);

    let layer_buttons_box = GtkBox::new(Orientation::Vertical, 6);
    layer_buttons_box.set_margin_start(12);
    layer_buttons_box.set_margin_end(12);

    let top_row = GtkBox::new(Orientation::Horizontal, 6);
    top_row.set_homogeneous(true);

    let bring_to_front_btn = Button::with_label("最前面へ");
    bring_to_front_btn.set_tooltip_text(Some("Ctrl+Shift+]"));
    bring_to_front_btn.set_sensitive(false);
    top_row.append(&bring_to_front_btn);

    let bring_forward_btn = Button::with_label("前面へ");
    bring_forward_btn.set_tooltip_text(Some("Ctrl+]"));
    bring_forward_btn.set_sensitive(false);
    top_row.append(&bring_forward_btn);

    layer_buttons_box.append(&top_row);

    let bottom_row = GtkBox::new(Orientation::Horizontal, 6);
    bottom_row.set_homogeneous(true);

    let send_to_back_btn = Button::with_label("最背面へ");
    send_to_back_btn.set_tooltip_text(Some("Ctrl+Shift+["));
    send_to_back_btn.set_sensitive(false);
    bottom_row.append(&send_to_back_btn);

    let send_backward_btn = Button::with_label("背面へ");
    send_backward_btn.set_tooltip_text(Some("Ctrl+["));
    send_backward_btn.set_sensitive(false);
    bottom_row.append(&send_backward_btn);

    layer_buttons_box.append(&bottom_row);
    container.append(&layer_buttons_box);

    (
        bring_to_front_btn,
        bring_forward_btn,
        send_to_back_btn,
        send_backward_btn,
    )
}

/// Build alignment section with 6 alignment buttons
pub(super) fn build_alignment_section(
    container: &GtkBox,
) -> (Button, Button, Button, Button, Button, Button) {
    let align_header = GtkBox::new(Orientation::Horizontal, 8);
    align_header.set_margin_start(12);
    align_header.set_margin_top(12);

    let align_icon = Label::new(Some("⬆"));
    align_icon.add_css_class("section-icon");

    let align_label = Label::new(Some("整列"));
    align_label.add_css_class("section-heading");
    align_label.set_halign(gtk4::Align::Start);

    align_header.append(&align_icon);
    align_header.append(&align_label);
    container.append(&align_header);

    let align_buttons_box = GtkBox::new(Orientation::Vertical, 6);
    align_buttons_box.set_margin_start(12);
    align_buttons_box.set_margin_end(12);

    let align_h_row = GtkBox::new(Orientation::Horizontal, 6);
    align_h_row.set_homogeneous(true);

    let align_left_btn = Button::with_label("左");
    align_left_btn.set_tooltip_text(Some("左揃え"));
    align_left_btn.set_sensitive(false);
    align_h_row.append(&align_left_btn);

    let align_center_h_btn = Button::with_label("中央\n(横)");
    align_center_h_btn.set_tooltip_text(Some("中央揃え(水平)"));
    align_center_h_btn.set_sensitive(false);
    align_h_row.append(&align_center_h_btn);

    let align_right_btn = Button::with_label("右");
    align_right_btn.set_tooltip_text(Some("右揃え"));
    align_right_btn.set_sensitive(false);
    align_h_row.append(&align_right_btn);

    align_buttons_box.append(&align_h_row);

    let align_v_row = GtkBox::new(Orientation::Horizontal, 6);
    align_v_row.set_homogeneous(true);

    let align_top_btn = Button::with_label("上");
    align_top_btn.set_tooltip_text(Some("上揃え"));
    align_top_btn.set_sensitive(false);
    align_v_row.append(&align_top_btn);

    let align_center_v_btn = Button::with_label("中央\n(縦)");
    align_center_v_btn.set_tooltip_text(Some("中央揃え(垂直)"));
    align_center_v_btn.set_sensitive(false);
    align_v_row.append(&align_center_v_btn);

    let align_bottom_btn = Button::with_label("下");
    align_bottom_btn.set_tooltip_text(Some("下揃え"));
    align_bottom_btn.set_sensitive(false);
    align_v_row.append(&align_bottom_btn);

    align_buttons_box.append(&align_v_row);
    container.append(&align_buttons_box);

    (
        align_left_btn,
        align_center_h_btn,
        align_right_btn,
        align_top_btn,
        align_center_v_btn,
        align_bottom_btn,
    )
}

/// Build group section
pub(super) fn build_group_section(container: &GtkBox) -> (Label, gtk4::Entry, Button) {
    let group_header = GtkBox::new(Orientation::Horizontal, 8);
    group_header.set_margin_start(12);
    group_header.set_margin_top(12);

    let group_icon = Label::new(Some("🔗"));
    group_icon.add_css_class("section-icon");

    let group_label = Label::new(Some("グループ"));
    group_label.add_css_class("section-heading");
    group_label.set_halign(gtk4::Align::Start);

    group_header.append(&group_icon);
    group_header.append(&group_label);
    container.append(&group_header);

    let group_info_section = GtkBox::new(Orientation::Vertical, 8);
    group_info_section.set_margin_start(12);
    group_info_section.set_margin_end(12);
    group_info_section.set_margin_bottom(12);

    let group_status_label = Label::new(Some("グループ化されていません"));
    group_status_label.set_xalign(0.0);
    group_status_label.add_css_class("dim-label");
    group_info_section.append(&group_status_label);

    let group_name_label = Label::new(Some("グループ名"));
    group_name_label.set_xalign(0.0);
    group_name_label.add_css_class("property-label");
    group_info_section.append(&group_name_label);

    let group_name_entry = gtk4::Entry::new();
    group_name_entry.set_placeholder_text(Some("グループ名を入力..."));
    group_name_entry.set_sensitive(false);
    group_info_section.append(&group_name_entry);

    let ungroup_btn = Button::with_label("グループ解除");
    ungroup_btn.add_css_class("flat");
    ungroup_btn.set_sensitive(false);
    group_info_section.append(&ungroup_btn);

    container.append(&group_info_section);

    (group_status_label, group_name_entry, ungroup_btn)
}

/// Build shape styling section (colors and stroke width)
pub(super) fn build_shape_styling_section(container: &GtkBox) -> (Button, Button, SpinButton) {
    let shape_header = GtkBox::new(Orientation::Horizontal, 8);
    shape_header.set_margin_start(12);
    shape_header.set_margin_top(12);

    let shape_icon = Label::new(Some("🎨"));
    shape_icon.add_css_class("section-icon");

    let shape_label = Label::new(Some("図形スタイル"));
    shape_label.add_css_class("section-heading");
    shape_label.set_halign(gtk4::Align::Start);

    shape_header.append(&shape_icon);
    shape_header.append(&shape_label);
    container.append(&shape_header);

    let shape_section = GtkBox::new(Orientation::Vertical, 8);
    shape_section.set_margin_start(12);
    shape_section.set_margin_end(12);

    // Fill Color
    let fill_color_box = GtkBox::new(Orientation::Horizontal, 8);
    let fill_label = Label::new(Some("塗りつぶし色"));
    fill_label.set_hexpand(true);
    fill_label.set_xalign(0.0);
    fill_color_box.append(&fill_label);

    let fill_color_button = Button::with_label("色を選択");
    fill_color_button.set_halign(gtk4::Align::End);
    fill_color_box.append(&fill_color_button);
    shape_section.append(&fill_color_box);

    // Stroke Color
    let stroke_color_box = GtkBox::new(Orientation::Horizontal, 8);
    let stroke_label = Label::new(Some("枠線色"));
    stroke_label.set_hexpand(true);
    stroke_label.set_xalign(0.0);
    stroke_color_box.append(&stroke_label);

    let stroke_color_button = Button::with_label("色を選択");
    stroke_color_button.set_halign(gtk4::Align::End);
    stroke_color_box.append(&stroke_color_button);
    shape_section.append(&stroke_color_box);

    // Stroke Width
    let stroke_width_box = GtkBox::new(Orientation::Horizontal, 8);
    let stroke_width_label = Label::new(Some("線幅 (pt)"));
    stroke_width_label.set_xalign(0.0);
    stroke_width_box.append(&stroke_width_label);

    let stroke_width_adj = Adjustment::new(2.0, 0.5, 10.0, 0.5, 1.0, 0.0);
    let stroke_width_spin = SpinButton::new(Some(&stroke_width_adj), 0.5, 1);
    stroke_width_spin.set_halign(gtk4::Align::End);
    stroke_width_box.append(&stroke_width_spin);
    shape_section.append(&stroke_width_box);

    container.append(&shape_section);

    (fill_color_button, stroke_color_button, stroke_width_spin)
}
