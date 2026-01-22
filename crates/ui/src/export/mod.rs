//! Document export functionality
//!
//! Provides PDF, PNG, SVG, and other format export capabilities using Cairo rendering.

pub mod image;
pub mod image_utils;
pub mod pdf;
pub mod svg;

use anyhow::Result;
use std::path::Path;
use testruct_core::typography::Color;
use testruct_core::workspace::assets::AssetCatalog;
use testruct_core::Document;

/// Resolution scale presets for export
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResolutionScale {
    /// 1x (96 DPI) - Standard screen resolution
    Standard,
    /// 2x (192 DPI) - High DPI / Retina
    High,
    /// 3x (288 DPI) - Ultra high resolution
    Ultra,
    /// Custom DPI value
    Custom(f64),
}

impl ResolutionScale {
    /// Convert resolution scale to DPI value
    pub fn to_dpi(&self) -> f64 {
        match self {
            ResolutionScale::Standard => 96.0,
            ResolutionScale::High => 192.0,
            ResolutionScale::Ultra => 288.0,
            ResolutionScale::Custom(dpi) => *dpi,
        }
    }

    /// Get the scale multiplier (1x, 2x, 3x)
    pub fn multiplier(&self) -> f64 {
        self.to_dpi() / 96.0
    }

    /// Get display name for UI
    pub fn display_name(&self) -> String {
        match self {
            ResolutionScale::Standard => "1x (96 DPI)".to_string(),
            ResolutionScale::High => "2x (192 DPI)".to_string(),
            ResolutionScale::Ultra => "3x (288 DPI)".to_string(),
            ResolutionScale::Custom(dpi) => format!("Custom ({:.0} DPI)", dpi),
        }
    }
}

impl Default for ResolutionScale {
    fn default() -> Self {
        ResolutionScale::Standard
    }
}

/// Background color options for export
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackgroundOption {
    /// Transparent background (PNG only, SVG uses transparent)
    Transparent,
    /// White background
    White,
    /// Custom color background
    Custom(Color),
}

impl BackgroundOption {
    /// Get the color value (None for transparent)
    pub fn to_color(&self) -> Option<Color> {
        match self {
            BackgroundOption::Transparent => None,
            BackgroundOption::White => Some(Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }),
            BackgroundOption::Custom(color) => Some(*color),
        }
    }

    /// Check if background is transparent
    pub fn is_transparent(&self) -> bool {
        matches!(self, BackgroundOption::Transparent)
    }

    /// Get display name for UI
    pub fn display_name(&self) -> String {
        match self {
            BackgroundOption::Transparent => "Transparent".to_string(),
            BackgroundOption::White => "White".to_string(),
            BackgroundOption::Custom(_) => "Custom Color".to_string(),
        }
    }
}

impl Default for BackgroundOption {
    fn default() -> Self {
        BackgroundOption::White
    }
}

/// Export file format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    PNG,
    JPEG,
    SVG,
    PDF,
}

impl ExportFormat {
    /// Get file extension for the format
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::PNG => "png",
            ExportFormat::JPEG => "jpg",
            ExportFormat::SVG => "svg",
            ExportFormat::PDF => "pdf",
        }
    }

    /// Get display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            ExportFormat::PNG => "PNG Image",
            ExportFormat::JPEG => "JPEG Image",
            ExportFormat::SVG => "SVG Vector",
            ExportFormat::PDF => "PDF Document",
        }
    }

    /// Check if format supports transparency
    pub fn supports_transparency(&self) -> bool {
        matches!(self, ExportFormat::PNG | ExportFormat::SVG)
    }
}

/// Configuration for document export
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Export file format
    pub format: ExportFormat,
    /// Resolution scale (for raster formats)
    pub resolution: ResolutionScale,
    /// Background color option
    pub background: BackgroundOption,
    /// JPEG quality (0-100, only for JPEG format)
    pub jpeg_quality: i32,
    /// Whether to export all pages or just the active page
    pub export_all_pages: bool,
    /// Page index to export (if not exporting all pages)
    pub page_index: Option<usize>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::PNG,
            resolution: ResolutionScale::Standard,
            background: BackgroundOption::White,
            jpeg_quality: 95,
            export_all_pages: true,
            page_index: None,
        }
    }
}

impl ExportConfig {
    /// Create a new export config with PNG format
    pub fn png() -> Self {
        Self {
            format: ExportFormat::PNG,
            ..Default::default()
        }
    }

    /// Create a new export config with SVG format
    pub fn svg() -> Self {
        Self {
            format: ExportFormat::SVG,
            background: BackgroundOption::Transparent,
            ..Default::default()
        }
    }

    /// Create a new export config with PDF format
    pub fn pdf() -> Self {
        Self {
            format: ExportFormat::PDF,
            ..Default::default()
        }
    }

