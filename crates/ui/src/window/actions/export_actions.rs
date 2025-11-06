//! Export action handlers (PDF, PNG, JPEG, SVG)

use super::common::add_window_action;
use gtk4::prelude::*;

/// Register export actions
pub fn register(window: &gtk4::ApplicationWindow, state: crate::app::AppState) {
    let export_state = state.clone();
    let window_weak_pdf = window.downgrade();
    add_window_action(window, "export-pdf", move |_| {
        tracing::info!("Action: export as PDF");
        if let Some(window) = window_weak_pdf.upgrade() {
            perform_pdf_export(&window, &export_state);
        }
    });

    let export_state = state.clone();
    let window_weak_png = window.downgrade();
    add_window_action(window, "export-png", move |_| {
        tracing::info!("Action: export as PNG");
        if let Some(window) = window_weak_png.upgrade() {
            perform_image_export(&window, &export_state, "png");
        }
    });

    let export_state = state.clone();
    let window_weak_jpeg = window.downgrade();
    add_window_action(window, "export-jpeg", move |_| {
        tracing::info!("Action: export as JPEG");
        if let Some(window) = window_weak_jpeg.upgrade() {
            perform_image_export(&window, &export_state, "jpeg");
        }
    });

    let export_state = state.clone();
    let window_weak_svg = window.downgrade();
    add_window_action(window, "export-svg", move |_| {
        tracing::info!("Action: export as SVG");
        if let Some(window) = window_weak_svg.upgrade() {
            perform_image_export(&window, &export_state, "svg");
        }
    });
}

/// Perform PDF export
fn perform_pdf_export(window: &gtk4::ApplicationWindow, state: &crate::app::AppState) {
    if state.active_document().is_some() {
        tracing::info!("Exporting active document to PDF");

        let window_clone = window.clone();
        let state_clone = state.clone();

        glib::spawn_future_local(async move {
            if let Some(path) =
                crate::io::file_dialog::show_export_dialog(&window_clone, "pdf").await
            {
                let catalog = state_clone.asset_catalog();
                let catalog_lock = catalog.lock().expect("Failed to lock asset catalog");

                match crate::export::export_pdf(
                    &state_clone.active_document().unwrap(),
                    &path,
                    &catalog_lock,
                ) {
                    Ok(_) => {
                        tracing::info!("✅ PDF export completed: {}", path.display());
                    }
                    Err(e) => {
                        tracing::error!("❌ PDF export failed: {}", e);
                    }
                }
            } else {
                tracing::info!("PDF export cancelled by user");
            }
        });
    } else {
        tracing::warn!("No active document to export");
    }
}

/// Perform image export (PNG/JPEG/SVG)
fn perform_image_export(
    window: &gtk4::ApplicationWindow,
    state: &crate::app::AppState,
    format: &str,
) {
    if state.active_document().is_some() {
        tracing::info!("Exporting active document to {}", format.to_uppercase());

        let window_clone = window.clone();
        let state_clone = state.clone();
        let format_str = format.to_string();

        glib::spawn_future_local(async move {
            if let Some(path) =
                crate::io::file_dialog::show_export_dialog(&window_clone, &format_str).await
            {
                let catalog = state_clone.asset_catalog();
                let catalog_lock = catalog.lock().expect("Failed to lock asset catalog");

                let result = match format_str.as_str() {
                    "png" => crate::export::export_png(
                        &state_clone.active_document().unwrap(),
                        &path,
                        96.0,
                        &catalog_lock,
                    ),
                    "jpeg" => crate::export::export_jpeg(
                        &state_clone.active_document().unwrap(),
                        &path,
                        96.0,
                        95,
                        &catalog_lock,
                    ),
                    "svg" => crate::export::export_svg(
                        &state_clone.active_document().unwrap(),
                        &path,
                        &catalog_lock,
                    ),
                    _ => Err(anyhow::anyhow!("Unknown format: {}", format_str)),
                };

                match result {
                    Ok(_) => {
                        tracing::info!(
                            "✅ {} export completed: {}",
                            format_str.to_uppercase(),
                            path.display()
                        );
                    }
                    Err(e) => {
                        tracing::error!("❌ {} export failed: {}", format_str.to_uppercase(), e);
                    }
                }
            } else {
                tracing::info!("{} export cancelled by user", format_str.to_uppercase());
            }
        });
    } else {
        tracing::warn!("No active document to export");
    }
}
