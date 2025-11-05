use super::WindowComponents;
use crate::app::AppState;
use gtk4::prelude::*;

pub fn bind_events(components: &WindowComponents, _state: AppState) {
    // CanvasView is already configured with event handlers and draw function
    // during CanvasView::new() in layout::build_widgets()
    // This function can be used for additional window-level event binding in the future

    components.window.set_focus_visible(true);
}
