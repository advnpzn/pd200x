//! Hardware integration tests — require a PD200X connected over USB.
//!
//! Run with:
//!   cargo test --test hardware -- --ignored --nocapture --test-threads=1
//!
//! --test-threads=1 is required: only one test may hold the USB interface at a time.

use std::time::Duration;
use pd200x::{PD200X, RgbColor, EqBand, EqBandParams, FilterType};

async fn open() -> PD200X {
    PD200X::open().await.expect("PD200X not found — plug in the device and retry")
}

/// Enable mic monitor, mute for 500 ms then unmute, then disable monitor.
/// You should hear yourself in the headphones, then silence during the mute,
/// then yourself again, then silence once monitor is disabled.
#[tokio::test]
#[ignore = "requires PD200X"]
async fn test_mute_toggle() {
    let mic = open().await;

    println!("Enabling mic monitor…");
    mic.set_mic_monitor(true).await.expect("set_mic_monitor(true) failed");
    let monitor_state = mic.get_mic_monitor().await.expect("get_mic_monitor failed");
    let ng_state = mic.get_noise_gate().await.expect("get_noise_gate failed");
    println!("  mic_monitor={monitor_state}, noise_gate={ng_state}");
    tokio::time::sleep(Duration::from_secs(5)).await;

    println!("Muting…");
    mic.set_mute(true).await.expect("set_mute(true) failed");
    tokio::time::sleep(Duration::from_millis(500)).await;

    println!("Unmuting…");
    mic.set_mute(false).await.expect("set_mute(false) failed");
    tokio::time::sleep(Duration::from_millis(500)).await;

    println!("Disabling mic monitor…");
    mic.set_mic_monitor(false).await.expect("set_mic_monitor(false) failed");
    tokio::time::sleep(Duration::from_millis(200)).await;

    println!("Done.");
}

/// Read current gain, step it up by 20, then step it back down.
/// Physically confirms the gain knob value changes and is restored.
#[tokio::test]
#[ignore = "requires PD200X"]
async fn test_gain_increase_decrease() {
    let mic = open().await;

    let original = mic.get_mic_gain().await.expect("get_mic_gain failed");
    println!("Original gain: {original}");

    let higher = original.saturating_add(20).min(100);
    println!("Setting gain: {higher}");
    mic.set_mic_gain(higher).await.expect("set_mic_gain(higher) failed");
    tokio::time::sleep(Duration::from_millis(400)).await;

    let readback = mic.get_mic_gain().await.expect("get_mic_gain (after increase) failed");
    assert_eq!(readback, higher, "gain readback mismatch after increase");
    println!("Readback: {readback}");

    println!("Restoring gain: {original}");
    mic.set_mic_gain(original).await.expect("set_mic_gain(original) failed");
    tokio::time::sleep(Duration::from_millis(200)).await;

    let final_val = mic.get_mic_gain().await.expect("get_mic_gain (after restore) failed");
    assert_eq!(final_val, original, "gain not restored correctly");
    println!("Restored: {final_val}");
}

