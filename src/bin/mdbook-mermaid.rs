extern crate clap;
extern crate mdbook;
extern crate mdbook_mermaid;
extern crate serde_json;

use clap::{App, ArgMatches};
use mdbook::errors::Error;
use mdbook::preprocessor::CmdPreprocessor;
use mdbook::MDBook;
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

    let result = match matches.subcommand_matches("supports") {
        Some(sub_args) => handle_supports(sub_args),
        None => handle_preprocessing(),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())
        .expect("Couldn't parse the input");

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        return Err(Error::from("The version check failed!"));
    }

    let processed_book = Mermaid.run(&ctx, &book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(sub_args: &ArgMatches) {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = Mermaid.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
