// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod renderer;

use anyhow::Context;
use mdbook_preprocessor::book::{Book, BookItem, Chapter};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag, TagEnd};
use std::sync::Arc;

pub struct Mermaid {
    renderer: Arc<renderer::Mermaid>,
}

impl Mermaid {
    pub fn new() -> Result<Self> {
        let renderer = renderer::Mermaid::try_init()
            .context("Failed to initialize SSR renderer. Chrome/Chromium must be installed.")?;
        Ok(Self {
            renderer: Arc::new(renderer),
        })
    }
}

impl Preprocessor for Mermaid {
    fn name(&self) -> &str {
        "mermaid-ssr"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        log::info!("Rendering mermaid diagrams with SSR");

        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(Self::add_mermaid(chapter, &self.renderer).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer == "html")
    }
}

fn add_mermaid(content: &str, renderer: &Arc<renderer::Mermaid>) -> Result<String> {
    let mut mermaid_content = String::new();
    let mut in_mermaid_block = false;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut code_span = 0..0;
    let mut start_new_code_span = true;

    let mut mermaid_blocks = vec![];

    let events = Parser::new_ext(content, opts);
    for (e, span) in events.into_offset_iter() {
        log::debug!("e={:?}, span={:?}", e, span);
        if let Event::Start(Tag::CodeBlock(Fenced(code))) = e.clone() {
            if &*code == "mermaid" {
                in_mermaid_block = true;
                mermaid_content.clear();
            }
            continue;
        }

        if !in_mermaid_block {
            continue;
        }

        // We're in the code block. The text is what we want.
        // Code blocks can come in multiple text events.
        if let Event::Text(_) = e {
            if start_new_code_span {
                code_span = span;
                start_new_code_span = false;
            } else {
                code_span = code_span.start..span.end;
            }

            continue;
        }

        if let Event::End(TagEnd::CodeBlock) = e {
            in_mermaid_block = false;

            let mermaid_content = &content[code_span.clone()];

            // Render to SVG directly using SSR
            let mermaid_code = match renderer.render(mermaid_content) {
                Ok(svg) => {
                    log::debug!("Successfully rendered mermaid diagram to SVG");
                    format!("{}\n\n", svg)
                }
                Err(e) => {
                    log::error!(
                        "Failed to render mermaid diagram: {}. Content: {}",
                        e,
                        mermaid_content
                    );
                    // Return error as comment in HTML
                    format!(
                        "<!-- Mermaid rendering error: {} -->\n<pre class=\"mermaid-error\">Error rendering diagram</pre>\n\n",
                        e
                    )
                }
            };

            mermaid_blocks.push((span, mermaid_code));
            start_new_code_span = true;
        }
    }

    let mut content = content.to_string();
    for (span, block) in mermaid_blocks.iter().rev() {
        let pre_content = &content[0..span.start];
        let post_content = &content[span.end..];
        content = format!("{}\n{}{}", pre_content, block, post_content);
    }
    Ok(content)
}

impl Mermaid {
    fn add_mermaid(chapter: &mut Chapter, renderer: &Arc<renderer::Mermaid>) -> Result<String> {
        add_mermaid(&chapter.content, renderer)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    use super::{add_mermaid, renderer};

    #[test]
    fn adds_mermaid() {
        let mermaid = renderer::Mermaid::try_init().unwrap();
        let content = r#"# Chapter

```mermaid
graph TD
A --> B
```

Text
"#;

        let result = add_mermaid(content, &Arc::new(mermaid)).unwrap();

        // Check that SVG was generated
        assert!(result.contains("<svg"));
        assert!(result.contains("</svg>"));
        assert!(result.contains("# Chapter"));
        assert!(result.contains("Text"));
    }

    #[test]
    fn leaves_tables_untouched() {
        // Regression test.
        // Previously we forgot to enable the same markdwon extensions as mdbook itself.
        let mermaid = renderer::Mermaid::try_init().unwrap();

        let content = r#"# Heading

| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

        let expected = r#"# Heading

| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

        assert_eq!(expected, add_mermaid(content, &Arc::new(mermaid)).unwrap());
    }

    #[test]
    fn leaves_html_untouched() {
        // Regression test.
        // Don't remove important newlines for syntax nested inside HTML
        let mermaid = renderer::Mermaid::try_init().unwrap();

        let content = r#"# Heading

<del>

*foo*

</del>
"#;

        let expected = r#"# Heading

<del>

*foo*

</del>
"#;

        assert_eq!(expected, add_mermaid(content, &Arc::new(mermaid)).unwrap());
    }

    #[test]
    fn html_in_list() {
        // Regression test.
        // Don't remove important newlines for syntax nested inside HTML
        let mermaid = renderer::Mermaid::try_init().unwrap();

        let content = r#"# Heading

1. paragraph 1
   ```
   code 1
   ```
2. paragraph 2
"#;

        let expected = r#"# Heading

1. paragraph 1
   ```
   code 1
   ```
2. paragraph 2
"#;

        assert_eq!(expected, add_mermaid(content, &Arc::new(mermaid)).unwrap());
    }

    #[test]
    fn escape_in_mermaid_block() {
        let _ = env_logger::try_init();
        let mermaid = renderer::Mermaid::try_init().unwrap();
        let content = r#"
```mermaid
classDiagram
    class PingUploader {
        <<interface>>
        +Upload() UploadResult
    }
```

hello
"#;

        let result = add_mermaid(content, &Arc::new(mermaid)).unwrap();

        // Check that SVG was generated and contains the interface markers
        assert!(result.contains("<svg"));
        assert!(result.contains("</svg>"));
        assert!(result.contains("hello"));
    }

    #[test]
    fn more_backticks() {
        let _ = env_logger::try_init();
        let mermaid = renderer::Mermaid::try_init().unwrap();
        let content = r#"# Chapter

````mermaid
graph TD
A --> B
````

Text
"#;

        let result = add_mermaid(content, &Arc::new(mermaid)).unwrap();

        // Check that SVG was generated
        assert!(result.contains("<svg"));
        assert!(result.contains("</svg>"));
        assert!(result.contains("# Chapter"));
        assert!(result.contains("Text"));
    }

    #[test]
    fn crlf_line_endings() {
        let _ = env_logger::try_init();
        let mermaid = renderer::Mermaid::try_init().unwrap();
        let content = "# Chapter\r\n\r\n````mermaid\r\n\r\ngraph TD\r\nA --> B\r\n````";

        let result = add_mermaid(content, &Arc::new(mermaid)).unwrap();

        // Check that SVG was generated
        assert!(result.contains("<svg"));
        assert!(result.contains("</svg>"));
    }
}
