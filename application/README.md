# Template for embedded application.

This template covers the setup of an entire embedded application, alongside with potential tests of host/target communication, host compilable and target specific code.

The setup contains 2 workspaces, an outer for code that can and may be tested on the host, and an inner that contains the target specific code.

The entire build-process utilizes [probe-rs], [cargo-embed] and [xtask].
Currently `xtask` supports these [Chips](./chips.txt), even if this template won't adjust used crates/hals to them all.

Big thanks to [ferrous-systems] for tools and ideas - They really do help make embedded rust great!

### Outer workspace

This workspace contains the general setup and all the crates that are not target specific - i.e. they can be compiled for, and unittested on, the host.

> NOTE: The `.cargo` folder holds the configuration needed for `cargo` to run the `xtask` commands.

> NOTE: `.vscode` holds task and launch configuration for use inside [vscode].

#### Crates

##### xtask

CLI tool responsible for running `xtask` commands via cargo.

The tool's subcommands can be executed by running:

```sh
cargo xtask <cmd>
```

For help, run:

```sh
cargo xtask --help
```

`xtask` will attempt to use the target chip specified during template expansion. This can be changed by giving the option `--chip <chipname>`, or fixing the default in `xtask/src/main.rs`.

##### messages

Example crate with definitions for messages that could potentially be exchanged between the host and the target.

The crate uses `postcard` for serialization, and `quicktest` in the unittests of the messages.

##### host-target-tests

Starting point for a test-suite used to test host/target communication.

### Inner workspace

Located in the `cross` folder, this workspace is for target specific code and files.

Chip and target information to the build-tools are provided by the `xtask` command system as CLI arguments and ENV variables .

> NOTE: `.cargo` folder holds e.g. linker specific configuration.

> NOTE: `*.svd` files may be placed here for use by the debugger.

> NOTE: `.embed.toml` is used to specify some configuration to the [probe-rs] and [cargo-embed] tools.

#### Crates

##### {{project-name}}

Target specific application.

Only contains example code for using the `defmt` log system.

##### board

Here to remind that board-specific code should be in its own crate!

##### self-tests

Example tests that needs to be run on the target.

[ferrous-systems]: https://ferrous-systems.com
[probe-rs]: https://github.com/probe-rs/probe-rs
[cargo-embed]: https://github.com/probe-rs/cargo-embed
[xtask]: https://github.com/matklad/cargo-xtask
[vscode]: https://code.visualstudio.com
