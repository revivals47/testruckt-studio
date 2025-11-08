//! Pages Panel for multi-page document management
//!
//! Provides:
//! - Page list display
//! - Page switching/navigation
//! - Add/delete pages
//! - Page properties editing

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation, ScrolledWindow, Spinner};
use testruct_core::document::{Document, Page};

use crate::app::AppState;
use crate::canvas::CanvasView;

/// Pages panel for document page management
pub struct PagesPanel {
    pub container: ScrolledWindow,
}

impl PagesPanel {
    /// Create a new pages panel
    pub fn new(document: &Document, app_state: &AppState, canvas_view: &CanvasView) -> Self {
        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_hexpand(false);
        scrolled.set_width_request(200);

        // Build pages list
        let pages_box = Self::build_pages_list(document, app_state, canvas_view);
        scrolled.set_child(Some(&pages_box));

        PagesPanel { container: scrolled }
    }

    /// Build pages list from document
    fn build_pages_list(
        document: &Document,
        app_state: &AppState,
        canvas_view: &CanvasView,
    ) -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 8);
        container.set_margin_start(8);
        container.set_margin_end(8);
        container.set_margin_top(8);
        container.set_margin_bottom(8);
        container.set_vexpand(true);
        container.set_hexpand(true);

        // Title
        let title = Label::new(Some("Pages"));
        title.set_markup("<b>Pages</b>");
        title.set_halign(gtk4::Align::Start);
        container.append(&title);

        // Pages list
        if document.pages.is_empty() {
            let placeholder = Label::new(Some("No pages"));
            placeholder.add_css_class("dim-label");
            container.append(&placeholder);
        } else {
            for (index, page) in document.pages.iter().enumerate() {
                let page_item = Self::create_page_item(page, index, app_state, canvas_view);
                container.append(&page_item);
            }
        }

        // Add page button
        let add_btn = Button::with_label("+ Add Page");
        add_btn.add_css_class("suggested-action");
        add_btn.set_halign(gtk4::Align::Fill);

        let state_c = app_state.clone();
        add_btn.connect_clicked(move |_| {
            // TODO: Implement add page functionality
            tracing::info!("✅ Add page button clicked");
            state_c.with_mutable_active_document(|doc| {
                let new_page = Page::empty();
                doc.pages.push(new_page);
                tracing::info!("📄 New page added. Total pages: {}", doc.pages.len());
            });
        });

        container.append(&add_btn);

        container
    }

    /// Create a single page item widget
    fn create_page_item(
        page: &Page,
        index: usize,
        app_state: &AppState,
        canvas_view: &CanvasView,
    ) -> GtkBox {
        let item_box = GtkBox::new(Orientation::Vertical, 4);
        item_box.add_css_class("page-item");
        item_box.set_margin_start(4);
        item_box.set_margin_end(4);
        item_box.set_margin_top(4);
        item_box.set_margin_bottom(4);
        item_box.set_halign(gtk4::Align::Fill);
        item_box.set_hexpand(true);

        // Page header (name and element count)
        let header_box = GtkBox::new(Orientation::Horizontal, 8);
        header_box.set_halign(gtk4::Align::Fill);

        let page_name = Label::new(Some(&format!("Page {}", index + 1)));
        page_name.set_halign(gtk4::Align::Start);
        page_name.add_css_class("monospace");
        header_box.append(&page_name);

        let element_count = Label::new(Some(&format!("({} items)", page.elements.len())));
        element_count.set_halign(gtk4::Align::Start);
        element_count.add_css_class("dim-label");
        element_count.add_css_class("small-text");
        header_box.append(&element_count);

        item_box.append(&header_box);

        // Page preview area (placeholder)
        let preview_box = GtkBox::new(Orientation::Vertical, 0);
        preview_box.set_height_request(80);
        preview_box.add_css_class("page-preview");
        preview_box.set_halign(gtk4::Align::Fill);

        // Placeholder for thumbnail
        let spinner = Spinner::new();
        spinner.start();
        preview_box.append(&spinner);

        item_box.append(&preview_box);

        // Page controls (select, delete)
        let controls_box = GtkBox::new(Orientation::Horizontal, 4);
        controls_box.set_halign(gtk4::Align::Fill);

        // Select button
        let select_btn = Button::with_label("Select");
        select_btn.set_hexpand(true);
        let state_c = app_state.clone();
        let page_id = page.id;

        select_btn.connect_clicked(move |_| {
            tracing::info!("✅ Page selected");
            // TODO: Implement page switching logic
        });

        controls_box.append(&select_btn);

        // Delete button
        let delete_btn = Button::with_label("Delete");
        delete_btn.add_css_class("destructive-action");

        let state_c2 = app_state.clone();

        delete_btn.connect_clicked(move |_| {
            tracing::info!("✅ Delete page clicked");
            // TODO: Implement delete page with confirmation
        });

        controls_box.append(&delete_btn);

        item_box.append(&controls_box);

        item_box
    }
}

/// Helper function to update pages panel
pub fn update_pages_panel(
    panel_container: &gtk4::ScrolledWindow,
    document: &Document,
    app_state: &AppState,
    canvas_view: &CanvasView,
) {
    let pages_box = PagesPanel::build_pages_list(document, app_state, canvas_view);
    panel_container.set_child(Some(&pages_box));
}

/// Get page count from document
pub fn get_page_count(document: &Document) -> usize {
    document.pages.len()
}

/// Get current page index (placeholder - would be stored in app state)
pub fn get_current_page_index(document: &Document) -> usize {
    // TODO: Track active page in AppState
    0
}
