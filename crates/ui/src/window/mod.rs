mod bindings;
mod layout;

use crate::app::AppState;
use crate::canvas::CanvasView;
use crate::toolbar::ToolbarWidgets;
use gtk4::{Application, ApplicationWindow};

pub struct MainWindow {
    pub window: ApplicationWindow,
}

impl MainWindow {
    pub fn build(app: &Application, state: AppState) -> ApplicationWindow {
        let components = layout::build_widgets(app, state.clone());
        bindings::bind_events(&components, state);
        components.window
    }
}

pub struct WindowComponents {
    pub window: ApplicationWindow,
    pub canvas_view: CanvasView,
    pub layer_panel: gtk4::ListView,
    pub property_panel: gtk4::Box,
    pub menu_bar: gtk4::PopoverMenuBar,
    pub toolbar: ToolbarWidgets,
}

impl WindowComponents {
    fn new(
        window: ApplicationWindow,
        canvas_view: CanvasView,
        layer_panel: gtk4::ListView,
        property_panel: gtk4::Box,
        menu_bar: gtk4::PopoverMenuBar,
        toolbar: ToolbarWidgets,
    ) -> Self {
        Self {
            window,
            canvas_view,
            layer_panel,
            property_panel,
            menu_bar,
            toolbar,
        }
    }
}
