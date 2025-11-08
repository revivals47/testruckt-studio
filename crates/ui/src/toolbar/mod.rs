//! Toolbar construction and management
//!
//! Provides builders for the primary and secondary toolbars with all controls
//! organized by functional groups.

pub mod toolbar_shapes;

use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Label, MenuButton, Orientation, Popover, ScrolledWindow, Separator,
    ToggleButton,
};

/// Container for all toolbar-related widgets
#[derive(Clone)]
pub struct ToolbarWidgets {
    /// Primary toolbar (document controls, history, workflow, view options, zoom)
    pub primary_toolbar: GtkBox,
    /// Secondary toolbar (object operations, view toggles)
    pub secondary_toolbar: GtkBox,
    /// All buttons referenced by event handlers
    pub buttons: ToolbarButtons,
    /// View options popover
    pub view_popover: Popover,
    /// Guide list popover
    pub guide_list_popover: Popover,
    /// Guide list box for dynamically adding guides
    pub guide_list_box: GtkBox,
}

/// All toolbar buttons organized by function
#[derive(Clone)]
pub struct ToolbarButtons {
    // Document controls (Primary toolbar)
    pub new_btn: Button,
    pub open_btn: Button,
    pub save_btn: Button,
    pub export_btn: Button,
    pub image_export_btn: Button,

    // History controls (Primary toolbar)
    pub undo_btn: Button,
    pub redo_btn: Button,

    // Workflow shortcuts (Primary toolbar)
    pub template_btn: Button,
    pub json_editor_btn: Button,
    pub settings_btn: Button,

    // Panel toggles (Primary toolbar)
    pub item_library_btn: ToggleButton,
    pub block_tools_btn: ToggleButton,

    // View menu button (Primary toolbar)
    pub view_menu_btn: MenuButton,

    // Zoom controls (Primary toolbar)
    pub zoom_out_btn: Button,
    pub zoom_100_btn: Button,
    pub zoom_in_btn: Button,

    // Page info label (Primary toolbar)
    pub page_info_label: Label,

    // View popover toggles
    pub ruler_menu_toggle: ToggleButton,
    pub guides_menu_toggle: ToggleButton,
    pub snap_to_guides_toggle: ToggleButton,
    pub guide_list_menu_btn: Button,

    // Object operations (Secondary toolbar)
    pub group_btn: Button,
    pub ungroup_btn: Button,
    pub lock_btn: Button,
    pub unlock_btn: Button,

    // View toggles (Secondary toolbar)
    pub grid_toggle_btn: ToggleButton,
    pub guides_visible_btn: ToggleButton,
    pub rulers_visible_btn: ToggleButton,
    pub snap_to_guides_btn: ToggleButton,
}

/// Public build function for toolbar
pub fn build_toolbar() -> ToolbarWidgets {
    ToolbarBuilder::build()
}

/// Builder for toolbar construction
pub struct ToolbarBuilder;

