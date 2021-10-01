mod args;
mod cargo_helpers;
mod chips;

use crate::cargo_helpers::*;
use crate::chips::Chip;
use anyhow::{Error, Result};
use duct::cmd;
use std::{path::PathBuf, str::FromStr};
use structopt::StructOpt;
use strum::EnumProperty;

fn main() -> Result<(), Error> {
    use args::Command::*;
    let args = args::Options::from_args();
    let chip = args.chip.unwrap_or(Chip::from_str("{{chip}}")?);
    println!();
    match args.cmd {
        ListChips => list_chips(),

        InstallDependencies => install_dependencies(&[chip]),

        BuildApp => build_target_crate("target-crates", "{{project-name}}", chip),
        FlashApp => flash_target_crate("target-crates", "{{project-name}}", chip),
        RunApp => run_target_crate("target-crates", "{{project-name}}", chip),
        GdbApp => debug_target_crate("target-crates", "{{project-name}}", chip),

        TestAll => {
            test_host(&["host-target-tests"])?;
            test_target("target-crates", "self-tests", chip)?;
            test_host_with_target("host-target-tests", "target-crates", "{{project-name}}", chip)
        }
        TestHost => test_host(&["host-target-tests"]),
        TestHostTarget => {
            test_host_with_target("host-target-tests", "target-crates", "{{project-name}}", chip)
        }
        TestTarget => test_target("target-crates", "self-tests", chip),
    }
}

fn install_dependencies(chips: &[Chip]) -> Result<()> {
    // Needed to run cargo size and other tools on the ARM binary
    println!("⏳ installing llvm-tools-preview");
    cmd!("rustup", "component", "add", "llvm-tools-preview").run()?;

    println!("⏳ installing cargo-binutils");
    cmd!("cargo", "install", "cargo-binutils").run()?;

    // Used for debugging, flashing and running with defmt output
    println!("⏳ installing cargo-embed");
    cmd!("cargo", "install", "cargo-embed").run()?;

    // Used for running tests on the target
    println!("⏳ installing probe-run");
    cmd!("cargo", "install", "probe-run").run()?;

    // Flip heap/stack allocation to deny stack usage growing into the heap
    println!("⏳ installing flip-link");
    cmd!("cargo", "install", "flip-link").run()?;

    // Supported targets
    for target in chips.iter().filter_map(|c| c.get_str("target")) {
        println!("⏳ adding target {}", target);
        cmd!("rustup", "target", "add", target).run()?;
    }

    Ok(())
}

fn host_workspace_dir() -> PathBuf {
    let mut xtask_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    xtask_dir.pop();
    xtask_dir
}
