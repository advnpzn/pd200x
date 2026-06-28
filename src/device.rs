use std::time::{Duration, Instant};

use hidapi::{HidApi, HidDevice};

use crate::error::{Error, Result};
use crate::packet;
use crate::protocol::{EqBand, EqBandParams, FilterType, encode_gain, encode_q};

const VID: u16 = 0x352F;
const PID: u16 = 0x0104;
const QUERY_TIMEOUT: Duration = Duration::from_millis(2000);

/// Handle to an open PD200X microphone.
///
/// Obtain one with [`PD200X::open`]. Only one `PD200X` instance may hold the
/// HID device at a time - attempting to open a second will fail with
/// [`Error::DeviceNotFound`].
pub struct PD200X {
    device: HidDevice,
}

impl PD200X {
    /// Open the first PD200X found on any USB bus.
    ///
    /// Returns [`Error::DeviceNotFound`] if no device is connected or the
    /// udev rule has not been installed (Linux).
    pub fn open() -> Result<Self> {
        let api = HidApi::new()?;
        let device = api.open(VID, PID).map_err(|_| Error::DeviceNotFound)?;
        Ok(PD200X { device })
    }

    fn send_raw(&self, data: [u8; 65]) -> Result<()> {
        self.device.write(&data)?;
        Ok(())
    }

    /// Send a raw SET command. The device applies it immediately with no ACK.
    pub fn send_command(&self, cmd: u16, value: u16) -> Result<()> {
        self.send_raw(packet::build_command_packet(cmd, value))
    }

    /// Query the current value of a device register.
    ///
    /// Unsolicited packets (e.g. the level-meter stream on `0x2034`) are
    /// discarded until a packet whose command code matches `cmd` arrives.
    /// Returns [`Error::Timeout`] if no matching response arrives within 2 s.
    pub fn query(&self, cmd: u16) -> Result<u16> {
        self.send_raw(packet::build_query_packet(cmd))?;
        let deadline = Instant::now() + QUERY_TIMEOUT;
        let mut buf = [0u8; 64];
        loop {
            let remaining_ms = deadline
                .saturating_duration_since(Instant::now())
                .as_millis();
            if remaining_ms == 0 {
                return Err(Error::Timeout);
            }
            let n = self.device.read_timeout(&mut buf, remaining_ms as i32)?;
            if n == 0 {
                return Err(Error::Timeout);
            }
            if let Ok(resp) = packet::parse_response(&buf) {
                if resp.command == cmd {
                    return Ok(resp.value);
                }
            }
        }
    }

    pub(crate) fn send_eq(&self, band: EqBand, params: &EqBandParams) -> Result<()> {
        self.send_raw(packet::build_eq_packet(eq_pairs(band, params)))
    }
}

fn eq_pairs(band: EqBand, params: &EqBandParams) -> [(u16, u16); 5] {
    let base = band.base_cmd();
    let enabled = params.enabled as u16;
    let freq = params.frequency;

    match band {
        EqBand::Hpf => [
            (base, enabled),
            (base + 1, FilterType::HighPass as u16),
            (base + 2, freq),
            (base + 3, 2000),
            (base + 4, 100),
        ],
        EqBand::Lpf => [
            (base, enabled),
            (base + 1, FilterType::LowPass as u16),
            (base + 2, freq),
            (base + 3, 2000),
            (base + 4, 100),
        ],
        _ => [
            (base, enabled),
            (base + 1, params.filter_type as u16),
            (base + 2, freq),
            (base + 3, encode_gain(params.gain_db)),
            (base + 4, encode_q(params.q)),
        ],
    }
}