impl ToolbarBuilder {
    /// Build both primary and secondary toolbars with all controls
    fn build() -> ToolbarWidgets {
        let (primary_toolbar, primary_buttons, guide_list_box) = Self::build_primary_toolbar();
        let (secondary_toolbar, secondary_buttons) = Self::build_secondary_toolbar();

        let buttons = ToolbarButtons {
            new_btn: primary_buttons.new_btn,
            open_btn: primary_buttons.open_btn,
            save_btn: primary_buttons.save_btn,
            export_btn: primary_buttons.export_btn,
            image_export_btn: primary_buttons.image_export_btn,
            undo_btn: primary_buttons.undo_btn,
            redo_btn: primary_buttons.redo_btn,
            template_btn: primary_buttons.template_btn,
            json_editor_btn: primary_buttons.json_editor_btn,
            settings_btn: primary_buttons.settings_btn,
            item_library_btn: primary_buttons.item_library_btn,
            block_tools_btn: primary_buttons.block_tools_btn,
            view_menu_btn: primary_buttons.view_menu_btn,
            zoom_out_btn: primary_buttons.zoom_out_btn,
            zoom_100_btn: primary_buttons.zoom_100_btn,
            zoom_in_btn: primary_buttons.zoom_in_btn,
            page_info_label: primary_buttons.page_info_label,
            ruler_menu_toggle: primary_buttons.ruler_menu_toggle,
            guides_menu_toggle: primary_buttons.guides_menu_toggle,
            snap_to_guides_toggle: primary_buttons.snap_to_guides_toggle,
            guide_list_menu_btn: primary_buttons.guide_list_menu_btn,
            group_btn: secondary_buttons.group_btn,
            ungroup_btn: secondary_buttons.ungroup_btn,
            lock_btn: secondary_buttons.lock_btn,
            unlock_btn: secondary_buttons.unlock_btn,
            grid_toggle_btn: secondary_buttons.grid_toggle_btn,
            guides_visible_btn: secondary_buttons.guides_visible_btn,
            rulers_visible_btn: secondary_buttons.rulers_visible_btn,
            snap_to_guides_btn: secondary_buttons.snap_to_guides_btn,
        };

        ToolbarWidgets {
            primary_toolbar,
            secondary_toolbar,
            buttons,
            view_popover: primary_buttons.view_popover,
            guide_list_popover: primary_buttons.guide_list_popover,
            guide_list_box,
        }
    }

