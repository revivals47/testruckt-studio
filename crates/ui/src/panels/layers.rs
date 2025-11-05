use gtk4::{prelude::*, Box as GtkBox, Label, Orientation, ScrolledWindow, CheckButton};

/// Represents a single layer item in the layers panel
#[derive(Clone)]
pub struct LayerItem {
    pub id: uuid::Uuid,
    pub name: String,
    pub visible: bool,
    pub layer_type: String, // "Text", "Image", "Shape", "Frame"
}

/// Build the layer panel with scrolled content
pub fn build_layer_panel() -> ScrolledWindow {
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(false);
    scrolled.set_width_request(240);

    // Create a placeholder label
    let placeholder = Label::new(Some("No layers"));
    placeholder.set_margin_top(20);
    placeholder.set_margin_bottom(20);
    placeholder.add_css_class("dim-label");

    scrolled.set_child(Some(&placeholder));
    scrolled
}

/// Build a layers list from document elements
pub fn build_layers_list(elements: &[testruct_core::document::DocumentElement]) -> GtkBox {
    let container = GtkBox::new(Orientation::Vertical, 1);
    container.set_vexpand(true);
    container.set_hexpand(true);

    // Iterate through elements in reverse order (bottom to top visually)
    for (index, element) in elements.iter().enumerate().rev() {
        let layer_item = create_layer_item(element, index);
        container.append(&layer_item);
    }

    container
}

/// Create a single layer item widget
fn create_layer_item(element: &testruct_core::document::DocumentElement, index: usize) -> GtkBox {
    let item_box = GtkBox::new(Orientation::Horizontal, 8);
    item_box.add_css_class("layer-item");
    item_box.set_margin_start(8);
    item_box.set_margin_end(8);
    item_box.set_margin_top(4);
    item_box.set_margin_bottom(4);
    item_box.set_halign(gtk4::Align::Fill);
    item_box.set_hexpand(true);

    // Create visibility toggle
    let visibility_btn = CheckButton::new();
    visibility_btn.set_active(true);
    visibility_btn.set_width_request(24);
    visibility_btn.set_tooltip_text(Some("Toggle visibility"));
    item_box.append(&visibility_btn);

    // Get element type and name
    let (_type_name, type_label) = match element {
        testruct_core::document::DocumentElement::Text(text) => (
            "Text",
            format!("Text: {}", if text.content.len() > 15 {
                format!("{}...", &text.content[..15])
            } else {
                text.content.clone()
            }),
        ),
        testruct_core::document::DocumentElement::Image(_) => (
            "Image",
            "Image".to_string(),
        ),
        testruct_core::document::DocumentElement::Shape(shape) => {
            let shape_type = match shape.kind {
                testruct_core::document::ShapeKind::Rectangle => "Rectangle",
                testruct_core::document::ShapeKind::Ellipse => "Ellipse",
                testruct_core::document::ShapeKind::Line => "Line",
                testruct_core::document::ShapeKind::Arrow => "Arrow",
                testruct_core::document::ShapeKind::Polygon => "Polygon",
            };
            ("Shape", shape_type.to_string())
        },
        testruct_core::document::DocumentElement::Frame(_) => (
            "Frame",
            "Frame".to_string(),
        ),
    };

    // Create element label
    let label_box = GtkBox::new(Orientation::Vertical, 2);
    label_box.set_hexpand(true);

    let type_label_widget = Label::new(Some(&type_label));
    type_label_widget.set_halign(gtk4::Align::Start);
    type_label_widget.add_css_class("monospace");
    label_box.append(&type_label_widget);

    let index_label = Label::new(Some(&format!("Layer {}", index + 1)));
    index_label.set_halign(gtk4::Align::Start);
    index_label.add_css_class("dim-label");
    index_label.add_css_class("small-text");
    label_box.append(&index_label);

    item_box.append(&label_box);

    // Add a spacer to push the visibility button to the right
    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    item_box.append(&spacer);

    item_box
}
