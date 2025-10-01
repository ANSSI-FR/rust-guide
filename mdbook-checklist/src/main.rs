#![forbid(unsafe_code)]

mod checklist;
mod checklist_pre;

use checklist_pre::ChecklistPre;

use clap::{Arg, ArgMatches, Command};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};

use std::io;
use std::process;

fn make_command() -> Command<'static> {
    Command::new("checklist-preprocessor")
        .about("A mdbook preprocessor to generate checklists")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() -> Result<(), Error> {
    let matches = make_command().get_matches();
    let preprocessor = ChecklistPre;

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args)
    } else {
        handle_preprocessing(&preprocessor)
    }
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> Result<(), Error> {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    process::exit(if supported { 0 } else { 1 });
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