    /// Build the primary toolbar with document, history, workflow, view, and zoom controls
    fn build_primary_toolbar() -> (GtkBox, PrimaryToolbarButtons, GtkBox) {
        let primary_toolbar = GtkBox::new(Orientation::Horizontal, 10);
        primary_toolbar.add_css_class("toolbar");
        primary_toolbar.set_margin_start(12);
        primary_toolbar.set_margin_end(12);
        primary_toolbar.set_margin_top(4);
        primary_toolbar.set_margin_bottom(4);

        // Document controls section
        let document_controls = GtkBox::new(Orientation::Horizontal, 2);
        document_controls.add_css_class("linked");

        let new_btn = Button::with_label("新規");
        new_btn.add_css_class("flat");
        new_btn.set_tooltip_text(Some("新しいドキュメントを作成"));
        document_controls.append(&new_btn);

        let open_btn = Button::with_label("開く…");
        open_btn.add_css_class("flat");
        open_btn.set_tooltip_text(Some("既存のドキュメントを開く"));
        document_controls.append(&open_btn);

        let save_btn = Button::with_label("保存");
        save_btn.add_css_class("flat");
        save_btn.set_tooltip_text(Some("現在のドキュメントを保存"));
        document_controls.append(&save_btn);

        let export_btn = Button::with_label("PDF");
        export_btn.add_css_class("flat");
        export_btn.set_tooltip_text(Some("PDFとして書き出す"));
        document_controls.append(&export_btn);

        let image_export_btn = Button::with_label("画像");
        image_export_btn.add_css_class("flat");
        image_export_btn.set_tooltip_text(Some("PNG/JPEG/SVGとして書き出す"));
        document_controls.append(&image_export_btn);

        primary_toolbar.append(&document_controls);

        let doc_separator = Separator::new(Orientation::Vertical);
        doc_separator.set_margin_start(8);
        doc_separator.set_margin_end(8);
        primary_toolbar.append(&doc_separator);

        // History section (Undo/Redo)
        let history_box = GtkBox::new(Orientation::Horizontal, 2);
        history_box.add_css_class("linked");

        let undo_btn = Button::with_label("元に戻す");
        undo_btn.add_css_class("flat");
        undo_btn.set_tooltip_text(Some("元に戻す (Ctrl+Z)"));
        undo_btn.set_sensitive(false);
        history_box.append(&undo_btn);

        let redo_btn = Button::with_label("やり直す");
        redo_btn.add_css_class("flat");
        redo_btn.set_tooltip_text(Some("やり直す (Ctrl+Shift+Z)"));
        redo_btn.set_sensitive(false);
        history_box.append(&redo_btn);

        primary_toolbar.append(&history_box);

        let history_separator = Separator::new(Orientation::Vertical);
        history_separator.set_margin_start(8);
        history_separator.set_margin_end(8);
        primary_toolbar.append(&history_separator);

        // Workflow section (Templates, JSON, Settings)
        let workflow_box = GtkBox::new(Orientation::Horizontal, 2);
        workflow_box.add_css_class("linked");

        let template_btn = Button::with_label("テンプレート");
        template_btn.add_css_class("flat");
        template_btn.set_tooltip_text(Some("テンプレートからドキュメントを作成"));
        workflow_box.append(&template_btn);

        let json_editor_btn = Button::with_label("JSON");
        json_editor_btn.add_css_class("flat");
        json_editor_btn.set_tooltip_text(Some("JSONとして編集"));
        workflow_box.append(&json_editor_btn);

        let settings_btn = Button::with_label("設定");
        settings_btn.add_css_class("flat");
        settings_btn.set_tooltip_text(Some("アプリケーション設定"));
        workflow_box.append(&settings_btn);

        primary_toolbar.append(&workflow_box);

        let workflow_separator = Separator::new(Orientation::Vertical);
        workflow_separator.set_margin_start(8);
        workflow_separator.set_margin_end(8);
        primary_toolbar.append(&workflow_separator);

        // Panel toggles (Item Library, Block Tools)
        let panel_toggle_box = GtkBox::new(Orientation::Horizontal, 2);
        panel_toggle_box.add_css_class("linked");

        let item_library_btn = ToggleButton::with_label("ライブラリ");
        item_library_btn.add_css_class("flat");
        item_library_btn.set_tooltip_text(Some("アイテムライブラリを表示/非表示"));
        panel_toggle_box.append(&item_library_btn);

        let block_tools_btn = ToggleButton::with_label("ブロック");
        block_tools_btn.add_css_class("flat");
        block_tools_btn.set_tooltip_text(Some("ブロックツールを表示/非表示"));
        panel_toggle_box.append(&block_tools_btn);

        primary_toolbar.append(&panel_toggle_box);

        let panel_separator = Separator::new(Orientation::Vertical);
        panel_separator.set_margin_start(8);
        panel_separator.set_margin_end(8);
        primary_toolbar.append(&panel_separator);

        // View popover content
        let view_popover = Popover::new();
        let view_popover_box = GtkBox::new(Orientation::Vertical, 8);
        view_popover_box.set_margin_start(12);
        view_popover_box.set_margin_end(12);
        view_popover_box.set_margin_top(12);
        view_popover_box.set_margin_bottom(12);

        let view_heading = Label::new(Some("表示オプション"));
        view_heading.add_css_class("section-heading");
        view_popover_box.append(&view_heading);

        let ruler_menu_toggle = ToggleButton::with_label("ルーラー");
        ruler_menu_toggle.add_css_class("flat");
        ruler_menu_toggle.set_active(true);
        view_popover_box.append(&ruler_menu_toggle);

        let guides_menu_toggle = ToggleButton::with_label("ガイド");
        guides_menu_toggle.add_css_class("flat");
        guides_menu_toggle.set_active(true);
        view_popover_box.append(&guides_menu_toggle);

        let snap_to_guides_toggle = ToggleButton::with_label("スナップ");
        snap_to_guides_toggle.add_css_class("flat");
        snap_to_guides_toggle.set_active(true);
        view_popover_box.append(&snap_to_guides_toggle);

        let guide_list_popover = Popover::new();
        let guide_list_box = GtkBox::new(Orientation::Vertical, 8);
        let guide_list_scroller = ScrolledWindow::new();
        guide_list_scroller.set_min_content_width(240);
        guide_list_scroller.set_min_content_height(260);
        guide_list_scroller.set_child(Some(&guide_list_box));
        guide_list_popover.set_child(Some(&guide_list_scroller));

        let guide_list_menu_btn = Button::with_label("ガイドを管理…");
        guide_list_menu_btn.add_css_class("flat");
        guide_list_menu_btn.set_halign(gtk4::Align::Start);
        guide_list_popover.set_parent(&guide_list_menu_btn);
        view_popover_box.append(&guide_list_menu_btn);

        view_popover.set_child(Some(&view_popover_box));

        let view_menu_btn = MenuButton::builder().label("表示").build();
        view_menu_btn.add_css_class("flat");
        view_menu_btn.set_popover(Some(&view_popover));
        view_menu_btn.set_tooltip_text(Some("ルーラーやガイドの表示設定"));
        primary_toolbar.append(&view_menu_btn);

        let view_separator = Separator::new(Orientation::Vertical);
        view_separator.set_margin_start(8);
        view_separator.set_margin_end(8);
        primary_toolbar.append(&view_separator);

        // Spacer before zoom controls
        let toolbar_spacer = GtkBox::new(Orientation::Horizontal, 0);
        toolbar_spacer.set_hexpand(true);
        primary_toolbar.append(&toolbar_spacer);

        // Zoom controls
        let zoom_box = GtkBox::new(Orientation::Horizontal, 2);
        zoom_box.add_css_class("linked");

        let zoom_out_btn = Button::with_label("−");
        zoom_out_btn.add_css_class("flat");
        zoom_out_btn.set_tooltip_text(Some("ズームアウト (Ctrl+-)"));
        zoom_box.append(&zoom_out_btn);

        let zoom_100_btn = Button::with_label("100%");
        zoom_100_btn.add_css_class("flat");
        zoom_100_btn.set_tooltip_text(Some("ズームをリセット (Ctrl+0)"));
        zoom_box.append(&zoom_100_btn);

        let zoom_in_btn = Button::with_label("+");
        zoom_in_btn.add_css_class("flat");
        zoom_in_btn.set_tooltip_text(Some("ズームイン (Ctrl+=)"));
        zoom_box.append(&zoom_in_btn);

        primary_toolbar.append(&zoom_box);

        let page_info_label = Label::new(Some("1 / 1"));
        page_info_label.add_css_class("dim-label");
        page_info_label.set_margin_start(12);
        primary_toolbar.append(&page_info_label);

        (
            primary_toolbar,
            PrimaryToolbarButtons {
                new_btn,
                open_btn,
                save_btn,
                export_btn,
                image_export_btn,
                undo_btn,
                redo_btn,
                template_btn,
                json_editor_btn,
                settings_btn,
                item_library_btn,
                block_tools_btn,
                view_menu_btn,
                zoom_out_btn,
                zoom_100_btn,
                zoom_in_btn,
                page_info_label,
                ruler_menu_toggle,
                guides_menu_toggle,
                snap_to_guides_toggle,
                guide_list_menu_btn,
                view_popover,
                guide_list_popover,
            },
            guide_list_box,
        )
    }

