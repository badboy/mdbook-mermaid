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

### From source

To install it from source:

```
cargo install mdbook-mermaid
```

This will build `mdbook-mermaid` from source.

### Using `cargo-binstall`

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) already:

```
cargo binstall mdbook-mermaid
```

This will download and install the pre-built binary for your system.

### Manually

Binary releases are available on the [Releases page](https://github.com/badboy/mdbook-mermaid/releases).
Download the relevant package for your system, unpack it, and move the `mdbook-mermaid` executable into `$HOME/.cargo/bin`:

## Configure your mdBook to use `mdbook-mermaid`

When adding `mdbook-mermaid` for the first time, let it add the required files and configuration:

```
mdbook-mermaid install path/to/your/book
```

This will add the following configuration to your `book.toml`:

```toml
[preprocessor.mermaid]
command = "mdbook-mermaid"

[output.html]
additional-js = ["mermaid.min.js", "mermaid-init.js"]
```

It will skip any unnecessary changes and detect if `mdbook-mermaid` was already configured.

Additionally it copies the files `mermaid.min.js` and  `mermaid-init.js` into your book's directory.
You find these files in the [`src/bin/assets`](src/bin/assets) directory.
You can modify `mermaid-init.js` to configure Mermaid, see the [Mermaid documentation] for all options.

[Mermaid documentation]: https://mermaid-js.github.io/mermaid/#/Setup?id=mermaidapi-configuration-defaults

Finally, build your book:

```
mdbook path/to/book
```

## Development

### Update the bundled mermaid.js

Find the latest version of `mermaid` on <https://github.com/mermaid-js/mermaid/releases>.
Then run:

```
cargo xtask <version>
```

This will fetch the minified mermaid.js file and commit it.

**Note:** `mdbook-mermaid` does NOT automatically update the `mermaid.min.js` file in your book. For that rerun

```
mdbook-mermaid install path/to/your/book
```

or manually replace the file.

## License

MPL. See [LICENSE](LICENSE).  
Copyright (c) 2018-2024 Jan-Erik Rediger <janerik@fnordig.de>

Mermaid is [MIT licensed](https://github.com/knsv/mermaid/blob/master/LICENSE).
The bundled assets (`mermaid.min.js`) are MIT licensed.
