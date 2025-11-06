use super::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextStyle {
    pub font_family: String,
    pub font_size: f32,
    pub weight: FontWeight,
    pub alignment: TextAlignment,
    pub color: Color,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub background_color: Option<Color>,
    pub line_height: f32, // 行間（相対値、例: 1.0 = 通常、1.5 = 1.5倍）
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_family: "Inter".into(),
            font_size: 14.0,
            weight: FontWeight::Regular,
            alignment: TextAlignment::Start,
            color: Color::from_rgb(0.1, 0.1, 0.1),
            italic: false,
            underline: false,
            strikethrough: false,
            background_color: None,
            line_height: 1.0, // デフォルトは通常の行間
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    Light,
    Regular,
    Medium,
    Bold,
    Black,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TextAlignment {
    Start,
    Center,
    End,
    Justified,
}
