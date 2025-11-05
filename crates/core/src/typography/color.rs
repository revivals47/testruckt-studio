use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Color {
    #[serde(default)]
    pub r: f32,
    #[serde(default)]
    pub g: f32,
    #[serde(default)]
    pub b: f32,
    #[serde(default = "Color::default_alpha")]
    pub a: f32,
}

impl Color {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }

    fn default_alpha() -> f32 {
        1.0
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Palette {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
}
