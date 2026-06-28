# pd200x

Async Rust driver for the **Maono PD200X** USB microphone.

Controls microphone gain, mute, EQ, noise cancellation, compressor, limiter,
and RGB lighting over USB HID — no official SDK required.

## Installation

```toml
[dependencies]
pd200x = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick start

```rust
use pd200x::PD200X;

#[tokio::main]
async fn main() -> pd200x::Result<()> {
    let mic = PD200X::open().await?;

    mic.set_mic_gain(80).await?;
    mic.set_mic_monitor(true).await?;
    mic.set_rgb_color(pd200x::RgbColor::Blue).await?;

    Ok(())
}
```

## Linux USB permissions

The kernel's HID driver holds the USB interface by default. Install the udev
rule so the library can claim it without root:

```bash
# copy the rule from the repository
sudo cp 99-pd200x.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

Then **unplug and replug** the microphone. The rule grants access to the
logged-in user automatically via `TAG+="uaccess"`.

## API overview

```rust
// Microphone
mic.set_mute(true).await?;
mic.set_mic_gain(80).await?;         // 0–100
mic.set_mic_monitor(true).await?;    // routes mic to headphone output

// Headphone & noise
mic.set_headphone_volume(70).await?; // 0–100
mic.set_noise_cancellation(true).await?;
mic.set_nc_strength(20).await?;      // 0–40

// Parametric EQ (7 bands: Band1–Band5, HPF, LPF)
use pd200x::{EqBand, EqBandParams, FilterType};
mic.set_eq_band(EqBand::Band1, &EqBandParams {
    enabled: true,
    filter_type: FilterType::Peaking,
    frequency: 120,
    gain_db: -3.0,
    q: 1.5,
}).await?;
mic.set_hpf(true, 80).await?;        // high-pass at 80 Hz
mic.set_lpf(false, 22000).await?;

// Dynamics
mic.set_compressor(true, 1800).await?;
mic.set_limiter(true, 1900).await?;

// RGB lighting
use pd200x::RgbColor;
mic.set_rgb_enabled(true).await?;
mic.set_rgb_brightness(80).await?;   // 0–100
mic.set_rgb_color(RgbColor::Cyan).await?;
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
| EQ bands (×5) | ✅ | — |
| HPF / LPF | ✅ | — |
| Compressor | ✅ | — |
| Limiter | ✅ | — |
| RGB enable | ✅ | — |
| RGB brightness | ✅ | — |
| RGB color | ✅ | — |
| Lighting linkage | ✅ | — |

## License

MIT — see [LICENSE](LICENSE).
