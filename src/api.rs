use crate::device::PD200X;
use crate::error::Result;
use crate::protocol::{cmd, EqBand, EqBandParams, RgbColor, encode_nc_strength};

impl PD200X {
    // ---- Microphone ----

    /// Mute or unmute the microphone.
    ///
    /// When muted the mic input is silenced at the firmware level regardless
    /// of the gain setting.
    pub async fn set_mute(&self, muted: bool) -> Result<()> {
        self.send_command(cmd::MIC_MUTE, muted as u16).await?;
        Ok(())
    }

    /// Returns `true` if the microphone is currently muted.
    pub async fn get_mute(&self) -> Result<bool> {
        Ok(self.query(cmd::MIC_MUTE).await? != 0)
    }

    /// Set the microphone input gain (0–100).
    pub async fn set_mic_gain(&self, gain: u8) -> Result<()> {
        self.send_command(cmd::MIC_GAIN, gain as u16).await?;
        Ok(())
    }

    /// Returns the current microphone input gain (0–100).
    pub async fn get_mic_gain(&self) -> Result<u8> {
        Ok(self.query(cmd::MIC_GAIN).await? as u8)
    }

    /// Enable or disable microphone monitoring.
    ///
    /// When enabled, the microphone input is routed directly to the headphone
    /// output with near-zero latency so you can hear yourself while speaking.
    ///
    /// Internally this writes two registers (`0x2031` and `0x2070`) with a
    /// short delay between them to give the device time to apply both settings.
    pub async fn set_mic_monitor(&self, enabled: bool) -> Result<()> {
        self.send_command(cmd::NOISE_GATE, if enabled { 2000 } else { 3000 }).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        self.send_command(cmd::MIC_MONITOR, !enabled as u16).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(())
    }

    /// Returns `true` if microphone monitoring is currently enabled.
    pub async fn get_mic_monitor(&self) -> Result<bool> {
        Ok(self.query(cmd::MIC_MONITOR).await? == 0)
    }

    // ---- Audio ----

    /// Set the headphone output volume (0–100).
    pub async fn set_headphone_volume(&self, volume: u8) -> Result<()> {
        self.send_command(cmd::HEADPHONE_VOL, volume as u16).await?;
        Ok(())
    }

    /// Returns the current headphone output volume (0–100).
    pub async fn get_headphone_volume(&self) -> Result<u8> {
        Ok(self.query(cmd::HEADPHONE_VOL).await? as u8)
    }

    /// Set the noise gate threshold (1200–3000).
    ///
    /// The noise gate silences the mic input when the signal falls below this
    /// threshold. Higher values mean a more aggressive gate.
    ///
    /// Note: this register is also written by [`set_mic_monitor`] —
    /// calling `set_noise_gate` while monitoring is enabled will change the
    /// threshold but leave monitoring unaffected.
    ///
    /// [`set_mic_monitor`]: PD200X::set_mic_monitor
    pub async fn set_noise_gate(&self, threshold: u16) -> Result<()> {
        self.send_command(cmd::NOISE_GATE, threshold).await?;
        Ok(())
    }

    /// Returns the current noise gate threshold (1200–3000).
    pub async fn get_noise_gate(&self) -> Result<u16> {
        self.query(cmd::NOISE_GATE).await
    }

    /// Enable or disable the AI noise cancellation.
    pub async fn set_noise_cancellation(&self, enabled: bool) -> Result<()> {
        self.send_command(cmd::NC_ENABLE, enabled as u16).await?;
        Ok(())
    }

    /// Returns `true` if noise cancellation is currently enabled.
    pub async fn get_noise_cancellation(&self) -> Result<bool> {
        Ok(self.query(cmd::NC_ENABLE).await? != 0)
    }

    /// Set the noise cancellation strength (0–40).
    ///
    /// Higher values apply stronger noise reduction. The value is encoded as
    /// `(strength + 120) × 10` before being sent to the firmware.
    pub async fn set_nc_strength(&self, strength: u8) -> Result<()> {
        self.send_command(cmd::NC_STRENGTH, encode_nc_strength(strength)).await?;
        Ok(())
    }

    /// Returns the current noise cancellation strength (0–40).
    pub async fn get_nc_strength(&self) -> Result<u8> {
        Ok((self.query(cmd::NC_STRENGTH).await? / 10).saturating_sub(120) as u8)
    }

