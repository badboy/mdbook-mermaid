use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::{Error, Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag};
use pulldown_cmark_to_cmark::{cmark_with_options, Options as COptions};

pub struct Mermaid;

impl Preprocessor for Mermaid {
    fn name(&self) -> &str {
        "mermaid"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(Mermaid::add_mermaid(chapter).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

fn escape_html(s: &str) -> String {
    let mut output = String::new();
    for c in s.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '&' => output.push_str("&amp;"),
            _ => output.push(c),
        }
    }
    output
}

fn add_mermaid(content: &str) -> Result<String> {
    let mut buf = String::with_capacity(content.len());
    let mut mermaid_content = String::new();
    let mut in_mermaid_block = false;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, opts);
    /* We only insert the top-level tags i.e when depth = 0 */
    let mut depth = 0;

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(Fenced(code))) => {
                if &*code == "mermaid" {
                    in_mermaid_block=true;
                    buf.push_str("<pre class=\"mermaid\">");
                } else {
                    buf.push_str(&content[range]);
                }
            }

            Event::End(Tag::CodeBlock(Fenced(code))) => {
                if in_mermaid_block{
                    assert_eq!(
                        "mermaid", &*code,
                        "After an opening mermaid code block we expect it to close again"
                    );
                    in_mermaid_block = false;
                    buf.push_str("</pre>\n\n");
                }
            }

            Event::Text(code) => {
                if in_mermaid_block{
                    buf.push_str(&escape_html(&code));
                }
            }

            Event::Start(_) => {
                if !in_mermaid_block && depth == 0{
                    buf.push_str(&content[range]);
                }
                depth += 1;
            }

            Event::End(_) => {
                depth -= 1;
            }
            
            _ => {}
        }
    }
    Ok(buf)
}

impl Mermaid {
    fn add_mermaid(chapter: &mut Chapter) -> Result<String> {
        add_mermaid(&chapter.content)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::add_mermaid;

    #[test]
    fn adds_mermaid() {
        let content = r#"# Chapter

```mermaid
graph TD
A --> B
```

Text
"#;

        let expected = r#"# Chapter

<pre class="mermaid">graph TD
A --&gt; B
</pre>


Text"#;

        assert_eq!(expected, add_mermaid(content).unwrap());
    }

    #[test]
    fn leaves_tables_untouched() {
        // Regression test.
        // Previously we forgot to enable the same markdwon extensions as mdbook itself.

        let content = r#"# Heading

| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

        // Markdown roundtripping removes some insignificant whitespace
        let expected = r#"# Heading

|Head 1|Head 2|
|------|------|
|Row 1|Row 2|"#;

        assert_eq!(expected, add_mermaid(content).unwrap());
    }

    #[test]
    fn leaves_html_untouched() {
        // Regression test.
        // Don't remove important newlines for syntax nested inside HTML

        let content = r#"# Heading

<del>

*foo*

</del>
"#;

        // Markdown roundtripping removes some insignificant whitespace
        let expected = r#"# Heading

<del>

*foo*

</del>
"#;

        assert_eq!(expected, add_mermaid(content).unwrap());
    }

    #[test]
    fn html_in_list() {
        // Regression test.
        // Don't remove important newlines for syntax nested inside HTML

        let content = r#"# Heading

1. paragraph 1
   ```
   code 1
   ```
2. paragraph 2
"#;

        // Markdown roundtripping removes some insignificant whitespace
        let expected = r#"# Heading

1. paragraph 1
   ````
   code 1
   ````
1. paragraph 2"#;

        assert_eq!(expected, add_mermaid(content).unwrap());
    }

    #[test]
    fn escape_in_mermaid_block() {
        let content = r#"
```mermaid
classDiagram
    class PingUploader {
        <<interface>>
        +Upload() UploadResult
    }
```

"#;

        let expected = r#"<pre class="mermaid">classDiagram
    class PingUploader {
        &lt;&lt;interface&gt;&gt;
        +Upload() UploadResult
    }
</pre>

"#;

        assert_eq!(expected, add_mermaid(content).unwrap());
    }
}
