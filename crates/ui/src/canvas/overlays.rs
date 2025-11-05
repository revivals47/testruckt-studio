use gtk4::{prelude::*, Label, Overlay};

pub fn add_ruler_overlay(overlay: &Overlay) {
    let label = Label::builder().label("Rulers TBD").build();
    overlay.add_overlay(&label);
    label.set_margin_top(8);
    label.set_margin_start(8);
}
