pub mod cmd {
    pub const MIC_MUTE: u16 = 0x2022;
    pub const MIC_GAIN: u16 = 0x2023;

    pub const COMPRESSOR_ENABLE: u16 = 0x2026;
    pub const COMPRESSOR_THRESH: u16 = 0x2027;

    pub const LIMITER_ENABLE: u16 = 0x202C;
    pub const LIMITER_THRESH: u16 = 0x202D;

    pub const HEADPHONE_VOL: u16 = 0x2030;
    pub const NOISE_GATE: u16 = 0x2031;

    pub const RGB_ENABLE: u16 = 0x2036;
    pub const RGB_BRIGHTNESS: u16 = 0x2037;
    pub const RGB_COLOR: u16 = 0x2038;
    pub const LIGHTING_LINKAGE: u16 = 0x2039;

    pub const NC_ENABLE: u16 = 0x203A;
    pub const NC_STRENGTH: u16 = 0x203B;

    // EQ band base offsets (stride 5: enable, type, freq, gain, q)
    pub const BAND1_BASE: u16 = 0x204D;
    pub const BAND2_BASE: u16 = 0x2052;
    pub const BAND3_BASE: u16 = 0x2057;
    pub const BAND4_BASE: u16 = 0x205C;
    pub const BAND5_BASE: u16 = 0x2061;

    // HPF: enable, type, freq, const(2000), const(100)
    pub const HPF_BASE: u16 = 0x2066;
    // LPF: enable, type, freq, const(2000), const(100)
    pub const LPF_BASE: u16 = 0x206B;

    pub const MIC_MONITOR: u16 = 0x2070;
}

/// EQ filter shape.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    /// Bell/peaking EQ — boosts or cuts around a centre frequency.
    Peaking = 0,
    /// Low-pass filter — attenuates frequencies above the cutoff.
    LowPass = 1,
    /// High-pass filter — attenuates frequencies below the cutoff.
    HighPass = 2,
    /// Low-shelf filter — boosts or cuts all frequencies below the shelf frequency.
    LowShelf = 3,
    /// High-shelf filter — boosts or cuts all frequencies above the shelf frequency.
    HighShelf = 4,
}

/// RGB LED colour preset.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RgbColor {
    White = 0,
    Red = 1,
    Orange = 2,
    Yellow = 3,
    Green = 4,
    Cyan = 5,
    Blue = 6,
    Purple = 7,
    Rainbow = 8,
}

/// Selects one of the seven parametric EQ bands.
///
/// `Band1`–`Band5` are fully parametric (peaking or shelf). `Hpf` and `Lpf`
/// are fixed high-pass and low-pass filters whose gain and Q are sent as
/// firmware constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EqBand {
    Band1,
    Band2,
    Band3,
    Band4,
    Band5,
    /// High-pass filter.
    Hpf,
    /// Low-pass filter.
    Lpf,
}

impl EqBand {
    pub(crate) fn base_cmd(self) -> u16 {
        match self {
            EqBand::Band1 => cmd::BAND1_BASE,
            EqBand::Band2 => cmd::BAND2_BASE,
            EqBand::Band3 => cmd::BAND3_BASE,
            EqBand::Band4 => cmd::BAND4_BASE,
            EqBand::Band5 => cmd::BAND5_BASE,
            EqBand::Hpf => cmd::HPF_BASE,
            EqBand::Lpf => cmd::LPF_BASE,
        }
    }
}

/// Parameters for a single EQ band.
///
/// Pass this to `PD200X::set_eq_band` to configure any of the seven bands.
///
/// For `Hpf` and `Lpf` bands, `gain_db` and `q` are ignored by the firmware —
/// use `PD200X::set_hpf` / `PD200X::set_lpf` for those if you only need
/// to set the cutoff frequency, or supply `EqBandParams` directly via
/// `set_eq_band` for full control over the constants.
#[derive(Debug, Clone, Copy)]
pub struct EqBandParams {
    /// Whether this band is active.
    pub enabled: bool,
    /// Filter shape.
    pub filter_type: FilterType,
    /// Cutoff or centre frequency in Hz.
    pub frequency: u16,
    /// Gain in dB. Positive values boost, negative values cut.
    /// Only meaningful for `Peaking`, `LowShelf`, and `HighShelf` filters.
    pub gain_db: f32,
    /// Q factor (bandwidth). Higher values produce a narrower band.
    /// Only meaningful for `Peaking` filters; use `1.0` for shelf/HPF/LPF.
    pub q: f32,
}

// ---- encoding helpers (crate-private) ----

pub(crate) fn encode_gain(db: f32) -> u16 {
    (2000.0 + db * 10.0) as u16
}

pub(crate) fn encode_q(q: f32) -> u16 {
    (q * 100.0) as u16
}

/// Converts UI noise-cancellation strength (0–40) to firmware value.
pub(crate) fn encode_nc_strength(ui: u8) -> u16 {
    (ui as u16 + 120) * 10
}
