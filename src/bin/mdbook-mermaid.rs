extern crate clap;
extern crate mdbook;
extern crate mdbook_mermaid;
extern crate serde_json;

use clap::{App, Arg, ArgMatches, SubCommand};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_mermaid::Mermaid;

use std::io;
use std::process;

pub fn make_app() -> App<'static, 'static> {
    App::new("mdbook-mermaid")
        .about("Build the book from the markdown files with mermaid support")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"))
}

fn main() {
    let matches = make_app().get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args);
    } else {
        if let Err(e) = handle_preprocessing() {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        eprintln!(
            "Warning: The mdbook-mermaid plugin was built against version \
             {} of mdbook, but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = Mermaid.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = Mermaid.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
