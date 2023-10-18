#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ddft0sel {
    #[doc = "wakeup"]
    WAKEUP = 0,
    #[doc = "awake"]
    AWAKE = 0x01,
    #[doc = "act_power_en"]
    ACT_POWER_EN = 0x02,
    #[doc = "act_power_up"]
    ACT_POWER_UP = 0x03,
    #[doc = "act_power_good"]
    ACT_POWER_GOOD = 0x04,
    #[doc = "srss_adft_control_act_ref_en"]
    ACT_REF_EN = 0x05,
    #[doc = "srss_adft_control_act_comp_en"]
    ACT_COMP_EN = 0x06,
    #[doc = "srss_adft_control_dpslp_ref_en"]
    DPSLP_REF_EN = 0x07,
    #[doc = "srss_adft_control_dpslp_reg_en"]
    DPSLP_REG_EN = 0x08,
    #[doc = "srss_adft_control_dpslp_comp_en"]
    DPSLP_COMP_EN = 0x09,
    #[doc = "pwr_control_over_temp_en"]
    OVER_TEMP_EN = 0x0a,
    #[doc = "sleepholdreq_n"]
    SLEEPHOLDREQ_N = 0x0b,
    #[doc = "adft_buf_en"]
    ADFT_BUF_EN = 0x0c,
    #[doc = "ATPG observe point (no functional purpose)"]
    ATPG_OBSERVE = 0x0d,
    #[doc = "1'b0"]
    GND = 0x0e,
    #[doc = "1'b1"]
    PWR = 0x0f,
}
impl Ddft0sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ddft0sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ddft0sel {
    #[inline(always)]
    fn from(val: u8) -> Ddft0sel {
        Ddft0sel::from_bits(val)
    }
}
impl From<Ddft0sel> for u8 {
    #[inline(always)]
    fn from(val: Ddft0sel) -> u8 {
        Ddft0sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ddft1sel {
    #[doc = "wakeup"]
    WAKEUP = 0,
    #[doc = "awake"]
    AWAKE = 0x01,
    #[doc = "act_power_en"]
    ACT_POWER_EN = 0x02,
    #[doc = "act_power_up"]
    ACT_POWER_UP = 0x03,
    #[doc = "act_power_good"]
    ACT_POWER_GOOD = 0x04,
    #[doc = "act_ref_valid"]
    ACT_REF_VALID = 0x05,
    #[doc = "act_reg_valid"]
    ACT_REG_VALID = 0x06,
    #[doc = "act_comp_out"]
    ACT_COMP_OUT = 0x07,
    #[doc = "act_temp_high"]
    ACT_TEMP_HIGH = 0x08,
    #[doc = "dpslp_comp_out"]
    DPSLP_COMP_OUT = 0x09,
    #[doc = "dpslp_power_up"]
    DPSLP_POWER_UP = 0x0a,
    #[doc = "awake_delayed"]
    AWAKE_DELAYED = 0x0b,
    #[doc = "lpm_ready"]
    LPM_READY = 0x0c,
    #[doc = "sleepholdack_n"]
    SLEEPHOLDACK_N = 0x0d,
    #[doc = "1'b0"]
    GND = 0x0e,
    #[doc = "1'b1"]
    PWR = 0x0f,
}
impl Ddft1sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ddft1sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ddft1sel {
    #[inline(always)]
    fn from(val: u8) -> Ddft1sel {
        Ddft1sel::from_bits(val)
    }
}
impl From<Ddft1sel> for u8 {
    #[inline(always)]
    fn from(val: Ddft1sel) -> u8 {
        Ddft1sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DebugSession {
    #[doc = "No debug session active"]
    NO_SESSION = 0,
    #[doc = "Debug session is active"]
    SESSION_ACTIVE = 0x01,
}
impl DebugSession {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugSession {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugSession {
    #[inline(always)]
    fn from(val: u8) -> DebugSession {
        DebugSession::from_bits(val)
    }
}
impl From<DebugSession> for u8 {
    #[inline(always)]
    fn from(val: DebugSession) -> u8 {
        DebugSession::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DftDiv0 {
    #[doc = "Direct Output"]
    NO_DIV = 0,
    #[doc = "Divide by 2"]
    DIV_BY_2 = 0x01,
    #[doc = "Divide by 4"]
    DIV_BY_4 = 0x02,
    #[doc = "Divide by 8"]
    DIV_BY_8 = 0x03,
}
impl DftDiv0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftDiv0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftDiv0 {
    #[inline(always)]
    fn from(val: u8) -> DftDiv0 {
        DftDiv0::from_bits(val)
    }
}
impl From<DftDiv0> for u8 {
    #[inline(always)]
    fn from(val: DftDiv0) -> u8 {
        DftDiv0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DftDiv1 {
    #[doc = "Direct Output"]
    NO_DIV = 0,
    #[doc = "Divide by 2"]
    DIV_BY_2 = 0x01,
    #[doc = "Divide by 4"]
    DIV_BY_4 = 0x02,
    #[doc = "Divide by 8"]
    DIV_BY_8 = 0x03,
}
impl DftDiv1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftDiv1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftDiv1 {
    #[inline(always)]
    fn from(val: u8) -> DftDiv1 {
        DftDiv1::from_bits(val)
    }
}
impl From<DftDiv1> for u8 {
    #[inline(always)]
    fn from(val: DftDiv1) -> u8 {
        DftDiv1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DftEdge0 {
    #[doc = "Use posedge for divider"]
    POSEDGE = 0,
    #[doc = "Use negedge for divider"]
    NEGEDGE = 0x01,
}
impl DftEdge0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftEdge0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftEdge0 {
    #[inline(always)]
    fn from(val: u8) -> DftEdge0 {
        DftEdge0::from_bits(val)
    }
}
impl From<DftEdge0> for u8 {
    #[inline(always)]
    fn from(val: DftEdge0) -> u8 {
        DftEdge0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DftEdge1 {
    #[doc = "Use posedge for divider"]
    POSEDGE = 0,
    #[doc = "Use negedge for divider"]
    NEGEDGE = 0x01,
}
impl DftEdge1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftEdge1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftEdge1 {
    #[inline(always)]
    fn from(val: u8) -> DftEdge1 {
        DftEdge1::from_bits(val)
    }
}
impl From<DftEdge1> for u8 {
    #[inline(always)]
    fn from(val: DftEdge1) -> u8 {
        DftEdge1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DftSel0 {
    #[doc = "Disabled - output is 0"]
    NC = 0,
    #[doc = "clk_ilo: ILO output"]
    ILO = 0x01,
    #[doc = "clk_imo: IMO primary output"]
    IMO = 0x02,
    #[doc = "clk_eco: ECO output"]
    ECO = 0x03,
    #[doc = "clk_ext: external clock input"]
    EXTCLK = 0x04,
    #[doc = "clk_hf: root of the high-speed clock tree"]
    HFCLK = 0x05,
    #[doc = "clk_lf: root of the low-speed clock tree"]
    LFCLK = 0x06,
    #[doc = "clk_sys: root of the CPU/AHB clock tree (gated version of clk_hf)"]
    SYSCLK = 0x07,
    #[doc = "clk_pump: clock provided to charge pumps in FLASH and PA"]
    PUMPCLK = 0x08,
    #[doc = "clk_slpctrl: clock provided to SleepController"]
    SLPCTRLCLK = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DftSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftSel0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftSel0 {
    #[inline(always)]
    fn from(val: u8) -> DftSel0 {
        DftSel0::from_bits(val)
    }
}
impl From<DftSel0> for u8 {
    #[inline(always)]
    fn from(val: DftSel0) -> u8 {
        DftSel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DftSel1 {
    #[doc = "Disabled - output is 0"]
    NC = 0,
    #[doc = "clk_ilo: ILO output"]
    ILO = 0x01,
    #[doc = "clk_imo: IMO primary output"]
    IMO = 0x02,
    #[doc = "clk_eco: ECO output"]
    ECO = 0x03,
    #[doc = "clk_ext: external clock input"]
    EXTCLK = 0x04,
    #[doc = "clk_hf: root of the high-speed clock tree"]
    HFCLK = 0x05,
    #[doc = "clk_lf: root of the low-speed clock tree"]
    LFCLK = 0x06,
    #[doc = "clk_sys: root of the CPU/AHB clock tree (gated version of clk_hf)"]
    SYSCLK = 0x07,
    #[doc = "clk_pump: clock provided to charge pumps in FLASH and PA"]
    PUMPCLK = 0x08,
    #[doc = "clk_slpctrl: clock provided to SleepController"]
    SLPCTRLCLK = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DftSel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftSel1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftSel1 {
    #[inline(always)]
    fn from(val: u8) -> DftSel1 {
        DftSel1::from_bits(val)
    }
}
impl From<DftSel1> for u8 {
    #[inline(always)]
    fn from(val: DftSel1) -> u8 {
        DftSel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Freq {
    #[doc = "IMO runs at 24 MHz"]
    _24_MHZ = 0,
    #[doc = "IMO runs at 28 MHz"]
    _28_MHZ = 0x01,
    #[doc = "IMO runs at 32 MHz"]
    _32_MHZ = 0x02,
    #[doc = "IMO runs at 36 MHz"]
    _36_MHZ = 0x03,
    #[doc = "IMO runs at 40 MHz"]
    _40_MHZ = 0x04,
    #[doc = "IMO runs at 44 MHz"]
    _44_MHZ = 0x05,
    #[doc = "IMO runs at 48 MHz"]
    _48_MHZ = 0x06,
    _RESERVED_7 = 0x07,
}
impl Freq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Freq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Freq {
    #[inline(always)]
    fn from(val: u8) -> Freq {
        Freq::from_bits(val)
    }
}
impl From<Freq> for u8 {
    #[inline(always)]
    fn from(val: Freq) -> u8 {
        Freq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkDiv {
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    NO_DIV = 0,
    #[doc = "Divide selected clock source by 2"]
    DIV_BY_2 = 0x01,
    #[doc = "Divide selected clock source by 4"]
    DIV_BY_4 = 0x02,
    #[doc = "Divide selected clock source by 8"]
    DIV_BY_8 = 0x03,
}
impl HfclkDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkDiv {
    #[inline(always)]
    fn from(val: u8) -> HfclkDiv {
        HfclkDiv::from_bits(val)
    }
}
impl From<HfclkDiv> for u8 {
    #[inline(always)]
    fn from(val: HfclkDiv) -> u8 {
        HfclkDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HfclkSel {
    #[doc = "IMO - Internal R/C Oscillator"]
    IMO = 0,
    #[doc = "EXTCLK - External Clock Pin"]
    EXTCLK = 0x01,
    #[doc = "ECO - External-Crystal Oscillator or PLL subsystem output"]
    ECO = 0x02,
    _RESERVED_3 = 0x03,
}
impl HfclkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HfclkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HfclkSel {
    #[inline(always)]
    fn from(val: u8) -> HfclkSel {
        HfclkSel::from_bits(val)
    }
}
impl From<HfclkSel> for u8 {
    #[inline(always)]
    fn from(val: HfclkSel) -> u8 {
        HfclkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PowerMode {
    #[doc = "RESET state"]
    RESET = 0,
    #[doc = "ACTIVE state"]
    ACTIVE = 0x01,
    #[doc = "SLEEP state"]
    SLEEP = 0x02,
    #[doc = "DEEP_SLEEP state"]
    DEEP_SLEEP = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PowerMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PowerMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PowerMode {
    #[inline(always)]
    fn from(val: u8) -> PowerMode {
        PowerMode::from_bits(val)
    }
}
impl From<PowerMode> for u8 {
    #[inline(always)]
    fn from(val: PowerMode) -> u8 {
        PowerMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PumpSel {
    #[doc = "No clock, connect to gnd"]
    GND = 0,
    #[doc = "Use main IMO output"]
    IMO = 0x01,
    #[doc = "Use clk_hf (using selected source after predivider but before prescaler)"]
    HFCLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl PumpSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PumpSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PumpSel {
    #[inline(always)]
    fn from(val: u8) -> PumpSel {
        PumpSel::from_bits(val)
    }
}
impl From<PumpSel> for u8 {
    #[inline(always)]
    fn from(val: PumpSel) -> u8 {
        PumpSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SysclkDiv {
    #[doc = "clk_sys= clk_hf/1"]
    NO_DIV = 0,
    #[doc = "clk_sys= clk_hf/2"]
    DIV_BY_2 = 0x01,
    #[doc = "clk_sys= clk_hf/4"]
    DIV_BY_4 = 0x02,
    #[doc = "clk_sys= clk_hf/8"]
    DIV_BY_8 = 0x03,
}
impl SysclkDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysclkDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysclkDiv {
    #[inline(always)]
    fn from(val: u8) -> SysclkDiv {
        SysclkDiv::from_bits(val)
    }
}
impl From<SysclkDiv> for u8 {
    #[inline(always)]
    fn from(val: SysclkDiv) -> u8 {
        SysclkDiv::to_bits(val)
    }
}
