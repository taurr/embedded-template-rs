#![cfg_attr(not(test), no_std)]

use serde_derive::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// A message sent from the host to the target
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Host2Target {
    GetLastMeasurement,
}

/// A message sent from the target to the host
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Target2Host {
    NotReady,
    Measurement(Measurement),
}

/// A measurement reported by the target
#[derive(Clone, Copy, PartialEq, Debug, Deserialize, Serialize)]
pub struct Measurement {
    /// The measurement "identifier"
    /// this is a monotonically increasing counter
    pub id: u32,
    /// A timestamp in unspecified units
    /// it may wrap around
    pub timestamp: u32,
    /// The CO2 concentration in parts per million (ppm)
    pub co2: f32,
}
