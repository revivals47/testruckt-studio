mod keyboard;
mod mouse;
mod gesture;

pub use self::keyboard::move_selected_objects;

use crate::app::AppState;
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::DrawingArea;

/// Initialize pointer events for canvas
pub fn wire_pointer_events(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    drawing_area.set_focusable(true);
    keyboard::setup_keyboard_events(drawing_area, render_state, app_state);
    mouse::setup_mouse_tracking(drawing_area, render_state, app_state);
    gesture::setup_gestures(drawing_area, render_state, app_state);
}
