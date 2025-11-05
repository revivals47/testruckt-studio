//! Help action handlers (manual, about, settings)

use super::common::add_window_action;
use gtk4::prelude::*;

/// Register help menu actions
pub fn register(window: &gtk4::ApplicationWindow, state: crate::app::AppState) {
    let window_weak_settings = window.downgrade();
    let state_settings = state.clone();
    add_window_action(window, "settings", move |_| {
        tracing::info!("Action: show settings");
        if let Some(window) = window_weak_settings.upgrade() {
            let window_base = window.clone().upcast::<gtk4::Window>();
            crate::dialogs::show_project_settings(&window_base, state_settings.clone());
            tracing::info!("✅ Settings dialog displayed");
        }
    });

    let window_weak_manual = window.downgrade();
    add_window_action(window, "user-manual", move |_| {
        tracing::info!("Action: open user manual");
        if let Some(window) = window_weak_manual.upgrade() {
            let window_base = window.clone().upcast::<gtk4::Window>();
            crate::dialogs::show_user_manual_dialog(&window_base);
            tracing::info!("✅ User manual dialog displayed");
        }
    });

    let window_weak_about = window.downgrade();
    add_window_action(window, "about", move |_| {
        tracing::info!("Action: show about dialog");
        if let Some(window) = window_weak_about.upgrade() {
            let window_base = window.clone().upcast::<gtk4::Window>();
            crate::dialogs::show_about_dialog(&window_base);
            tracing::info!("✅ About dialog displayed");
        }
    });

    let window_weak_json = window.downgrade();
    let state_json = state.clone();
    add_window_action(window, "json-editor", move |_| {
        tracing::info!("Action: open JSON editor");
        if let Some(window) = window_weak_json.upgrade() {
            let window_base = window.clone().upcast::<gtk4::Window>();
            crate::dialogs::show_json_editor(&window_base, state_json.clone());
            tracing::info!("✅ JSON editor displayed");
        }
    });
}
