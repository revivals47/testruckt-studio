//! Template manager UI integration
//!
//! Provides UI components for managing templates, including browsing,
//! selecting, and creating documents from templates.

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, ListBox, ListBoxRow, Orientation, ScrolledWindow, SearchEntry};
use std::cell::RefCell;
use std::rc::Rc;
use testruct_core::template::Template;

/// Template browser UI components
pub struct TemplateBrowser {
    pub window: gtk4::ApplicationWindow,
    pub list_box: ListBox,
    pub search_entry: SearchEntry,
    pub templates: Rc<RefCell<Vec<Template>>>,
    pub selected_template: Rc<RefCell<Option<Template>>>,
}

impl TemplateBrowser {
    /// Create a new template browser
    pub fn new(parent: &gtk4::ApplicationWindow) -> Self {
        let window = gtk4::ApplicationWindow::builder()
            .transient_for(parent)
            .modal(true)
            .title("テンプレートから作成")
            .default_width(600)
            .default_height(400)
            .build();

        let main_box = GtkBox::new(Orientation::Vertical, 12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);

        // Search bar
        let search_box = GtkBox::new(Orientation::Horizontal, 6);
        let search_label = gtk4::Label::new(Some("検索:"));
        let search_entry = SearchEntry::new();
        search_entry.set_hexpand(true);
        search_entry.set_placeholder_text(Some("テンプレート名で検索..."));
        search_box.append(&search_label);
        search_box.append(&search_entry);
        main_box.append(&search_box);

        // Template list
        let scroller = ScrolledWindow::new();
        scroller.set_vexpand(true);
        scroller.set_hexpand(true);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::Single);
        scroller.set_child(Some(&list_box));
        main_box.append(&scroller);

        // Button bar
        let button_box = GtkBox::new(Orientation::Horizontal, 6);
        button_box.set_homogeneous(true);

        let create_btn = Button::with_label("このテンプレートで作成");
        create_btn.add_css_class("suggested-action");

        let cancel_btn = Button::with_label("キャンセル");

        button_box.append(&create_btn);
        button_box.append(&cancel_btn);
        main_box.append(&button_box);

        window.set_child(Some(&main_box));

        let templates = Rc::new(RefCell::new(Vec::new()));
        let selected_template = Rc::new(RefCell::new(None));

        Self {
            window,
            list_box,
            search_entry,
            templates,
            selected_template,
        }
    }

    /// Load templates into the browser
    pub fn load_templates(&self, templates: Vec<Template>) {
        *self.templates.borrow_mut() = templates.clone();
        self.populate_list(&templates);
    }

    /// Populate the list box with templates
    fn populate_list(&self, templates: &[Template]) {
        // Clear existing rows
        while let Some(row) = self.list_box.first_child() {
            row.unparent();
        }

        // Add template rows
        for template in templates {
            let row = self.create_template_row(template);
            self.list_box.append(&row);
        }
    }

    /// Create a list row for a template
    fn create_template_row(&self, template: &Template) -> ListBoxRow {
        let row = ListBoxRow::new();

        let box_content = GtkBox::new(Orientation::Vertical, 4);
        box_content.set_margin_start(12);
        box_content.set_margin_end(12);
        box_content.set_margin_top(8);
        box_content.set_margin_bottom(8);

        // Template name
        let name_label = gtk4::Label::new(Some(&template.name));
        name_label.add_css_class("heading");
        name_label.set_halign(gtk4::Align::Start);
        box_content.append(&name_label);

        // Template description
        if let Some(description) = &template.description {
            let desc_label = gtk4::Label::new(Some(description));
            desc_label.add_css_class("dim-label");
            desc_label.set_halign(gtk4::Align::Start);
            desc_label.set_wrap(true);
            box_content.append(&desc_label);
        }

        // Template ID
        let id_label = gtk4::Label::new(Some(&format!("ページ数: {}", template.pages.len())));
        id_label.add_css_class("monospace");
        id_label.add_css_class("dim-label");
        box_content.append(&id_label);

        row.set_child(Some(&box_content));
        row
    }

    /// Filter templates by search text
    pub fn filter_templates(&self, search_text: &str) {
        let search_lower = search_text.to_lowercase();
        let templates = self.templates.borrow();

        let filtered: Vec<_> = templates
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&search_lower)
                    || t.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&search_lower))
                        .unwrap_or(false)
            })
            .cloned()
            .collect();

        self.populate_list(&filtered);
    }

    /// Get the selected template
    pub fn selected_template(&self) -> Option<Template> {
        self.selected_template.borrow().clone()
    }

    /// Set the selected template
    pub fn set_selected_template(&self, template: Template) {
        *self.selected_template.borrow_mut() = Some(template);
    }

    /// Show the template browser window
    pub fn show(&self) {
        self.window.present();
    }

    /// Close the template browser window
    pub fn close(&self) {
        self.window.close();
    }
}

/// Template builder for creating documents from templates
pub struct TemplateBuilder;

impl TemplateBuilder {
    /// Create a document from a template
    pub fn create_document_from_template(template: &Template) -> testruct_core::document::Document {
        // Create a new document from template
        // The document gets initialized with a blank page and the template's name
        testruct_core::document::Document::empty(&template.name)
    }
}

/// Template selector dialog for creating new documents
pub fn create_template_selector_dialog(parent: &gtk4::ApplicationWindow) -> TemplateBrowser {
    TemplateBrowser::new(parent)
}
