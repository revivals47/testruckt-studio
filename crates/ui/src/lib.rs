//! GTK UI for the Testruct desktop rewrite.

pub mod app;
pub mod canvas;
pub mod clipboard;
pub mod dialogs;
pub mod error;
pub mod export;
pub mod io;
pub mod menu;
pub mod panels;
pub mod template_manager;
pub mod templates;
pub mod theme;
pub mod toolbar;
pub mod undo_redo;
pub mod window;

pub use app::{AppConfig, TestructApplication};
pub use error::{AppError, AppResult, ValidationError, ValidationResult};

pub fn launch(config: AppConfig) -> glib::ExitCode {
    TestructApplication::new(config).run()
}
