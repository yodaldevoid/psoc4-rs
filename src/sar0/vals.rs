#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DifferentialSigned {
    #[doc = "result data is unsigned (zero extended if needed)"]
    UNSIGNED = 0,
    #[doc = "Default: result data is signed (sign extended if needed)"]
    SIGNED = 0x01,
}
impl DifferentialSigned {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DifferentialSigned {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DifferentialSigned {
    #[inline(always)]
    fn from(val: u8) -> DifferentialSigned {
        DifferentialSigned::from_bits(val)
    }
}
impl From<DifferentialSigned> for u8 {
    #[inline(always)]
    fn from(val: DifferentialSigned) -> u8 {
        DifferentialSigned::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IcontLv {
    #[doc = "normal power (default), max clk_sar is 18MHz."]
    NORMAL_PWR = 0,
    #[doc = "1/2 power mode, max clk_sar is 9MHz."]
    HALF_PWR = 0x01,
    #[doc = "1.333 power mode, max clk_sar is 18MHz."]
    MORE_PWR = 0x02,
    #[doc = "1/4 power mode, max clk_sar is 4.5MHz."]
    QUARTER_PWR = 0x03,
}
impl IcontLv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IcontLv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IcontLv {
    #[inline(always)]
    fn from(val: u8) -> IcontLv {
        IcontLv::from_bits(val)
    }
}
impl From<IcontLv> for u8 {
    #[inline(always)]
    fn from(val: IcontLv) -> u8 {
        IcontLv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InjPortAddr {
    #[doc = "SARMUX pins."]
    SARMUX = 0,
    #[doc = "CTB0"]
    CTB0 = 0x01,
    #[doc = "CTB1"]
    CTB1 = 0x02,
    #[doc = "CTB2"]
    CTB2 = 0x03,
    #[doc = "CTB3"]
    CTB3 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "AROUTE virtual port"]
    AROUTE_VIRT = 0x06,
    #[doc = "SARMUX virtual port"]
    SARMUX_VIRT = 0x07,
}
impl InjPortAddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InjPortAddr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InjPortAddr {
    #[inline(always)]
    fn from(val: u8) -> InjPortAddr {
        InjPortAddr::from_bits(val)
    }
}
impl From<InjPortAddr> for u8 {
    #[inline(always)]
    fn from(val: InjPortAddr) -> u8 {
        InjPortAddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InjResolution {
    #[doc = "12-bit resolution is used for this channel."]
    _12B = 0,
    #[doc = "The resolution specified by SUB_RESOLUTION in the SAR_SAMPLE_CTRL register is used for this channel."]
    SUBRES = 0x01,
}
impl InjResolution {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InjResolution {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InjResolution {
    #[inline(always)]
    fn from(val: u8) -> InjResolution {
        InjResolution::from_bits(val)
    }
}
impl From<InjResolution> for u8 {
    #[inline(always)]
    fn from(val: InjResolution) -> u8 {
        InjResolution::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum NegSel {
    #[doc = "NEG input of SARADC is connected to 'vssa_kelvin', gives more precision around zero. Note this opens both SARADC internal switches, therefore use this value to insert a break-before-make cycle on those switches when SWITCH_DISABLE is high."]
    VSSA_KELVIN = 0,
    #[doc = "NEG input of SARADC is connected to VSSA in AROUTE close to the SARADC"]
    ART_VSSA = 0x01,
    #[doc = "NEG input of SARADC is connected to P1 pin of SARMUX"]
    P1 = 0x02,
    #[doc = "NEG input of SARADC is connected to P3 pin of SARMUX"]
    P3 = 0x03,
    #[doc = "NEG input of SARADC is connected to P5 pin of SARMUX"]
    P5 = 0x04,
    #[doc = "NEG input of SARADC is connected to P7 pin of SARMUX"]
    P7 = 0x05,
    #[doc = "NEG input of SARADC is connected to an ACORE in AROUTE"]
    ACORE = 0x06,
    #[doc = "NEG input of SARADC is shorted with VREF input of SARADC."]
    VREF = 0x07,
}
impl NegSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NegSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NegSel {
    #[inline(always)]
    fn from(val: u8) -> NegSel {
        NegSel::from_bits(val)
    }
}
impl From<NegSel> for u8 {
    #[inline(always)]
    fn from(val: NegSel) -> u8 {
        NegSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PortAddr {
    #[doc = "SARMUX pins."]
    SARMUX = 0,
    #[doc = "CTB0"]
    CTB0 = 0x01,
    #[doc = "CTB1"]
    CTB1 = 0x02,
    #[doc = "CTB2"]
    CTB2 = 0x03,
    #[doc = "CTB3"]
    CTB3 = 0x04,
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2 = 0x05,
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1 = 0x06,
    #[doc = "SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT = 0x07,
}
impl PortAddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PortAddr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PortAddr {
    #[inline(always)]
    fn from(val: u8) -> PortAddr {
        PortAddr::from_bits(val)
    }
}
impl From<PortAddr> for u8 {
    #[inline(always)]
    fn from(val: PortAddr) -> u8 {
        PortAddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PwrCtrlVref {
    #[doc = "normal power (default), bypass cap, max clk_sar is 18MHz."]
    NORMAL_PWR = 0,
    #[doc = "deprecated"]
    HALF_PWR = 0x01,
    #[doc = "Invalid for PSoC4A, otherwise 2X power, no bypass cap, max clk_sar is 1.8MHz"]
    THIRD_PWR = 0x02,
    #[doc = "deprecated"]
    QUARTER_PWR = 0x03,
}
impl PwrCtrlVref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrCtrlVref {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrCtrlVref {
    #[inline(always)]
    fn from(val: u8) -> PwrCtrlVref {
        PwrCtrlVref::from_bits(val)
    }
}
impl From<PwrCtrlVref> for u8 {
    #[inline(always)]
    fn from(val: PwrCtrlVref) -> u8 {
        PwrCtrlVref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RangeCond {
    #[doc = "result < RANGE_LOW"]
    BELOW = 0,
    #[doc = "RANGE_LOW <= result < RANGE_HIGH"]
    INSIDE = 0x01,
    #[doc = "RANGE_HIGH <= result"]
    ABOVE = 0x02,
    #[doc = "result < RANGE_LOW || RANGE_HIGH <= result"]
    OUTSIDE = 0x03,
}
impl RangeCond {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RangeCond {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RangeCond {
    #[inline(always)]
    fn from(val: u8) -> RangeCond {
        RangeCond::from_bits(val)
    }
}
impl From<RangeCond> for u8 {
    #[inline(always)]
    fn from(val: RangeCond) -> u8 {
        RangeCond::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resolution {
    #[doc = "The maximum resolution is used for this channel (maximum resolution depends on wounding)."]
    MAXRES = 0,
    #[doc = "The resolution specified by SUB_RESOLUTION in the SAR_SAMPLE_CTRL register is used for this channel."]
    SUBRES = 0x01,
}
impl Resolution {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resolution {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resolution {
    #[inline(always)]
    fn from(val: u8) -> Resolution {
        Resolution::from_bits(val)
    }
}
impl From<Resolution> for u8 {
    #[inline(always)]
    fn from(val: Resolution) -> u8 {
        Resolution::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SingleEndedSigned {
    #[doc = "Default: result data is unsigned (zero extended if needed)"]
    UNSIGNED = 0,
    #[doc = "result data is signed (sign extended if needed)"]
    SIGNED = 0x01,
}
impl SingleEndedSigned {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleEndedSigned {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleEndedSigned {
    #[inline(always)]
    fn from(val: u8) -> SingleEndedSigned {
        SingleEndedSigned::from_bits(val)
    }
}
impl From<SingleEndedSigned> for u8 {
    #[inline(always)]
    fn from(val: SingleEndedSigned) -> u8 {
        SingleEndedSigned::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SubResolution {
    #[doc = "8-bit."]
    _8B = 0,
    #[doc = "10-bit."]
    _10B = 0x01,
}
impl SubResolution {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SubResolution {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SubResolution {
    #[inline(always)]
    fn from(val: u8) -> SubResolution {
        SubResolution::from_bits(val)
    }
}
impl From<SubResolution> for u8 {
    #[inline(always)]
    fn from(val: SubResolution) -> u8 {
        SubResolution::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum VrefSel {
    #[doc = "VREF0 from PRB (VREF buffer on)"]
    VREF0 = 0,
    #[doc = "VREF1 from PRB (VREF buffer on)"]
    VREF1 = 0x01,
    #[doc = "VREF2 from PRB (VREF buffer on)"]
    VREF2 = 0x02,
    #[doc = "VREF from AROUTE (VREF buffer on)"]
    VREF_AROUTE = 0x03,
    #[doc = "1.024V from BandGap (VREF buffer on)"]
    VBGR = 0x04,
    #[doc = "External precision Vref direct from a pin (low impedance path)."]
    VREF_EXT = 0x05,
    #[doc = "Vdda/2 (VREF buffer on)"]
    VDDA_DIV_2 = 0x06,
    #[doc = "Vdda."]
    VDDA = 0x07,
}
impl VrefSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VrefSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VrefSel {
    #[inline(always)]
    fn from(val: u8) -> VrefSel {
        VrefSel::from_bits(val)
    }
}
impl From<VrefSel> for u8 {
    #[inline(always)]
    fn from(val: VrefSel) -> u8 {
        VrefSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WoundResolution {
    #[doc = "unwounded: up to full 12-bit SAR resolution allowed"]
    _12BIT = 0,
    #[doc = "wounded: max resolution upto 10-bit SAR resolution allowed"]
    _10BIT = 0x01,
    #[doc = "wounded: only 8-bit SAR resolution allowed"]
    _8BIT = 0x02,
    #[doc = "wounded: only 8-bit SAR resolution allowed"]
    _8BIT_TOO = 0x03,
}
impl WoundResolution {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WoundResolution {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WoundResolution {
    #[inline(always)]
    fn from(val: u8) -> WoundResolution {
        WoundResolution::from_bits(val)
    }
}
impl From<WoundResolution> for u8 {
    #[inline(always)]
    fn from(val: WoundResolution) -> u8 {
        WoundResolution::to_bits(val)
    }
}
