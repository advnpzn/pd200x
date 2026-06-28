use std::time::Duration;

use nusb::transfer::RequestBuffer;
use tokio::time::timeout;

use crate::error::{Error, Result};
use crate::packet;
use crate::protocol::{EqBand, EqBandParams, FilterType, encode_gain, encode_q};

const VID: u16 = 0x352F;
const PID: u16 = 0x0104;
const QUERY_TIMEOUT: Duration = Duration::from_millis(2000);

/// Handle to an open PD200X microphone.
///
/// Obtain one with [`PD200X::open`]. All methods are `async` and require a
/// Tokio runtime. Only one `PD200X` instance may hold the USB interface at a
/// time — attempting to open a second will fail with [`Error::Usb`].
pub struct PD200X {
    interface: nusb::Interface,
    iface_num: u8,
    ep_in: u8,
    ep_out: Option<u8>,
}

impl PD200X {
    /// Open the first PD200X found on any USB bus.
    ///
    /// Returns [`Error::DeviceNotFound`] if no device is connected or the
    /// udev rule has not been installed (Linux).
    pub async fn open() -> Result<Self> {
        // nusb enumeration and interface claim are brief synchronous syscalls;
        // they do not block on I/O so calling them in async context is fine.
        let device_info = nusb::list_devices()?
            .find(|d| d.vendor_id() == VID && d.product_id() == PID)
            .ok_or(Error::DeviceNotFound)?;

        let device = device_info.open()?;
        let (iface_num, ep_in, ep_out) = find_hid_interface(&device)?;
        let interface = device.detach_and_claim_interface(iface_num)?;

        Ok(PD200X { interface, iface_num, ep_in, ep_out })
    }

    async fn send_raw(&self, data: [u8; 65]) -> Result<()> {
        if let Some(ep) = self.ep_out {
            self.interface
                .interrupt_out(ep, data.to_vec())
                .await
                .into_result()?;
        } else {
            use nusb::transfer::{ControlOut, ControlType, Recipient};
            self.interface
                .control_out(ControlOut {
                    control_type: ControlType::Class,
                    recipient: Recipient::Interface,
                    request: 0x09, // SET_REPORT
                    value: 0x0300, // report type Output (3), report ID 0
                    index: self.iface_num as u16,
                    data: &data,
                })
                .await
                .into_result()?;
        }
        Ok(())
    }

    // Read one raw 64-byte packet. No timeout — callers own the deadline.
    async fn recv_one(&self) -> Result<[u8; 64]> {
        let mut queue = self.interface.interrupt_in_queue(self.ep_in);
        queue.submit(RequestBuffer::new(64));
        let completion = queue.next_complete().await;
        completion.status.map_err(Error::Transfer)?;
        completion.data.try_into().map_err(|_| Error::BadResponse)
    }

    /// Send a raw SET command. The device applies it immediately with no ACK.
    pub async fn send_command(&self, cmd: u16, value: u16) -> Result<()> {
        self.send_raw(packet::build_command_packet(cmd, value)).await
    }

    /// Query the current value of a device register.
    ///
    /// Unsolicited packets (e.g. the level-meter stream on `0x2034`) are
    /// discarded until a packet whose command code matches `cmd` arrives.
    /// Returns [`Error::Timeout`] if no matching response arrives within 2 s.
    pub async fn query(&self, cmd: u16) -> Result<u16> {
        self.send_raw(packet::build_query_packet(cmd)).await?;
        timeout(QUERY_TIMEOUT, async {
            loop {
                let raw = self.recv_one().await?;
                if let Ok(resp) = packet::parse_response(&raw) {
                    if resp.command == cmd {
                        return Ok::<u16, Error>(resp.value);
                    }
                }
            }
        })
        .await
        .map_err(|_| Error::Timeout)?
    }

    pub(crate) async fn send_eq(&self, band: EqBand, params: &EqBandParams) -> Result<()> {
        self.send_raw(packet::build_eq_packet(eq_pairs(band, params))).await
    }
}

// Returns (interface_number, ep_in, ep_out) for the HID interface (class 0x03)
// that has an interrupt IN endpoint.
fn find_hid_interface(device: &nusb::Device) -> Result<(u8, u8, Option<u8>)> {
    let config = device.active_configuration().map_err(|_| Error::DeviceNotFound)?;

    for iface in config.interfaces() {
        for alt in iface.alt_settings() {
            if alt.class() != 0x03 {
                continue;
            }
            let mut ep_in = None;
            let mut ep_out = None;
            for ep in alt.endpoints() {
                use nusb::transfer::EndpointType;
                if ep.transfer_type() != EndpointType::Interrupt {
                    continue;
                }
                if ep.address() & 0x80 != 0 {
                    ep_in = Some(ep.address());
                } else {
                    ep_out = Some(ep.address());
                }
            }
            if let Some(ep_in) = ep_in {
                return Ok((iface.interface_number(), ep_in, ep_out));
            }
        }
    }

    Err(Error::DeviceNotFound)
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
