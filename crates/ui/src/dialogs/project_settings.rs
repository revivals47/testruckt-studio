use gtk4::{prelude::*, Box as GtkBox, Button, Label, Orientation, Window, SpinButton, CheckButton, Adjustment};
use gtk4::Align;
use crate::app::AppState;

pub fn show_project_settings(parent: &Window, app_state: AppState) {
    // Create dialog window for project settings
    let dialog = gtk4::ApplicationWindow::builder()
        .transient_for(parent)
        .modal(true)
        .title("プロジェクト設定")
        .default_width(500)
        .default_height(600)
        .build();

    let main_box = GtkBox::new(Orientation::Vertical, 12);
    main_box.set_margin_start(16);
    main_box.set_margin_end(16);
    main_box.set_margin_top(16);
    main_box.set_margin_bottom(16);

    // Title
    let title = Label::new(Some("プロジェクト設定"));
    title.add_css_class("title-2");
    main_box.append(&title);

    // Canvas Settings Section
    let canvas_label = Label::new(Some("キャンバス設定"));
    canvas_label.add_css_class("title-3");
    canvas_label.set_halign(Align::Start);
    canvas_label.set_margin_top(12);
    main_box.append(&canvas_label);

    // Canvas width setting
    let width_box = GtkBox::new(Orientation::Horizontal, 8);
    width_box.set_homogeneous(false);
    let width_label = Label::new(Some("デフォルト幅 (pt):"));
    width_label.set_size_request(150, -1);
    width_box.append(&width_label);
    let current_width = app_state.project().settings.default_canvas_width as f64;
    let width_adj = Adjustment::new(current_width, 100.0, 2000.0, 10.0, 50.0, 0.0);
    let width_spin = SpinButton::new(Some(&width_adj), 1.0, 0);
    width_box.append(&width_spin);
    main_box.append(&width_box);

    // Canvas height setting
    let height_box = GtkBox::new(Orientation::Horizontal, 8);
    height_box.set_homogeneous(false);
    let height_label = Label::new(Some("デフォルト高さ (pt):"));
    height_label.set_size_request(150, -1);
    height_box.append(&height_label);
    let current_height = app_state.project().settings.default_canvas_height as f64;
    let height_adj = Adjustment::new(current_height, 100.0, 3000.0, 10.0, 50.0, 0.0);
    let height_spin = SpinButton::new(Some(&height_adj), 1.0, 0);
    height_box.append(&height_spin);
    main_box.append(&height_box);

    // Grid Settings Section
    let grid_label = Label::new(Some("グリッド設定"));
    grid_label.add_css_class("title-3");
    grid_label.set_halign(Align::Start);
    grid_label.set_margin_top(12);
    main_box.append(&grid_label);

    // Grid size setting
    let grid_box = GtkBox::new(Orientation::Horizontal, 8);
    grid_box.set_homogeneous(false);
    let grid_size_label = Label::new(Some("グリッドサイズ (pt):"));
    grid_size_label.set_size_request(150, -1);
    grid_box.append(&grid_size_label);
    let current_grid_size = app_state.project().settings.grid_size as f64;
    let grid_adj = Adjustment::new(current_grid_size, 1.0, 100.0, 1.0, 5.0, 0.0);
    let grid_spin = SpinButton::new(Some(&grid_adj), 1.0, 0);
    grid_box.append(&grid_spin);
    main_box.append(&grid_box);

    // Snap to grid checkbox
    let snap_grid_check = CheckButton::with_label("グリッドにスナップ");
    snap_grid_check.set_active(app_state.project().settings.snap_to_grid);
    main_box.append(&snap_grid_check);

    // Guide Settings Section
    let guide_label = Label::new(Some("ガイド設定"));
    guide_label.add_css_class("title-3");
    guide_label.set_halign(Align::Start);
    guide_label.set_margin_top(12);
    main_box.append(&guide_label);

    // Snap to guides checkbox
    let snap_guides_check = CheckButton::with_label("ガイドにスナップ");
    snap_guides_check.set_active(app_state.project().settings.snap_to_guides);
    main_box.append(&snap_guides_check);

    // Guide snap distance setting
    let snap_dist_box = GtkBox::new(Orientation::Horizontal, 8);
    snap_dist_box.set_homogeneous(false);
    let snap_dist_label = Label::new(Some("スナップ距離 (pt):"));
    snap_dist_label.set_size_request(150, -1);
    snap_dist_box.append(&snap_dist_label);
    let current_snap_dist = app_state.project().settings.snap_distance as f64;
    let snap_adj = Adjustment::new(current_snap_dist, 1.0, 50.0, 1.0, 5.0, 0.0);
    let snap_spin = SpinButton::new(Some(&snap_adj), 1.0, 0);
    snap_dist_box.append(&snap_spin);
    main_box.append(&snap_dist_box);

    // Autosave Settings Section
    let autosave_label = Label::new(Some("自動保存設定"));
    autosave_label.add_css_class("title-3");
    autosave_label.set_halign(Align::Start);
    autosave_label.set_margin_top(12);
    main_box.append(&autosave_label);

    // Autosave checkbox
    let autosave_check = CheckButton::with_label("自動保存を有効にする");
    autosave_check.set_active(app_state.project().settings.autosave_enabled);
    main_box.append(&autosave_check);

    // Autosave interval setting
    let autosave_box = GtkBox::new(Orientation::Horizontal, 8);
    autosave_box.set_homogeneous(false);
    let autosave_label = Label::new(Some("自動保存間隔 (分):"));
    autosave_label.set_size_request(150, -1);
    autosave_box.append(&autosave_label);
    let current_autosave = app_state.project().settings.autosave_minutes as f64;
    let autosave_adj = Adjustment::new(current_autosave, 1.0, 60.0, 1.0, 5.0, 0.0);
    let autosave_spin = SpinButton::new(Some(&autosave_adj), 1.0, 0);
    autosave_box.append(&autosave_spin);
    main_box.append(&autosave_box);

    // Add scrolled window for better layout with many settings
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_child(Some(&main_box));
    scrolled.set_vexpand(true);

    // Button box
    let button_box = GtkBox::new(Orientation::Horizontal, 6);
    button_box.set_halign(Align::End);
    button_box.set_homogeneous(true);
    button_box.set_margin_top(12);

    let save_btn = Button::with_label("保存");
    let dialog_ref = dialog.clone();
    let app_state_save = app_state.clone();
    save_btn.connect_clicked(move |_| {
        // Read values from UI controls
        let new_width = width_spin.value() as f32;
        let new_height = height_spin.value() as f32;
        let new_grid_size = grid_spin.value() as f32;
        let new_snap_grid = snap_grid_check.is_active();
        let new_snap_guides = snap_guides_check.is_active();
        let new_snap_dist = snap_spin.value() as f32;
        let new_autosave_enabled = autosave_check.is_active();
        let new_autosave_minutes = autosave_spin.value() as u32;

        // Update project settings
        if let Some(_) = app_state_save.with_active_document(|_doc| {
            let mut project = app_state_save.project();
            project.settings.default_canvas_width = new_width;
            project.settings.default_canvas_height = new_height;
            project.settings.grid_size = new_grid_size;
            project.settings.snap_to_grid = new_snap_grid;
            project.settings.snap_to_guides = new_snap_guides;
            project.settings.snap_distance = new_snap_dist;
            project.settings.autosave_enabled = new_autosave_enabled;
            project.settings.autosave_minutes = new_autosave_minutes;
        }) {
            tracing::info!("✅ Project settings saved successfully");
        }

        dialog_ref.close();
    });
    button_box.append(&save_btn);

    let close_btn = Button::with_label("キャンセル");
    let dialog_ref = dialog.clone();
    close_btn.connect_clicked(move |_| {
        dialog_ref.close();
    });
    button_box.append(&close_btn);

    let main_container = GtkBox::new(Orientation::Vertical, 0);
    main_container.append(&scrolled);
    main_container.append(&button_box);

    dialog.set_child(Some(&main_container));
    dialog.present();
}
