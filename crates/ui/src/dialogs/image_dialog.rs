//! Image file selection dialog

use gtk4::prelude::*;
use gtk4::{FileChooserAction, FileChooserNative, Window};
use std::path::PathBuf;
use tracing::info;

/// Show an image file selection dialog (async)
///
/// Returns the selected file path if a file was chosen, None if canceled
pub fn show_image_chooser(parent: &Window) -> Option<PathBuf> {
    let dialog = FileChooserNative::new(
        Some("画像を選択"),
        Some(parent),
        FileChooserAction::Open,
        Some("開く"),
        Some("キャンセル"),
    );

    // Add file filters for image formats
    let all_filter = gtk4::FileFilter::new();
    all_filter.add_mime_type("image/png");
    all_filter.add_mime_type("image/jpeg");
    all_filter.add_mime_type("image/gif");
    all_filter.add_mime_type("image/webp");
    all_filter.set_name(Some("サポートされた画像形式"));
    dialog.add_filter(&all_filter);

    let png_filter = gtk4::FileFilter::new();
    png_filter.add_mime_type("image/png");
    png_filter.add_pattern("*.png");
    png_filter.set_name(Some("PNG 画像"));
    dialog.add_filter(&png_filter);

    let jpeg_filter = gtk4::FileFilter::new();
    jpeg_filter.add_mime_type("image/jpeg");
    jpeg_filter.add_pattern("*.jpg");
    jpeg_filter.add_pattern("*.jpeg");
    jpeg_filter.set_name(Some("JPEG 画像"));
    dialog.add_filter(&jpeg_filter);

    let gif_filter = gtk4::FileFilter::new();
    gif_filter.add_mime_type("image/gif");
    gif_filter.add_pattern("*.gif");
    gif_filter.set_name(Some("GIF 画像"));
    dialog.add_filter(&gif_filter);

    let webp_filter = gtk4::FileFilter::new();
    webp_filter.add_mime_type("image/webp");
    webp_filter.add_pattern("*.webp");
    webp_filter.set_name(Some("WebP 画像"));
    dialog.add_filter(&webp_filter);

    // Set the first filter as default
    dialog.set_filter(&all_filter);

    info!("Showing image file chooser dialog");

    // This would need to be async in a real implementation
    // For now, return None as a placeholder
    None
}

/// Show image selection dialog with callback
///
/// Handles asynchronous file selection
pub fn show_image_chooser_async(parent: &Window, on_selected: Box<dyn Fn(PathBuf)>) {
    let dialog = FileChooserNative::new(
        Some("画像を選択"),
        Some(parent),
        FileChooserAction::Open,
        Some("開く"),
        Some("キャンセル"),
    );

    // Add file filters for image formats
    let all_filter = gtk4::FileFilter::new();
    all_filter.add_mime_type("image/png");
    all_filter.add_mime_type("image/jpeg");
    all_filter.add_mime_type("image/gif");
    all_filter.add_mime_type("image/webp");
    all_filter.set_name(Some("サポートされた画像形式"));
    dialog.add_filter(&all_filter);

    let png_filter = gtk4::FileFilter::new();
    png_filter.add_mime_type("image/png");
    png_filter.add_pattern("*.png");
    png_filter.set_name(Some("PNG 画像"));
    dialog.add_filter(&png_filter);

    let jpeg_filter = gtk4::FileFilter::new();
    jpeg_filter.add_mime_type("image/jpeg");
    jpeg_filter.add_pattern("*.jpg");
    jpeg_filter.add_pattern("*.jpeg");
    jpeg_filter.set_name(Some("JPEG 画像"));
    dialog.add_filter(&jpeg_filter);

    let gif_filter = gtk4::FileFilter::new();
    gif_filter.add_mime_type("image/gif");
    gif_filter.add_pattern("*.gif");
    gif_filter.set_name(Some("GIF 画像"));
    dialog.add_filter(&gif_filter);

    let webp_filter = gtk4::FileFilter::new();
    webp_filter.add_mime_type("image/webp");
    webp_filter.add_pattern("*.webp");
    webp_filter.set_name(Some("WebP 画像"));
    dialog.add_filter(&webp_filter);

    // Set the first filter as default
    dialog.set_filter(&all_filter);

    dialog.connect_response(move |native_dialog, response_id| {
        if response_id == gtk4::ResponseType::Accept {
            if let Some(file) = native_dialog.file() {
                if let Some(path) = file.path() {
                    info!("Selected image: {}", path.display());
                    on_selected(path);
                }
            }
        } else {
            info!("Image selection canceled");
        }
    });

    dialog.show();
}
