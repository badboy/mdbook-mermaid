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
cargo install mdbook-mermaid
```

Add the following to your `book.toml`

```toml
[preprocessor.mermaid]
command = "mdbook-mermaid"
renderer = ["html"]

[output.html]
additional-css = ["mermaid.css"]
additional-js = ["mermaid.min.js", "mermaid-init.js"]
```

Copy the files (`mermaid.css`, `mermaid.min.js`, `mermaid-init.js`) from the [`assets/`](assets) directory into your source directory.

Finally, build your book:

```
mdbook path/to/book
```

## License

MPL. See [LICENSE](LICENSE).  
Copyright (c) 2018-2020 Jan-Erik Rediger <janerik@fnordig.de>

Mermaid is [MIT licensed](https://github.com/knsv/mermaid/blob/master/LICENSE).
The bundled assets (`mermaid.css`, `mermaid.min.js`) are MIT licensed.
