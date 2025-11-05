use gtk4::{prelude::*, Box as GtkBox, Button, Label, Orientation, Window};

pub fn show_template_browser(parent: &Window) {
    // Create a simple dialog window for template browser
    let dialog = gtk4::ApplicationWindow::builder()
        .transient_for(parent)
        .modal(true)
        .title("Templates")
        .default_width(400)
        .default_height(300)
        .build();

    let main_box = GtkBox::new(Orientation::Vertical, 12);
    main_box.set_margin_start(12);
    main_box.set_margin_end(12);
    main_box.set_margin_top(12);
    main_box.set_margin_bottom(12);

    let title = Label::new(Some("Templates"));
    title.add_css_class("title-2");
    main_box.append(&title);

    let content = Label::new(Some("Template browser.\n\nUse the template manager from the Tools menu to browse available templates."));
    content.set_wrap(true);
    main_box.append(&content);

    let button_box = GtkBox::new(Orientation::Horizontal, 6);
    button_box.set_halign(gtk4::Align::End);
    button_box.set_homogeneous(true);

    let close_btn = Button::with_label("Close");
    let dialog_ref = dialog.clone();
    close_btn.connect_clicked(move |_| {
        dialog_ref.close();
    });
    button_box.append(&close_btn);
    main_box.append(&button_box);

    dialog.set_child(Some(&main_box));
    dialog.present();
}
