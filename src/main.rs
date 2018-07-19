extern crate env_logger;
extern crate mdbook;
extern crate mdbook_mermaid;

use mdbook::MDBook;
use mdbook::errors::Result;
use mdbook_mermaid::Mermaid;

use std::ffi::OsString;
use std::env::{args, args_os};
use std::process;

fn do_it(book: OsString) -> Result<()> {
    let mut book = MDBook::load(book)?;
    book.with_preprecessor(Mermaid);
    book.build()
}

fn main() {
    env_logger::init();

    if args_os().count() != 2 {
        eprintln!("USAGE: {} <book>", args().next().expect("executable"));
        return;
    }
    if let Err(e) = do_it(args_os().skip(1).next().expect("one argument")) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
