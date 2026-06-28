# pd200x

[![crates.io](https://img.shields.io/crates/v/pd200x.svg)](https://crates.io/crates/pd200x)
[![docs.rs](https://docs.rs/pd200x/badge.svg)](https://docs.rs/pd200x)
[![CI](https://github.com/advnpzn/pd200x/actions/workflows/ci.yml/badge.svg)](https://github.com/advnpzn/pd200x/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Rust driver for the **Maono PD200X** USB microphone.

Controls microphone gain, mute, EQ, noise cancellation, compressor, limiter,
and RGB lighting over USB HID - no official SDK required.

## Installation

```toml
[dependencies]
pd200x = "0.2"
```

## Quick start

```rust
use pd200x::PD200X;

fn main() -> pd200x::Result<()> {
    let mic = PD200X::open()?;

    mic.set_mic_gain(80)?;
    mic.set_mic_monitor(true)?;
    mic.set_rgb_color(pd200x::RgbColor::Blue)?;

    Ok(())
}
```

## Linux USB permissions

Install the udev rule so the device is accessible without root:

```bash
sudo cp 99-pd200x.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

Then **unplug and replug** the microphone. The rule grants access to the
logged-in user automatically via `TAG+="uaccess"`.

## API overview

```rust
// Microphone
mic.set_mute(true)?;
mic.set_mic_gain(80)?;          // 0–100
mic.set_mic_monitor(true)?;     // routes mic to headphone output

// Headphone & noise
mic.set_headphone_volume(70)?;  // 0–100
mic.set_noise_cancellation(true)?;
mic.set_nc_strength(20)?;       // 0–40

// EQ - named presets (reverse-engineered from MAONO Link)
use pd200x::presets;
mic.set_eq_preset(&presets::BRIGHT)?;
mic.set_eq_preset(&presets::DEEP)?;
// available: FLAT, LOW_CUT, MID_BOOST, LOW_CUT_MID_BOOST,
//            BRIGHT, NATURAL, CLASSIC, DEEP

// EQ - custom (full 7-band parametric control)
use pd200x::{EqBand, EqBandParams, FilterType};
mic.set_eq_preset(&[
    (EqBand::Band1, EqBandParams { enabled: true, filter_type: FilterType::Peaking,
                                   frequency: 120, gain_db: -3.0, q: 1.5 }),
    (EqBand::Hpf,   EqBandParams { enabled: true, filter_type: FilterType::HighPass,
                                   frequency: 80,  gain_db:  0.0, q: 1.0 }),
])?;

// EQ - single band
mic.set_eq_band(EqBand::Band1, &EqBandParams {
    enabled: true,
    filter_type: FilterType::Peaking,
    frequency: 120,
    gain_db: -3.0,
    q: 1.5,
})?;
mic.set_hpf(true, 80)?;         // high-pass at 80 Hz
mic.set_lpf(false, 22000)?;

// Dynamics
mic.set_compressor(true, 1800)?;
mic.set_limiter(true, 1900)?;

// RGB lighting
use pd200x::RgbColor;
mic.set_rgb_enabled(true)?;
mic.set_rgb_brightness(80)?;    // 0–100
mic.set_rgb_color(RgbColor::Cyan)?;
```

## Supported features

| Feature | Set | Get |
|---|---|---|
| Mute | ✅ | ✅ |
| Mic gain | ✅ | ✅ |
| Mic monitor | ✅ | ✅ |
| Headphone volume | ✅ | ✅ |
| Noise gate | ✅ | ✅ |
| Noise cancellation | ✅ | ✅ |
| NC strength | ✅ | ✅ |
| EQ presets (×8) | ✅ | - |
| EQ bands (×5) | ✅ | - |
| HPF / LPF | ✅ | - |
| Compressor | ✅ | - |
| Limiter | ✅ | - |
| RGB enable | ✅ | - |
| RGB brightness | ✅ | - |
| RGB color | ✅ | - |
| Lighting linkage | ✅ | - |

## License

see [LICENSE](LICENSE).
