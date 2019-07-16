extern crate mdbook;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::{Error, Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark_to_cmark::fmt::cmark;

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

fn add_mermaid(content: &str) -> Result<String> {
    let mut buf = String::with_capacity(content.len());
    let mut mermaid_content = String::new();
    let mut in_mermaid_block = false;
    let events = Parser::new(content).map(|e| {
        if let Event::Start(Tag::CodeBlock(code)) = e.clone() {
            if &*code == "mermaid" {
                in_mermaid_block = true;
                mermaid_content.clear();
                return None;
            } else {
                return Some(e);
            }
        }

        if !in_mermaid_block {
            return Some(e);
        }

        match e {
            Event::End(Tag::CodeBlock(code)) => {
                assert_eq!(
                    "mermaid", &*code,
                    "After an opening mermaid code block we expect it to close again"
                );
                in_mermaid_block = false;

                let mermaid_code = format!("<pre class=\"mermaid\">{}</pre>\n\n", mermaid_content);
                return Some(Event::Text(mermaid_code.into()));
            }
            Event::Text(code) => {
                mermaid_content.push_str(&code);
            }
            _ => return Some(e),
        }

        None
    });
    let events = events.filter_map(|e| e);
    cmark(events, &mut buf, None)
        .map(|_| buf)
        .map_err(|err| Error::from(format!("Markdown serialization failed: {}", err)))
}

impl Mermaid {
    fn add_mermaid(chapter: &mut Chapter) -> Result<String> {
        add_mermaid(&chapter.content)
    }
}

#[cfg(test)]
mod test {
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
A --> B
</pre>

Text"#;

        assert_eq!(expected, add_mermaid(content).unwrap());
    }
}
