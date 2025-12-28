use std::sync::Arc;

use anyhow::{bail, Result};
use escape_string::escape;
use headless_chrome::{Browser, Tab};
use unescape::unescape;

/// The Mermaid struct holds the embedded Chromium instance that is used to render Mermaid
/// diagrams
#[derive(Clone)]
pub struct Mermaid {
    _browser: Browser,
    tab: Arc<Tab>,
}

impl Mermaid {
    /// Initializes Mermaid
    pub fn try_init() -> Result<Self> {
        let browser = Browser::default()?;
        let mermaid_js = include_str!("../payload/mermaid.min.js");
        let html_payload = include_str!("../payload/index.html");

        let tab = browser.new_tab()?;
        tab.navigate_to(&format!("data:text/html;charset=utf-8,{}", html_payload))?;

        // Load mermaid library
        tab.evaluate(mermaid_js, false)?;

        // Initialize mermaid and set up render function in global scope
        let init_script = r#"
            mermaid.initialize({
                startOnLoad: false,
                theme: 'default',
                securityLevel: 'loose'
            });

            window.render = async function(code) {
                try {
                    const { svg } = await mermaid.render('mermaid-diagram-' + Date.now(), code);
                    return svg;
                } catch (error) {
                    console.error('Mermaid rendering error:', error);
                    return null;
                }
            };
        "#;
        tab.evaluate(init_script, false)?;

        Ok(Self {
            _browser: browser,
            tab,
        })
    }

    /// Renders a diagram
    ///
    /// # Example:
    /// ```no_run
    /// # use mdbook_mermaid_ssr::renderer::Mermaid;
    /// let mermaid = Mermaid::try_init().expect("Failed to initialize");
    /// let svg = mermaid.render("graph TB\na-->b").expect("Unable to render!");
    /// ```
    pub fn render(&self, input: &str) -> Result<String> {
        // Call the async render function and await its result
        let script = format!(
            "(async () => {{ return await window.render('{}'); }})()",
            escape(input)
        );
        let data = self.tab.evaluate(&script, true)?;
        let string = data.value.unwrap_or_default().to_string();
        let slice = unescape(string.trim_matches('"')).unwrap_or_default();

        if slice == "null" || slice.is_empty() {
            bail!("Failed to compile Mermaid diagram");
        }

        Ok(slice.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_mermaid_instance_without_crashing() {
        let mermaid = Mermaid::try_init();
        assert!(mermaid.is_ok());
    }

    #[test]
    fn render_mermaid() {
        let mermaid = Mermaid::try_init().unwrap();
        let rendered = mermaid.render("graph TB\na-->b");
        if let Err(ref e) = rendered {
            eprintln!("Render error: {}", e);
        }
        assert!(
            rendered.is_ok(),
            "Failed to render mermaid diagram: {:?}",
            rendered.err()
        );
        // TODO: Perform visual image comparison
        assert!(rendered.unwrap().starts_with("<svg"));
    }

    #[test]
    fn syntax_error() {
        let mermaid = Mermaid::try_init().unwrap();
        let rendered = mermaid.render("grph TB\na-->b");
        assert!(rendered.is_err());
    }
}
