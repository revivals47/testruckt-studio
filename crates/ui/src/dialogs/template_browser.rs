use gtk4::{prelude::*, ListView, SignalListItemFactory, Window};

pub fn show_template_browser(parent: &Window) {
    // Note: Dialog is deprecated in GTK4 since 4.10, but kept for compatibility
    // TODO: Migrate to modern GTK4 dialogs using AlertDialog or custom windows
    let dialog = gtk4::Dialog::with_buttons(
        Some("Templates"),
        Some(parent),
        gtk4::DialogFlags::MODAL,
        &[(&"Close", gtk4::ResponseType::Close)],
    );

    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_, item| {
        let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row.append(&gtk4::Label::new(Some("Template")));
        item.downcast_ref::<gtk4::ListItem>().unwrap().set_child(Some(&row));
    });

    let model = gtk4::gio::ListStore::new::<gtk4::glib::Object>();
    let selection = gtk4::SingleSelection::new(Some(model));
    // Note: SingleSelection doesn't have set_selection_mode - it's always single selection

    let list = ListView::new(Some(selection), Some(factory));
    list.set_vexpand(true);
    dialog.content_area().append(&list);

    dialog.connect_response(|dialog, _| dialog.close());
    dialog.show();
}
