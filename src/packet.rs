use crate::error::{Error, Result};

pub struct Response {
    pub command: u16,
    pub value: u16,
}

// checksum = -sum(packet[1..end]) & 0xFFFF
fn checksum(packet: &[u8], end: usize) -> u16 {
    let sum: u32 = packet[1..end].iter().map(|&b| b as u32).sum();
    ((!sum).wrapping_add(1)) as u16
}

pub fn build_command_packet(cmd: u16, value: u16) -> [u8; 65] {
    let mut p = [0u8; 65];
    p[0..6].copy_from_slice(&[0x4B, 0xC4, 0x0B, 0x00, 0x00, 0x03]);
    p[6..8].copy_from_slice(&cmd.to_le_bytes());
    p[8..10].copy_from_slice(&value.to_le_bytes());
    let cs = checksum(&p, 10);
    p[10..12].copy_from_slice(&cs.to_le_bytes());
    p
}

pub fn build_query_packet(cmd: u16) -> [u8; 65] {
    let mut p = [0u8; 65];
    p[0..6].copy_from_slice(&[0x4B, 0xC4, 0x09, 0x00, 0x00, 0x04]);
    p[6..8].copy_from_slice(&cmd.to_le_bytes());
    let cs = checksum(&p, 8);
    p[8..10].copy_from_slice(&cs.to_le_bytes());
    p
}

// pairs: [(cmd, value); 5] - one per EQ parameter slot
pub fn build_eq_packet(pairs: [(u16, u16); 5]) -> [u8; 65] {
    let mut p = [0u8; 65];
    p[0..6].copy_from_slice(&[0x4B, 0xC4, 0x1B, 0x00, 0x00, 0x03]);
    let mut offset = 6usize;
    for (cmd, value) in pairs {
        p[offset..offset + 2].copy_from_slice(&cmd.to_le_bytes());
        p[offset + 2..offset + 4].copy_from_slice(&value.to_le_bytes());
        offset += 4;
    }
    // checksum covers bytes 1..0x1A (exclusive), i.e. up to the last value field end
    let cs = checksum(&p, 0x1A);
    p[0x1A..0x1C].copy_from_slice(&cs.to_le_bytes());
    p
}

// Response packets from the device are 64 bytes (not 65 - the device does not
// echo back a report ID byte the way the host transmits one).
pub fn parse_response(raw: &[u8; 64]) -> Result<Response> {
    if raw[1] != 0xC4 {
        return Err(Error::BadResponse);
    }
    let command = u16::from_le_bytes([raw[6], raw[7]]);
    let value = u16::from_le_bytes([raw[8], raw[9]]);
    let stored_cs = u16::from_le_bytes([raw[10], raw[11]]);
    let computed_cs = checksum(raw, 10);
    if stored_cs != computed_cs {
        return Err(Error::BadResponse);
    }
    Ok(Response { command, value })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_packet_checksum_roundtrip() {
        let p = build_command_packet(0x2023, 50);
        assert_eq!(p[0], 0x4B);
        assert_eq!(p[1], 0xC4);
        assert_eq!(p[2], 0x0B);
        // verify checksum recomputation matches stored value
        let stored = u16::from_le_bytes([p[10], p[11]]);
        assert_eq!(stored, checksum(&p, 10));
    }

    #[test]
    fn query_packet_layout() {
        let p = build_query_packet(0x2023);
        assert_eq!(p[2], 0x09);
        assert_eq!(u16::from_le_bytes([p[6], p[7]]), 0x2023);
    }

    #[test]
    fn eq_packet_checksum_roundtrip() {
        let pairs = [
            (0x204D, 1u16),
            (0x204E, 0u16),
            (0x204F, 110u16),
            (0x2050, 2030u16),
            (0x2051, 50u16),
        ];
        let p = build_eq_packet(pairs);
        assert_eq!(p[2], 0x1B);
        let stored = u16::from_le_bytes([p[0x1A], p[0x1B]]);
        assert_eq!(stored, checksum(&p, 0x1A));
    }

    fn make_response(cmd: u16, value: u16) -> [u8; 64] {
        // Simulate a 64-byte device response with the same layout as a command packet.
        let mut raw = [0u8; 64];
        raw[0] = 0x4B;
        raw[1] = 0xC4;
        raw[6..8].copy_from_slice(&cmd.to_le_bytes());
        raw[8..10].copy_from_slice(&value.to_le_bytes());
        let cs = checksum(&raw, 10);
        raw[10..12].copy_from_slice(&cs.to_le_bytes());
        raw
    }

    #[test]
    fn parse_response_rejects_bad_magic() {
        let mut raw = [0u8; 64];
        raw[1] = 0xFF;
        assert!(parse_response(&raw).is_err());
    }

    #[test]
    fn parse_response_rejects_bad_checksum() {
        let mut raw = make_response(0x2023, 50);
        raw[10] ^= 0xFF;
        assert!(parse_response(&raw).is_err());
    }

    #[test]
    fn parse_response_ok() {
        let raw = make_response(0x2023, 50);
        let r = parse_response(&raw).unwrap();
        assert_eq!(r.value, 50);
    }
}
