//! Complete window layout matching the original design
//!
//! Implements the 3-pane layout with:
//! - Left: Tool Palette (fixed width)
//! - Center: Canvas with overlay panels and page navigation
//! - Right: Properties Panel (fixed width)
//! - Bottom: Status Bar

use crate::app::AppState;
use crate::canvas::CanvasView;
use crate::toolbar::ToolbarWidgets;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Label, Notebook, Orientation, Popover, Separator, ToggleButton,
};

/// Tool palette button references
#[derive(Clone)]
pub struct ToolPaletteButtons {
    pub select_btn: ToggleButton,
    pub text_btn: ToggleButton,
    pub image_btn: ToggleButton,
    pub rect_btn: ToggleButton,
    pub circle_btn: ToggleButton,
    pub line_btn: ToggleButton,
    pub arrow_btn: ToggleButton,
}

/// Build the complete window layout (content only - menubar and toolbars are added separately)
pub fn build_layout(
    app_state: AppState,
    _toolbar: ToolbarWidgets,
) -> (
    GtkBox,
    CanvasView,
    ToolPaletteButtons,
    GtkBox,
    GtkBox,
    crate::panels::PropertyPanelComponents,
) {
    let main_container = GtkBox::new(Orientation::Vertical, 0);

    // Note: Menubar and toolbars are added in layout.rs
    // We only build the content pane here to avoid duplication

    // 3. Main horizontal layout: Tool Palette | Canvas | Properties
    let panes_box = GtkBox::new(Orientation::Horizontal, 0);
    panes_box.set_vexpand(true);
    panes_box.set_hexpand(true);

    // LEFT: Tool Palette
    let (tool_palette, tool_buttons) = build_tool_palette();
    panes_box.append(&tool_palette);

    // CENTER: Canvas with overlays
    let canvas_view = CanvasView::new(app_state.clone());
    let (canvas_section, _page_nav_bar) = build_canvas_section(&canvas_view);
    panes_box.append(&canvas_section);

    // RIGHT: Properties Panel with Item Library
    let (properties_panel, property_components) = build_properties_panel(&app_state);
    panes_box.append(&properties_panel);

    main_container.append(&panes_box);

    // 4. Status Bar
    let status_bar = build_status_bar();
    main_container.append(&status_bar);

    (
        main_container,
        canvas_view,
        tool_buttons,
        tool_palette,
        properties_panel,
        property_components,
    )
}

/// Build the left tool palette
fn build_tool_palette() -> (GtkBox, ToolPaletteButtons) {
    let palette = GtkBox::new(Orientation::Vertical, 6);
    palette.add_css_class("tool-palette");
    palette.set_width_request(180); // Fixed width for tool palette
    palette.set_margin_start(12);
    palette.set_margin_end(12);
    palette.set_margin_top(16);
    palette.set_margin_bottom(16);

    // Tools section
    let tools_heading = Label::new(Some("ツール"));
    tools_heading.add_css_class("section-heading");
    tools_heading.set_halign(Align::Start);
    palette.append(&tools_heading);

    let select_btn = ToggleButton::with_label("選択");
    select_btn.add_css_class("tool-button");
    select_btn.set_halign(Align::Fill);
    select_btn.set_tooltip_text(Some("選択ツール (V)"));
    select_btn.set_active(true); // Select tool is active by default
    palette.append(&select_btn);

    let text_btn = ToggleButton::with_label("テキスト");
    text_btn.add_css_class("tool-button");
    text_btn.set_halign(Align::Fill);
    text_btn.set_tooltip_text(Some("テキストツール (T)"));
    palette.append(&text_btn);

    let image_btn = ToggleButton::with_label("画像");
    image_btn.add_css_class("tool-button");
    image_btn.set_halign(Align::Fill);
    image_btn.set_tooltip_text(Some("画像ツール (I)"));
    palette.append(&image_btn);

    palette.append(&Separator::new(Orientation::Horizontal));

    // Shapes section
    let shapes_heading = Label::new(Some("基本図形"));
    shapes_heading.add_css_class("section-heading");
    shapes_heading.set_halign(Align::Start);
    palette.append(&shapes_heading);

    let rect_btn = ToggleButton::with_label("長方形");
    rect_btn.add_css_class("tool-button");
    rect_btn.set_halign(Align::Fill);
    palette.append(&rect_btn);

    let circle_btn = ToggleButton::with_label("円");
    circle_btn.add_css_class("tool-button");
    circle_btn.set_halign(Align::Fill);
    palette.append(&circle_btn);

    let line_btn = ToggleButton::with_label("直線");
    line_btn.add_css_class("tool-button");
    line_btn.set_halign(Align::Fill);
    palette.append(&line_btn);

    let arrow_btn = ToggleButton::with_label("矢印");
    arrow_btn.add_css_class("tool-button");
    arrow_btn.set_halign(Align::Fill);
    palette.append(&arrow_btn);

    let tool_buttons = ToolPaletteButtons {
        select_btn,
        text_btn,
        image_btn,
        rect_btn,
        circle_btn,
        line_btn,
        arrow_btn,
    };

    (palette, tool_buttons)
}

