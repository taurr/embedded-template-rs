#![no_std]

use defmt_rtt as _; // global logger
use panic_probe as _; // panic behaviour

{% case board %}
    {% when "Microbit" %}use microbit as _; // HW definitions
    {% when "Custom" %}TODO: use HAL as _;
{% endcase %}
// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

// NOTE: uncommenting these lines will make dfmt traces contain a 'timestamp'
//
// use core::sync::atomic::{AtomicUsize, Ordering};
// static COUNT: AtomicUsize = AtomicUsize::new(0);
// defmt::timestamp!("{=usize}", {
//     // NOTE(no-CAS) `timestamps` runs with interrupts disabled
//     let n = COUNT.load(Ordering::Relaxed);
//     COUNT.store(n + 1, Ordering::Relaxed);
//     n
// });

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
