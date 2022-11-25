use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag};

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
        if let Event::Text(_) = e {
            if start_new_code_span {
                code_span = span;
                start_new_code_span = false;
            } else {
                code_span = code_span.start..span.end;
            }
            println!("{:?}", e);
            
            continue;
        }

        if let Event::End(Tag::CodeBlock(Fenced(code))) = e {
            assert_eq!(
                "mermaid", &*code,
                "After an opening mermaid code block we expect it to close again"
            );
            in_mermaid_block = false;

            let mermaid_content = &content[code_span.clone()];
            let mermaid_content = escape_html(mermaid_content);
            let mermaid_content = mermaid_content.replace("\r\n", "\n");
            let mermaid_code = format!("<pre class=\"mermaid\">{}</pre>\n\n", mermaid_content);
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



Text
"#;

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

        let expected = r#"# Heading

| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

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

        let expected = r#"# Heading

1. paragraph 1
   ```
   code 1
   ```
2. paragraph 2
"#;

        assert_eq!(expected, add_mermaid(content).unwrap());
    }

    #[test]
    fn escape_in_mermaid_block() {
        let _ = env_logger::try_init();
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

        let expected = r#"

<pre class="mermaid">classDiagram
    class PingUploader {
        &lt;&lt;interface&gt;&gt;
        +Upload() UploadResult
    }
</pre>



hello
"#;

        assert_eq!(expected, add_mermaid(content).unwrap());
    }

    #[test]
    fn more_backticks() {
        let _ = env_logger::try_init();
        let content = r#"# Chapter

````mermaid
graph TD
A --> B
````

Text
"#;

        let expected = r#"# Chapter


<pre class="mermaid">graph TD
A --&gt; B
</pre>



Text
"#;

        assert_eq!(expected, add_mermaid(content).unwrap());
    }

    #[test]
    fn crlf_line_endings() {
        let _ = env_logger::try_init();
        let content = "# Chapter\r\n\r\n````mermaid\r\n\r\ngraph TD\r\nA --> B\r\n````";
        let expected = "# Chapter\r\n\r\n\n<pre class=\"mermaid\">\ngraph TD\nA --&gt; B\n</pre>\n\n";

        assert_eq!(expected, add_mermaid(content).unwrap());
    }

}
