use super::WindowComponents;
use crate::app::AppState;
use crate::canvas::CanvasView;
use crate::menu::build_menu_bar;
use crate::panels::{build_layer_panel, build_property_panel};
use crate::toolbar::build_toolbar;
use gtk4::{prelude::*, Application, ApplicationWindow, Box as GtkBox, Orientation, Paned};

pub fn build_widgets(app: &Application, app_state: AppState) -> WindowComponents {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Testruct Studio")
        .default_width(1600)
        .default_height(900)
        .build();

    // Root container (vertical)
    let root = GtkBox::new(Orientation::Vertical, 0);

    // Menu bar
    let menu_bar = build_menu_bar();
    root.append(&menu_bar);

    // Toolbars
    let toolbar_widgets = build_toolbar();
    root.append(&toolbar_widgets.primary_toolbar);
    root.append(&toolbar_widgets.secondary_toolbar);

    // Build main layout and extract components
    let (main_content, canvas_view, layer_panel, property_panel) =
        build_main_layout(app_state.clone());
    root.append(&main_content);

    window.set_child(Some(&root));

    WindowComponents::new(
        window,
        canvas_view,
        layer_panel,
        property_panel,
        menu_bar,
        toolbar_widgets,
    )
}

/// Build the main layout (3-pane structure)
/// Returns: (container_box, canvas_view, layer_panel, property_panel)
fn build_main_layout(
    app_state: AppState,
) -> (gtk4::Box, CanvasView, gtk4::ListView, gtk4::Box) {
    let main_paned = Paned::new(Orientation::Horizontal);
    main_paned.set_wide_handle(true);
    main_paned.set_vexpand(true);
    main_paned.set_hexpand(true);

    // Left panel: Layers
    let layer_panel = build_layer_panel();
    main_paned.set_start_child(Some(&layer_panel));
    main_paned.set_position(280);

    // Center & Right: Canvas + Properties
    let right_split = Paned::new(Orientation::Horizontal);
    right_split.set_wide_handle(true);
    right_split.set_vexpand(true);
    right_split.set_hexpand(true);

    // Canvas (must be created before adding to layout, so we can keep reference)
    let canvas_view = CanvasView::new(app_state);
    right_split.set_start_child(Some(&canvas_view.container()));
    right_split.set_position(1050);

    // Properties panel
    let property_panel = build_property_panel();
    right_split.set_end_child(Some(&property_panel));

    main_paned.set_end_child(Some(&right_split));

    // Wrap in a box for expansion
    let container = gtk4::Box::new(Orientation::Vertical, 0);
    container.set_vexpand(true);
    container.set_hexpand(true);
    container.append(&main_paned);

    (container, canvas_view, layer_panel, property_panel)
}
