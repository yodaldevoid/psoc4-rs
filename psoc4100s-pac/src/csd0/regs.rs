#[doc = "ADC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCtl(pub u32);
impl AdcCtl {
    #[doc = "ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
    #[inline(always)]
    pub const fn adc_time(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
    #[inline(always)]
    pub fn set_adc_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub const fn adc_mode(&self) -> super::vals::AdcMode {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::AdcMode::from_bits(val as u8)
    }
    #[doc = "Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub fn set_adc_mode(&mut self, val: super::vals::AdcMode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for AdcCtl {
    #[inline(always)]
    fn default() -> AdcCtl {
        AdcCtl(0)
    }
}
#[doc = "ADC measurement"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcRes(pub u32);
impl AdcRes {
    #[doc = "Count to source/sink Cref1 + Cref2 from Vin to Vrefhi. This is also the current counter value for the HSCMP counter"]
    #[inline(always)]
    pub const fn vin_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Count to source/sink Cref1 + Cref2 from Vin to Vrefhi. This is also the current counter value for the HSCMP counter"]
    #[inline(always)]
    pub fn set_vin_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Polarity used for IDACB for this last ADC result, 0= source, 1= sink"]
    #[inline(always)]
    pub const fn hscmp_pol(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity used for IDACB for this last ADC result, 0= source, 1= sink"]
    #[inline(always)]
    pub fn set_hscmp_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This flag is set when the ADC counter overflows. This is an indication to the firmware that the IDACB current level is too low."]
    #[inline(always)]
    pub const fn adc_overflow(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This flag is set when the ADC counter overflows. This is an indication to the firmware that the IDACB current level is too low."]
    #[inline(always)]
    pub fn set_adc_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This flag is set when the ADC sequencer was aborted before tripping HSCMP."]
    #[inline(always)]
    pub const fn adc_abort(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This flag is set when the ADC sequencer was aborted before tripping HSCMP."]
    #[inline(always)]
    pub fn set_adc_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for AdcRes {
    #[inline(always)]
    fn default() -> AdcRes {
        AdcRes(0)
    }
}
#[doc = "Reference Generator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ambuf(pub u32);
impl Ambuf {
    #[doc = "Amux buffer power level"]
    #[inline(always)]
    pub const fn pwr_mode(&self) -> super::vals::PwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PwrMode::from_bits(val as u8)
    }
    #[doc = "Amux buffer power level"]
    #[inline(always)]
    pub fn set_pwr_mode(&mut self, val: super::vals::PwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Ambuf {
    #[inline(always)]
    fn default() -> Ambuf {
        Ambuf(0)
    }
}
#[doc = "Configuration and Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    pub const fn filter_delay(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    pub fn set_filter_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    pub const fn shield_delay(&self) -> super::vals::ShieldDelay {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::ShieldDelay::from_bits(val as u8)
    }
    #[doc = "Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    pub fn set_shield_delay(&mut self, val: super::vals::ShieldDelay) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    pub const fn sense_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    pub fn set_sense_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable charging of the Cmod/Csh_tank capacitor using the GPIO digital output buffer using the csd_charge signal. Note that using the GPIO requires proper configuraiton of the GPIO pin."]
    #[inline(always)]
    pub const fn charge_mode(&self) -> super::vals::ChargeMode {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::ChargeMode::from_bits(val as u8)
    }
    #[doc = "Enable charging of the Cmod/Csh_tank capacitor using the GPIO digital output buffer using the csd_charge signal. Note that using the GPIO requires proper configuraiton of the GPIO pin."]
    #[inline(always)]
    pub fn set_charge_mode(&mut self, val: super::vals::ChargeMode) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables mutual cap sensing mode"]
    #[inline(always)]
    pub const fn mutual_cap(&self) -> super::vals::MutualCap {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::MutualCap::from_bits(val as u8)
    }
    #[doc = "Enables mutual cap sensing mode"]
    #[inline(always)]
    pub fn set_mutual_cap(&mut self, val: super::vals::MutualCap) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    pub const fn csx_dual_cnt(&self) -> super::vals::CsxDualCnt {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::CsxDualCnt::from_bits(val as u8)
    }
    #[doc = "Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    pub fn set_csx_dual_cnt(&mut self, val: super::vals::CsxDualCnt) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Select what to output on the dsi_count bus."]
    #[inline(always)]
    pub const fn dsi_count_sel(&self) -> super::vals::DsiCountSel {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::DsiCountSel::from_bits(val as u8)
    }
    #[doc = "Select what to output on the dsi_count bus."]
    #[inline(always)]
    pub fn set_dsi_count_sel(&mut self, val: super::vals::DsiCountSel) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    pub const fn dsi_sample_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    pub fn set_dsi_sample_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    pub const fn sample_sync(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    pub fn set_sample_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    pub const fn dsi_sense_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    pub fn set_dsi_sense_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    pub const fn lp_mode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    pub fn set_lp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "CSD Comparator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csdcmp(pub u32);
impl Csdcmp {
    #[doc = "CSD Comparator Enable"]
    #[inline(always)]
    pub const fn csdcmp_en(&self) -> super::vals::CsdcmpEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsdcmpEn::from_bits(val as u8)
    }
    #[doc = "CSD Comparator Enable"]
    #[inline(always)]
    pub fn set_csdcmp_en(&mut self, val: super::vals::CsdcmpEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Select which IDAC polarity to use to detect CSDCMP triggering"]
    #[inline(always)]
    pub const fn polarity_sel(&self) -> super::vals::PolaritySel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PolaritySel::from_bits(val as u8)
    }
    #[doc = "Select which IDAC polarity to use to detect CSDCMP triggering"]
    #[inline(always)]
    pub fn set_polarity_sel(&mut self, val: super::vals::PolaritySel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Select in what phase(s) the comparator is active. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
    #[inline(always)]
    pub const fn cmp_phase(&self) -> super::vals::CmpPhase {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CmpPhase::from_bits(val as u8)
    }
    #[doc = "Select in what phase(s) the comparator is active. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
    #[inline(always)]
    pub fn set_cmp_phase(&mut self, val: super::vals::CmpPhase) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Select which signal to output on dsi_sample_out."]
    #[inline(always)]
    pub const fn cmp_mode(&self) -> super::vals::CmpMode {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::CmpMode::from_bits(val as u8)
    }
    #[doc = "Select which signal to output on dsi_sample_out."]
    #[inline(always)]
    pub fn set_cmp_mode(&mut self, val: super::vals::CmpMode) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
    #[inline(always)]
    pub const fn feedback_mode(&self) -> super::vals::FeedbackMode {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FeedbackMode::from_bits(val as u8)
    }
    #[doc = "This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
    #[inline(always)]
    pub fn set_feedback_mode(&mut self, val: super::vals::FeedbackMode) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub const fn az_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn set_az_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Csdcmp {
    #[inline(always)]
    fn default() -> Csdcmp {
        Csdcmp(0)
    }
}
#[doc = "High Speed Comparator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hscmp(pub u32);
impl Hscmp {
    #[doc = "High Speed Comparator enable"]
    #[inline(always)]
    pub const fn hscmp_en(&self) -> super::vals::HscmpEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HscmpEn::from_bits(val as u8)
    }
    #[doc = "High Speed Comparator enable"]
    #[inline(always)]
    pub fn set_hscmp_en(&mut self, val: super::vals::HscmpEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub const fn hscmp_invert(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the HSCMP output before it is used to control switches and the CSD sequencer. This bit does not affect the ADC sequencer or the STATUS.HSCMP_OUT"]
    #[inline(always)]
    pub fn set_hscmp_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub const fn az_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn set_az_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hscmp {
    #[inline(always)]
    fn default() -> Hscmp {
        Hscmp(0)
    }
}
#[doc = "IDACA Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idaca(pub u32);
impl Idaca {
    #[doc = "Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub const fn pol_dyn(&self) -> super::vals::IdacaPolDyn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::IdacaPolDyn::from_bits(val as u8)
    }
    #[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub fn set_pol_dyn(&mut self, val: super::vals::IdacaPolDyn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 & config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub const fn polarity(&self) -> super::vals::IdacaPolarity {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IdacaPolarity::from_bits(val as u8)
    }
    #[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 & config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub fn set_polarity(&mut self, val: super::vals::IdacaPolarity) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Balancing mode: only applies to legs configured as CSD."]
    #[inline(always)]
    pub const fn bal_mode(&self) -> super::vals::IdacaBalMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IdacaBalMode::from_bits(val as u8)
    }
    #[doc = "Balancing mode: only applies to legs configured as CSD."]
    #[inline(always)]
    pub fn set_bal_mode(&mut self, val: super::vals::IdacaBalMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub const fn leg1_mode(&self) -> super::vals::IdacaLeg1mode {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IdacaLeg1mode::from_bits(val as u8)
    }
    #[doc = "Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub fn set_leg1_mode(&mut self, val: super::vals::IdacaLeg1mode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Controls the usage mode of LEG2"]
    #[inline(always)]
    pub const fn leg2_mode(&self) -> super::vals::IdacaLeg2mode {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::IdacaLeg2mode::from_bits(val as u8)
    }
    #[doc = "Controls the usage mode of LEG2"]
    #[inline(always)]
    pub fn set_leg2_mode(&mut self, val: super::vals::IdacaLeg2mode) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
    #[inline(always)]
    pub const fn dsi_ctrl_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
    #[inline(always)]
    pub fn set_dsi_ctrl_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "IDAC multiplier"]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::IdacaRange {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::IdacaRange::from_bits(val as u8)
    }
    #[doc = "IDAC multiplier"]
    #[inline(always)]
    pub fn set_range(&mut self, val: super::vals::IdacaRange) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "output enable for leg 1 to CSDBUSA"]
    #[inline(always)]
    pub const fn leg1_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg 1 to CSDBUSA"]
    #[inline(always)]
    pub fn set_leg1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "output enable for leg 2 to CSDBUSA"]
    #[inline(always)]
    pub const fn leg2_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg 2 to CSDBUSA"]
    #[inline(always)]
    pub fn set_leg2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Idaca {
    #[inline(always)]
    fn default() -> Idaca {
        Idaca(0)
    }
}
#[doc = "IDACB Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idacb(pub u32);
impl Idacb {
    #[doc = "Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub const fn pol_dyn(&self) -> super::vals::IdacbPolDyn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::IdacbPolDyn::from_bits(val as u8)
    }
    #[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub fn set_pol_dyn(&mut self, val: super::vals::IdacbPolDyn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_EN==1 and LEG1_MODE==CSD also mixed with the CSD configuration and operation. In mutual cap mode however (see config.mutual_cap) the polarity of the IDAC is controlled by csd_sense. If LEG3_EN=1 (the other two legs must be off) then the ADC sequencer controls the IDACB polarity, optionally mixed with DSI."]
    #[inline(always)]
    pub const fn polarity(&self) -> super::vals::IdacbPolarity {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IdacbPolarity::from_bits(val as u8)
    }
    #[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_EN==1 and LEG1_MODE==CSD also mixed with the CSD configuration and operation. In mutual cap mode however (see config.mutual_cap) the polarity of the IDAC is controlled by csd_sense. If LEG3_EN=1 (the other two legs must be off) then the ADC sequencer controls the IDACB polarity, optionally mixed with DSI."]
    #[inline(always)]
    pub fn set_polarity(&mut self, val: super::vals::IdacbPolarity) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub const fn bal_mode(&self) -> super::vals::IdacbBalMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IdacbBalMode::from_bits(val as u8)
    }
    #[doc = "same as corresponding IDACA Balancing mode"]
    #[inline(always)]
    pub fn set_bal_mode(&mut self, val: super::vals::IdacbBalMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub const fn leg1_mode(&self) -> super::vals::IdacbLeg1mode {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IdacbLeg1mode::from_bits(val as u8)
    }
    #[doc = "Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub fn set_leg1_mode(&mut self, val: super::vals::IdacbLeg1mode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Controls the usage mode of LEG2"]
    #[inline(always)]
    pub const fn leg2_mode(&self) -> super::vals::IdacbLeg2mode {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::IdacbLeg2mode::from_bits(val as u8)
    }
    #[doc = "Controls the usage mode of LEG2"]
    #[inline(always)]
    pub fn set_leg2_mode(&mut self, val: super::vals::IdacbLeg2mode) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled) 0: no DSI control IDACB_POLARITY = IDACB.POLARITY IDACB_LEG1_EN = IDACB.LEG1_EN IDACB_LEG2_EN = IDACB.LEG2_EN IDACB_LEG3_EN = IDACB.LEG3_EN 1: Mix MMIO with DSI control IDACB_POLARITY = IDACB.POLARITY EXOR dsi_idacb_pol IDACB_LEG1_EN = IDACB.LEG1_EN AND dsi_idacb_leg1_en IDACB_LEG2_EN = IDACB.LEG2_EN AND dsi_idacb_leg2_en IDACB_LEG3_EN = IDACB.LEG3_EN AND dsi_idacb_leg3_en"]
    #[inline(always)]
    pub const fn dsi_ctrl_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled) 0: no DSI control IDACB_POLARITY = IDACB.POLARITY IDACB_LEG1_EN = IDACB.LEG1_EN IDACB_LEG2_EN = IDACB.LEG2_EN IDACB_LEG3_EN = IDACB.LEG3_EN 1: Mix MMIO with DSI control IDACB_POLARITY = IDACB.POLARITY EXOR dsi_idacb_pol IDACB_LEG1_EN = IDACB.LEG1_EN AND dsi_idacb_leg1_en IDACB_LEG2_EN = IDACB.LEG2_EN AND dsi_idacb_leg2_en IDACB_LEG3_EN = IDACB.LEG3_EN AND dsi_idacb_leg3_en"]
    #[inline(always)]
    pub fn set_dsi_ctrl_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "IDAC multiplier"]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::IdacbRange {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::IdacbRange::from_bits(val as u8)
    }
    #[doc = "IDAC multiplier"]
    #[inline(always)]
    pub fn set_range(&mut self, val: super::vals::IdacbRange) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "output enable for leg 1 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub const fn leg1_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg 1 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub fn set_leg1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "output enable for leg 2 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub const fn leg2_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg 2 to CSDBUSB or CSDBUSA"]
    #[inline(always)]
    pub fn set_leg2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "output enable for leg3 to CSDBUSC, only allowed when RANGE = IDAC_LO. When this bit is set both other legs should be off. Note that leg3 can only be used for ADC mode, not GP mode. Which means that leg3 can only be on when the ADC Sequencer is in the ADC_measure or Calib_measure state. In those states leg3 is controlled by the ADC configuration and the HSCMP output. In addition this leg3 enable bit can optionally be mixed with DSI (see DSI_CTRL_EN). When LEG3_EN=1 also the IDACB polarity is controlled by the ADC sequencer."]
    #[inline(always)]
    pub const fn leg3_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "output enable for leg3 to CSDBUSC, only allowed when RANGE = IDAC_LO. When this bit is set both other legs should be off. Note that leg3 can only be used for ADC mode, not GP mode. Which means that leg3 can only be on when the ADC Sequencer is in the ADC_measure or Calib_measure state. In those states leg3 is controlled by the ADC configuration and the HSCMP output. In addition this leg3 enable bit can optionally be mixed with DSI (see DSI_CTRL_EN). When LEG3_EN=1 also the IDACB polarity is controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn set_leg3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Idacb {
    #[inline(always)]
    fn default() -> Idacb {
        Idacb(0)
    }
}
#[doc = "CSD Interrupt Request Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "A normal sample is complete (CSDv1 compatible interrupt)"]
    #[inline(always)]
    pub const fn sample(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "A normal sample is complete (CSDv1 compatible interrupt)"]
    #[inline(always)]
    pub fn set_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC Result ready"]
    #[inline(always)]
    pub const fn adc_res(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Result ready"]
    #[inline(always)]
    pub fn set_adc_res(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "CSD Interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn sample(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn adc_res(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_adc_res(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "CSD Interrupt masked register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn sample(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn adc_res(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_adc_res(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "CSD Interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn sample(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn adc_res(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_adc_res(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Reference Generator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Refgen(pub u32);
impl Refgen {
    #[doc = "Reference Generator Enable"]
    #[inline(always)]
    pub const fn refgen_en(&self) -> super::vals::RefgenEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RefgenEn::from_bits(val as u8)
    }
    #[doc = "Reference Generator Enable"]
    #[inline(always)]
    pub fn set_refgen_en(&mut self, val: super::vals::RefgenEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass selected input reference unbuffered to Vrefhi"]
    #[inline(always)]
    pub fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub const fn vdda_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Close Vdda switch to top of resistor string (or Vrefhi?)"]
    #[inline(always)]
    pub fn set_vdda_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub const fn res_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Resistor string enable; 0= open switch on top of the resistor string (Vreflo=Vssa)"]
    #[inline(always)]
    pub fn set_res_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub const fn gain(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Select resistor string tap for feedback, 0= minimum vout, 31= maximum vout = vrefhi -> gain=1 (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn set_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub const fn vreflo_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Select resistor string tap for Vreflo/Vreflo_int, 0= minimum vout, 31= maximum vout = vrefhi (only works if the resistor string is enabled; RES_EN=1)"]
    #[inline(always)]
    pub fn set_vreflo_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    pub const fn vreflo_int(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Ouput the resistor string tap either to Vreflo (0) or Vreflo_int (1)."]
    #[inline(always)]
    pub fn set_vreflo_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Refgen {
    #[inline(always)]
    fn default() -> Refgen {
        Refgen(0)
    }
}
#[doc = "Result CSD/CSX accumulation counter value 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResultVal1(pub u32);
impl ResultVal1 {
    #[doc = "Accumulated counter value for this result. In case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt) this counter counts when csd_sense is high."]
    #[inline(always)]
    pub const fn value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Accumulated counter value for this result. In case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt) this counter counts when csd_sense is high."]
    #[inline(always)]
    pub fn set_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Number of 'bad' conversion for which the CSD comparator did not trigger within the normal time window, either because Vref was not crossed at all, or if the Vref was already crossed before the window started. This counter is reset when the sequencer is started and will saturate at 255 when more than 255 conversions are bad."]
    #[inline(always)]
    pub const fn bad_convs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of 'bad' conversion for which the CSD comparator did not trigger within the normal time window, either because Vref was not crossed at all, or if the Vref was already crossed before the window started. This counter is reset when the sequencer is started and will saturate at 255 when more than 255 conversions are bad."]
    #[inline(always)]
    pub fn set_bad_convs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ResultVal1 {
    #[inline(always)]
    fn default() -> ResultVal1 {
        ResultVal1(0)
    }
}
#[doc = "Result CSX accumulation counter value 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResultVal2(pub u32);
impl ResultVal2 {
    #[doc = "Only used in case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt), this counter counts when csd_sense is low."]
    #[inline(always)]
    pub const fn value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Only used in case of Mutual cap with two counters (CSX = config.mutual_cap & config.csx_dual_cnt), this counter counts when csd_sense is low."]
    #[inline(always)]
    pub fn set_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ResultVal2 {
    #[inline(always)]
    fn default() -> ResultVal2 {
        ResultVal2(0)
    }
}
#[doc = "Sense clock duty cycle"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SenseDuty(pub u32);
impl SenseDuty {
    #[doc = "Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap), if this rule is violated the result is undefined."]
    #[inline(always)]
    pub const fn sense_width(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Defines the length of the first phase of the sense clock in clk_csd cycles. A value of 0 disables this feature and the duty cycle of csd_sense will be 50 percent, which is equal to SENSE_WIDTH = (SENSE_DIV+1)/2, or when clock dithering is used that becomes \\[(SENSE_DIV+1) + (LFSR_OUT << LSFR_SCALE)\\]/2. At all times it must be assured that the phases are at least 2 clk_csd cycles (1 for non overlap), if this rule is violated the result is undefined."]
    #[inline(always)]
    pub fn set_sense_width(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub const fn sense_pol(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of the sense clock 0 = start with low phase (typical for regular negative transfer CSD) 1 = start with high phase"]
    #[inline(always)]
    pub fn set_sense_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub const fn overlap_phi1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "NonOverlap or not for Phi1 (csd_sense=0). 0 = Non-overlap for Phi1, the Phi1 signal is csd_sense inverted except that the signal goes low 1 clk_sample before csd_sense goes high. Intended usage: new low EMI CSD/CSX with static GPIO. 1 = 'Overlap' (= not non-overlap) for Phi1, the Phi1 signal is csd_sense inverted. Intended usage: legacy CSD with GPIO switching, the GPIO internal circuit ensures that the switches are non-overlapping."]
    #[inline(always)]
    pub fn set_overlap_phi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub const fn overlap_phi2(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Same as OVERLAP_PHI1 but for Phi2 (csd_sense=1)."]
    #[inline(always)]
    pub fn set_overlap_phi2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for SenseDuty {
    #[inline(always)]
    fn default() -> SenseDuty {
        SenseDuty(0)
    }
}
#[doc = "Sense clock period"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SensePeriod(pub u32);
impl SensePeriod {
    #[doc = "The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3."]
    #[inline(always)]
    pub const fn sense_div(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3."]
    #[inline(always)]
    pub fn set_sense_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Selects the length of the LFSR which determines the LFSR repeat period. The 5 LSB of the LFSR are used for the clock dithering variation (from -16 to 15) on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set. Caveat do not use clock dithering unless SENSE_DIV > 16, otherwise results are undefined."]
    #[inline(always)]
    pub const fn lfsr_size(&self) -> super::vals::LfsrSize {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::LfsrSize::from_bits(val as u8)
    }
    #[doc = "Selects the length of the LFSR which determines the LFSR repeat period. The 5 LSB of the LFSR are used for the clock dithering variation (from -16 to 15) on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set. Caveat do not use clock dithering unless SENSE_DIV > 16, otherwise results are undefined."]
    #[inline(always)]
    pub fn set_lfsr_size(&mut self, val: super::vals::LfsrSize) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    pub const fn lfsr_scale(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    pub fn set_lfsr_scale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    pub const fn lfsr_clear(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    pub fn set_lfsr_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled."]
    #[inline(always)]
    pub const fn sel_lfsr_msb(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled."]
    #[inline(always)]
    pub fn set_sel_lfsr_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for SensePeriod {
    #[inline(always)]
    fn default() -> SensePeriod {
        SensePeriod(0)
    }
}
#[doc = "Sequencer Initial conversion and sample counts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqInitCnt(pub u32);
impl SeqInitCnt {
    #[doc = "Number of conversion per sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    pub const fn conv_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of conversion per sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    pub fn set_conv_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SeqInitCnt {
    #[inline(always)]
    fn default() -> SeqInitCnt {
        SeqInitCnt(0)
    }
}
#[doc = "Sequencer Normal conversion and sample counts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqNormCnt(pub u32);
impl SeqNormCnt {
    #[doc = "Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    pub const fn conv_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    pub fn set_conv_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SeqNormCnt {
    #[inline(always)]
    fn default() -> SeqNormCnt {
        SeqNormCnt(0)
    }
}
#[doc = "Sequencer start"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqStart(pub u32);
impl SeqStart {
    #[doc = "Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub const fn seq_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub fn set_seq_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub const fn abort(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub fn set_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub const fn dsi_start_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub fn set_dsi_start_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub const fn az0_skip(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub fn set_az0_skip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub const fn az1_skip(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub fn set_az1_skip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for SeqStart {
    #[inline(always)]
    fn default() -> SeqStart {
        SeqStart(0)
    }
}
#[doc = "Sequencer Timing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqTime(pub u32);
impl SeqTime {
    #[doc = "Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub const fn az_time(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub fn set_az_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SeqTime {
    #[inline(always)]
    fn default() -> SeqTime {
        SeqTime(0)
    }
}
#[doc = "Spare MMIO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spare(pub u32);
impl Spare {
    #[doc = "Spare MMIO"]
    #[inline(always)]
    pub const fn spare(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Spare MMIO"]
    #[inline(always)]
    pub fn set_spare(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Spare {
    #[inline(always)]
    fn default() -> Spare {
        Spare(0)
    }
}
#[doc = "Current status counts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatCnts(pub u32);
impl StatCnts {
    #[doc = "Current number of conversions remaining when in Sample_* states (note that in AutoZero* states the same down counter is reused to count the cycles)"]
    #[inline(always)]
    pub const fn num_conv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current number of conversions remaining when in Sample_* states (note that in AutoZero* states the same down counter is reused to count the cycles)"]
    #[inline(always)]
    pub fn set_num_conv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for StatCnts {
    #[inline(always)]
    fn default() -> StatCnts {
        StatCnts(0)
    }
}
#[doc = "Current Sequencer status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatSeq(pub u32);
impl StatSeq {
    #[doc = "CSD sequencer state"]
    #[inline(always)]
    pub const fn seq_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "CSD sequencer state"]
    #[inline(always)]
    pub fn set_seq_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "ADC sequencer state (only relevant after SEQ_STATE has reached SAMPLE_NORM and ADC sequencer has started)"]
    #[inline(always)]
    pub const fn adc_state(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "ADC sequencer state (only relevant after SEQ_STATE has reached SAMPLE_NORM and ADC sequencer has started)"]
    #[inline(always)]
    pub fn set_adc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for StatSeq {
    #[inline(always)]
    fn default() -> StatSeq {
        StatSeq(0)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Qualified, and possible inverted value of COMP_OUT that is used to drive GPIO's charging Cmod or Csh_tank."]
    #[inline(always)]
    pub const fn csd_charge(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Qualified, and possible inverted value of COMP_OUT that is used to drive GPIO's charging Cmod or Csh_tank."]
    #[inline(always)]
    pub fn set_csd_charge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Signal used to drive the Cs switches."]
    #[inline(always)]
    pub const fn csd_sense(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Signal used to drive the Cs switches."]
    #[inline(always)]
    pub fn set_csd_sense(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output of reference buffer comparator used to charge up Cmod and/or Csh_tank (synchronized)"]
    #[inline(always)]
    pub const fn hscmp_out(&self) -> super::vals::HscmpOut {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::HscmpOut::from_bits(val as u8)
    }
    #[doc = "Output of reference buffer comparator used to charge up Cmod and/or Csh_tank (synchronized)"]
    #[inline(always)]
    pub fn set_hscmp_out(&mut self, val: super::vals::HscmpOut) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Output of main sensing comparator (synchronized)"]
    #[inline(always)]
    pub const fn csdcmp_out(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output of main sensing comparator (synchronized)"]
    #[inline(always)]
    pub fn set_csdcmp_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Amuxbuffer switches Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwAmuxbufSel(pub u32);
impl SwAmuxbufSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irby(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irby(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irlb(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irlb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_ica(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_ica(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_icb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_icb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irli(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irli(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irh(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_irl(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_irl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwAmuxbufSel {
    #[inline(always)]
    fn default() -> SwAmuxbufSel {
        SwAmuxbufSel(0)
    }
}
#[doc = "AMUXBUS bypass switches Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwBypSel(pub u32);
impl SwBypSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_bya(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_bya(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_byb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_byb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub const fn sw_cbcc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn set_sw_cbcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for SwBypSel {
    #[inline(always)]
    fn default() -> SwBypSel {
        SwBypSel(0)
    }
}
#[doc = "CSDCMP Neg Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwCmpNsel(pub u32);
impl SwCmpNsel {
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_scrh(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_scrh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_scrl(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_scrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for SwCmpNsel {
    #[inline(always)]
    fn default() -> SwCmpNsel {
        SwCmpNsel(0)
    }
}
#[doc = "CSDCMP Pos Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwCmpPsel(pub u32);
impl SwCmpPsel {
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfpm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfpm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfpt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfps(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfma(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfmb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfca(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sfcb(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sfcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for SwCmpPsel {
    #[inline(always)]
    fn default() -> SwCmpPsel {
        SwCmpPsel(0)
    }
}
#[doc = "DSI output switch control Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwDsiSel(pub u32);
impl SwDsiSel {
    #[doc = "Select waveform for dsi_csh_tank signal (called dsi_cap_lo_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
    #[inline(always)]
    pub const fn dsi_csh_tank(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for dsi_csh_tank signal (called dsi_cap_lo_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
    #[inline(always)]
    pub fn set_dsi_csh_tank(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Select waveform for dsi_cmod signal (called dsi_cap_hi_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
    #[inline(always)]
    pub const fn dsi_cmod(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for dsi_cmod signal (called dsi_cap_hi_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
    #[inline(always)]
    pub fn set_dsi_cmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for SwDsiSel {
    #[inline(always)]
    fn default() -> SwDsiSel {
        SwDsiSel(0)
    }
}
#[doc = "Full Wave Cmod Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwFwModSel(pub u32);
impl SwFwModSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_f1pm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f1pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f1ma(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f1ma(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f1ca(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f1ca(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c1cc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c1cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c1cd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c1cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c1f1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c1f1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwFwModSel {
    #[inline(always)]
    fn default() -> SwFwModSel {
        SwFwModSel(0)
    }
}
#[doc = "Full Wave Csh_tank Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwFwTankSel(pub u32);
impl SwFwTankSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_f2pt(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f2pt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f2ma(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f2ma(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f2ca(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f2ca(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_f2cb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_f2cb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c2cc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c2cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c2cd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c2cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_c2f2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_c2f2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwFwTankSel {
    #[inline(always)]
    fn default() -> SwFwTankSel {
        SwFwTankSel(0)
    }
}
#[doc = "HSCMP Neg input switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwHsNsel(pub u32);
impl SwHsNsel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hccc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hccd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hccd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_hcrh(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hcrh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_hcrl(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hcrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for SwHsNsel {
    #[inline(always)]
    fn default() -> SwHsNsel {
        SwHsNsel(0)
    }
}
#[doc = "HSCMP Pos input switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwHsPsel(pub u32);
impl SwHsPsel {
    #[doc = "Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub const fn sw_hmpm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub fn set_sw_hmpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmpt(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmps(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmma(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmmb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmca(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmcb(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hmrh(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hmrh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwHsPsel {
    #[inline(always)]
    fn default() -> SwHsPsel {
        SwHsPsel(0)
    }
}
#[doc = "Reference Generator Switch Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwRefgenSel(pub u32);
impl SwRefgenSel {
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_iaib(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_iaib(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_ibcb(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_ibcb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sgmb(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sgmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sgre(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sgre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_sgr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_sgr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SwRefgenSel {
    #[inline(always)]
    fn default() -> SwRefgenSel {
        SwRefgenSel(0)
    }
}
#[doc = "Switch Resistance configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwRes(pub u32);
impl SwRes {
    #[doc = "Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub const fn res_hcav(&self) -> super::vals::ResHcav {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::ResHcav::from_bits(val as u8)
    }
    #[doc = "Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub fn set_res_hcav(&mut self, val: super::vals::ResHcav) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub const fn res_hcag(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_hcag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub const fn res_hcbv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_hcbv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub const fn res_hcbg(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_hcbg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Select resistance for the corresponding switch"]
    #[inline(always)]
    pub const fn res_f1pm(&self) -> super::vals::ResF1pm {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::ResF1pm::from_bits(val as u8)
    }
    #[doc = "Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_f1pm(&mut self, val: super::vals::ResF1pm) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select resistance for the corresponding switch"]
    #[inline(always)]
    pub const fn res_f2pt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn set_res_f2pt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for SwRes {
    #[inline(always)]
    fn default() -> SwRes {
        SwRes(0)
    }
}
#[doc = "Shielding switches Waveform selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwShieldSel(pub u32);
impl SwShieldSel {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn sw_hcav(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_sw_hcav(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub const fn sw_hcag(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hcag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn sw_hcbv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_sw_hcbv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub const fn sw_hcbg(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub fn set_sw_hcbg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub const fn sw_hccv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch"]
    #[inline(always)]
    pub fn set_sw_hccv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub const fn sw_hccg(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn set_sw_hccg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for SwShieldSel {
    #[inline(always)]
    fn default() -> SwShieldSel {
        SwShieldSel(0)
    }
}
#[doc = "Trim control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimCtrl(pub u32);
impl TrimCtrl {
    #[doc = "Trim input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
    #[inline(always)]
    pub const fn delay_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Trim input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
    #[inline(always)]
    pub fn set_delay_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Hystersis input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
    #[inline(always)]
    pub const fn delay_hys(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Hystersis input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
    #[inline(always)]
    pub fn set_delay_hys(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for TrimCtrl {
    #[inline(always)]
    fn default() -> TrimCtrl {
        TrimCtrl(0)
    }
}
