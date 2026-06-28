/// Errors returned by all `PD200X` operations.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// No PD200X was found on any USB bus.
    #[error("PD200X not found (VID 0x352F PID 0x0104)")]
    DeviceNotFound,

    /// A low-level USB error from `nusb`.
    #[error("USB error: {0}")]
    Usb(#[from] nusb::Error),

    /// A USB transfer completed with an error status.
    #[error("transfer error: {0}")]
    Transfer(#[from] nusb::transfer::TransferError),

    /// The device did not respond within the I/O timeout (1 second).
    #[error("I/O timeout")]
    Timeout,

    /// The device returned a packet that failed magic-byte or checksum validation.
    #[error("unexpected response")]
    BadResponse,
}

pub type Result<T> = std::result::Result<T, Error>;
