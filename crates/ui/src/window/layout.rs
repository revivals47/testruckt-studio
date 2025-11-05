use super::{WindowComponents, layout_v2};
use crate::app::AppState;
use crate::menu::build_menu_bar;
use crate::panels::build_property_panel;
use crate::toolbar::build_toolbar;
use gtk4::{prelude::*, Application, ApplicationWindow, Box as GtkBox, BuilderListItemFactory, ListView, NoSelection, Orientation};

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

    // Build main layout using layout_v2
    let (main_content, canvas_view, tool_palette_buttons) = layout_v2::build_layout(app_state.clone(), toolbar_widgets.clone());
    root.append(&main_content);

    window.set_child(Some(&root));

    // Create a dummy layer panel (ListView) - placeholder for future implementation
    let layer_panel: ListView = ListView::new(None::<NoSelection>, None::<BuilderListItemFactory>);

    // Get property panel from current properties module
    let property_panel = build_property_panel();

    WindowComponents::new(
        window,
        canvas_view,
        layer_panel,
        property_panel,
        menu_bar,
        toolbar_widgets,
        tool_palette_buttons,
    )
}
