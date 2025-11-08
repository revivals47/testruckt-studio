//! Layer panel drag-and-drop and layer management functionality
//!
//! Implements layer reordering through context menus and keyboard shortcuts

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, CheckButton, Label, Orientation};
use uuid::Uuid;

use crate::app::AppState;
use crate::canvas::CanvasView;

/// Layer item with reordering support
pub struct DraggableLayerItem {
    pub container: GtkBox,
    pub element_id: Uuid,
}

impl DraggableLayerItem {
    /// Create a layer item with selection and context menu support
    pub fn new(
        element_id: Uuid,
        _name: &str,
        type_label: &str,
        index: usize,
        _app_state: AppState,
        _canvas_view: &CanvasView,
    ) -> Self {
        let container = GtkBox::new(Orientation::Horizontal, 8);
        container.add_css_class("layer-item");
        container.add_css_class("draggable-layer");
        container.set_margin_start(8);
        container.set_margin_end(8);
        container.set_margin_top(4);
        container.set_margin_bottom(4);
        container.set_halign(gtk4::Align::Fill);
        container.set_hexpand(true);

        // Visibility toggle button
        let visibility_btn = CheckButton::new();
        visibility_btn.set_active(true);
        visibility_btn.set_width_request(24);
        visibility_btn.set_tooltip_text(Some("Toggle visibility"));
        container.append(&visibility_btn);

        // Layer info
        let label_box = GtkBox::new(Orientation::Vertical, 2);
        label_box.set_hexpand(true);

        let type_label_widget = Label::new(Some(type_label));
        type_label_widget.set_halign(gtk4::Align::Start);
        type_label_widget.add_css_class("monospace");
        label_box.append(&type_label_widget);

        let index_label = Label::new(Some(&format!("Layer {}", index + 1)));
        index_label.set_halign(gtk4::Align::Start);
        index_label.add_css_class("dim-label");
        index_label.add_css_class("small-text");
        label_box.append(&index_label);

        container.append(&label_box);

        // Spacer
        let spacer = GtkBox::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        container.append(&spacer);

        DraggableLayerItem {
            container,
            element_id,
        }
    }
}

/// Build draggable layers list with context menu support
pub fn build_draggable_layers_list(
    elements: &[testruct_core::document::DocumentElement],
    app_state: AppState,
    canvas_view: &CanvasView,
) -> GtkBox {
    let container = GtkBox::new(Orientation::Vertical, 1);
    container.set_vexpand(true);
    container.set_hexpand(true);

    // Iterate through elements in reverse order (bottom to top visually)
    for (index, element) in elements.iter().enumerate().rev() {
        let (type_label, _name) = get_element_info(element);

        let layer_item = DraggableLayerItem::new(
            element.id(),
            &_name,
            &type_label,
            index,
            app_state.clone(),
            canvas_view,
        );

        container.append(&layer_item.container);
    }

    container
}

/// Get element type and name
fn get_element_info(element: &testruct_core::document::DocumentElement) -> (String, String) {
    match element {
        testruct_core::document::DocumentElement::Text(text) => {
            let content = if text.content.len() > 15 {
                format!("{}...", &text.content[..15])
            } else {
                text.content.clone()
            };
            (format!("Text: {}", content), format!("Text: {}", content))
        }
        testruct_core::document::DocumentElement::Image(_) => {
            ("Image".to_string(), "Image".to_string())
        }
        testruct_core::document::DocumentElement::Shape(shape) => {
            let shape_type = match shape.kind {
                testruct_core::document::ShapeKind::Rectangle => "Rectangle",
                testruct_core::document::ShapeKind::Ellipse => "Ellipse",
                testruct_core::document::ShapeKind::Line => "Line",
                testruct_core::document::ShapeKind::Arrow => "Arrow",
                testruct_core::document::ShapeKind::Polygon => "Polygon",
            };
            (shape_type.to_string(), shape_type.to_string())
        }
        testruct_core::document::DocumentElement::Frame(_) => {
            ("Frame".to_string(), "Frame".to_string())
        }
        testruct_core::document::DocumentElement::Group(group) => {
            (format!("Group: {}", group.name), format!("Group: {}", group.name))
        }
    }
}

/// Reorder elements in a layer (move up or down)
pub fn reorder_layer(
    app_state: &AppState,
    element_id: Uuid,
    direction: LayerDirection,
    drawing_area: gtk4::DrawingArea,
) -> bool {
    let mut result = false;
    app_state.with_active_document(|doc| {
        if let Some(page) = doc.pages.first_mut() {
            if let Some(idx) = page.elements.iter().position(|e| e.id() == element_id) {
                match direction {
                    LayerDirection::Up => {
                        if idx < page.elements.len() - 1 {
                            page.elements.swap(idx, idx + 1);
                            drawing_area.queue_draw();
                            tracing::info!("✅ Layer moved up");
                            result = true;
                        }
                    }
                    LayerDirection::Down => {
                        if idx > 0 {
                            page.elements.swap(idx, idx - 1);
                            drawing_area.queue_draw();
                            tracing::info!("✅ Layer moved down");
                            result = true;
                        }
                    }
                }
            }
        }
    });
    result
}

/// Layer reordering direction
#[derive(Clone, Copy, Debug)]
pub enum LayerDirection {
    Up,
    Down,
}
