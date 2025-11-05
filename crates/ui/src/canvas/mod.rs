pub mod input;
pub mod overlays;
pub mod rendering;
pub mod mouse;
pub mod keyboard;
pub mod selection;
pub mod tools;

use gtk4::{prelude::*, DrawingArea, Overlay, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

use crate::app::AppState;
use rendering::{RenderConfig, RulerConfig};

/// Render state tracking
#[derive(Clone)]
pub struct CanvasRenderState {
    pub config: Rc<RefCell<RenderConfig>>,
    pub ruler_config: Rc<RefCell<RulerConfig>>,
    pub selected_ids: Rc<RefCell<Vec<uuid::Uuid>>>,
    pub drag_box: Rc<RefCell<Option<testruct_core::layout::Rect>>>,
    pub tool_state: Rc<RefCell<tools::ToolState>>,
}

impl Default for CanvasRenderState {
    fn default() -> Self {
        Self {
            config: Rc::new(RefCell::new(RenderConfig::default())),
            ruler_config: Rc::new(RefCell::new(RulerConfig::default())),
            selected_ids: Rc::new(RefCell::new(Vec::new())),
            drag_box: Rc::new(RefCell::new(None)),
            tool_state: Rc::new(RefCell::new(tools::ToolState::default())),
        }
    }
}

pub struct CanvasView {
    container: ScrolledWindow,
    drawing_area: DrawingArea,
    overlay: Overlay,
    render_state: CanvasRenderState,
}

impl CanvasView {
    pub fn new(app_state: AppState) -> Self {
        let drawing_area = DrawingArea::builder()
            .content_width(1200)
            .content_height(1600)
            .build();
        drawing_area.set_hexpand(true);
        drawing_area.set_vexpand(true);

        let overlay = Overlay::new();
        overlay.set_child(Some(&drawing_area));
        overlays::add_ruler_overlay(&overlay);

        let container = ScrolledWindow::new();
        container.set_child(Some(&overlay));
        container.set_hexpand(true);
        container.set_vexpand(true);

        let render_state = CanvasRenderState::default();

        // Setup drawing function
        Self::setup_draw_func(&drawing_area, &app_state, &render_state);

        input::wire_pointer_events(&drawing_area, &render_state, &app_state);

        Self {
            container,
            drawing_area,
            overlay,
            render_state,
        }
    }

    /// Setup the Cairo draw function for the canvas
    fn setup_draw_func(
        drawing_area: &DrawingArea,
        app_state: &AppState,
        render_state: &CanvasRenderState,
    ) {
        let app_state_clone = app_state.clone();
        let render_state = render_state.clone();

        drawing_area.set_draw_func(move |_area, ctx, width, height| {
            if let Err(e) = Self::draw_canvas(
                ctx,
                width as f64,
                height as f64,
                &app_state_clone,
                &render_state,
            ) {
                eprintln!("Canvas draw error: {}", e);
            }
        });
    }

    /// Main canvas drawing function
    fn draw_canvas(
        ctx: &gtk4::cairo::Context,
        width: f64,
        height: f64,
        app_state: &AppState,
        render_state: &CanvasRenderState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ruler_config = render_state.ruler_config.borrow();
        let config = render_state.config.borrow();

        // Draw background and rulers
        rendering::draw_background(ctx, width, height, &ruler_config)?;

        // Get the active document
        let Some(document) = app_state.active_document() else {
            return Ok(());
        };

        // Get the first page (for now, single-page support)
        let Some(page) = document.pages.first() else {
            return Ok(());
        };

        // TODO: Get page size from document metadata or configuration
        // For now, using standard A4 dimensions in points (595 x 842)
        let page_size = testruct_core::layout::Size {
            width: 595.0,
            height: 842.0,
        };

        // Draw page border
        rendering::draw_page_border(ctx, &page_size)?;

        // Apply zoom and pan
        ctx.translate(ruler_config.size + config.pan_x, ruler_config.size + config.pan_y);
        ctx.scale(config.zoom, config.zoom);

        // Draw grid if enabled
        if config.show_grid {
            rendering::draw_grid(ctx, &page_size)?;
        }

        // Draw page elements
        let selected = render_state.selected_ids.borrow();
        Self::draw_elements(ctx, page, &selected)?;

        Ok(())
    }

    /// Draw all document elements on the page
    fn draw_elements(
        ctx: &gtk4::cairo::Context,
        page: &testruct_core::document::Page,
        selected_ids: &[uuid::Uuid],
    ) -> Result<(), Box<dyn std::error::Error>> {
        use testruct_core::document::{DocumentElement, ShapeKind};

        for element in &page.elements {
            match element {
                DocumentElement::Frame(frame) => {
                    // Draw frame border
                    ctx.set_source_rgb(0.9, 0.9, 0.9);
                    ctx.set_line_width(1.0);
                    ctx.rectangle(
                        frame.bounds.origin.x as f64,
                        frame.bounds.origin.y as f64,
                        frame.bounds.size.width as f64,
                        frame.bounds.size.height as f64,
                    );
                    ctx.stroke()?;

                    // Recursively draw children
                    // TODO: Implement frame children rendering
                }
                DocumentElement::Text(text) => {
                    // TODO: TextElement needs bounds field in core crate
                    // For now, create a placeholder bounds for rendering
                    let text_bounds = testruct_core::layout::Rect {
                        origin: testruct_core::layout::Point { x: 10.0, y: 10.0 },
                        size: testruct_core::layout::Size { width: 200.0, height: 50.0 },
                    };

                    let is_selected = selected_ids.contains(&text.id);
                    rendering::draw_text_element(
                        ctx,
                        &text_bounds,
                        &text.content,
                        &text.style,
                    )?;

                    if is_selected {
                        let selection_color = testruct_core::typography::Color {
                            r: 0.05,
                            g: 0.49,
                            b: 0.86,
                            a: 1.0,
                        };
                        rendering::draw_selection_box(ctx, &text_bounds, &selection_color)?;
                        rendering::draw_resize_handles(ctx, &text_bounds, &selection_color)?;
                    }
                }
                DocumentElement::Image(image) => {
                    // TODO: Load and render image from asset reference
                    ctx.set_source_rgb(0.8, 0.8, 0.8);
                    ctx.rectangle(
                        image.bounds.origin.x as f64,
                        image.bounds.origin.y as f64,
                        image.bounds.size.width as f64,
                        image.bounds.size.height as f64,
                    );
                    ctx.fill()?;

                    let is_selected = selected_ids.contains(&image.id);
                    if is_selected {
                        let selection_color = testruct_core::typography::Color {
                            r: 0.05,
                            g: 0.49,
                            b: 0.86,
                            a: 1.0,
                        };
                        rendering::draw_selection_box(ctx, &image.bounds, &selection_color)?;
                        rendering::draw_resize_handles(ctx, &image.bounds, &selection_color)?;
                    }
                }
                DocumentElement::Shape(shape) => {
                    match shape.kind {
                        ShapeKind::Rectangle => {
                            rendering::draw_rectangle(
                                ctx,
                                &shape.bounds,
                                &shape.stroke,
                                &shape.fill,
                            )?;
                        }
                        ShapeKind::Ellipse => {
                            rendering::draw_ellipse(
                                ctx,
                                &shape.bounds,
                                &shape.stroke,
                                &shape.fill,
                            )?;
                        }
                        ShapeKind::Line => {
                            rendering::draw_line(ctx, &shape.bounds, &shape.stroke)?;
                        }
                        ShapeKind::Polygon => {
                            // TODO: Implement polygon rendering
                        }
                    }

                    let is_selected = selected_ids.contains(&shape.id);
                    if is_selected {
                        let selection_color = testruct_core::typography::Color {
                            r: 0.05,
                            g: 0.49,
                            b: 0.86,
                            a: 1.0,
                        };
                        rendering::draw_selection_box(ctx, &shape.bounds, &selection_color)?;
                        rendering::draw_resize_handles(ctx, &shape.bounds, &selection_color)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn container(&self) -> ScrolledWindow {
        self.container.clone()
    }

    pub fn drawing_area(&self) -> DrawingArea {
        self.drawing_area.clone()
    }

    pub fn overlay(&self) -> Overlay {
        self.overlay.clone()
    }

    pub fn render_state(&self) -> &CanvasRenderState {
        &self.render_state
    }

    /// Set the current tool mode
    pub fn set_tool_mode(&self, tool: tools::ToolMode) {
        self.render_state.tool_state.borrow_mut().current_tool = tool;
        self.drawing_area.queue_draw();
    }

    /// Get the current tool mode
    pub fn get_tool_mode(&self) -> tools::ToolMode {
        self.render_state.tool_state.borrow().current_tool
    }

    /// Clear all selected objects
    pub fn clear_selection(&self) {
        self.render_state.selected_ids.borrow_mut().clear();
        self.drawing_area.queue_draw();
    }

    /// Select a single object by ID
    pub fn select_object(&self, id: uuid::Uuid) {
        let mut selected = self.render_state.selected_ids.borrow_mut();
        selected.clear();
        selected.push(id);
        drop(selected);
        self.drawing_area.queue_draw();
    }

    /// Add an object to the selection
    pub fn add_to_selection(&self, id: uuid::Uuid) {
        let mut selected = self.render_state.selected_ids.borrow_mut();
        if !selected.contains(&id) {
            selected.push(id);
        }
        drop(selected);
        self.drawing_area.queue_draw();
    }

    /// Remove an object from the selection
    pub fn remove_from_selection(&self, id: uuid::Uuid) {
        let mut selected = self.render_state.selected_ids.borrow_mut();
        selected.retain(|&selected_id| selected_id != id);
        drop(selected);
        self.drawing_area.queue_draw();
    }

    /// Toggle object selection
    pub fn toggle_selection(&self, id: uuid::Uuid) {
        let mut selected = self.render_state.selected_ids.borrow_mut();
        if let Some(pos) = selected.iter().position(|&selected_id| selected_id == id) {
            selected.remove(pos);
        } else {
            selected.push(id);
        }
        drop(selected);
        self.drawing_area.queue_draw();
    }
}
