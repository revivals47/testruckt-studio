use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
    SearchEntry, Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use testruct_core::template::Template;

/// Show the Template Manager dialog with callback
pub fn show_template_browser_async(
    parent: &Window,
    templates: Vec<Template>,
    on_selected: Box<dyn Fn(Option<Template>)>,
) {
    let selected_template: Rc<RefCell<Option<Template>>> = Rc::new(RefCell::new(None));

    // Create dialog window for template browser
    let dialog = gtk4::ApplicationWindow::builder()
        .transient_for(parent)
        .modal(true)
        .title("テンプレートマネージャー")
        .default_width(600)
        .default_height(500)
        .build();

    let main_box = GtkBox::new(Orientation::Vertical, 12);
    main_box.set_margin_start(16);
    main_box.set_margin_end(16);
    main_box.set_margin_top(16);
    main_box.set_margin_bottom(16);

    // Title
    let title = Label::new(Some("テンプレートマネージャー"));
    title.add_css_class("title-2");
    main_box.append(&title);

    // Search box
    let search_entry = SearchEntry::new();
    search_entry.set_placeholder_text(Some("テンプレートを検索..."));
    main_box.append(&search_entry);

    // Template list
    let list_box = ListBox::new();
    list_box.set_vexpand(true);
    list_box.set_hexpand(true);
    list_box.add_css_class("boxed-list");

    // Store template data for selection
    let template_data: Rc<RefCell<Vec<Template>>> = Rc::new(RefCell::new(templates.clone()));

    // Populate list
    for template in templates.iter() {
        let row = ListBoxRow::new();
        let item_box = GtkBox::new(Orientation::Vertical, 6);
        item_box.set_margin_start(12);
        item_box.set_margin_end(12);
        item_box.set_margin_top(8);
        item_box.set_margin_bottom(8);

        // Template name
        let name_label = Label::new(Some(&template.name));
        name_label.add_css_class("title-3");
        name_label.set_halign(Align::Start);
        item_box.append(&name_label);

        // Template description (if available)
        if let Some(desc) = &template.description {
            let desc_label = Label::new(Some(desc));
            desc_label.add_css_class("dim-label");
            desc_label.set_halign(Align::Start);
            desc_label.set_wrap(true);
            item_box.append(&desc_label);
        }

        // Page count info
        let info_label = Label::new(Some(&format!("ページ数: {}", template.pages.len())));
        info_label.add_css_class("dim-label");
        info_label.set_halign(Align::Start);
        info_label.set_size_request(200, -1);
        item_box.append(&info_label);

        row.set_child(Some(&item_box));
        list_box.append(&row);
    }

    // Handle template selection
    let selected_ref = selected_template.clone();
    let template_data_clone = template_data.clone();
    list_box.connect_row_selected(move |_list_box, row| {
        if let Some(row) = row {
            let index = row.index() as usize;
            if let Some(template) = template_data_clone.borrow().get(index).cloned() {
                *selected_ref.borrow_mut() = Some(template);
                tracing::info!("✅ Template selected: {}", row.index());
            }
        }
    });

    // Scrolled window for list
    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&list_box));
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);
    main_box.append(&scrolled);

    // Template preview area (if selected)
    let preview_box = GtkBox::new(Orientation::Vertical, 6);
    preview_box.set_halign(Align::Start);
    let preview_label = Label::new(Some("選択したテンプレート:"));
    preview_label.add_css_class("dim-label");
    preview_box.append(&preview_label);

    let selected_name_label = Label::new(Some("なし"));
    selected_name_label.add_css_class("title-4");
    preview_box.append(&selected_name_label);

    // Update preview when selection changes
    let selected_ref_preview = selected_template.clone();
    let selected_name_clone = selected_name_label.clone();
    list_box.connect_row_selected(move |_, _| {
        if let Some(template) = selected_ref_preview.borrow().as_ref() {
            selected_name_clone.set_text(&template.name);
        }
    });

    main_box.append(&preview_box);

    // Button box
    let button_box = GtkBox::new(Orientation::Horizontal, 6);
    button_box.set_halign(Align::End);
    button_box.set_homogeneous(true);

    let create_btn = Button::with_label("このテンプレートから作成");
    let dialog_ref = dialog.clone();
    let selected_for_create = selected_template.clone();
    create_btn.connect_clicked(move |_| {
        if selected_for_create.borrow().is_some() {
            tracing::info!("✅ Template accepted, closing dialog");
            dialog_ref.close();
        } else {
            tracing::warn!("⚠️  No template selected");
        }
    });
    button_box.append(&create_btn);

    let close_btn = Button::with_label("キャンセル");
    let dialog_ref = dialog.clone();
    let selected_for_cancel = selected_template.clone();
    close_btn.connect_clicked(move |_| {
        *selected_for_cancel.borrow_mut() = None;
        dialog_ref.close();
    });
    button_box.append(&close_btn);

    main_box.append(&button_box);

    dialog.set_child(Some(&main_box));

    // Connect close event to call callback with selected template
    let selected_final = selected_template.clone();
    let on_selected_arc = std::sync::Arc::new(std::sync::Mutex::new(Some(on_selected)));
    let on_selected_clone = on_selected_arc.clone();

    dialog.connect_close_request(move |_| {
        let selected = selected_final.borrow().clone();
        if let Ok(mut callback) = on_selected_clone.try_lock() {
            if let Some(cb) = callback.take() {
                cb(selected);
            }
        }
        false.into()
    });

    dialog.present();
}
