//! Menu model construction for the application window
//!
//! This module provides a builder for constructing the application menu
//! structure with all file, edit, view, tools, and help menus.

use glib::Cast;
use gtk4::{gio, glib, PopoverMenuBar};

/// Builder for constructing the application menu model
pub struct MenuBuilder;

impl MenuBuilder {
    /// Build the complete menu model with all submenus
    pub fn build_menu_model() -> gio::MenuModel {
        let menu = gio::Menu::new();

        menu.append_submenu(Some("_File"), &Self::build_file_menu());
        menu.append_submenu(Some("_Edit"), &Self::build_edit_menu());
        menu.append_submenu(Some("_View"), &Self::build_view_menu());
        menu.append_submenu(Some("_Tools"), &Self::build_tools_menu());
        menu.append_submenu(Some("_Help"), &Self::build_help_menu());

        menu.upcast()
    }

    /// Build the File menu with document operations
    fn build_file_menu() -> gio::Menu {
        let file_menu = gio::Menu::new();
        file_menu.append(Some("_New"), Some("win.new"));
        file_menu.append(Some("_Open..."), Some("win.open"));
        file_menu.append(Some("_Recent Files..."), Some("win.recent-files"));
        file_menu.append(Some("_Save"), Some("win.save"));
        file_menu.append(Some("Save _As..."), Some("win.save-as"));

        let export_section = gio::Menu::new();
        export_section.append(Some("Export as PDF"), Some("win.export-pdf"));
        export_section.append(Some("Export as PNG"), Some("win.export-png"));
        export_section.append(Some("Export as JPEG"), Some("win.export-jpeg"));
        export_section.append(Some("Export as SVG"), Some("win.export-svg"));
        file_menu.append_section(None, &export_section);

        file_menu
    }

    /// Build the Edit menu with undo/redo and selection
    fn build_edit_menu() -> gio::Menu {
        let edit_menu = gio::Menu::new();
        edit_menu.append(Some("_Undo"), Some("win.undo"));
        edit_menu.append(Some("_Redo"), Some("win.redo"));

        let clipboard_section = gio::Menu::new();
        clipboard_section.append(Some("Cu_t"), Some("win.cut"));
        clipboard_section.append(Some("_Copy"), Some("win.copy"));
        clipboard_section.append(Some("_Paste"), Some("win.paste"));
        edit_menu.append_section(None, &clipboard_section);

        let edit_section = gio::Menu::new();
        edit_section.append(Some("Select _All"), Some("win.select-all"));
        edit_section.append(Some("_Duplicate"), Some("win.duplicate"));
        edit_section.append(Some("_Delete"), Some("win.delete"));
        edit_menu.append_section(None, &edit_section);

        edit_menu
    }

    /// Build the View menu with grid, guides, and panel toggles
    fn build_view_menu() -> gio::Menu {
        let view_menu = gio::Menu::new();
        view_menu.append(Some("Show _Grid"), Some("win.toggle-grid"));
        view_menu.append(Some("Show G_uides"), Some("win.toggle-guides"));
        view_menu.append(Some("Show _Rulers"), Some("win.toggle-rulers"));

        let panels_section = gio::Menu::new();
        panels_section.append(Some("_Layers Panel"), Some("win.toggle-layers"));
        panels_section.append(Some("_Properties Panel"), Some("win.toggle-properties"));
        panels_section.append(Some("_JSON Editor"), Some("win.open-json-editor"));
        view_menu.append_section(None, &panels_section);

        view_menu
    }

    /// Build the Tools menu with templates and utilities
    fn build_tools_menu() -> gio::Menu {
        let tools_menu = gio::Menu::new();
        tools_menu.append(Some("_Templates"), Some("win.templates"));
        tools_menu.append(Some("_Item Library"), Some("win.toggle-item-library"));
        tools_menu.append(Some("_Block Tools"), Some("win.toggle-block-tools"));

        let insert_section = gio::Menu::new();
        insert_section.append(Some("Insert _Image..."), Some("win.insert-image"));
        tools_menu.append_section(None, &insert_section);

        let tools_section = gio::Menu::new();
        tools_section.append(Some("_Settings"), Some("win.settings"));
        tools_menu.append_section(None, &tools_section);

        tools_menu
    }

    /// Build the Help menu with documentation and about
    fn build_help_menu() -> gio::Menu {
        let help_menu = gio::Menu::new();
        help_menu.append(Some("_User Manual"), Some("win.user-manual"));
        help_menu.append(Some("_About"), Some("win.about"));

        help_menu
    }
}

pub fn build_menu_bar() -> PopoverMenuBar {
    let model = MenuBuilder::build_menu_model();
    let menu_bar = PopoverMenuBar::from_model(Some(&model));
    glib::set_application_name("Testruct Studio");
    menu_bar
}
