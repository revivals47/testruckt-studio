use super::{CanvasLayout, Rect, Size};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutRequest {
    pub canvas: CanvasLayout,
    pub available_area: Size,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayoutResult {
    pub frames: Vec<Rect>,
}

#[derive(Debug, Error)]
pub enum LayoutError {
    #[error("insufficient room to place widget")]
    NotEnoughRoom,
}

pub struct LayoutEngine;

impl LayoutEngine {
    pub fn compute(request: &LayoutRequest) -> Result<LayoutResult, LayoutError> {
        let mut frames = Vec::new();
        for section in &request.canvas.sections {
            if section.bounds.size.width > request.available_area.width
                || section.bounds.size.height > request.available_area.height
            {
                return Err(LayoutError::NotEnoughRoom);
            }
            frames.push(section.bounds);
        }
        Ok(LayoutResult { frames })
    }
}
