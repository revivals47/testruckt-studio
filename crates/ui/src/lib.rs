//! GTK UI for the Testruct desktop rewrite.

pub mod app;
pub mod canvas;
pub mod dialogs;
pub mod io;
pub mod menu;
pub mod panels;
pub mod template_manager;
pub mod theme;
pub mod toolbar;
pub mod undo_redo;
pub mod window;

pub use app::{AppConfig, TestructApplication};

pub fn launch(config: AppConfig) -> glib::ExitCode {
    TestructApplication::new(config).run()
}
