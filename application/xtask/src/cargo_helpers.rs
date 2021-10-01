use crate::{host_workspace_dir, Chip};
use anyhow::Result;
use console::style;
use duct::cmd;
use std::str::FromStr;
use strum::EnumProperty;

pub(crate) fn list_chips() -> Result<()> {
    println!(
        "This is a list of all the chips currently supported by {}/{}.",
        style("probe-rs").green(),
        style("cargo-embed").green()
    );
    println!("Chips marked with a 💀 are currently unsupported by this task!");
    println!();

    let output = cmd!("cargo", "embed", "--list-chips").read()?;
    let lines = output.lines();
    let mut group = None;
    for line in lines {
        if line.contains("Variants:") {
            continue;
        }
        if line.starts_with(' ') {
            if let Some(group) = group.take() {
                println!("{}", style(group).bold().yellow());
            }
            match Chip::from_str(line.trim()) {
                Ok(_) => println!("    {}", line.trim()),
                Err(_) => println!("    💀 {}", style(line.trim()).red()),
            }
            continue;
        }
        group = Some(line.trim().to_string());
    }

    Ok(())
}

pub(crate) fn test_host(excluded_crates: &[&str]) -> Result<()> {
    println!("🧪  {}", style("testing workspace crates on host").green());
    let mut args = vec!["test", "--workspace"];
    for crate_ in excluded_crates {
        args.push("--exclude");
        args.push(crate_);
    }
    args.push("--");
    args.push("--test-threads=1");
    args.push("--nocapture");

    cmd("cargo", args).dir(host_workspace_dir()).run()?;
    Ok(())
}

pub(crate) fn test_host_with_target(
    host_test_crate: &str,
    target_workspace_path: &str,
    target_bin_crate: &str,
    chip: Chip,
) -> Result<()> {
    flash_target_crate(target_workspace_path, target_bin_crate, chip)?;

    println!(
        "\n🧪  {} {}\n",
        style("testing target communication with host crate:").green(),
        style(host_test_crate).bold().yellow()
    );
    cmd!("cargo", "test", "-p", host_test_crate)
        .dir(host_workspace_dir())
        .run()?;

    Ok(())
}

pub(crate) fn test_target(
    target_workspace_path: &str,
    target_test_crate: &str,
    chip: Chip,
) -> Result<()> {
    println!(
        "🧪  {} {}",
        style("testing on target with crate:").green(),
        style(target_test_crate).bold().yellow(),
    );
    cargo_run_target(
        vec!["test", "--features", "defmt-trace", "-p", target_test_crate],
        target_workspace_path,
        chip,
    )
}

pub(crate) fn build_target_crate(
    target_workspace_path: &str,
    bin_crate: &str,
    chip: Chip,
) -> Result<()> {
    println!(
        "⏳  {} {}",
        style("building target:").green(),
        style(bin_crate).bold().yellow()
    );
    cargo_run_target(
        vec!["build", "--release", "--bin", bin_crate],
        target_workspace_path,
        chip,
    )?;
    cargo_run_target(
        vec!["size", "--release", "--bin", bin_crate],
        target_workspace_path,
        chip,
    )
}

pub(crate) fn flash_target_crate(
    target_workspace_path: &str,
    bin_crate: &str,
    chip: Chip,
) -> Result<()> {
    println!(
        "✈️  {} {}",
        style("flashing target with:").green(),
        style(bin_crate).bold().yellow()
    );
    cargo_run_target(
        vec!["embed", "--release", "--bin", bin_crate],
        target_workspace_path,
        chip,
    )
}

pub(crate) fn run_target_crate(
    target_workspace_path: &str,
    bin_crate: &str,
    chip: Chip,
) -> Result<()> {
    println!(
        "🚗  {} {}",
        style("flashing/running target with:").green(),
        style(bin_crate).bold().yellow(),
    );
    cargo_run_target(
        vec!["embed", "run", "--release", "--bin", bin_crate],
        target_workspace_path,
        chip,
    )
}

pub(crate) fn debug_target_crate(
    target_workspace_path: &str,
    bin_crate: &str,
    chip: Chip,
) -> Result<()> {
    println!(
        "🐞  {} {}",
        style("flashing target & launching GDB server with:").green(),
        style(bin_crate).bold().yellow(),
    );
    println!(
        "👀  {} {}",
        style("for specific settings, see:"),
        style(".embed.toml").bold().yellow(),
    );
    cargo_run_target(
        vec!["embed", "debug", "--bin", bin_crate],
        target_workspace_path,
        chip,
    )
}

fn cargo_run_target(args: Vec<&str>, target_workspace_path: &str, chip: Chip) -> Result<()> {
    let target = chip
        .get_str("target")
        .ok_or_else(|| anyhow::anyhow!("Target not specified for selected chip!"))?;
    let mut args: Vec<String> = args.iter().map(|&s| s.to_owned()).collect();

    println!("👀  {} {}", style("chip:"), style(chip).bold().yellow(),);
    println!("👀  {} {}", style("target:"), style(target).bold().yellow(),);

    let runner_env = format!(
        "CARGO_TARGET_{}_RUNNER",
        target.to_uppercase().replace("-", "_")
    );
    let runner_val = format!("probe-run --chip {}", chip);

    if args[0] == "embed" {
        args.push("--chip".to_string());
        args.push(chip.to_string());
    }

    println!();
    cmd("cargo", args)
        .env(runner_env, runner_val)
        .env("CARGO_BUILD_TARGET", target)
        .dir(target_workspace_path)
        .run()?;

    Ok(())
}
