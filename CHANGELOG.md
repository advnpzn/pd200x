# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-06-28

### Changed

- **Breaking**: replaced async `nusb` + `tokio` stack with synchronous `hidapi` - all methods are now plain `fn` instead of `async fn`, no runtime required
- Removed `tokio` dependency entirely; `hidapi` is the only USB dependency
- `Error` no longer wraps `nusb::Error` / `nusb::transfer::TransferError`; USB errors now surface as `Error::Hid(hidapi::HidError)`
- `PD200X::open()` is now synchronous and no longer calls `detach_and_claim_interface`; the kernel HID driver remains attached, allowing concurrent readers

### Added

- `pd200x::presets` module with 8 named EQ presets reverse-engineered from USB captures of the MAONO Link app: `FLAT`, `LOW_CUT`, `MID_BOOST`, `LOW_CUT_MID_BOOST`, `BRIGHT`, `NATURAL`, `CLASSIC`, `DEEP`
- `PD200X::set_eq_preset(bands: &[(EqBand, EqBandParams)])` - applies a full preset or any custom slice of bands in one call
- `presets::Preset` type alias for `[(EqBand, EqBandParams); 7]`

### Fixed

- Race condition in `query()`: `hidapi`'s `read_timeout` is called with the deadline already set before the query packet is sent, so device responses can never arrive before the read is armed

## [0.1.0] - 2026-06-28

### Added

- `PD200X::open()` - async USB device discovery and HID interface claim by VID `0x352F` / PID `0x0104`
- `send_command` / `query` primitives for raw register read/write over USB HID
- Parametric EQ control across 7 bands (Band1–Band5, HPF, LPF) via a single `set_eq_band` call
- High-level async API:
  - `set_mute` / `get_mute`
  - `set_mic_gain` / `get_mic_gain`
  - `set_mic_monitor` / `get_mic_monitor`
  - `set_headphone_volume` / `get_headphone_volume`
  - `set_noise_gate` / `get_noise_gate`
  - `set_noise_cancellation` / `get_noise_cancellation`
  - `set_nc_strength` / `get_nc_strength`
  - `set_eq_band`, `set_hpf`, `set_lpf`
  - `set_compressor`, `set_limiter`
  - `set_rgb_enabled`, `set_rgb_brightness`, `set_rgb_color`, `set_lighting_linkage`
- `RgbColor`, `EqBand`, `EqBandParams`, `FilterType` public types
- Automatic HID interface discovery by USB class code - no hardcoded interface numbers
- Fallback to HID `SET_REPORT` control transfer when no interrupt OUT endpoint is present
- Unsolicited packet filtering in `query` loop (discards continuous level-meter `0x2034` packets)
- udev rule (`99-pd200x.rules`) for rootless USB access on Linux
- Unit tests for packet builders, checksum roundtrips, and response parsing
- Hardware integration tests (require physical device, opt-in via `--ignored`)
- GitHub Actions CI (fmt, clippy, tests) and tag-triggered release pipeline

[0.1.0]: https://github.com/advnpzn/pd200x/releases/tag/v0.1.0
