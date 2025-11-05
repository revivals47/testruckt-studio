use clap::{Parser, Subcommand};
use testruct_core::{Project, Template};

#[derive(Parser)]
#[command(author, version, about = "Utilities for the Testruct desktop suite")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the GTK user interface
    Ui,
    /// List available templates from the default library
    Templates,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Ui => {
            #[cfg(feature = "ui")]
            {
                let exit = testruct_ui::launch(Default::default());
                std::process::exit(exit.value());
            }
            #[cfg(not(feature = "ui"))]
            {
                anyhow::bail!("UI feature not enabled");
            }
        }
        Commands::Templates => {
            let mut project = Project::default();
            project.templates.register(Template::single_page(
                "A4 Reading",
                testruct_core::CanvasLayout::new(testruct_core::Size::new(595.0, 842.0)),
            ));
            for template in project.templates.iter() {
                println!("{}", template.name);
            }
        }
    }
    Ok(())
}
