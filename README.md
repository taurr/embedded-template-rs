# embedded-template-rs

This is a repository with templates for embedded projects in [Rust]. Each template is placed in its own subfolder, intended to be used with [cargo-generate].

> **NOTE:** *These templates need [cargo-generate] version 0.10.1 or newer.

## Templates

#### [application](./application/README.md).

Template for a comlete embedded application with test setup, flashing, debugging, [defmt] logging and [vscode] integration.

This template utilizes [probe-rs] and [cargo-embed] for flashing, running and testing.

[Rust]: https://www.rust-lang.org
[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[defmt]: https://ferrous-systems.com/blog/defmt/
[vscode]: https://code.visualstudio.com
[probe-rs]: https://github.com/probe-rs/probe-rs
[cargo-embed]: https://github.com/probe-rs/cargo-embed
