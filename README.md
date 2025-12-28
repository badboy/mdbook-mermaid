# mdbook-mermaid-ssr

A preprocessor for [mdbook][https://github.com/rust-lang-nursery/mdBook] to add [mermaid.js][https://mermaidjs.github.io/] support.

> [!IMPORTANT]
> `mdbook-mermaid-ssr` provides server-side rendering for Mermaid diagrams in mdBook.
Unlike the original `mdbook-mermaid` which uses client-side JavaScript rendering, `mdbook-mermaid-ss` pre-renders all diagrams to SVG during the build process using headless Chrome/firefox.
>
> This is not an upgrade/competition - it's a separate package with different requirements and behavior.
> 
> For the original client-side rendering version, see [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid).
>
> To migrate between the two, please apply this diff:
> ```diff
> + [preprocessor.mermaid-ssr]
> - [preprocessor.mermaid]
> - command = "mdbook-mermaid"
> - 
> - [output.html]
> - additional-js = ["mermaid.min.js", "mermaid-init.js"]
> ```

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

## Installation

### From source

To install it from source:

```
cargo install mdbook-mermaid-ssr
```

This will build `mdbook-mermaid-ssr` from source.

### Using `cargo-binstall`

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) already:

```
cargo binstall mdbook-mermaid-ssr
```

This will download and install the pre-built binary for your system.

### Manually

Binary releases are available on the Releases page.
Download the relevant package for your system, unpack it, and move the `mdbook-mermaid-ssr` executable into `$HOME/.cargo/bin`:

## Requirements

**Chrome or Chromium** must be installed on the system where you build your book.
The preprocessor uses this to render Mermaid diagrams to SVG.

## Configure your mdBook to use `mdbook-mermaid-ssr`

Add the following to your `book.toml`:

```toml
[preprocessor.mermaid-ssr]
```

That's it! No JavaScript files or additional configuration needed. Diagrams are pre-rendered to SVG during the build process.

Finally, build your book:

```
mdbook path/to/book
```

## How It Works

1. During the build process, `mdbook-mermaid-ssr` launches a headless Chrome browser
2. For each Mermaid code block, it renders the diagram to SVG
3. The SVG is embedded directly in the HTML output
4. No client-side JavaScript execution is needed when viewing the book

## Development

### Update the bundled mermaid.js

Find the latest version of `mermaid` on <https://github.com/mermaid-js/mermaid/releases>.
Then run:

```
cargo xtask <version>
```

This will fetch the minified mermaid.js file and commit it to the `payload/` directory for SSR rendering.

### Testing

Run the test suite:

```
cargo test
```

> [!NOTE]
> Tests require Chrome/Chromium to be installed.

## Troubleshooting

### "Failed to initialize SSR renderer"

This error means Chrome/Chromium could not be found or launched. Ensure Chrome or Chromium is installed and accessible in your system's PATH.

If you need client-side rendering instead, use the original [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid) package.

### Diagrams not rendering

1. Verify your Mermaid syntax is correct using the [Mermaid Live Editor](https://mermaid.live/)
2. Check the build output for error messages
3. Ensure Chrome/Chromium is installed

### Build takes a long time

Server-side rendering adds some overhead to the build process.
Each diagram must be rendered in headless Chrome.
For large books with many diagrams, this can take additional time.
This is a trade-off for the benefits of pre-rendered SVG output.

We have not currently explored parallelization or caching strategies to optimize build times.

## Migration from mdbook-mermaid

If you're migrating from the original `mdbook-mermaid` (client-side rendering):

1. Install `mdbook-mermaid-ssr`:
   ```bash
   cargo install mdbook-mermaid-ssr
   ```

2. Update your `book.toml`:
   ```toml
   [preprocessor.mermaid-ssr]
   ```

3. Remove client-side JavaScript configuration:
   ```toml
   # Remove these lines if present:
   [output.html]
   additional-js = ["mermaid.min.js", "mermaid-init.js"]
   ```

4. Delete old JavaScript files from your book directory:
   ```bash
   rm -f mermaid.min.js mermaid-init.js
   ```

5. Ensure Chrome/Chromium is installed on your build system

6. Rebuild your book:
   ```bash
   mdbook build
   ```

See [MIGRATION.md](MIGRATION.md) for detailed migration instructions.

## License

MPL. See [LICENSE](LICENSE).  
Copyright (c) 2018-2024 Jan-Erik Rediger <janerik@fnordig.de>

This is a fork focusing on server-side rendering only.

Mermaid is [MIT licensed](https://github.com/knsv/mermaid/blob/master/LICENSE).
The bundled assets (`payload/mermaid.min.js`) are MIT licensed.