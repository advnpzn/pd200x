/// Errors returned by all `PD200X` operations.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// No PD200X was found on any USB bus.
    #[error("PD200X not found (VID 0x352F PID 0x0104)")]
    DeviceNotFound,

    /// A low-level HID error from `hidapi`.
    #[error("HID error: {0}")]
    Hid(#[from] hidapi::HidError),

    /// The device did not respond within the I/O timeout (2 seconds).
    #[error("I/O timeout")]
    Timeout,

    /// The device returned a packet that failed magic-byte or checksum validation.
    #[error("unexpected response")]
    BadResponse,
}

pub type Result<T> = std::result::Result<T, Error>;
