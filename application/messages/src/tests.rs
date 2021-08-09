use crate::*;

/// Max payload size for a USB (2.0 Full Speed) HID packet
const MAX_SIZE: usize = 64;

#[test]
fn host2target_message_size() -> postcard::Result<()> {
    let msg = Host2Target::GetLastMeasurement;
    let bytes = postcard::to_allocvec(&msg)?;
    assert!(dbg!(bytes.len()) <= MAX_SIZE);
    Ok(())
}

#[cfg_attr(test, quickcheck_macros::quickcheck)]
fn target2host_measurement_message_size(id: u32, timestamp: u32, co2: f32) -> postcard::Result<()> {
    let msg = Target2Host::Measurement(Measurement { id, timestamp, co2 });
    let bytes = postcard::to_allocvec(&msg)?;
    assert!(dbg!(bytes.len()) <= MAX_SIZE);
    Ok(())
}
