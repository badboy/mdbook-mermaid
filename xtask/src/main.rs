// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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
    let payload_path = "payload/mermaid.min.js";

    let commit_msg = format!(
        "Upgrade to mermaid v{version}\n\nRelease: {release_url}\nAsset URL: {asset_url}\n\nSSR-only: Updated payload for headless Chrome rendering."
    );

    let sh = Shell::new()?;

    let asset_content = cmd!(sh, "curl {asset_url}").read()?;
    let full_content = format!("{LICENSE_HEADER}{asset_content}");

    // Write to payload/mermaid.min.js for SSR rendering in headless Chrome
    let mut fp = File::create(payload_path)?;
    write!(fp, "{full_content}")?;
    drop(fp);

    cmd!(sh, "git add payload").run()?;
    cmd!(sh, "git commit -m {commit_msg}").run()?;

    Ok(())
}
