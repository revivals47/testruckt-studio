//! Common action utilities and helpers

use gtk4::{gio, prelude::*};

/// Add a window action with a callback
pub fn add_window_action<F>(window: &gtk4::ApplicationWindow, name: &str, callback: F)
where
    F: Fn(&gtk4::ApplicationWindow) + 'static,
{
    let action = gio::SimpleAction::new(name, None);
    let window_ref = window.clone();
    action.connect_activate(move |_, _| callback(&window_ref));
    window.add_action(&action);
}

/// Add a window action with captured state
pub fn add_window_action_with_capture<F, T>(
    window: &gtk4::ApplicationWindow,
    name: &str,
    capture: T,
    callback: F,
) where
    F: Fn(&gtk4::ApplicationWindow, T) + 'static,
    T: Clone + 'static,
{
    let action = gio::SimpleAction::new(name, None);
    let window_ref = window.clone();
    action.connect_activate(move |_, _| callback(&window_ref, capture.clone()));
    window.add_action(&action);
}

/// Set keyboard accelerators for window-level actions
pub fn set_accelerators(window: &gtk4::ApplicationWindow) {
    let app = window.application().unwrap();

    let shortcuts = [
        ("win.new", "<Primary>n"),
        ("win.open", "<Primary>o"),
        ("win.save", "<Primary>s"),
        ("win.save-as", "<Primary><Shift>s"),
        ("win.undo", "<Primary>z"),
        ("win.redo", "<Primary><Shift>z"),
        ("win.select-all", "<Primary>a"),
        ("win.copy", "<Primary>c"),
        ("win.paste", "<Primary>v"),
        ("win.group", "<Primary>g"),
        ("win.ungroup", "<Primary><Shift>g"),
        ("win.add-page", "<Primary><Shift>n"),
        ("win.delete-page", "<Primary><Shift>d"),
        ("win.duplicate-page", "<Primary><Shift>d"),
        ("win.move-page-up", "<Primary><Shift>Page_Up"),
        ("win.move-page-down", "<Primary><Shift>Page_Down"),
        ("win.insert-image", "<Primary>i"),
        ("win.toggle-grid", "F8"),
        ("win.toggle-guides", "F7"),
        ("win.toggle-rulers", "F6"),
    ];

    for (action, accel) in &shortcuts {
        app.set_accels_for_action(action, &[accel]);
    }
}
