use gtk4::{prelude::*, Window};

pub fn show_project_settings(parent: &Window) {
    // Note: Dialog is deprecated in GTK4 since 4.10, but kept for compatibility
    // TODO: Migrate to modern GTK4 dialogs using AlertDialog or custom windows
    let dialog = gtk4::Dialog::with_buttons(
        Some("Project Settings"),
        Some(parent),
        gtk4::DialogFlags::MODAL,
        &[(&"Close", gtk4::ResponseType::Close)],
    );
    dialog.connect_response(|dialog, _| dialog.close());
    dialog.show();
}