/// Verify every getter round-trips with its corresponding setter.
/// Reads the original value, sets a different one, asserts the getter reflects
/// the change, then restores the original.
#[tokio::test]
#[ignore = "requires PD200X"]
async fn test_getters() {
    let mic = open().await;

    // -- mute --
    let orig_mute = mic.get_mute().await.expect("get_mute failed");
    println!("mute: original={orig_mute}, setting={}", !orig_mute);
    mic.set_mute(!orig_mute).await.expect("set_mute failed");
    let readback_mute = mic.get_mute().await.expect("get_mute readback failed");
    println!("mute: readback={readback_mute}");
    assert_eq!(readback_mute, !orig_mute, "mute mismatch");
    mic.set_mute(orig_mute).await.expect("set_mute restore failed");
    println!("mute: restored={orig_mute}");

    // -- mic gain --
    let orig_gain = mic.get_mic_gain().await.expect("get_mic_gain failed");
    let new_gain = orig_gain.saturating_add(10).min(100);
    println!("mic gain: original={orig_gain}, setting={new_gain}");
    mic.set_mic_gain(new_gain).await.expect("set_mic_gain failed");
    let readback_gain = mic.get_mic_gain().await.expect("get_mic_gain readback failed");
    println!("mic gain: readback={readback_gain}");
    assert_eq!(readback_gain, new_gain, "mic gain mismatch");
    mic.set_mic_gain(orig_gain).await.expect("set_mic_gain restore failed");
    println!("mic gain: restored={orig_gain}");

    // -- mic monitor --
    let orig_monitor = mic.get_mic_monitor().await.expect("get_mic_monitor failed");
    println!("mic monitor: original={orig_monitor}, setting={}", !orig_monitor);
    mic.set_mic_monitor(!orig_monitor).await.expect("set_mic_monitor failed");
    let readback_monitor = mic.get_mic_monitor().await.expect("get_mic_monitor readback failed");
    println!("mic monitor: readback={readback_monitor}");
    assert_eq!(readback_monitor, !orig_monitor, "mic monitor mismatch");
    mic.set_mic_monitor(orig_monitor).await.expect("set_mic_monitor restore failed");
    println!("mic monitor: restored={orig_monitor}");

    // -- headphone volume --
    let orig_vol = mic.get_headphone_volume().await.expect("get_headphone_volume failed");
    let new_vol = orig_vol.saturating_add(10).min(100);
    println!("headphone volume: original={orig_vol}, setting={new_vol}");
    mic.set_headphone_volume(new_vol).await.expect("set_headphone_volume failed");
    let readback_vol = mic.get_headphone_volume().await.expect("get_headphone_volume readback failed");
    println!("headphone volume: readback={readback_vol}");
    assert_eq!(readback_vol, new_vol, "headphone volume mismatch");
    mic.set_headphone_volume(orig_vol).await.expect("set_headphone_volume restore failed");
    println!("headphone volume: restored={orig_vol}");

    // -- noise gate --
    let orig_ng = mic.get_noise_gate().await.expect("get_noise_gate failed");
    let new_ng = if orig_ng == 2500 { 2600 } else { 2500 };
    println!("noise gate: original={orig_ng}, setting={new_ng}");
    mic.set_noise_gate(new_ng).await.expect("set_noise_gate failed");
    let readback_ng = mic.get_noise_gate().await.expect("get_noise_gate readback failed");
    println!("noise gate: readback={readback_ng}");
    assert_eq!(readback_ng, new_ng, "noise gate mismatch");
    mic.set_noise_gate(orig_ng).await.expect("set_noise_gate restore failed");
    println!("noise gate: restored={orig_ng}");

    // -- noise cancellation --
    let orig_nc = mic.get_noise_cancellation().await.expect("get_noise_cancellation failed");
    println!("noise cancellation: original={orig_nc}, setting={}", !orig_nc);
    mic.set_noise_cancellation(!orig_nc).await.expect("set_noise_cancellation failed");
    let readback_nc = mic.get_noise_cancellation().await.expect("get_noise_cancellation readback failed");
    println!("noise cancellation: readback={readback_nc}");
    assert_eq!(readback_nc, !orig_nc, "noise cancellation mismatch");
    mic.set_noise_cancellation(orig_nc).await.expect("set_noise_cancellation restore failed");
    println!("noise cancellation: restored={orig_nc}");

    // -- NC strength --
    let orig_strength = mic.get_nc_strength().await.expect("get_nc_strength failed");
    let new_strength = if orig_strength >= 20 { orig_strength - 10 } else { orig_strength + 10 };
    println!("NC strength: original={orig_strength}, setting={new_strength}");
    mic.set_nc_strength(new_strength).await.expect("set_nc_strength failed");
    let readback_strength = mic.get_nc_strength().await.expect("get_nc_strength readback failed");
    println!("NC strength: readback={readback_strength}");
    assert_eq!(readback_strength, new_strength, "NC strength mismatch");
    mic.set_nc_strength(orig_strength).await.expect("set_nc_strength restore failed");
    println!("NC strength: restored={orig_strength}");
}

