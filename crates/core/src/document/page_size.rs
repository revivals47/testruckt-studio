//! Page size definitions and presets

use crate::layout::Size;
use serde::{Deserialize, Serialize};

/// Common page size presets
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum PageSize {
    /// A4 size (210mm x 297mm = 595 x 842 points at 72 DPI)
    #[default]
    A4,
    /// Letter size (8.5" x 11" = 612 x 792 points at 72 DPI)
    Letter,
    /// A3 size (297mm x 420mm = 842 x 1191 points at 72 DPI)
    A3,
    /// A5 size (148mm x 210mm = 420 x 595 points at 72 DPI)
    A5,
    /// Tabloid/B4 size (11" x 17" = 792 x 1224 points at 72 DPI)
    Tabloid,
    /// Square (512 x 512 points)
    Square,
    /// Custom size (width x height in points)
    Custom(f32, f32),
}

impl PageSize {
    /// Get the size in points
    pub fn to_size(&self) -> Size {
        match self {
            PageSize::A4 => Size::new(595.0, 842.0),
            PageSize::Letter => Size::new(612.0, 792.0),
            PageSize::A3 => Size::new(842.0, 1191.0),
            PageSize::A5 => Size::new(420.0, 595.0),
            PageSize::Tabloid => Size::new(792.0, 1224.0),
            PageSize::Square => Size::new(512.0, 512.0),
            PageSize::Custom(width, height) => Size::new(*width, *height),
        }
    }

    /// Get the name of the page size
    pub fn name(&self) -> &str {
        match self {
            PageSize::A4 => "A4",
            PageSize::Letter => "Letter",
            PageSize::A3 => "A3",
            PageSize::A5 => "A5",
            PageSize::Tabloid => "Tabloid",
            PageSize::Square => "Square",
            PageSize::Custom(_, _) => "Custom",
        }
    }

    /// Get all available preset sizes
    pub fn presets() -> &'static [PageSize] {
        &[
            PageSize::A4,
            PageSize::Letter,
            PageSize::A3,
            PageSize::A5,
            PageSize::Tabloid,
            PageSize::Square,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a4_size() {
        let size = PageSize::A4.to_size();
        assert_eq!(size.width, 595.0);
        assert_eq!(size.height, 842.0);
    }

    #[test]
    fn test_custom_size() {
        let size = PageSize::Custom(800.0, 600.0).to_size();
        assert_eq!(size.width, 800.0);
        assert_eq!(size.height, 600.0);
    }

    #[test]
    fn test_default_is_a4() {
        assert_eq!(PageSize::default(), PageSize::A4);
    }
}
