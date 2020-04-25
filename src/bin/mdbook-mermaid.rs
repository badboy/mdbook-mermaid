use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_mermaid::Mermaid;
use toml_edit::{Array, Document, Item, Table, Value};

use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    process,
};

const MERMAID_JS: &[u8] = include_bytes!("assets/mermaid.min.js");
const MERMAID_INIT_JS: &[u8] = include_bytes!("assets/mermaid-init.js");
const MERMAID_CSS: &[u8] = include_bytes!("assets/mermaid.css");
const MERMAID_FILES: &[(&str, &[u8])] = &[
    ("mermaid.min.js", MERMAID_JS),
    ("mermaid-init.js", MERMAID_INIT_JS),
    ("mermaid.css", MERMAID_CSS),
];

pub fn make_app() -> App<'static, 'static> {
    App::new("mdbook-mermaid")
        .version(crate_version!())
        .about("mdbook preprocessor to add mermaid support")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
            SubCommand::with_name("install")
                .arg(
                    Arg::with_name("dir")
                    .default_value(".")
                    .help("Root directory for the book,\nshould contain the configuration file (`book.toml`)")
                    )
                .about("Install the required assset files and include it in the config"),
        )
}

fn main() {
    env_logger::init();

    let matches = make_app().get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args);
    } else if let Some(sub_args) = matches.subcommand_matches("install") {
        handle_install(sub_args);
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
        eprintln!(
            "Warning: The mdbook-mermaid preprocessor was built against version \
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

fn handle_install(sub_args: &ArgMatches) -> ! {
    let dir = sub_args.value_of("dir").expect("Required argument");
    let proj_dir = PathBuf::from(dir);
    let config = proj_dir.join("book.toml");

    if !config.exists() {
        log::error!("Configuration file '{}' missing", config.display());
        process::exit(1);
    }

    log::info!("Reading configuration file {}", config.display());
    let toml = fs::read_to_string(&config).expect("can't read configuration file");
    let mut doc = toml
        .parse::<Document>()
        .expect("configuration is not valid TOML");

    log::info!("Adding additional files to configuration (if necessary)");
    if add_additional_files(&mut doc) {
        log::info!("Saving changed configuration to {}", config.display());
        let toml = doc.to_string_in_original_order();
        let mut file = File::create(config).expect("can't open configuration file for writing.");
        file.write_all(toml.as_bytes()).expect("can't write configuration");
    }

    log::info!("Writing additional files to project directory at {}", proj_dir.display());
    for (name, content) in MERMAID_FILES {
        let filepath = proj_dir.join(name);
        if filepath.exists() {
            log::debug!("'{}' already exists (Path: {}). Skipping.", name, filepath.display());
        } else {
            log::debug!("Writing content for '{}' into {}", name, filepath.display());
            let mut file = File::create(filepath).expect("can't open file for writing");
            file.write_all(content).expect("can't write content to file");
        }

    }

    log::info!("mermaid is active. You can start using it in your book.");

    process::exit(0);
}

fn add_additional_files(doc: &mut Document) -> bool {
    let mut changed = false;

    let file = "mermaid.css";
    let additional_css = additional(doc, "css");
    if has_file(&additional_css, file) {
        log::debug!("'{}' already in 'additional-css'. Skipping", file)
    } else {
        log::debug!("Adding '{}' to 'additional-css'", file);
        insert_additional(doc, "css", file);
        changed = true;
    }

    let file = "mermaid.min.js";
    let additional_js = additional(doc, "js");
    if has_file(&additional_js, file) {
        log::debug!("'{}' already in 'additional-js'. Skipping", file)
    } else {
        log::debug!("Adding '{}' to 'additional-js'", file);
        insert_additional(doc, "js", file);
        changed = true;
    }

    let file = "mermaid-init.js";
    let additional_js = additional(doc, "js");
    if has_file(&additional_js, file) {
        log::debug!("'{}' already in 'additional-js'. Skipping", file)
    } else {
        log::debug!("Adding '{}' to 'additional-js'", file);
        insert_additional(doc, "js", file);
        changed = true;
    }

    changed
}

fn additional<'a>(doc: &'a mut Document, additional_type: &str) -> Option<&'a mut Array> {
    let doc = doc.as_table_mut();

    let item = doc.entry("output");
    let item = item.as_table_mut()?.entry("html");
    let item = item
        .as_table_mut()?
        .entry(&format!("additional-{}", additional_type));
    item.as_array_mut()
}

fn has_file(elem: &Option<&mut Array>, file: &str) -> bool {
    match elem {
        Some(elem) => elem.iter().any(|elem| match elem.as_str() {
            None => true,
            Some(s) => s.ends_with(file),
        }),
        None => false,
    }
}

fn insert_additional(doc: &mut Document, additional_type: &str, file: &str) {
    let doc = doc.as_table_mut();

    let empty_table = Item::Table(Table::default());
    let empty_array = Item::Value(Value::Array(Array::default()));
    let item = doc.entry("output").or_insert(empty_table.clone());
    let item = item
        .as_table_mut()
        .unwrap()
        .entry("html")
        .or_insert(empty_table.clone());
    let array = item
        .as_table_mut()
        .unwrap()
        .entry(&format!("additional-{}", additional_type))
        .or_insert(empty_array);
    array
        .as_value_mut()
        .unwrap()
        .as_array_mut()
        .unwrap()
        .push(file);
}