    // ---- EQ ----

    /// Configure a parametric EQ band.
    ///
    /// All seven bands (`Band1`–`Band5`, `Hpf`, `Lpf`) are set in a single
    /// USB packet. Changes take effect immediately.
    ///
    /// For the `Hpf` and `Lpf` bands `gain_db` and `q` in [`EqBandParams`]
    /// are sent as firmware constants; use [`set_hpf`] / [`set_lpf`] for a
    /// simpler interface, or supply a full [`EqBandParams`] here for complete
    /// control.
    ///
    /// [`set_hpf`]: PD200X::set_hpf
    /// [`set_lpf`]: PD200X::set_lpf
    pub async fn set_eq_band(&self, band: EqBand, params: &EqBandParams) -> Result<()> {
        self.send_eq(band, params).await
    }

    /// Enable or disable the high-pass filter at the given cutoff frequency (Hz).
    ///
    /// Uses fixed Q = 1.0. For a custom Q use [`set_eq_band`] with
    /// [`EqBand::Hpf`] directly.
    ///
    /// [`set_eq_band`]: PD200X::set_eq_band
    pub async fn set_hpf(&self, enabled: bool, frequency: u16) -> Result<()> {
        use crate::protocol::FilterType;
        self.send_eq(EqBand::Hpf, &EqBandParams {
            enabled,
            filter_type: FilterType::HighPass,
            frequency,
            gain_db: 0.0,
            q: 1.0,
        })
        .await
    }

    /// Enable or disable the low-pass filter at the given cutoff frequency (Hz).
    ///
    /// Uses fixed Q = 1.0. For a custom Q use [`set_eq_band`] with
    /// [`EqBand::Lpf`] directly.
    ///
    /// [`set_eq_band`]: PD200X::set_eq_band
    pub async fn set_lpf(&self, enabled: bool, frequency: u16) -> Result<()> {
        use crate::protocol::FilterType;
        self.send_eq(EqBand::Lpf, &EqBandParams {
            enabled,
            filter_type: FilterType::LowPass,
            frequency,
            gain_db: 0.0,
            q: 1.0,
        })
        .await
    }

    // ---- Compressor ----

    /// Enable or disable the compressor and set its threshold.
    ///
    /// The compressor reduces the dynamic range of the microphone signal.
    /// `threshold` is a raw firmware value; higher values mean the compressor
    /// activates at a louder input level.
    pub async fn set_compressor(&self, enabled: bool, threshold: u16) -> Result<()> {
        self.send_command(cmd::COMPRESSOR_ENABLE, enabled as u16).await?;
        self.send_command(cmd::COMPRESSOR_THRESH, threshold).await?;
        Ok(())
    }

    // ---- Limiter ----

    /// Enable or disable the limiter and set its threshold.
    ///
    /// The limiter hard-clips the output above `threshold` to prevent
    /// clipping downstream. `threshold` is a raw firmware value.
    pub async fn set_limiter(&self, enabled: bool, threshold: u16) -> Result<()> {
        self.send_command(cmd::LIMITER_ENABLE, enabled as u16).await?;
        self.send_command(cmd::LIMITER_THRESH, threshold).await?;
        Ok(())
    }

    // ---- RGB Lighting ----

    /// Turn the RGB LED strip on or off.
    pub async fn set_rgb_enabled(&self, enabled: bool) -> Result<()> {
        self.send_command(cmd::RGB_ENABLE, enabled as u16).await?;
        Ok(())
    }

    /// Set the RGB LED strip brightness (0–100).
    pub async fn set_rgb_brightness(&self, brightness: u8) -> Result<()> {
        self.send_command(cmd::RGB_BRIGHTNESS, brightness as u16).await?;
        Ok(())
    }

    /// Set the RGB LED strip colour.
    pub async fn set_rgb_color(&self, color: RgbColor) -> Result<()> {
        self.send_command(cmd::RGB_COLOR, color as u16).await?;
        Ok(())
    }

    /// Link the RGB lighting to the mic activity (voice/mute state).
    ///
    /// When enabled the LED colour reflects whether the mic is active.
    pub async fn set_lighting_linkage(&self, enabled: bool) -> Result<()> {
        self.send_command(cmd::LIGHTING_LINKAGE, enabled as u16).await?;
        Ok(())
    }
}
