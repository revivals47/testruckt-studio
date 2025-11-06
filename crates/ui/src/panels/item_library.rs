//! Item library panel for viewing and managing items from the database

use gtk4::{
    prelude::*, Box as GtkBox, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow,
    SearchEntry,
};
use std::sync::{Arc, Mutex};
use testruct_db::ItemBank;

/// Components for the item library panel
pub struct ItemLibraryComponents {
    pub container: GtkBox,
    pub search_entry: SearchEntry,
    pub item_list: ListBox,
    pub add_button: Button,
}

/// Build the item library panel
pub fn build_item_library_panel(item_bank: Arc<Mutex<ItemBank>>) -> ItemLibraryComponents {
    // Main container
    let container = GtkBox::new(gtk4::Orientation::Vertical, 6);
    container.set_margin_top(12);
    container.set_margin_bottom(12);
    container.set_margin_start(12);
    container.set_margin_end(12);

    // Title
    let title = Label::new(Some("Item Library"));
    title.add_css_class("title-3");
    container.append(&title);

    // Search and action box
    let search_action_box = GtkBox::new(Orientation::Horizontal, 6);
    search_action_box.set_halign(gtk4::Align::Fill);

    let search_entry = SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search items..."));
    search_entry.set_halign(gtk4::Align::Fill);
    search_entry.set_hexpand(true);
    search_action_box.append(&search_entry);

    let add_button = Button::with_label("+ 追加");
    add_button.add_css_class("flat");
    add_button.set_tooltip_text(Some("新しいアイテムを追加"));
    search_action_box.append(&add_button);

    container.append(&search_action_box);

    // Item list
    let item_list = ListBox::new();
    item_list.set_selection_mode(gtk4::SelectionMode::Single);
    item_list.set_hexpand(true);
    item_list.set_vexpand(true);
    // Prevent ListBox from stealing focus from canvas
    item_list.set_can_focus(true);

    // Scrolled window for item list
    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&item_list));
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);
    container.append(&scrolled);

    // Load initial items
    refresh_item_list(&item_list, &item_bank);

    // Set up row selection handler - prevent event propagation
    let item_list_clone = item_list.clone();
    item_list.connect_row_selected(move |_listbox, row| {
        if let Some(row) = row {
            eprintln!("📦 Item selected: {:?}", row);
            // Item selection is handled here - prevent further propagation
        }
    });

    // Search functionality
    let item_list_clone = item_list.clone();
    let item_bank_clone = item_bank.clone();
    search_entry.connect_search_changed(move |entry| {
        let query = entry.text().to_string();
        if query.is_empty() {
            refresh_item_list(&item_list_clone, &item_bank_clone);
        } else {
            search_items(&item_list_clone, &item_bank_clone, &query);
        }
    });

    // Add button handler
    let item_list_clone = item_list.clone();
    let item_bank_clone = item_bank.clone();
    add_button.connect_clicked(move |_| {
        tracing::info!("✅ Add button clicked - opening item creation dialog");
        eprintln!("📝 Opening item creation dialog...");

        // TODO: Open item creation dialog with:
        // - Title input field
        // - Difficulty selector
        // - Category selector
        // - Save/Cancel buttons
        // For now, just log the action

        // This feature requires:
        // 1. A new dialog widget
        // 2. Database write functionality
        // 3. UI state management for the dialog
    });

    ItemLibraryComponents {
        container,
        search_entry,
        item_list,
        add_button,
    }
}

/// Refresh the item list with all items from the database
fn refresh_item_list(list: &ListBox, item_bank: &Arc<Mutex<ItemBank>>) {
    // Clear existing items
    while let Some(row) = list.first_child() {
        list.remove(&row);
    }

    // Load items from database
    match item_bank.lock() {
        Ok(bank) => match bank.get_all_items(Some(100)) {
            Ok(items) => {
                for item in items {
                    let row_label = format!("{}\n{}", item.title, item.difficulty.as_str());
                    let row = ListBoxRow::new();
                    let label = Label::new(Some(&row_label));
                    label.set_wrap(true);
                    label.set_margin_top(6);
                    label.set_margin_bottom(6);
                    label.set_margin_start(6);
                    label.set_margin_end(6);
                    label.set_halign(gtk4::Align::Start);
                    row.set_child(Some(&label));
                    list.insert(&row, -1);
                }
            }
            Err(e) => {
                tracing::warn!("Failed to load items: {}", e);
            }
        },
        Err(e) => {
            tracing::warn!("Failed to lock item bank: {}", e);
        }
    }
}

/// Search items in the database
fn search_items(list: &ListBox, item_bank: &Arc<Mutex<ItemBank>>, query: &str) {
    // Clear existing items
    while let Some(row) = list.first_child() {
        list.remove(&row);
    }

    // Search items
    match item_bank.lock() {
        Ok(bank) => match bank.search_items(query) {
            Ok(items) => {
                for item in items {
                    let row_label = format!("{}\n{}", item.title, item.difficulty.as_str());
                    let row = ListBoxRow::new();
                    let label = Label::new(Some(&row_label));
                    label.set_wrap(true);
                    label.set_margin_top(6);
                    label.set_margin_bottom(6);
                    label.set_margin_start(6);
                    label.set_margin_end(6);
                    label.set_halign(gtk4::Align::Start);
                    row.set_child(Some(&label));
                    list.insert(&row, -1);
                }
            }
            Err(e) => {
                tracing::warn!("Search failed: {}", e);
            }
        },
        Err(e) => {
            tracing::warn!("Failed to lock item bank: {}", e);
        }
    }
}
