//! # pd200x
//!
//! Rust driver for the PD200X USB microphone (VID `0x352F`, PID `0x0104`).
//!
//! ## Usage
//!
//! ```rust,no_run
//! use pd200x::PD200X;
//!
//! fn main() -> pd200x::Result<()> {
//!     let mic = PD200X::open()?;
//!
//!     mic.set_mic_gain(80)?;
//!     mic.set_mic_monitor(true)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Linux USB permissions
//!
//! Copy `99-pd200x.rules` to `/etc/udev/rules.d/` and reload udev so the
//! device is accessible without root:
//!
//! ```bash
//! sudo cp 99-pd200x.rules /etc/udev/rules.d/
//! sudo udevadm control --reload-rules && sudo udevadm trigger
//! ```

mod api;
mod device;
mod error;
mod packet;
mod protocol;
pub mod presets;

pub use device::PD200X;
pub use error::{Error, Result};
pub use protocol::{EqBand, EqBandParams, FilterType, RgbColor};