    /// Build the secondary toolbar with object operations and view toggles
    fn build_secondary_toolbar() -> (GtkBox, SecondaryToolbarButtons) {
        let secondary_toolbar = GtkBox::new(Orientation::Horizontal, 10);
        secondary_toolbar.add_css_class("toolbar");
        secondary_toolbar.set_margin_start(12);
        secondary_toolbar.set_margin_end(12);
        secondary_toolbar.set_margin_top(2);
        secondary_toolbar.set_margin_bottom(2);

        let ops_label = Label::new(Some("オブジェクト操作"));
        ops_label.add_css_class("section-heading");
        secondary_toolbar.append(&ops_label);

        let ops_box = GtkBox::new(Orientation::Horizontal, 2);
        ops_box.add_css_class("linked");

        let group_btn = Button::with_label("グループ");
        group_btn.add_css_class("flat");
        group_btn.set_tooltip_text(Some("グループ化 (Alt+G)"));
        ops_box.append(&group_btn);

        let ungroup_btn = Button::with_label("グループ解除");
        ungroup_btn.add_css_class("flat");
        ungroup_btn.set_tooltip_text(Some("グループ解除"));
        ops_box.append(&ungroup_btn);

        let lock_btn = Button::with_label("ロック");
        lock_btn.add_css_class("flat");
        lock_btn.set_tooltip_text(Some("選択オブジェクトをロック"));
        ops_box.append(&lock_btn);

        let unlock_btn = Button::with_label("ロック解除");
        unlock_btn.add_css_class("flat");
        unlock_btn.set_tooltip_text(Some("ロック解除"));
        ops_box.append(&unlock_btn);

        secondary_toolbar.append(&ops_box);

        let secondary_spacer = GtkBox::new(Orientation::Horizontal, 0);
        secondary_spacer.set_hexpand(true);
        secondary_toolbar.append(&secondary_spacer);

        let view_toggle_label = Label::new(Some("表示"));
        view_toggle_label.add_css_class("section-heading");
        secondary_toolbar.append(&view_toggle_label);

        let view_toggles_box = GtkBox::new(Orientation::Horizontal, 2);
        view_toggles_box.add_css_class("linked");

        let grid_toggle_btn = ToggleButton::with_label("グリッド: OFF");
        grid_toggle_btn.add_css_class("flat");
        grid_toggle_btn.set_tooltip_text(Some("グリッド表示の ON/OFF"));
        grid_toggle_btn.set_active(false);
        view_toggles_box.append(&grid_toggle_btn);

        let guides_visible_btn = ToggleButton::with_label("ガイド: ON");
        guides_visible_btn.add_css_class("flat");
        guides_visible_btn.set_tooltip_text(Some("ガイド表示の ON/OFF"));
        guides_visible_btn.set_active(true);
        guides_visible_btn.add_css_class("suggested-action");
        view_toggles_box.append(&guides_visible_btn);

        let rulers_visible_btn = ToggleButton::with_label("ルーラー: ON");
        rulers_visible_btn.add_css_class("flat");
        rulers_visible_btn.set_tooltip_text(Some("ルーラー表示の ON/OFF"));
        rulers_visible_btn.set_active(true);
        rulers_visible_btn.add_css_class("suggested-action");
        view_toggles_box.append(&rulers_visible_btn);

        let snap_to_guides_btn = ToggleButton::with_label("スナップ: ON");
        snap_to_guides_btn.add_css_class("flat");
        snap_to_guides_btn.set_tooltip_text(Some("ガイドにスナップさせる"));
        snap_to_guides_btn.set_active(true);
        snap_to_guides_btn.add_css_class("suggested-action");
        view_toggles_box.append(&snap_to_guides_btn);

        secondary_toolbar.append(&view_toggles_box);

        (
            secondary_toolbar,
            SecondaryToolbarButtons {
                group_btn,
                ungroup_btn,
                lock_btn,
                unlock_btn,
                grid_toggle_btn,
                guides_visible_btn,
                rulers_visible_btn,
                snap_to_guides_btn,
            },
        )
    }
}

