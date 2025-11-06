mod actions;
mod bindings;
mod layout;
mod layout_v2;

use crate::app::AppState;
use crate::canvas::CanvasView;
use crate::toolbar::ToolbarWidgets;
use crate::window::layout_v2::ToolPaletteButtons;
use gtk4::{Application, ApplicationWindow};

pub struct MainWindow {
    pub window: ApplicationWindow,
}

impl MainWindow {
    pub fn build(app: &Application, state: AppState) -> ApplicationWindow {
        let components = layout::build_widgets(app, state.clone());

        // Store window reference in AppState for later access
        state.set_window(&components.window);

        actions::register_window_actions(
            &components.window,
            state.clone(),
            &components.canvas_view,
            &components.tool_palette,
            &components.properties_panel,
            &components.property_components,
            &components.toolbar.buttons,
        );
        crate::panels::wire_property_signals(
            &components.property_components,
            state.clone(),
            &components.canvas_view,
        );
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
    pub tool_palette_buttons: ToolPaletteButtons,
    pub tool_palette: gtk4::Box,
    pub properties_panel: gtk4::Box,
    pub property_components: crate::panels::PropertyPanelComponents,
}

impl WindowComponents {
    fn new(
        window: ApplicationWindow,
        canvas_view: CanvasView,
        layer_panel: gtk4::ListView,
        property_panel: gtk4::Box,
        menu_bar: gtk4::PopoverMenuBar,
        toolbar: ToolbarWidgets,
        tool_palette_buttons: ToolPaletteButtons,
        tool_palette: gtk4::Box,
        properties_panel: gtk4::Box,
        property_components: crate::panels::PropertyPanelComponents,
    ) -> Self {
        Self {
            window,
            canvas_view,
            layer_panel,
            property_panel,
            menu_bar,
            toolbar,
            tool_palette_buttons,
            tool_palette,
            properties_panel,
            property_components,
        }
    }
}
