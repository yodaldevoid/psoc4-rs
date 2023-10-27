#[doc = "WCO Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Force block into Low Power Mode: 0: Do not force low power mode (LPM) on 1: Force low power mode (LPM) on"]
    #[inline(always)]
    pub const fn lpm_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Force block into Low Power Mode: 0: Do not force low power mode (LPM) on 1: Force low power mode (LPM) on"]
    #[inline(always)]
    pub fn set_lpm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Automatically control low power mode (only relevant when LPM_EN=0): 0: Do not enter low power mode (LPM) in DeepSleep 1: Enter low power mode (LPM) in DeepSleep. The logic monitors !act_power_en to determine the device has entered DeepSleep."]
    #[inline(always)]
    pub const fn lpm_auto(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Automatically control low power mode (only relevant when LPM_EN=0): 0: Do not enter low power mode (LPM) in DeepSleep 1: Enter low power mode (LPM) in DeepSleep. The logic monitors !act_power_en to determine the device has entered DeepSleep."]
    #[inline(always)]
    pub fn set_lpm_auto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Disables the load resistor and allows external clock input for pad_xin"]
    #[inline(always)]
    pub const fn ext_input_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the load resistor and allows external clock input for pad_xin"]
    #[inline(always)]
    pub fn set_ext_input_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Test Mode Control bits enbus\\[7\\] - N/A enbus\\[6\\] - 1=enable both primary Beta Multipliers enbus\\[5\\] - N/A enbus\\[4\\] - N/A enbus\\[3\\] - Load Resistor Control enbus\\[2\\] - Load Resistor Control enbus\\[1\\] - Load Resistor Control enbus\\[0\\] - Load Resistor Control"]
    #[inline(always)]
    pub const fn enbus(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Test Mode Control bits enbus\\[7\\] - N/A enbus\\[6\\] - 1=enable both primary Beta Multipliers enbus\\[5\\] - N/A enbus\\[4\\] - N/A enbus\\[3\\] - Load Resistor Control enbus\\[2\\] - Load Resistor Control enbus\\[1\\] - Load Resistor Control enbus\\[0\\] - Load Resistor Control"]
    #[inline(always)]
    pub fn set_enbus(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Enable DPLL operation. The Oscillator is specified to be stable after 500 ms thus the DPLL should be asserted no sooner than that after IP_ENABLE is set."]
    #[inline(always)]
    pub const fn dpll_enable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DPLL operation. The Oscillator is specified to be stable after 500 ms thus the DPLL should be asserted no sooner than that after IP_ENABLE is set."]
    #[inline(always)]
    pub fn set_dpll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Master enable for IP - disables both WCO and DPLL"]
    #[inline(always)]
    pub const fn ip_enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for IP - disables both WCO and DPLL"]
    #[inline(always)]
    pub fn set_ip_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "WCO DPLL Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpll(pub u32);
impl Dpll {
    #[doc = "Multiplier to determine IMO frequency in multiples of the WCO frequency Fimo = (DPLL_MULT + 1) * Fwco"]
    #[inline(always)]
    pub const fn dpll_mult(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Multiplier to determine IMO frequency in multiples of the WCO frequency Fimo = (DPLL_MULT + 1) * Fwco"]
    #[inline(always)]
    pub fn set_dpll_mult(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "DPLL Loop Filter Integral Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
    #[inline(always)]
    pub const fn dpll_lf_igain(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "DPLL Loop Filter Integral Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
    #[inline(always)]
    pub fn set_dpll_lf_igain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "DPLL Loop Filter Proportionial Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
    #[inline(always)]
    pub const fn dpll_lf_pgain(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "DPLL Loop Filter Proportionial Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
    #[inline(always)]
    pub fn set_dpll_lf_pgain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "Maximum IMO offset allowed (used to prevent DPLL dynamics from selecting an IMO frequency that the logic cannot support)"]
    #[inline(always)]
    pub const fn dpll_lf_limit(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum IMO offset allowed (used to prevent DPLL dynamics from selecting an IMO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn set_dpll_lf_limit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 22usize)) | (((val as u32) & 0xff) << 22usize);
    }
}
impl Default for Dpll {
    #[inline(always)]
    fn default() -> Dpll {
        Dpll(0)
    }
}
#[doc = "WCO Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Indicates that output has transitioned - This bit is intended for Test Mode Only and is not a reliable indicator."]
    #[inline(always)]
    pub const fn out_blnk_a(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that output has transitioned - This bit is intended for Test Mode Only and is not a reliable indicator."]
    #[inline(always)]
    pub fn set_out_blnk_a(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "WCO Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trim(pub u32);
impl Trim {
    #[doc = "Amplifier GM setting - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode. 0x0 - 3370 nA 0x1 - 2620 nA 0x2 - 2250 nA 0x3 - 1500 nA 0x4 - 1870 nA 0x5 - 1120 nA 0x6 - 750 nA 0x7 - 0 nA"]
    #[inline(always)]
    pub const fn xgm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Amplifier GM setting - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode. 0x0 - 3370 nA 0x1 - 2620 nA 0x2 - 2250 nA 0x3 - 1500 nA 0x4 - 1870 nA 0x5 - 1120 nA 0x6 - 750 nA 0x7 - 0 nA"]
    #[inline(always)]
    pub fn set_xgm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "GM setting for LPM (bandwidth = DC/ms) - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode."]
    #[inline(always)]
    pub const fn lpm_gm(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "GM setting for LPM (bandwidth = DC/ms) - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode."]
    #[inline(always)]
    pub fn set_lpm_gm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Trim {
    #[inline(always)]
    fn default() -> Trim {
        Trim(0)
    }
}
#[doc = "Watchdog Counters Clock Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtClken(pub u32);
impl WdtClken {
    #[doc = "Enables the WCO clock for use by the WDT logic. Wait at least 4 WCO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_ILO_EN_FOR_WDT=1"]
    #[inline(always)]
    pub const fn clk_wco_en_for_wdt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the WCO clock for use by the WDT logic. Wait at least 4 WCO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_ILO_EN_FOR_WDT=1"]
    #[inline(always)]
    pub fn set_clk_wco_en_for_wdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the ILO clock for use by the WDT logic. Wait at least 4 ILO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_WCO_EN_FOR_WDT=1."]
    #[inline(always)]
    pub const fn clk_ilo_en_for_wdt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the ILO clock for use by the WDT logic. Wait at least 4 ILO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_WCO_EN_FOR_WDT=1."]
    #[inline(always)]
    pub fn set_clk_ilo_en_for_wdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for WdtClken {
    #[inline(always)]
    fn default() -> WdtClken {
        WdtClken(0)
    }
}
#[doc = "Watchdog Counters Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtConfig(pub u32);
impl WdtConfig {
    #[doc = "Watchdog Counter Action on Match (WDT_CTR0=WDT_MATCH0)."]
    #[inline(always)]
    pub const fn wdt_mode(&self, n: usize) -> super::vals::WdtMode {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        let val = (self.0 >> offs) & 0x03;
        super::vals::WdtMode::from_bits(val as u8)
    }
    #[doc = "Watchdog Counter Action on Match (WDT_CTR0=WDT_MATCH0)."]
    #[inline(always)]
    pub fn set_wdt_mode(&mut self, n: usize, val: super::vals::WdtMode) {
        assert!(n < 2usize);
        let offs = 0usize + n * 8usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
    #[doc = "Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match"]
    #[inline(always)]
    pub const fn wdt_clear(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Clear Watchdog Counter when WDT_CTR0=WDT_MATCH0. In other words WDT_CTR0 divides LFCLK by (WDT_MATCH0+1). 0: Free running counter 1: Clear on match"]
    #[inline(always)]
    pub fn set_wdt_clear(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 2usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
    #[inline(always)]
    pub const fn wdt_cascade0_1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Cascade Watchdog Counters 0,1. Counter 1 increments the cycle after WDT_CTR0=WDT_MATCH0. 0: Independent counters 1: Cascaded counters"]
    #[inline(always)]
    pub fn set_wdt_cascade0_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1"]
    #[inline(always)]
    pub const fn wdt_cascade1_2(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Cascade Watchdog Counters 1,2. Counter 2 increments the cycle after WDT_CTR1=WDT_MATCH1. It is allowed to cascade all three WDT counters. 0: Independent counters 1: Cascaded counters. When cascading all three counters, WDT_CLEAR1 must be 1"]
    #[inline(always)]
    pub fn set_wdt_cascade1_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Watchdog Counter 2 Mode."]
    #[inline(always)]
    pub const fn wdt_mode2(&self) -> super::vals::WdtMode2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::WdtMode2::from_bits(val as u8)
    }
    #[doc = "Watchdog Counter 2 Mode."]
    #[inline(always)]
    pub fn set_wdt_mode2(&mut self, val: super::vals::WdtMode2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bit to observe for WDT_INT2: 0: Assert when bit0 of WDT_CTR2 toggles (one int every tick) .. 31: Assert when bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
    #[inline(always)]
    pub const fn wdt_bits2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit to observe for WDT_INT2: 0: Assert when bit0 of WDT_CTR2 toggles (one int every tick) .. 31: Assert when bit31 of WDT_CTR2 toggles (one int every 2^31 ticks)"]
    #[inline(always)]
    pub fn set_wdt_bits2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn lfclk_sel(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_lfclk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for WdtConfig {
    #[inline(always)]
    fn default() -> WdtConfig {
        WdtConfig(0)
    }
}
#[doc = "Watchdog Counters Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtControl(pub u32);
impl WdtControl {
    #[doc = "Enable Counter 0 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
    #[inline(always)]
    pub const fn wdt_enable(&self, n: usize) -> bool {
        assert!(n < 3usize);
        let offs = 0usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enable Counter 0 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
    #[inline(always)]
    pub fn set_wdt_enable(&mut self, n: usize, val: bool) {
        assert!(n < 3usize);
        let offs = 0usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Indicates actual state of counter. May lag WDT_ENABLE0 by up to 3 LFCLK cycles. After changing WDT_ENABLE0, do not enter DEEPSLEEP mode until this field acknowledges the change."]
    #[inline(always)]
    pub const fn wdt_enabled(&self, n: usize) -> bool {
        assert!(n < 3usize);
        let offs = 1usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Indicates actual state of counter. May lag WDT_ENABLE0 by up to 3 LFCLK cycles. After changing WDT_ENABLE0, do not enter DEEPSLEEP mode until this field acknowledges the change."]
    #[inline(always)]
    pub fn set_wdt_enabled(&mut self, n: usize, val: bool) {
        assert!(n < 3usize);
        let offs = 1usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODEx=3. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
    #[inline(always)]
    pub const fn wdt_int(&self, n: usize) -> bool {
        assert!(n < 3usize);
        let offs = 2usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODEx=3. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
    #[inline(always)]
    pub fn set_wdt_int(&mut self, n: usize, val: bool) {
        assert!(n < 3usize);
        let offs = 2usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
    #[inline(always)]
    pub const fn wdt_reset(&self, n: usize) -> bool {
        assert!(n < 3usize);
        let offs = 3usize + n * 8usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
    #[inline(always)]
    pub fn set_wdt_reset(&mut self, n: usize, val: bool) {
        assert!(n < 3usize);
        let offs = 3usize + n * 8usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for WdtControl {
    #[inline(always)]
    fn default() -> WdtControl {
        WdtControl(0)
    }
}
#[doc = "Watchdog Counters 0/1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtCtrlow(pub u32);
impl WdtCtrlow {
    #[doc = "Current value of WDT Counter 0"]
    #[inline(always)]
    pub const fn wdt_ctr(&self, n: usize) -> u16 {
        assert!(n < 2usize);
        let offs = 0usize + n * 16usize;
        let val = (self.0 >> offs) & 0xffff;
        val as u16
    }
    #[doc = "Current value of WDT Counter 0"]
    #[inline(always)]
    pub fn set_wdt_ctr(&mut self, n: usize, val: u16) {
        assert!(n < 2usize);
        let offs = 0usize + n * 16usize;
        self.0 = (self.0 & !(0xffff << offs)) | (((val as u32) & 0xffff) << offs);
    }
}
impl Default for WdtCtrlow {
    #[inline(always)]
    fn default() -> WdtCtrlow {
        WdtCtrlow(0)
    }
}
#[doc = "Watchdog counter match values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtMatch(pub u32);
impl WdtMatch {
    #[doc = "Match value for Watchdog Counter 0"]
    #[inline(always)]
    pub const fn wdt_match(&self, n: usize) -> u16 {
        assert!(n < 2usize);
        let offs = 0usize + n * 16usize;
        let val = (self.0 >> offs) & 0xffff;
        val as u16
    }
    #[doc = "Match value for Watchdog Counter 0"]
    #[inline(always)]
    pub fn set_wdt_match(&mut self, n: usize, val: u16) {
        assert!(n < 2usize);
        let offs = 0usize + n * 16usize;
        self.0 = (self.0 & !(0xffff << offs)) | (((val as u32) & 0xffff) << offs);
    }
}
impl Default for WdtMatch {
    #[inline(always)]
    fn default() -> WdtMatch {
        WdtMatch(0)
    }
}