/// Apply three EQ presets in sequence — Flat, Low Cut, Bright — holding each
/// for 2 seconds so the effect is audible, then restore Flat.
#[tokio::test]
#[ignore = "requires PD200X"]
async fn test_eq_presets() {
    let mic = open().await;

    type Band = (EqBand, EqBandParams);

    let flat: [Band; 7] = [
        (EqBand::Band1, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 3000,  gain_db:  3.0, q: 1.39 }),
        (EqBand::Band2, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
        (EqBand::Band3, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
        (EqBand::Band4, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
        (EqBand::Band5, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
        (EqBand::Hpf,   EqBandParams { enabled: false, filter_type: FilterType::HighPass, frequency: 108,   gain_db:  0.0, q: 0.69 }),
        (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
    ];

    let low_cut: [Band; 7] = [
        (EqBand::Band1, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 3000,  gain_db:  3.0, q: 1.39 }),
        (EqBand::Band2, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
        (EqBand::Band3, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
        (EqBand::Band4, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
        (EqBand::Band5, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
        (EqBand::Hpf,   EqBandParams { enabled: true,  filter_type: FilterType::HighPass, frequency: 108,   gain_db:  0.0, q: 0.69 }),
        (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
    ];

    let bright: [Band; 7] = [
        (EqBand::Band1, EqBandParams { enabled: true,  filter_type: FilterType::LowShelf, frequency: 200,   gain_db: -5.0, q: 0.80 }),
        (EqBand::Band2, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 366,   gain_db: -2.0, q: 0.50 }),
        (EqBand::Band3, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 1.00 }),
        (EqBand::Band4, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
        (EqBand::Band5, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 10000, gain_db:  3.0, q: 0.50 }),
        (EqBand::Hpf,   EqBandParams { enabled: true,  filter_type: FilterType::HighPass, frequency: 110,   gain_db:  0.0, q: 1.00 }),
        (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
    ];

    mic.set_mic_monitor(false).await.expect("set_mic_monitor(false) reset failed");
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("Enabling mic monitor");
    mic.set_mic_monitor(true).await.expect("set_mic_monitor(true) failed");
    let monitor_on = mic.get_mic_monitor().await.expect("get_mic_monitor failed");
    let ng = mic.get_noise_gate().await.expect("get_noise_gate failed");
    println!("  mic_monitor={monitor_on}, noise_gate={ng}");
    tokio::time::sleep(Duration::from_secs(2)).await;

    let presets: [(&str, &[Band; 7]); 3] = [
        ("Flat",     &flat),
        ("Low Cut",  &low_cut),
        ("Bright",   &bright),
    ];

    for (name, preset) in &presets {
        println!("Preset: {name}");
        for (band, params) in preset.iter() {
            mic.set_eq_band(*band, params).await.expect("set_eq_band failed");
            println!("  {band:?}: enabled={}, freq={}Hz, gain={:.1}dB, q={:.2}",
                params.enabled, params.frequency, params.gain_db, params.q);
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    println!("Restoring flat preset");
    for (band, params) in &flat {
        mic.set_eq_band(*band, params).await.expect("set_eq_band restore failed");
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Disabling mic monitor");
    mic.set_mic_monitor(false).await.expect("set_mic_monitor(false) failed");
    println!("Done.");
}

/// Cycle the RGB strip through red → green → blue → rainbow, 400 ms each,
/// then restore white.
#[tokio::test]
#[ignore = "requires PD200X"]
async fn test_rgb_color_cycle() {
    let mic = open().await;

    // Make sure the strip is on and visible.
    mic.set_rgb_enabled(true).await.expect("set_rgb_enabled failed");
    mic.set_rgb_brightness(80).await.expect("set_rgb_brightness failed");

    for color in [RgbColor::Red, RgbColor::Green, RgbColor::Blue, RgbColor::Rainbow] {
        println!("Color: {color:?}");
        mic.set_rgb_color(color).await.expect("set_rgb_color failed");
        tokio::time::sleep(Duration::from_millis(400)).await;
    }

    println!("Restoring: White");
    mic.set_rgb_color(RgbColor::White).await.expect("set_rgb_color(White) failed");
}