    /// Set resolution scale
    pub fn with_resolution(mut self, resolution: ResolutionScale) -> Self {
        self.resolution = resolution;
        self
    }

    /// Set background option
    pub fn with_background(mut self, background: BackgroundOption) -> Self {
        self.background = background;
        self
    }

    /// Set to export only specific page
    pub fn with_page(mut self, page_index: usize) -> Self {
        self.export_all_pages = false;
        self.page_index = Some(page_index);
        self
    }

    /// Get DPI value for export
    pub fn dpi(&self) -> f64 {
        self.resolution.to_dpi()
    }
}

/// Export a document using ExportConfig
///
/// This is the primary export function that handles all formats with full configuration.
pub fn export_with_config(
    document: &Document,
    output_path: &Path,
    config: &ExportConfig,
    catalog: &AssetCatalog,
) -> Result<()> {
    match config.format {
        ExportFormat::PNG => {
            image::render_to_png_with_config(document, output_path, config, catalog)
        }
        ExportFormat::JPEG => {
            image::render_to_jpeg_with_config(document, output_path, config, catalog)
        }
        ExportFormat::SVG => {
            svg::render_to_svg_with_config(document, output_path, config, catalog)
        }
        ExportFormat::PDF => {
            pdf::render_to_pdf(document, output_path, catalog)
        }
    }
}

/// Export a document to PDF format
pub fn export_pdf(document: &Document, output_path: &Path, catalog: &AssetCatalog) -> Result<()> {
    pdf::render_to_pdf(document, output_path, catalog)
}

/// Export a document to PNG format (legacy API)
pub fn export_png(
    document: &Document,
    output_path: &Path,
    dpi: f64,
    catalog: &AssetCatalog,
) -> Result<()> {
    image::render_to_png(document, output_path, dpi, catalog)
}

/// Export a document to PNG format with full configuration
pub fn export_png_with_config(
    document: &Document,
    output_path: &Path,
    config: &ExportConfig,
    catalog: &AssetCatalog,
) -> Result<()> {
    image::render_to_png_with_config(document, output_path, config, catalog)
}

/// Export a document to JPEG format (legacy API)
pub fn export_jpeg(
    document: &Document,
    output_path: &Path,
    dpi: f64,
    quality: i32,
    catalog: &AssetCatalog,
) -> Result<()> {
    image::render_to_jpeg(document, output_path, dpi, quality, catalog)
}

/// Export a document to SVG format (legacy API)
pub fn export_svg(document: &Document, output_path: &Path, catalog: &AssetCatalog) -> Result<()> {
    svg::render_to_svg(document, output_path, catalog)
}

/// Export a document to SVG format with full configuration
pub fn export_svg_with_config(
    document: &Document,
    output_path: &Path,
    config: &ExportConfig,
    catalog: &AssetCatalog,
) -> Result<()> {
    svg::render_to_svg_with_config(document, output_path, config, catalog)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loads() {
        // Module loads successfully if this compiles and runs
    }

    #[test]
    fn test_resolution_scale_to_dpi() {
        assert_eq!(ResolutionScale::Standard.to_dpi(), 96.0);
        assert_eq!(ResolutionScale::High.to_dpi(), 192.0);
        assert_eq!(ResolutionScale::Ultra.to_dpi(), 288.0);
        assert_eq!(ResolutionScale::Custom(150.0).to_dpi(), 150.0);
    }

    #[test]
    fn test_resolution_scale_multiplier() {
        assert_eq!(ResolutionScale::Standard.multiplier(), 1.0);
        assert_eq!(ResolutionScale::High.multiplier(), 2.0);
        assert_eq!(ResolutionScale::Ultra.multiplier(), 3.0);
    }

    #[test]
    fn test_background_option_to_color() {
        assert!(BackgroundOption::Transparent.to_color().is_none());
        assert!(BackgroundOption::White.to_color().is_some());

        let custom = BackgroundOption::Custom(Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 });
        let color = custom.to_color().unwrap();
        assert_eq!(color.r, 0.5);
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::PNG.extension(), "png");
        assert_eq!(ExportFormat::JPEG.extension(), "jpg");
        assert_eq!(ExportFormat::SVG.extension(), "svg");
        assert_eq!(ExportFormat::PDF.extension(), "pdf");
    }

    #[test]
    fn test_export_config_builder() {
        let config = ExportConfig::png()
            .with_resolution(ResolutionScale::High)
            .with_background(BackgroundOption::Transparent);

        assert_eq!(config.format, ExportFormat::PNG);
        assert_eq!(config.resolution, ResolutionScale::High);
        assert_eq!(config.background, BackgroundOption::Transparent);
    }
}
