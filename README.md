# embedded-template-rs

This is a repository with templates for embedded projects in [Rust]. Each template is placed in its own subfolder, intended to be used with [cargo-generate].

> **NOTE:** *These templates need [cargo-generate] version > 0.8*

> **KNOWN ISSUE:**
> These templates utilize [probe-rs] and [cargo-embed].
>
> When expanding through [cargo-generate] a `chip` needs to be selected. Unfortunately [cargo-generate] currently doesn't handle huge selection lists that well.
>
> *WORKAROUND:*
> To expand the template, use
> ```
> cargo generate --git https://github.com/taurr/embedded-template-rs --subfolder <template> -d chip=<chip>
> ```
> where `<chip>` is one of the supported chips (see [last checked supported chips](./chips.txt)).

## Templates

#### [application](./application/README.md).

Template for a comlete embedded application with test setup, flashing, debugging, [defmt] logging and [vscode] integration.

[Rust]: https://www.rust-lang.org
[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[defmt]: https://ferrous-systems.com/blog/defmt/
[vscode]: https://code.visualstudio.com
[probe-rs]: https://github.com/probe-rs/probe-rs
[cargo-embed]: https://github.com/probe-rs/cargo-embed
