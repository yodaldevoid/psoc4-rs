#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AdcMode {
    #[doc = "No ADC measurement"]
    OFF = 0,
    #[doc = "Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    VREF_CNT = 0x01,
    #[doc = "Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    VREF_BY2_CNT = 0x02,
    #[doc = "Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    VIN_CNT = 0x03,
}
impl AdcMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcMode {
    #[inline(always)]
    fn from(val: u8) -> AdcMode {
        AdcMode::from_bits(val)
    }
}
impl From<AdcMode> for u8 {
    #[inline(always)]
    fn from(val: AdcMode) -> u8 {
        AdcMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChargeMode {
    #[doc = "Use this to keep csd_charge signal low. For charging Cmod/Csh_tank capacitor CSD internal switches (HCBV) can be used but that is a separate configuration."]
    CHARGE_OFF = 0,
    #[doc = "Use csd_charge to enable the GPIO Driver to charge capacitor. The capacitor must be sensed with HSCMP using the appropriate switches (HMPM or HMPT)."]
    CHARGE_IO = 0x01,
}
impl ChargeMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChargeMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChargeMode {
    #[inline(always)]
    fn from(val: u8) -> ChargeMode {
        ChargeMode::from_bits(val)
    }
}
impl From<ChargeMode> for u8 {
    #[inline(always)]
    fn from(val: ChargeMode) -> u8 {
        ChargeMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CmpMode {
    #[doc = "CSD mode: output the filtered sample signal on dsi_sample_out"]
    CSD = 0,
    #[doc = "General Purpose mode: output the unfiltered sample unfiltered comparator output, either asynchronous or flopped."]
    GP = 0x01,
}
impl CmpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpMode {
    #[inline(always)]
    fn from(val: u8) -> CmpMode {
        CmpMode::from_bits(val)
    }
}
impl From<CmpMode> for u8 {
    #[inline(always)]
    fn from(val: CmpMode) -> u8 {
        CmpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CmpPhase {
    #[doc = "Comparator is active from start of Phi2 and kept active into Phi1. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    FULL = 0,
    #[doc = "Comparator is active during Phi1 only. Currently no known use-case."]
    PHI1 = 0x01,
    #[doc = "Comparator is active during Phi2 only. Intended usage: CSD Low EMI."]
    PHI2 = 0x02,
    #[doc = "Comparator is activated at the start of both Phi1 and Phi2 (non-overlap should be enabled). Intended usage: CSX, or Full-Wave."]
    PHI1_2 = 0x03,
}
impl CmpPhase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpPhase {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpPhase {
    #[inline(always)]
    fn from(val: u8) -> CmpPhase {
        CmpPhase::from_bits(val)
    }
}
impl From<CmpPhase> for u8 {
    #[inline(always)]
    fn from(val: CmpPhase) -> u8 {
        CmpPhase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CsdcmpEn {
    #[doc = "Disable comparator, output is zero"]
    OFF = 0,
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 0x01,
}
impl CsdcmpEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsdcmpEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsdcmpEn {
    #[inline(always)]
    fn from(val: u8) -> CsdcmpEn {
        CsdcmpEn::from_bits(val)
    }
}
impl From<CsdcmpEn> for u8 {
    #[inline(always)]
    fn from(val: CsdcmpEn) -> u8 {
        CsdcmpEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CsxDualCnt {
    #[doc = "Use one counter for both phases (source and sink)."]
    ONE = 0,
    #[doc = "Use two counters, separate count for when csd_sense is high and when csd_sense is low."]
    TWO = 0x01,
}
impl CsxDualCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsxDualCnt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsxDualCnt {
    #[inline(always)]
    fn from(val: u8) -> CsxDualCnt {
        CsxDualCnt::from_bits(val)
    }
}
impl From<CsxDualCnt> for u8 {
    #[inline(always)]
    fn from(val: CsxDualCnt) -> u8 {
        CsxDualCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DsiCountSel {
    #[doc = "depending on the dsi_count_val_sel input either output RESULT_VAL1.VALUE (0) or RESULT_VAL2.VALUE (1) on the dsi_count bus. Note that dsi_count_val_sel is not synchronized, i.e. it controls the mux combinatorially."]
    CSD_RESULT = 0,
    #[doc = "output ADC_RES.VIN_CNT on the dsi_count bus"]
    ADC_RESULT = 0x01,
}
impl DsiCountSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DsiCountSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DsiCountSel {
    #[inline(always)]
    fn from(val: u8) -> DsiCountSel {
        DsiCountSel::from_bits(val)
    }
}
impl From<DsiCountSel> for u8 {
    #[inline(always)]
    fn from(val: DsiCountSel) -> u8 {
        DsiCountSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FeedbackMode {
    #[doc = "Use feedback from sampling flip-flop (used in most modes)."]
    FLOP = 0,
    #[doc = "Use feedback from comparator directly (used in single Cmod mutual cap sensing only)"]
    COMP = 0x01,
}
impl FeedbackMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FeedbackMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FeedbackMode {
    #[inline(always)]
    fn from(val: u8) -> FeedbackMode {
        FeedbackMode::from_bits(val)
    }
}
impl From<FeedbackMode> for u8 {
    #[inline(always)]
    fn from(val: FeedbackMode) -> u8 {
        FeedbackMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HscmpEn {
    #[doc = "Disable comparator, output is zero"]
    OFF = 0,
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 0x01,
}
impl HscmpEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HscmpEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HscmpEn {
    #[inline(always)]
    fn from(val: u8) -> HscmpEn {
        HscmpEn::from_bits(val)
    }
}
impl From<HscmpEn> for u8 {
    #[inline(always)]
    fn from(val: HscmpEn) -> u8 {
        HscmpEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HscmpOut {
    #[doc = "Vin < Vref"]
    C_LT_VREF = 0,
    #[doc = "Vin > Vref"]
    C_GT_VREF = 0x01,
}
impl HscmpOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HscmpOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HscmpOut {
    #[inline(always)]
    fn from(val: u8) -> HscmpOut {
        HscmpOut::from_bits(val)
    }
}
impl From<HscmpOut> for u8 {
    #[inline(always)]
    fn from(val: HscmpOut) -> u8 {
        HscmpOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaBalMode {
    #[doc = "enabled from start of Phi2 until disabled by CSDCMP. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    FULL = 0,
    #[doc = "enabled from start of Phi1 and disabled by CSDCMP or at end of Phi1. Enables dual IDAC CSX or Full-Wave, one for sourcing and the other for sinking."]
    PHI1 = 0x01,
    #[doc = "enabled from start of Phi2 and disabled by CSDCMP or at end of Phi2. Intended usage: CSD Low EMI or dual IDAC CSX or Full-Wave."]
    PHI2 = 0x02,
    #[doc = "enabled from start of both Phi1 and Phi2 and disabled by CSDCMP or at end of Phi1 or Phi2 (if non-overlap enabled). Intended usage: single IDAC CSX, or Full-Wave."]
    PHI1_2 = 0x03,
}
impl IdacaBalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaBalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaBalMode {
    #[inline(always)]
    fn from(val: u8) -> IdacaBalMode {
        IdacaBalMode::from_bits(val)
    }
}
impl From<IdacaBalMode> for u8 {
    #[inline(always)]
    fn from(val: IdacaBalMode) -> u8 {
        IdacaBalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaLeg1mode {
    #[doc = "General Purpose static mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    GP_STATIC = 0,
    #[doc = "General Purpose dynamic mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    GP = 0x01,
    #[doc = "CSD static mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG1 is controlled by LEG1_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    CSD_STATIC = 0x02,
    #[doc = "CSD dynamic mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In thoses states LEG1 is controlled by LEG1_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    CSD = 0x03,
}
impl IdacaLeg1mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaLeg1mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaLeg1mode {
    #[inline(always)]
    fn from(val: u8) -> IdacaLeg1mode {
        IdacaLeg1mode::from_bits(val)
    }
}
impl From<IdacaLeg1mode> for u8 {
    #[inline(always)]
    fn from(val: IdacaLeg1mode) -> u8 {
        IdacaLeg1mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaLeg2mode {
    #[doc = "General Purpose static mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    GP_STATIC = 0,
    #[doc = "General Purpose dynamic mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    GP = 0x01,
    #[doc = "CSD static mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg2 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    CSD_STATIC = 0x02,
    #[doc = "CSD dynamic mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). In addition leg2 enable can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    CSD = 0x03,
}
impl IdacaLeg2mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaLeg2mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaLeg2mode {
    #[inline(always)]
    fn from(val: u8) -> IdacaLeg2mode {
        IdacaLeg2mode::from_bits(val)
    }
}
impl From<IdacaLeg2mode> for u8 {
    #[inline(always)]
    fn from(val: IdacaLeg2mode) -> u8 {
        IdacaLeg2mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaPolDyn {
    #[doc = "Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    STATIC = 0,
    #[doc = "Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    DYNAMIC = 0x01,
}
impl IdacaPolDyn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaPolDyn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaPolDyn {
    #[inline(always)]
    fn from(val: u8) -> IdacaPolDyn {
        IdacaPolDyn::from_bits(val)
    }
}
impl From<IdacaPolDyn> for u8 {
    #[inline(always)]
    fn from(val: IdacaPolDyn) -> u8 {
        IdacaPolDyn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaPolarity {
    #[doc = "Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    VSSA_SRC = 0,
    #[doc = "Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    VDDA_SNK = 0x01,
    #[doc = "The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE = 0x02,
    #[doc = "The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE_INV = 0x03,
}
impl IdacaPolarity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaPolarity {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaPolarity {
    #[inline(always)]
    fn from(val: u8) -> IdacaPolarity {
        IdacaPolarity::from_bits(val)
    }
}
impl From<IdacaPolarity> for u8 {
    #[inline(always)]
    fn from(val: IdacaPolarity) -> u8 {
        IdacaPolarity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacaRange {
    #[doc = "1 LSB = 37.5 nA"]
    IDAC_LO = 0,
    #[doc = "1 LSB = 300 nA"]
    IDAC_MED = 0x01,
    #[doc = "1 LSB = 2400 nA"]
    IDAC_HI = 0x02,
    _RESERVED_3 = 0x03,
}
impl IdacaRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacaRange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacaRange {
    #[inline(always)]
    fn from(val: u8) -> IdacaRange {
        IdacaRange::from_bits(val)
    }
}
impl From<IdacaRange> for u8 {
    #[inline(always)]
    fn from(val: IdacaRange) -> u8 {
        IdacaRange::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbBalMode {
    #[doc = "same as corresponding IDACA Balancing mode"]
    FULL = 0,
    #[doc = "same as corresponding IDACA Balancing mode"]
    PHI1 = 0x01,
    #[doc = "same as corresponding IDACA Balancing mode"]
    PHI2 = 0x02,
    #[doc = "same as corresponding IDACA Balancing mode"]
    PHI1_2 = 0x03,
}
impl IdacbBalMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbBalMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbBalMode {
    #[inline(always)]
    fn from(val: u8) -> IdacbBalMode {
        IdacbBalMode::from_bits(val)
    }
}
impl From<IdacbBalMode> for u8 {
    #[inline(always)]
    fn from(val: IdacbBalMode) -> u8 {
        IdacbBalMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbLeg1mode {
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    GP_STATIC = 0,
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    GP = 0x01,
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    CSD_STATIC = 0x02,
    #[doc = "same as corresponding IDACA.LEG1_MODE"]
    CSD = 0x03,
}
impl IdacbLeg1mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbLeg1mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbLeg1mode {
    #[inline(always)]
    fn from(val: u8) -> IdacbLeg1mode {
        IdacbLeg1mode::from_bits(val)
    }
}
impl From<IdacbLeg1mode> for u8 {
    #[inline(always)]
    fn from(val: IdacbLeg1mode) -> u8 {
        IdacbLeg1mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbLeg2mode {
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    GP_STATIC = 0,
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    GP = 0x01,
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    CSD_STATIC = 0x02,
    #[doc = "same as corresponding IDACA.LEG2_MODE"]
    CSD = 0x03,
}
impl IdacbLeg2mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbLeg2mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbLeg2mode {
    #[inline(always)]
    fn from(val: u8) -> IdacbLeg2mode {
        IdacbLeg2mode::from_bits(val)
    }
}
impl From<IdacbLeg2mode> for u8 {
    #[inline(always)]
    fn from(val: IdacbLeg2mode) -> u8 {
        IdacbLeg2mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbPolDyn {
    #[doc = "Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    STATIC = 0,
    #[doc = "Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    DYNAMIC = 0x01,
}
impl IdacbPolDyn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbPolDyn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbPolDyn {
    #[inline(always)]
    fn from(val: u8) -> IdacbPolDyn {
        IdacbPolDyn::from_bits(val)
    }
}
impl From<IdacbPolDyn> for u8 {
    #[inline(always)]
    fn from(val: IdacbPolDyn) -> u8 {
        IdacbPolDyn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbPolarity {
    #[doc = "Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    VSSA_SRC = 0,
    #[doc = "Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    VDDA_SNK = 0x01,
    #[doc = "The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE = 0x02,
    #[doc = "The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SENSE_INV = 0x03,
}
impl IdacbPolarity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbPolarity {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbPolarity {
    #[inline(always)]
    fn from(val: u8) -> IdacbPolarity {
        IdacbPolarity::from_bits(val)
    }
}
impl From<IdacbPolarity> for u8 {
    #[inline(always)]
    fn from(val: IdacbPolarity) -> u8 {
        IdacbPolarity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IdacbRange {
    #[doc = "1 LSB = 37.5 nA"]
    IDAC_LO = 0,
    #[doc = "1 LSB = 300 nA"]
    IDAC_MED = 0x01,
    #[doc = "1 LSB = 2400 nA"]
    IDAC_HI = 0x02,
    _RESERVED_3 = 0x03,
}
impl IdacbRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdacbRange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdacbRange {
    #[inline(always)]
    fn from(val: u8) -> IdacbRange {
        IdacbRange::from_bits(val)
    }
}
impl From<IdacbRange> for u8 {
    #[inline(always)]
    fn from(val: IdacbRange) -> u8 {
        IdacbRange::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LfsrSize {
    #[doc = "Don't use clock dithering (=spreadspectrum) (LFSR output value is zero)"]
    OFF = 0,
    #[doc = "6-bit LFSR (G(x)=X^6 +X^4+X^3+ X+1, period= 63)"]
    _6B = 0x01,
    #[doc = "7-bit LFSR (G(x)=X^7 +X^4+X^3+X^2+1, period= 127)"]
    _7B = 0x02,
    #[doc = "9-bit LFSR (G(x)=X^9 +X^4+X^3+ X+1, period= 511)"]
    _9B = 0x03,
    #[doc = "10-bit LFSR (G(x)=X^10+X^4+X^3+ X+1, period= 1023)"]
    _10B = 0x04,
    #[doc = "8-bit LFSR (G(x)=X^8 +X^4+X^3+X^2+1, period= 255)"]
    _8B = 0x05,
    #[doc = "12-bit LFSR (G(x)=X^12+X^7+X^4+X^3+1, period= 4095)"]
    _12B = 0x06,
    _RESERVED_7 = 0x07,
}
impl LfsrSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LfsrSize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LfsrSize {
    #[inline(always)]
    fn from(val: u8) -> LfsrSize {
        LfsrSize::from_bits(val)
    }
}
impl From<LfsrSize> for u8 {
    #[inline(always)]
    fn from(val: LfsrSize) -> u8 {
        LfsrSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MutualCap {
    #[doc = "Self-cap mode (configure sense line as CSD_SENSE)"]
    SELFCAP = 0,
    #[doc = "Mutual-cap mode (configure Tx line as CSD_SENSE, inverted Tx line as CSD_SHIELD and Rx Line as AMUXA). In this mode the polarity bit of the IDAC is controlled by csd_sense."]
    MUTUALCAP = 0x01,
}
impl MutualCap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MutualCap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MutualCap {
    #[inline(always)]
    fn from(val: u8) -> MutualCap {
        MutualCap::from_bits(val)
    }
}
impl From<MutualCap> for u8 {
    #[inline(always)]
    fn from(val: MutualCap) -> u8 {
        MutualCap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PolaritySel {
    #[doc = "Use idaca_pol (firmware setting with CSX and optionally DSI mixed in) to determine the direction, this is the most common use-case, used for normal CSD and normal CSX"]
    IDACA_POL = 0,
    #[doc = "Use idacb_pol (firmware setting with optional DSI mixed in) to determine the direction, this is only used for normal CSD if IDACB is used i.s.o. IDACA (not common)"]
    IDACB_POL = 0x01,
    #[doc = "Use the expression (csd_sense ? idaca_pol : idacb_pol) to determine the direction, this is only useful for the CSX with DUAL_IDAC use-case"]
    DUAL_POL = 0x02,
    _RESERVED_3 = 0x03,
}
impl PolaritySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolaritySel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolaritySel {
    #[inline(always)]
    fn from(val: u8) -> PolaritySel {
        PolaritySel::from_bits(val)
    }
}
impl From<PolaritySel> for u8 {
    #[inline(always)]
    fn from(val: PolaritySel) -> u8 {
        PolaritySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PwrMode {
    #[doc = "Disable buffer"]
    OFF = 0,
    #[doc = "On, normal or low power level depending on CONFIG.LP_MODE."]
    NORM = 0x01,
    #[doc = "On, high or low power level depending on CONFIG.LP_MODE."]
    HI = 0x02,
    _RESERVED_3 = 0x03,
}
impl PwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrMode {
    #[inline(always)]
    fn from(val: u8) -> PwrMode {
        PwrMode::from_bits(val)
    }
}
impl From<PwrMode> for u8 {
    #[inline(always)]
    fn from(val: PwrMode) -> u8 {
        PwrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RefgenEn {
    #[doc = "Disable Reference Generator"]
    OFF = 0,
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    ON = 0x01,
}
impl RefgenEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefgenEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefgenEn {
    #[inline(always)]
    fn from(val: u8) -> RefgenEn {
        RefgenEn::from_bits(val)
    }
}
impl From<RefgenEn> for u8 {
    #[inline(always)]
    fn from(val: RefgenEn) -> u8 {
        RefgenEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResF1pm {
    #[doc = "Low"]
    LOW = 0,
    #[doc = "Medium"]
    MED = 0x01,
    #[doc = "High"]
    HIGH = 0x02,
    #[doc = "N/A"]
    RSVD = 0x03,
}
impl ResF1pm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ResF1pm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ResF1pm {
    #[inline(always)]
    fn from(val: u8) -> ResF1pm {
        ResF1pm::from_bits(val)
    }
}
impl From<ResF1pm> for u8 {
    #[inline(always)]
    fn from(val: ResF1pm) -> u8 {
        ResF1pm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ResHcav {
    #[doc = "Low"]
    LOW = 0,
    #[doc = "Medium"]
    MED = 0x01,
    #[doc = "High"]
    HIGH = 0x02,
    #[doc = "Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    LOWEMI = 0x03,
}
impl ResHcav {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ResHcav {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ResHcav {
    #[inline(always)]
    fn from(val: u8) -> ResHcav {
        ResHcav::from_bits(val)
    }
}
impl From<ResHcav> for u8 {
    #[inline(always)]
    fn from(val: ResHcav) -> u8 {
        ResHcav::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ShieldDelay {
    #[doc = "Delay line is off, csd_shield=csd_sense"]
    OFF = 0,
    #[doc = "Introduces a 5ns delay (typ)"]
    D5NS = 0x01,
    #[doc = "Introduces a 10ns delay (typ)"]
    D10NS = 0x02,
    #[doc = "Introduces a 20ns delay (typ)"]
    D20NS = 0x03,
}
impl ShieldDelay {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShieldDelay {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShieldDelay {
    #[inline(always)]
    fn from(val: u8) -> ShieldDelay {
        ShieldDelay::from_bits(val)
    }
}
impl From<ShieldDelay> for u8 {
    #[inline(always)]
    fn from(val: ShieldDelay) -> u8 {
        ShieldDelay::to_bits(val)
    }
}
