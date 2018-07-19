extern crate env_logger;
extern crate mdbook;
extern crate mdbook_mermaid;
extern crate clap;

use mdbook::MDBook;
use mdbook::errors::Result;
use mdbook_mermaid::Mermaid;
use clap::{App, ArgMatches};

use std::env;
use std::process;
use std::path::{Path, PathBuf};

fn get_book_dir(args: &ArgMatches) -> PathBuf {
    if let Some(dir) = args.value_of("dir") {
        // Check if path is relative from current dir, or absolute...
        let p = Path::new(dir);
        if p.is_relative() {
            env::current_dir().unwrap().join(dir)
        } else {
            p.to_path_buf()
        }
    } else {
        env::current_dir().expect("Unable to determine the current directory")
    }
}

pub fn make_app<'a, 'b>() -> App<'a, 'b> {
    App::new("mdbook-mermaid")
        .about("Build the book from the markdown files with mermaid support")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'The output directory for your book{n}(Defaults to ./book \
             when omitted)'",
        )
        .arg_from_usage(
            "[dir] 'A directory for your book{n}(Defaults to Current Directory when omitted)'",
        )
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(&book_dir)?;

    if let Some(dest_dir) = args.value_of("dest-dir") {
        book.config.build.build_dir = PathBuf::from(dest_dir);
    }

    book.with_preprecessor(Mermaid);
    book.build()?;

    Ok(())
}

fn main() {
    env_logger::init();
    let app = make_app();
    let matches = app.get_matches();

    if let Err(e) = execute(&matches) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
