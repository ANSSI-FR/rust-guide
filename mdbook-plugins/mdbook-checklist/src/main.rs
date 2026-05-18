#![forbid(unsafe_code)]

mod checklist;
mod checklist_pre;

use checklist_pre::ChecklistPre;

use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};

use std::io;
use std::process;

use clap::{Parser, Subcommand};
/// A mdbook preprocessor to generate checklists
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check whether a renderer is supported by this preprocessor
    Supports {
        /// Renderer name
        renderer: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let preprocessor = ChecklistPre;

    if let Some(Commands::Supports { renderer }) = &cli.command {
        handle_supports(&preprocessor, renderer);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{e:?}");
        process::exit(1);
    }
}

fn handle_supports(pre: &dyn Preprocessor, renderer: &str) -> ! {
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    eprintln!("{}: Running checklist preprocessor", pre.name());
    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
