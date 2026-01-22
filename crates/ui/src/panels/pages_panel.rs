//! Pages Panel for multi-page document management
//!
//! Provides:
//! - Page list display with thumbnails
//! - Page switching/navigation
//! - Add/delete pages with confirmation
//! - Active page tracking and highlighting

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Label, Orientation, ScrolledWindow};
use testruct_core::document::{Document, Page};

use crate::app::AppState;
use crate::canvas::{page_thumbnail::generate_page_thumbnail, CanvasView};

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

        PagesPanel {
            container: scrolled,
        }
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

        {
            let state_c = app_state.clone();
            let canvas_c = canvas_view.drawing_area();

            add_btn.connect_clicked(move |_| {
                let new_page_index = state_c.with_mutable_active_document(|doc| {
                    let new_page = Page::empty();
                    doc.pages.push(new_page);
                    let new_index = doc.pages.len() - 1;
                    tracing::info!("📄 New page added. Total pages: {}", doc.pages.len());
                    new_index
                });

                // Switch to the new page
                if let Some(idx) = new_page_index {
                    if let Err(e) = state_c.set_active_page_index(idx) {
                        tracing::warn!("Failed to switch to new page: {}", e);
                    }
                }

                canvas_c.queue_draw();
                tracing::info!("✅ Page added and selected");
            });
        }

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

        // Check if this is the active page
        let is_active = app_state.active_page_index() == index;
        if is_active {
            item_box.add_css_class("page-item-active");
        }

        // Page header (name and element count)
        let header_box = GtkBox::new(Orientation::Horizontal, 8);
        header_box.set_halign(gtk4::Align::Fill);

        let page_name = if is_active {
            Label::new(Some(&format!("▶ Page {}", index + 1)))
        } else {
            Label::new(Some(&format!("  Page {}", index + 1)))
        };
        page_name.set_halign(gtk4::Align::Start);
        page_name.add_css_class("monospace");
        if is_active {
            page_name.add_css_class("accent");
        }
        header_box.append(&page_name);

        let element_count = Label::new(Some(&format!("({} items)", page.elements.len())));
        element_count.set_halign(gtk4::Align::Start);
        element_count.add_css_class("dim-label");
        element_count.add_css_class("small-text");
        header_box.append(&element_count);

        item_box.append(&header_box);

        // Page preview area with thumbnail
        let preview_box = GtkBox::new(Orientation::Vertical, 0);
        preview_box.set_height_request(90);
        preview_box.add_css_class("page-preview");
        preview_box.set_halign(gtk4::Align::Fill);

        // Generate and display thumbnail
        match generate_page_thumbnail(page) {
            Ok(_png_data) => {
                // Successfully generated thumbnail - show visual indicator
                // Full thumbnail rendering requires complex GTK4 image loading
                // For now, display element summary as a preview indicator
                let summary_box = GtkBox::new(Orientation::Vertical, 4);
                summary_box.set_margin_top(8);
                summary_box.set_margin_bottom(8);

                // Show element type summary
                let mut shape_count = 0;
                let mut text_count = 0;
                let mut image_count = 0;
                let mut frame_count = 0;
                let mut group_count = 0;

                for element in &page.elements {
                    match element {
                        testruct_core::document::DocumentElement::Shape(_) => shape_count += 1,
                        testruct_core::document::DocumentElement::Text(_) => text_count += 1,
                        testruct_core::document::DocumentElement::Image(_) => image_count += 1,
                        testruct_core::document::DocumentElement::Frame(_) => frame_count += 1,
                        testruct_core::document::DocumentElement::Group(_) => group_count += 1,
                    }
                }

                let mut summary_str = String::new();
                if shape_count > 0 {
                    summary_str.push_str(&format!("🔷 {}", shape_count));
                }
                if text_count > 0 {
                    if !summary_str.is_empty() {
                        summary_str.push(' ');
                    }
                    summary_str.push_str(&format!("📝 {}", text_count));
                }
                if image_count > 0 {
                    if !summary_str.is_empty() {
                        summary_str.push(' ');
                    }
                    summary_str.push_str(&format!("🖼 {}", image_count));
                }
                if frame_count > 0 || group_count > 0 {
                    if !summary_str.is_empty() {
                        summary_str.push(' ');
                    }
                    if frame_count > 0 {
                        summary_str.push_str(&format!("📦 {}", frame_count));
                    }
                    if group_count > 0 {
                        summary_str.push_str(&format!(" 👥 {}", group_count));
                    }
                }

                if summary_str.is_empty() {
                    summary_str = "Empty page".to_string();
                }

                let summary_label = Label::new(Some(&summary_str));
                summary_label.set_halign(gtk4::Align::Center);
                summary_label.add_css_class("small-text");
                summary_box.append(&summary_label);

                preview_box.append(&summary_box);
            }
            Err(e) => {
                tracing::warn!("Failed to generate thumbnail for page {}: {}", index, e);
                let error_label = Label::new(Some(&format!("Thumbnail unavailable")));
                error_label.add_css_class("dim-label");
                error_label.set_wrap(true);
                error_label.set_margin_top(20);
                error_label.set_margin_bottom(20);
                preview_box.append(&error_label);
            }
        }

        item_box.append(&preview_box);

        // Page controls (select, delete)
        let controls_box = GtkBox::new(Orientation::Horizontal, 4);
        controls_box.set_halign(gtk4::Align::Fill);

        // Select button
        let select_btn = Button::with_label("Select");
        select_btn.set_hexpand(true);
        let state_c = app_state.clone();
        let canvas_c = canvas_view.drawing_area();
        let page_index = index;

        select_btn.connect_clicked(move |_| {
            match state_c.set_active_page_index(page_index) {
                Ok(()) => {
                    tracing::info!("✅ Page {} selected", page_index + 1);
                    canvas_c.queue_draw();
                }
                Err(e) => {
                    tracing::warn!("⚠️  Failed to select page: {}", e);
                }
            }
        });

        controls_box.append(&select_btn);

        // Delete button
        let delete_btn = Button::with_label("🗑");
        delete_btn.set_width_request(32);
        delete_btn.add_css_class("destructive-action");
        delete_btn.set_tooltip_text(Some("Delete page"));

        {
            let state_c = app_state.clone();
            let canvas_c = canvas_view.drawing_area();
            let page_index = index;

            delete_btn.connect_clicked(move |btn| {
                // Check if this is the last page
                let page_count = state_c
                    .with_active_document(|doc| doc.pages.len())
                    .unwrap_or(0);

                if page_count <= 1 {
                    tracing::warn!("⚠️ Cannot delete the last page");
                    btn.set_sensitive(false);
                    btn.set_tooltip_text(Some("Cannot delete the last page"));
                    return;
                }

                // Delete the page
                state_c.with_mutable_active_document(|doc| {
                    if page_index < doc.pages.len() {
                        doc.pages.remove(page_index);
                        tracing::info!("🗑 Page {} deleted. Remaining: {}", page_index + 1, doc.pages.len());
                    }
                });

                // Adjust active page index if needed
                let current_active = state_c.active_page_index();
                let new_page_count = state_c
                    .with_active_document(|doc| doc.pages.len())
                    .unwrap_or(0);

                if current_active >= new_page_count && new_page_count > 0 {
                    // Active page was deleted or is now out of bounds
                    let new_active = new_page_count - 1;
                    if let Err(e) = state_c.set_active_page_index(new_active) {
                        tracing::warn!("Failed to adjust active page: {}", e);
                    }
                } else if page_index < current_active {
                    // A page before the active page was deleted, adjust index
                    let new_active = current_active.saturating_sub(1);
                    if let Err(e) = state_c.set_active_page_index(new_active) {
                        tracing::warn!("Failed to adjust active page: {}", e);
                    }
                }

                canvas_c.queue_draw();
            });
        }

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
pub fn get_current_page_index(_document: &Document) -> usize {
    // TODO: Track active page in AppState
    0
}
