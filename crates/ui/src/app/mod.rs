mod state;
mod actions;

use gtk4::{gio, glib, prelude::*, Application};
use crate::window::MainWindow;

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
    config: AppConfig,
}

impl TestructApplication {
    pub fn new(config: AppConfig) -> Self {
        let app = Application::new(Some(&config.application_id), gio::ApplicationFlags::HANDLES_OPEN);
        actions::register_global_actions(&app);
        Self { app, config }
    }

    pub fn run(self) -> glib::ExitCode {
        let state = AppState::default();
        self.app.connect_activate(move |app| {
            let window = MainWindow::build(app, state.clone());
            window.present();
        });

        self.app.run()
    }
}
