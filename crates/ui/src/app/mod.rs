mod actions;
mod state;

use crate::window::MainWindow;
use gtk4::{gio, glib, prelude::*, Application};

pub use state::AppState;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub application_id: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            application_id: "com.testruct.desktop".into(),
        }
    }
}

pub struct TestructApplication {
    app: Application,
}

impl TestructApplication {
    pub fn new(config: AppConfig) -> Self {
        // Use builder pattern for proper GTK4 initialization (works better on macOS)
        // Set HANDLES_OPEN flag to properly handle file open requests on macOS
        let app = gtk4::Application::builder()
            .application_id(&config.application_id)
            .flags(gio::ApplicationFlags::HANDLES_OPEN)
            .build();
        actions::register_global_actions(&app);
        Self { app }
    }

    pub fn run(self) -> glib::ExitCode {
        // Initialize logging
        tracing_subscriber::fmt::init();

        eprintln!("🚀 Starting GTK application...");
        eprintln!("ℹ️  Window should appear on your screen...");
        eprintln!("📌 Connecting signal handlers...");

        // On macOS, the app is opened via the open signal, not activate
        // We handle both signals to support different launching methods

        self.app.connect_activate(|app| {
            eprintln!("🎯 ACTIVATE SIGNAL FIRED!");
            let state = AppState::default();
            let window = MainWindow::build(app, state);
            window.present();
            eprintln!("✅ Window presented");
        });

        self.app.connect_open(|app, _files, _hint| {
            eprintln!("📂 OPEN SIGNAL FIRED! (macOS startup)");
            let state = AppState::default();
            let window = MainWindow::build(app, state);
            eprintln!("✅ Window created from open signal");
            window.present();
            eprintln!("✅ Window presented to screen");
        });

        eprintln!("🔄 Calling app.run()...");
        let exit_code = self.app.run();
        eprintln!("🛑 Application terminated with code: {:?}", exit_code);
        exit_code
    }
}