/// Build the center canvas section with page navigation
fn build_canvas_section(canvas_view: &CanvasView) -> (GtkBox, GtkBox) {
    let canvas_frame = GtkBox::new(Orientation::Vertical, 0);
    canvas_frame.add_css_class("canvas-container");
    canvas_frame.set_hexpand(true);
    canvas_frame.set_vexpand(true);
    canvas_frame.set_margin_start(16);
    canvas_frame.set_margin_end(16);

    // Canvas scroll area
    let canvas_container = canvas_view.container();
    canvas_frame.append(&canvas_container);

    // Page navigation bar
    let page_nav_bar = build_page_nav_bar();
    canvas_frame.append(&page_nav_bar);

    (canvas_frame, page_nav_bar)
}

/// Build the page navigation bar
fn build_page_nav_bar() -> GtkBox {
    let nav_bar = GtkBox::new(Orientation::Horizontal, 8);
    nav_bar.add_css_class("page-nav-bar");
    nav_bar.set_margin_start(12);
    nav_bar.set_margin_end(12);
    nav_bar.set_margin_top(6);
    nav_bar.set_margin_bottom(6);
    nav_bar.set_hexpand(true);

    // Navigation buttons
    let nav_buttons = GtkBox::new(Orientation::Horizontal, 0);
    nav_buttons.add_css_class("linked");

    let prev_btn = Button::with_label("◀");
    prev_btn.add_css_class("flat");
    prev_btn.set_tooltip_text(Some("前のページ (PageUp)"));
    nav_buttons.append(&prev_btn);

    let next_btn = Button::with_label("▶");
    next_btn.add_css_class("flat");
    next_btn.set_tooltip_text(Some("次のページ (PageDown)"));
    nav_buttons.append(&next_btn);

    nav_bar.append(&nav_buttons);

    // Page list popover
    let page_list_popover = Popover::new();
    let page_list_btn = Button::with_label("一覧");
    page_list_btn.add_css_class("flat");
    page_list_btn.set_tooltip_text(Some("ページ一覧を開く"));
    page_list_popover.set_parent(&page_list_btn);
    nav_bar.append(&page_list_btn);

    // Page count label
    let page_label = Label::new(Some("ページ 1 / 1"));
    page_label.add_css_class("dim-label");
    page_label.set_hexpand(true);
    page_label.set_halign(Align::Center);
    nav_bar.append(&page_label);

    // Page actions
    let action_buttons = GtkBox::new(Orientation::Horizontal, 2);
    action_buttons.add_css_class("linked");

    let add_btn = Button::with_label("追加");
    add_btn.add_css_class("flat");
    add_btn.set_tooltip_text(Some("ページを追加 (Ctrl+Shift+N)"));
    action_buttons.append(&add_btn);

    let duplicate_btn = Button::with_label("複製");
    duplicate_btn.add_css_class("flat");
    duplicate_btn.set_tooltip_text(Some("現在のページを複製"));
    action_buttons.append(&duplicate_btn);

    let move_up_btn = Button::with_label("↑");
    move_up_btn.add_css_class("flat");
    move_up_btn.set_tooltip_text(Some("ページを上に移動"));
    action_buttons.append(&move_up_btn);

    let move_down_btn = Button::with_label("↓");
    move_down_btn.add_css_class("flat");
    move_down_btn.set_tooltip_text(Some("ページを下に移動"));
    action_buttons.append(&move_down_btn);

    let delete_btn = Button::with_label("削除");
    delete_btn.add_css_class("flat");
    delete_btn.set_tooltip_text(Some("現在のページを削除"));
    action_buttons.append(&delete_btn);

    nav_bar.append(&action_buttons);

    nav_bar
}

/// Build the right properties panel with tabbed interface
fn build_properties_panel(
    app_state: &AppState,
) -> (GtkBox, crate::panels::PropertyPanelComponents) {
    let properties = GtkBox::new(Orientation::Vertical, 0);
    properties.add_css_class("properties-panel");
    properties.set_width_request(240); // Fixed width
    properties.set_hexpand(false);
    properties.set_vexpand(true);

    // Create notebook for tabs
    let notebook = Notebook::new();
    notebook.set_vexpand(true);
    notebook.set_hexpand(true);
    properties.append(&notebook);

    // Tab 1: Properties
    let (props_content, property_components) =
        crate::panels::build_property_panel_with_components();
    let props_label = Label::new(Some("プロパティ"));
    notebook.append_page(&props_content, Some(&props_label));

    // Tab 2: Item Library
    let item_bank = app_state.item_bank();
    let item_lib_components = crate::panels::build_item_library_panel(item_bank);
    let item_lib_label = Label::new(Some("アイテムライブラリ"));
    notebook.append_page(&item_lib_components.container, Some(&item_lib_label));

    (properties, property_components)
}

/// Build the status bar
fn build_status_bar() -> GtkBox {
    let status_bar = GtkBox::new(Orientation::Horizontal, 12);
    status_bar.add_css_class("status-bar");
    status_bar.set_margin_start(8);
    status_bar.set_margin_end(8);
    status_bar.set_margin_top(4);
    status_bar.set_margin_bottom(4);

    let status_label = Label::new(Some("準備完了"));
    status_label.set_halign(Align::Start);
    status_bar.append(&status_label);

    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    status_bar.append(&spacer);

    let page_label = Label::new(Some("ページ 1 / 1"));
    page_label.add_css_class("dim-label");
    status_bar.append(&page_label);

    status_bar.append(&Separator::new(Orientation::Vertical));

    let objects_label = Label::new(Some("オブジェクト 0個"));
    objects_label.add_css_class("dim-label");
    status_bar.append(&objects_label);

    status_bar
}
