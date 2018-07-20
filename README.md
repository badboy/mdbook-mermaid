# mdbook-mermaid

A preprocessor for [mdbook][] to add [mermaid.js][] support.

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[mermaid.js]: https://mermaidjs.github.io/

It turns this:

~~~
```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```
~~~

into this:

![Simple Graph](simple-graph.png)

in your book.
(Graph provided by [Mermaid Live Editor](https://mermaidjs.github.io/mermaid-live-editor/#/view/eyJjb2RlIjoiZ3JhcGggVEQ7XG4gICAgQS0tPkI7XG4gICAgQS0tPkM7XG4gICAgQi0tPkQ7XG4gICAgQy0tPkQ7IiwibWVybWFpZCI6eyJ0aGVtZSI6ImRlZmF1bHQifX0))

## Installation

If you want to use only this preprocessor, install the tool:

```
cargo install --git https://github.com/badboy/mdbook-mermaid
```

Add the following to your `book.toml`

```toml
[output.html]
additional-css = ["mermaid.css"]
additional-js = ["mermaid.min.js", "mermaid-init.js"]
```

Copy the files (`mermaid.css`, `mermaid.min.js`, `mermaid-init.js`) from the [`assets/`] (assets) directory into your source directory.

Finally, build your book:

```
mdbook-mermaid path/to/book
```

### Programmatic use

You can also use this programmatically, e.g. in order to use multiple additional preprocessors.
Add `mdbook-mermaid` as a dependency in your `Cargo.toml`:

```toml
[dependencies.mdbook-mermaid]
git = "https://github.com/badboy/mdbook-mermaid"
```

Then add it to your code:

```rust
extern crate mdbook_mermaid;

// ...

let mut book = MDBook::load(&book_dir)?;
book.with_preprecessor(mdbook_mermaid::Mermaid);
```

Don't forget to copy the files (`mermaid.css`, `mermaid.min.js`, `mermaid-init.js`) from the [`assets/`] (assets) directory into your source directory.

Add the following to your `book.toml` to include these files in your build:

```toml
[output.html]
additional-css = ["mermaid.css"]
additional-js = ["mermaid.min.js", "mermaid-init.js"]
```




## License

MPL. See [LICENSE](LICENSE).  
Copyright (c) 2018 Jan-Erik Rediger <janerik@fnordig.de>

Mermaid is [MIT licensed](https://github.com/knsv/mermaid/blob/master/LICENSE).
The bundled assets (`mermaid.css`, `mermaid.min.js`) are MIT licensed.
