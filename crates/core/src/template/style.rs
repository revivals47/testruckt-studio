use crate::typography::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateStyle {
    pub theme: ThemeColors,
    pub font_family: String,
}

impl Default for TemplateStyle {
    fn default() -> Self {
        Self {
            theme: ThemeColors::default(),
            font_family: "Inter".into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: Color,
    pub accent: Color,
    pub text: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            background: Color::from_rgb(0.95, 0.95, 0.95),
            accent: Color::from_rgb(0.3, 0.5, 0.9),
            text: Color::from_rgb(0.1, 0.1, 0.1),
        }
    }
}
