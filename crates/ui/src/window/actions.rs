//! Window-level action handlers for menu and toolbar events
//!
//! Implements action callbacks for file operations, editing, view toggles, and tools.

use gtk4::{gio, prelude::*};

/// Register all window-level actions
pub fn register_window_actions(window: &gtk4::ApplicationWindow, _state: crate::app::AppState) {
    // File menu actions
    add_window_action(window, "new", |_| {
        tracing::info!("Action: new document");
        // TODO: Create new document
    });

    add_window_action(window, "open", |_| {
        tracing::info!("Action: open document");
        // TODO: Open document dialog
    });

    add_window_action(window, "save", |_| {
        tracing::info!("Action: save document");
        // TODO: Save current document
    });

    add_window_action(window, "save-as", |_| {
        tracing::info!("Action: save document as");
        // TODO: Save as dialog
    });

    // Export actions
    for format in &["pdf", "png", "jpeg", "svg"] {
        let action_name = format!("export-{}", format);
        let format_str = format.to_string();
        add_window_action_with_capture(window, &action_name, format_str, |_, format| {
            tracing::info!("Action: export as {}", format);
            // TODO: Export in specified format
        });
    }

    // Edit menu actions
    add_window_action(window, "undo", |_| {
        tracing::info!("Action: undo");
        // TODO: Perform undo operation
    });

    add_window_action(window, "redo", |_| {
        tracing::info!("Action: redo");
        // TODO: Perform redo operation
    });

    add_window_action(window, "select-all", |_| {
        tracing::info!("Action: select all objects");
        // TODO: Select all objects on canvas
    });

    // View menu actions
    add_window_action(window, "toggle-grid", |_| {
        tracing::info!("Action: toggle grid visibility");
        // TODO: Toggle grid visibility on canvas
    });

    add_window_action(window, "toggle-guides", |_| {
        tracing::info!("Action: toggle guides visibility");
        // TODO: Toggle guides visibility
    });

    add_window_action(window, "toggle-rulers", |_| {
        tracing::info!("Action: toggle rulers");
        // TODO: Toggle rulers visibility
    });

    add_window_action(window, "toggle-layers", |_| {
        tracing::info!("Action: toggle layers panel");
        // TODO: Toggle layers panel visibility
    });

    add_window_action(window, "toggle-properties", |_| {
        tracing::info!("Action: toggle properties panel");
        // TODO: Toggle properties panel visibility
    });

    add_window_action(window, "open-json-editor", |_| {
        tracing::info!("Action: open JSON editor");
        // TODO: Open JSON editor panel
    });

    // Tools menu actions
    add_window_action(window, "templates", |_| {
        tracing::info!("Action: show templates");
        // TODO: Show template manager
    });

    add_window_action(window, "toggle-item-library", |_| {
        tracing::info!("Action: toggle item library");
        // TODO: Toggle item library panel
    });

    add_window_action(window, "toggle-block-tools", |_| {
        tracing::info!("Action: toggle block tools");
        // TODO: Toggle block tools panel
    });

    add_window_action(window, "settings", |_| {
        tracing::info!("Action: show settings");
        // TODO: Show settings dialog
    });

    // Help menu actions
    add_window_action(window, "user-manual", |_| {
        tracing::info!("Action: open user manual");
        // TODO: Open user manual
    });

    add_window_action(window, "about", |_| {
        tracing::info!("Action: show about dialog");
        // TODO: Show about dialog
    });

    // Set keyboard accelerators
    set_accelerators(window);
}

/// Add a window action with a callback
fn add_window_action<F>(window: &gtk4::ApplicationWindow, name: &str, callback: F)
where
    F: Fn(&gtk4::ApplicationWindow) + 'static,
{
    let action = gio::SimpleAction::new(name, None);
    let window_ref = window.clone();
    action.connect_activate(move |_, _| callback(&window_ref));
    window.add_action(&action);
}

/// Add a window action with captured state
fn add_window_action_with_capture<F, T>(window: &gtk4::ApplicationWindow, name: &str, capture: T, callback: F)
where
    F: Fn(&gtk4::ApplicationWindow, T) + 'static,
    T: Clone + 'static,
{
    let action = gio::SimpleAction::new(name, None);
    let window_ref = window.clone();
    action.connect_activate(move |_, _| callback(&window_ref, capture.clone()));
    window.add_action(&action);
}

/// Set keyboard accelerators for window-level actions
fn set_accelerators(window: &gtk4::ApplicationWindow) {
    let app = window.application().unwrap();

    let shortcuts = [
        ("win.new", "<Primary>n"),
        ("win.open", "<Primary>o"),
        ("win.save", "<Primary>s"),
        ("win.save-as", "<Primary><Shift>s"),
        ("win.undo", "<Primary>z"),
        ("win.redo", "<Primary><Shift>z"),
        ("win.select-all", "<Primary>a"),
        ("win.toggle-grid", "F8"),
        ("win.toggle-guides", "F7"),
        ("win.toggle-rulers", "F6"),
    ];

    for (action, accel) in &shortcuts {
        app.set_accels_for_action(action, &[accel]);
    }
}
