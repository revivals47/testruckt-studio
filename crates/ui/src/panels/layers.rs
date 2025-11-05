use gtk4::{prelude::*, Box as GtkBox, Label, ListItem, ListView, Orientation, SignalListItemFactory, SingleSelection};

pub fn build_layer_panel() -> ListView {
    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_, item| {
        let item = item.downcast_ref::<ListItem>().unwrap();
        let row = GtkBox::new(Orientation::Horizontal, 8);
        let label = Label::new(Some("Layer"));
        row.append(&label);
        item.set_child(Some(&row));
    });

    let model = gtk4::gio::ListStore::new::<gtk4::glib::Object>();
    let selection = SingleSelection::new(Some(model));

    let view = ListView::new(Some(selection), Some(factory));
    view.set_vexpand(true);
    view.set_hexpand(false);
    view
}
