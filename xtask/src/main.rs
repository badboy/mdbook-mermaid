use std::env;
use std::fs::File;
use std::io::Write;

use xshell::{Shell, cmd};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

const LICENSE_HEADER: &str = r#"/* MIT Licensed. Copyright (c) 2014 - 2022 Knut Sveidqvist */
/* For license information please see https://github.com/mermaid-js/mermaid/blob/develop/LICENSE */
"#;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let version = args.next().expect("Need mermaid.js version");

    let release_url =
        format!("https://github.com/mermaid-js/mermaid/releases/tag/mermaid%40{version}");
    let asset_url = format!("https://unpkg.com/mermaid@{version}/dist/mermaid.min.js");
    let asset_path = "src/bin/assets/mermaid.min.js";

    let commit_msg =
        format!("Upgrade to mermaid v{version}\n\nRelease: {release_url}\nAsset URL: {asset_url}");

    let sh = Shell::new()?;

    let mut fp = File::create(asset_path)?;
    let asset_content = cmd!(sh, "curl {asset_url}").read()?;

    write!(fp, "{LICENSE_HEADER}")?;
    write!(fp, "{asset_content}")?;
    drop(fp);

    cmd!(sh, "git add src/bin/assets").run()?;
    cmd!(sh, "git commit -m {commit_msg}").run()?;

    Ok(())
}
