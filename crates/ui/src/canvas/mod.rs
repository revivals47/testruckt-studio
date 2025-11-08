pub mod alignment;
pub mod dirty_region;
pub mod grid_rendering;
pub mod input;
pub mod keyboard;
pub mod mouse;
pub mod overlays;
pub mod page_thumbnail;
pub mod rendering;
pub mod rendering_images;
pub mod rendering_selection;
pub mod rendering_text;
pub mod selection;
pub mod shapes_rendering;
pub mod snapping;
pub mod text_editor;
pub mod tools;

use gtk4::{prelude::*, DrawingArea, Overlay, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

use crate::app::AppState;
use dirty_region::DirtyRegionTracker;
use grid_rendering::RulerConfig;
use rendering::RenderConfig;

/// Render state tracking
#[derive(Clone)]
pub struct CanvasRenderState {
    pub config: Rc<RefCell<RenderConfig>>,
    pub ruler_config: Rc<RefCell<RulerConfig>>,
    pub selected_ids: Rc<RefCell<Vec<uuid::Uuid>>>,
    pub drag_box: Rc<RefCell<Option<testruct_core::layout::Rect>>>,
    pub tool_state: Rc<RefCell<tools::ToolState>>,
    pub dirty_region: DirtyRegionTracker,
}

impl Default for CanvasRenderState {
    fn default() -> Self {
        Self {
            config: Rc::new(RefCell::new(RenderConfig::default())),
            ruler_config: Rc::new(RefCell::new(RulerConfig::default())),
            selected_ids: Rc::new(RefCell::new(Vec::new())),
            drag_box: Rc::new(RefCell::new(None)),
            tool_state: Rc::new(RefCell::new(tools::ToolState::default())),
            dirty_region: dirty_region::new_tracker(),
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
        drawing_area.set_can_target(true); // Enable mouse events - CRITICAL for GTK4
        drawing_area.set_focusable(true); // Enable keyboard focus

        // Create overlay with drawing area (rulers will be drawn on canvas, not as overlay widgets)
        let overlay = Overlay::new();
        overlay.set_child(Some(&drawing_area));
        // NOTE: Removed add_ruler_overlay() - rulers will be drawn directly on canvas instead
        overlay.set_can_target(true); // Ensure overlay also targets events
        overlay.set_hexpand(true);
        overlay.set_vexpand(true);

        // TEST: Use overlay directly instead of ScrolledWindow to isolate event issue
        // Wrap in ScrolledWindow for panning/zooming
        let container = ScrolledWindow::new();
        container.set_child(Some(&overlay));
        container.set_hexpand(true);
        container.set_vexpand(true);
        container.set_can_target(true); // Ensure ScrolledWindow also targets events

        let render_state = CanvasRenderState::default();

        // Setup drawing function
        Self::setup_draw_func(&drawing_area, &app_state, &render_state);

        // Wire up all event handlers - must happen AFTER container setup
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

        // Get page size from document metadata
        let page_size = document.metadata.page_size.to_size();

        // Draw page border
        rendering::draw_page_border(ctx, &page_size)?;

        // Apply zoom and pan
        ctx.translate(
            ruler_config.size + config.pan_x,
            ruler_config.size + config.pan_y,
        );
        ctx.scale(config.zoom, config.zoom);

        // Draw grid if enabled
        if config.show_grid {
            grid_rendering::draw_grid(ctx, &page_size)?;
        }

        // Draw guides if enabled
        if config.show_guides && !config.guides.is_empty() {
            grid_rendering::draw_guides(ctx, &config.guides, &page_size)?;
        }

        // Draw page elements
        let selected = render_state.selected_ids.borrow();
        Self::draw_elements(ctx, page, &selected, render_state, app_state)?;
        drop(selected);

        // Draw drag preview box (blue outline while dragging)
        if let Some(drag_rect) = render_state.drag_box.borrow().as_ref() {
            ctx.set_source_rgb(0.05, 0.49, 0.86); // Blue color
            ctx.set_line_width(2.0 / config.zoom); // Account for zoom
            ctx.rectangle(
                drag_rect.origin.x as f64,
                drag_rect.origin.y as f64,
                drag_rect.size.width as f64,
                drag_rect.size.height as f64,
            );
            ctx.stroke()?;
        }

        Ok(())
    }

    /// Draw all document elements on the page
    fn draw_elements(
        ctx: &gtk4::cairo::Context,
        page: &testruct_core::document::Page,
        selected_ids: &[uuid::Uuid],
        render_state: &CanvasRenderState,
        app_state: &AppState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for element in &page.elements {
            Self::draw_element(ctx, element, selected_ids, render_state, app_state)?;
        }
        Ok(())
    }

    /// Draw a single document element, recursively handling frames
    fn draw_element(
        ctx: &gtk4::cairo::Context,
        element: &testruct_core::document::DocumentElement,
        selected_ids: &[uuid::Uuid],
        render_state: &CanvasRenderState,
        app_state: &AppState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use testruct_core::document::{DocumentElement, ShapeKind};

        match element {
            DocumentElement::Group(group) => {
                // Draw group border (similar to frame but with different styling)
                ctx.set_source_rgb(0.7, 0.7, 0.9);
                ctx.set_line_width(1.0);
                ctx.rectangle(
                    group.bounds.origin.x as f64,
                    group.bounds.origin.y as f64,
                    group.bounds.size.width as f64,
                    group.bounds.size.height as f64,
                );
                ctx.stroke()?;

                // Draw group selection highlight if selected
                let is_selected = selected_ids.contains(&group.id);
                if is_selected {
                    let selection_color = testruct_core::typography::Color {
                        r: 0.05,
                        g: 0.49,
                        b: 0.86,
                        a: 1.0,
                    };
                    rendering::draw_selection_box(ctx, &group.bounds, &selection_color)?;
                    rendering::draw_resize_handles(ctx, &group.bounds, &selection_color)?;
                }

                // Recursively draw group children
                for child in &group.children {
                    Self::draw_element(ctx, child, selected_ids, render_state, app_state)?;
                }
            }
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

                // Draw frame selection highlight if selected
                let is_selected = selected_ids.contains(&frame.id);
                if is_selected {
                    let selection_color = testruct_core::typography::Color {
                        r: 0.05,
                        g: 0.49,
                        b: 0.86,
                        a: 1.0,
                    };
                    rendering::draw_selection_box(ctx, &frame.bounds, &selection_color)?;
                    rendering::draw_resize_handles(ctx, &frame.bounds, &selection_color)?;
                }

                // Recursively draw frame children
                for child in &frame.children {
                    Self::draw_element(ctx, child, selected_ids, render_state, app_state)?;
                }
            }
            DocumentElement::Text(text) => {
                // Use actual bounds from text element
                let text_bounds = &text.bounds;

                let is_selected = selected_ids.contains(&text.id);

                // Check if this text element is being edited
                let tool_state = render_state.tool_state.borrow();
                let is_editing = tool_state.editing_text_id == Some(text.id);
                let cursor_pos = tool_state.editing_cursor_pos;
                drop(tool_state);

                rendering::draw_text_element(ctx, text_bounds, &text.content, &text.style)?;

                if is_editing {
                    // Draw editing frame
                    rendering::draw_text_editing_frame(ctx, text_bounds)?;
                    // Draw cursor
                    rendering::draw_text_cursor(
                        ctx,
                        text_bounds,
                        &text.content,
                        cursor_pos,
                        &text.style,
                    )?;
                } else if is_selected {
                    let selection_color = testruct_core::typography::Color {
                        r: 0.05,
                        g: 0.49,
                        b: 0.86,
                        a: 1.0,
                    };
                    rendering::draw_selection_box(ctx, text_bounds, &selection_color)?;
                    rendering::draw_resize_handles(ctx, text_bounds, &selection_color)?;
                }
            }
            DocumentElement::Image(image) => {
                // Draw image element with actual image or fallback to placeholder
                if let Err(e) = rendering::draw_image_element(
                    ctx,
                    &image.bounds,
                    &image.source,
                    app_state,
                ) {
                    tracing::warn!("Failed to render image: {}", e);
                    // Fallback to placeholder if rendering fails
                    rendering::draw_image_placeholder(ctx, &image.bounds)?;
                }

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
                        rendering::draw_rectangle(ctx, &shape.bounds, &shape.stroke, &shape.fill)?;
                    }
                    ShapeKind::Ellipse => {
                        rendering::draw_ellipse(ctx, &shape.bounds, &shape.stroke, &shape.fill)?;
                    }
                    ShapeKind::Line => {
                        rendering::draw_line(ctx, &shape.bounds, &shape.stroke)?;
                    }
                    ShapeKind::Arrow => {
                        rendering::draw_arrow(ctx, &shape.bounds, &shape.stroke)?;
                    }
                    ShapeKind::Polygon => {
                        rendering::draw_polygon(ctx, &shape.bounds, &shape.stroke)?;
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
