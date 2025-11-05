//! About dialog for the application

use gtk4::prelude::*;
use gtk4::{AboutDialog, Window};

/// Show the About dialog
pub fn show_about_dialog(parent: &Window) {
    let dialog = AboutDialog::new();

    dialog.set_transient_for(Some(parent));
    dialog.set_modal(true);

    // Application info
    dialog.set_program_name(Some("Testruct Studio"));
    dialog.set_version(Some("0.1.0"));
    dialog.set_copyright(Some("© 2025 Testruct Team"));
    dialog.set_license_type(gtk4::License::MitX11);

    // Description
    dialog.set_comments(Some(
        "A powerful design and test creation tool for creating professional documents and test items."
    ));

    // Website
    dialog.set_website(Some("https://testruct.example.com"));
    dialog.set_website_label("Visit Website");

    // Authors
    let authors = ["Testruct Team"];
    dialog.set_authors(&authors);

    // Show the dialog
    dialog.show();
}