/// Primary toolbar buttons (temporary structure for internal use)
struct PrimaryToolbarButtons {
    new_btn: Button,
    open_btn: Button,
    save_btn: Button,
    export_btn: Button,
    image_export_btn: Button,
    undo_btn: Button,
    redo_btn: Button,
    template_btn: Button,
    json_editor_btn: Button,
    settings_btn: Button,
    item_library_btn: ToggleButton,
    block_tools_btn: ToggleButton,
    view_menu_btn: MenuButton,
    zoom_out_btn: Button,
    zoom_100_btn: Button,
    zoom_in_btn: Button,
    page_info_label: Label,
    ruler_menu_toggle: ToggleButton,
    guides_menu_toggle: ToggleButton,
    snap_to_guides_toggle: ToggleButton,
    guide_list_menu_btn: Button,
    view_popover: Popover,
    guide_list_popover: Popover,
}

/// Secondary toolbar buttons (temporary structure for internal use)
struct SecondaryToolbarButtons {
    group_btn: Button,
    ungroup_btn: Button,
    lock_btn: Button,
    unlock_btn: Button,
    grid_toggle_btn: ToggleButton,
    guides_visible_btn: ToggleButton,
    rulers_visible_btn: ToggleButton,
    snap_to_guides_btn: ToggleButton,
}
