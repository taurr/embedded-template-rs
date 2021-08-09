use crate::chips::Chip;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub(crate) struct Options {
    #[structopt(subcommand)]
    pub cmd: Command,

    #[structopt(long)]
    pub chip: Option<Chip>,
}

#[derive(Debug, StructOpt)]
//#[allow(clippy::enum_variant_names)]
pub(crate) enum Command {
    // List chips supported by cargo-embed, but nut yet by this xtask
    ListChips,

    /// Install needed dependencies
    InstallDependencies,

    /// Build release version of the target app
    BuildApp,
    /// Flash release version of the target app
    FlashApp,
    /// Run release version of the target app, attach RTT trace viewer
    RunApp,
    /// Launch gdb server with the target app
    GdbApp,

    TestAll,
    TestHost,
    TestHostTarget,
    TestTarget,
}
