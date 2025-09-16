# v0.16.0 (2025-09-16)

* Update to mdbook v0.4.52

# v0.15.0 (2025-03-28)

* Upgrade to mermaid v11.2.0

# v0.14.1 (2025-01-05)

* Update dependencies
* Add aarch64-unknown-linux-musl as a build target (Thanks, @sekhat)

# v0.14.0 (2024-09-11)

* Upgrade to mermaid v11.2.0

# v0.13.0 (2023-12-13)

* Update dependencies
* Upgrade to mermaid v10.6.1

# v0.12.6 (2022-12-19)

* Fix issue with extracting the `dir` parameter on `mdbook-mermaid install`

# v0.12.5 (2022-12-15)

* Ensure the right features on dependencies are enabled

# v0.12.4 (2022-12-15)

* Update dependencies, including clap, to avoid breakage
* Upgrade to mermaid v9.2.2

# v0.12.3 (2022-12-06)

* Use Ubuntu 20.04 when building `x86_64-unknown-linux-gnu` to avoid newer glibc

# v0.12.2 (2022-11-29)

* BUGFIX: Handle CRLF line endings in code block correctly ([#27](https://github.com/badboy/mdbook-mermaid/pull/27)).
  Thanks, @pseiko1989.

# v0.12.1 (2022-10-18)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.12.0...v0.12.1)

* BUGFIX: Handle arbitrary code span starts

# v0.12.0 (2022-10-11)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.11.2...v0.12.0)

* Bump to mdbook 0.4.21
* Dependency updates

# v0.11.2 (2022-07-29)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.11.1...v0.11.2)

* Upgrade to mermaid v9.1.3

# v0.11.1 (2022-07-16)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.11.0...v0.11.1)

* Reduce dependency tree by disabling default features of direct dependencies

# v0.11.0 (2022-05-26)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.10.0...v0.11.0)

* Upgrade to mermaid v9.1.1
* Upgrade dependencies

# v0.10.0 (2022-02-07)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.9.0...v0.10.0)

* Upgrade to mermaid v8.13.10

# v0.9.0 (2022-01-26)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.8.3...v0.9.0)

* Avoid roundtripping through pulldown-cmark.
  Should make the produced markdown much more consistent.

# v0.8.3 (2021-06-11)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.8.2...v0.8.3)

* Bump to mdbook v0.4.10

# v0.8.2 (2021-06-11)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.8.1...v0.8.2)

* Bump to mdbook v0.4.9

# v0.8.1 (2021-04-06)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.8.0...v0.8.1)

* Add tests to ensure `mdbook-mermaid install` works correctly

# v0.8.0 (2021-02-09)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.7.1...v0.8.0)

* Upgrade to mermaid v8.9.0
* Ensure additional files are added on `mdbook-mermaid install`

# v0.7.1 (2021-01-07)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.7.0...v0.7.1)

* Fix Windows release assets

# v0.7.0 (2021-01-06)

[Full changelog](https://github.com/badboy/mdbook-mermaid/compare/v0.6.1...v0.7.0)

* Upgrade to mermaid v8.8.4
* Build against mdbook v0.4.5
