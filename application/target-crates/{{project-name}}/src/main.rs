#![no_std]
#![no_main]

use {{crate_name}} as _;
use messages as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::trace!("trace");
    defmt::info!("info");
    defmt::warn!("warn");
    defmt::debug!("debug");
    defmt::error!("error");
    {{crate_name}}::exit()
}
