//! Official EQ presets from the MAONO Link app, reverse-engineered from USB captures.
//!
//! Each preset is a `[(EqBand, EqBandParams); 7]` covering all seven bands.
//! Apply one with [`PD200X::set_eq_preset`], or iterate the array yourself
//! to apply only the bands you care about.
//!
//! Custom EQ: build your own `EqBandParams` and call [`PD200X::set_eq_band`] directly.

use crate::protocol::{EqBand, EqBandParams, FilterType};

pub type Preset = [(EqBand, EqBandParams); 7];

/// All bands disabled. Stored register values are the factory defaults.
pub const FLAT: Preset = [
    (EqBand::Band1, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 3000,  gain_db:  3.0, q: 1.39 }),
    (EqBand::Band2, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band3, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band4, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band5, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Hpf,   EqBandParams { enabled: false, filter_type: FilterType::HighPass, frequency: 108,   gain_db:  0.0, q: 0.69 }),
    (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
];

/// High-pass filter at 108 Hz removes low-frequency rumble.
pub const LOW_CUT: Preset = [
    (EqBand::Band1, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 3000,  gain_db:  3.0, q: 1.39 }),
    (EqBand::Band2, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band3, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band4, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band5, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Hpf,   EqBandParams { enabled: true,  filter_type: FilterType::HighPass, frequency: 108,   gain_db:  0.0, q: 0.69 }),
    (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
];

/// +3 dB peaking boost centred at 3 kHz adds presence and clarity.
pub const MID_BOOST: Preset = [
    (EqBand::Band1, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 3000,  gain_db:  3.0, q: 1.39 }),
    (EqBand::Band2, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band3, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band4, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band5, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Hpf,   EqBandParams { enabled: false, filter_type: FilterType::HighPass, frequency: 108,   gain_db:  0.0, q: 0.69 }),
    (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
];

/// Combines [`LOW_CUT`] and [`MID_BOOST`]: HPF at 108 Hz plus +3 dB at 3 kHz.
pub const LOW_CUT_MID_BOOST: Preset = [
    (EqBand::Band1, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 3000,  gain_db:  3.0, q: 1.39 }),
    (EqBand::Band2, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band3, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band4, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 0.69 }),
    (EqBand::Band5, EqBandParams { enabled: false, filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Hpf,   EqBandParams { enabled: true,  filter_type: FilterType::HighPass, frequency: 108,   gain_db:  0.0, q: 0.69 }),
    (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
];

/// Thin, airy sound: low-shelf cut at 200 Hz, high-frequency boost, HPF at 110 Hz.
pub const BRIGHT: Preset = [
    (EqBand::Band1, EqBandParams { enabled: true,  filter_type: FilterType::LowShelf, frequency: 200,   gain_db: -5.0, q: 0.80 }),
    (EqBand::Band2, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 366,   gain_db: -2.0, q: 0.50 }),
    (EqBand::Band3, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Band4, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Band5, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 10000, gain_db:  3.0, q: 0.50 }),
    (EqBand::Hpf,   EqBandParams { enabled: true,  filter_type: FilterType::HighPass, frequency: 110,   gain_db:  0.0, q: 1.00 }),
    (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
];

/// Balanced, realistic vocal tone with gentle low cut and upper-mid lift.
pub const NATURAL: Preset = [
    (EqBand::Band1, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 120,   gain_db: -3.0, q: 1.50 }),
    (EqBand::Band2, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 500,   gain_db:  0.0, q: 1.00 }),
    (EqBand::Band3, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 1320,  gain_db:  2.5, q: 1.00 }),
    (EqBand::Band4, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 4310,  gain_db:  2.0, q: 1.50 }),
    (EqBand::Band5, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Hpf,   EqBandParams { enabled: true,  filter_type: FilterType::HighPass, frequency: 80,    gain_db:  0.0, q: 1.00 }),
    (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
];

/// Narrow low-mid cuts and upper-mid lift for a vintage broadcast character.
pub const CLASSIC: Preset = [
    (EqBand::Band1, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 400,   gain_db: -8.0, q: 3.00 }),
    (EqBand::Band2, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 250,   gain_db: -3.0, q: 1.50 }),
    (EqBand::Band3, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 1300,  gain_db:  2.5, q: 0.69 }),
    (EqBand::Band4, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 10000, gain_db:  0.0, q: 1.00 }),
    (EqBand::Band5, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Hpf,   EqBandParams { enabled: true,  filter_type: FilterType::HighPass, frequency: 50,    gain_db:  0.0, q: 1.00 }),
    (EqBand::Lpf,   EqBandParams { enabled: false, filter_type: FilterType::LowPass,  frequency: 22000, gain_db:  0.0, q: 1.00 }),
];

/// Bass-forward: low-end boost at 110 Hz, low-pass filter at 7 kHz rolls off highs.
pub const DEEP: Preset = [
    (EqBand::Band1, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 110,   gain_db:  3.0, q: 0.50 }),
    (EqBand::Band2, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 500,   gain_db:  0.0, q: 1.00 }),
    (EqBand::Band3, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 1000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Band4, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 2000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Band5, EqBandParams { enabled: true,  filter_type: FilterType::Peaking,  frequency: 5000,  gain_db:  0.0, q: 1.00 }),
    (EqBand::Hpf,   EqBandParams { enabled: false, filter_type: FilterType::HighPass, frequency: 10,    gain_db:  0.0, q: 1.00 }),
    (EqBand::Lpf,   EqBandParams { enabled: true,  filter_type: FilterType::LowPass,  frequency: 7000,  gain_db:  0.0, q: 1.00 }),
];
