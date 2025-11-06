use super::{layout_v2, WindowComponents};
use crate::app::AppState;
use crate::menu::build_menu_bar;
use crate::toolbar::build_toolbar;
use gtk4::{
    prelude::*, Application, ApplicationWindow, Box as GtkBox, BuilderListItemFactory, ListView,
    NoSelection, Orientation,
};

pub fn build_widgets(app: &Application, app_state: AppState) -> WindowComponents {
    let start = std::time::Instant::now();
    eprintln!("📐 Creating window...");

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Testruct Studio")
        .default_width(1600)
        .default_height(900)
        .build();
    eprintln!("⏱️  Window created: {}ms", start.elapsed().as_millis());

    // Root container (vertical)
    let root = GtkBox::new(Orientation::Vertical, 0);

    // Menu bar
    eprintln!("📋 Building menu bar...");
    let t1 = std::time::Instant::now();
    let menu_bar = build_menu_bar();
    eprintln!("⏱️  Menu bar built: {}ms", t1.elapsed().as_millis());
    root.append(&menu_bar);

    // Toolbars
    eprintln!("🛠️  Building toolbars...");
    let t2 = std::time::Instant::now();
    let toolbar_widgets = build_toolbar();
    eprintln!("⏱️  Toolbars built: {}ms", t2.elapsed().as_millis());
    root.append(&toolbar_widgets.primary_toolbar);
    root.append(&toolbar_widgets.secondary_toolbar);

    // Build main layout using layout_v2
    eprintln!("🎨 Building main layout...");
    let t3 = std::time::Instant::now();
    let (
        main_content,
        canvas_view,
        tool_palette_buttons,
        tool_palette,
        properties_panel,
        property_components,
    ) = layout_v2::build_layout(app_state.clone(), toolbar_widgets.clone());
    eprintln!("⏱️  Main layout built: {}ms", t3.elapsed().as_millis());
    root.append(&main_content);

    eprintln!("🎯 Setting window content...");
    window.set_child(Some(&root));

    // Create a dummy layer panel (ListView) - placeholder for future implementation
    let layer_panel: ListView = ListView::new(None::<NoSelection>, None::<BuilderListItemFactory>);

    eprintln!(
        "✅ Total widget build time: {}ms",
        start.elapsed().as_millis()
    );
    WindowComponents::new(
        window,
        canvas_view,
        layer_panel,
        properties_panel.clone(),
        menu_bar,
        toolbar_widgets,
        tool_palette_buttons,
        tool_palette,
        properties_panel,
        property_components,
    )
}
