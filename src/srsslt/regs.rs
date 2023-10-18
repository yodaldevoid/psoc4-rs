#[doc = "Clock DFT Mode Selection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkDftSelect(pub u32);
impl ClkDftSelect {
    #[doc = "Select signal for DFT output #0"]
    #[inline(always)]
    pub const fn dft_sel0(&self) -> super::vals::DftSel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::DftSel0::from_bits(val as u8)
    }
    #[doc = "Select signal for DFT output #0"]
    #[inline(always)]
    pub fn set_dft_sel0(&mut self, val: super::vals::DftSel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "DFT Output Divide Down."]
    #[inline(always)]
    pub const fn dft_div0(&self) -> super::vals::DftDiv0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DftDiv0::from_bits(val as u8)
    }
    #[doc = "DFT Output Divide Down."]
    #[inline(always)]
    pub fn set_dft_div0(&mut self, val: super::vals::DftDiv0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Edge sensitivity for in-line divider on output #0 (only relevant when DIV0>0)."]
    #[inline(always)]
    pub const fn dft_edge0(&self) -> super::vals::DftEdge0 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DftEdge0::from_bits(val as u8)
    }
    #[doc = "Edge sensitivity for in-line divider on output #0 (only relevant when DIV0>0)."]
    #[inline(always)]
    pub fn set_dft_edge0(&mut self, val: super::vals::DftEdge0) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Select signal for DFT output #1"]
    #[inline(always)]
    pub const fn dft_sel1(&self) -> super::vals::DftSel1 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::DftSel1::from_bits(val as u8)
    }
    #[doc = "Select signal for DFT output #1"]
    #[inline(always)]
    pub fn set_dft_sel1(&mut self, val: super::vals::DftSel1) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "DFT Output Divide Down."]
    #[inline(always)]
    pub const fn dft_div1(&self) -> super::vals::DftDiv1 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::DftDiv1::from_bits(val as u8)
    }
    #[doc = "DFT Output Divide Down."]
    #[inline(always)]
    pub fn set_dft_div1(&mut self, val: super::vals::DftDiv1) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Edge sensitivity for in-line divider on output #1 (only relevant when DIV1>0)."]
    #[inline(always)]
    pub const fn dft_edge1(&self) -> super::vals::DftEdge1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::DftEdge1::from_bits(val as u8)
    }
    #[doc = "Edge sensitivity for in-line divider on output #1 (only relevant when DIV1>0)."]
    #[inline(always)]
    pub fn set_dft_edge1(&mut self, val: super::vals::DftEdge1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
}
impl Default for ClkDftSelect {
    #[inline(always)]
    fn default() -> ClkDftSelect {
        ClkDftSelect(0)
    }
}
#[doc = "ILO Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkIloConfig(pub u32);
impl ClkIloConfig {
    #[doc = "Master enable for ILO oscillator. This bit is hardware set whenever the WDT_DISABLE_KEY is not set to the magic value."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for ILO oscillator. This bit is hardware set whenever the WDT_DISABLE_KEY is not set to the magic value."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkIloConfig {
    #[inline(always)]
    fn default() -> ClkIloConfig {
        ClkIloConfig(0)
    }
}
#[doc = "IMO Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkImoConfig(pub u32);
impl ClkImoConfig {
    #[doc = "Master enable for IMO oscillator. Clearing this bit will disable the IMO. Don't do this if the system is running off it."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master enable for IMO oscillator. Clearing this bit will disable the IMO. Don't do this if the system is running off it."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkImoConfig {
    #[inline(always)]
    fn default() -> ClkImoConfig {
        ClkImoConfig(0)
    }
}
#[doc = "IMO Frequency Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkImoSelect(pub u32);
impl ClkImoSelect {
    #[doc = "Select operating frequency"]
    #[inline(always)]
    pub const fn freq(&self) -> super::vals::Freq {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Freq::from_bits(val as u8)
    }
    #[doc = "Select operating frequency"]
    #[inline(always)]
    pub fn set_freq(&mut self, val: super::vals::Freq) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for ClkImoSelect {
    #[inline(always)]
    fn default() -> ClkImoSelect {
        ClkImoSelect(0)
    }
}
#[doc = "IMO Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkImoTrim1(pub u32);
impl ClkImoTrim1 {
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting in CLK_IMO_SELECT and stored in SFLASH. This field is hardware updated during USB osclock mode or when a WCO uses this mechanism for PLL locking the WCO. This is only available with devices that have either USB or a WCO. This field is mapped to the most significant bits of the IMO trim imo_clk_trim\\[10:3\\]. The step size of 1 LSB on this field is approximately 120 kHz."]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting in CLK_IMO_SELECT and stored in SFLASH. This field is hardware updated during USB osclock mode or when a WCO uses this mechanism for PLL locking the WCO. This is only available with devices that have either USB or a WCO. This field is mapped to the most significant bits of the IMO trim imo_clk_trim\\[10:3\\]. The step size of 1 LSB on this field is approximately 120 kHz."]
    #[inline(always)]
    pub fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ClkImoTrim1 {
    #[inline(always)]
    fn default() -> ClkImoTrim1 {
        ClkImoTrim1(0)
    }
}
#[doc = "IMO Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkImoTrim2(pub u32);
impl ClkImoTrim2 {
    #[doc = "Frequency trim bits. These bits are not trimmed during manufacturing and kept at 0 under normal operation. This field is hardware updated during USB osclock mode or when a WCO uses this mechanism for PLL locking the WCO. This is only available with devices that have either USB or a WCO. . This field is mapped to the least significant bits of the IMO trim imo_clk_trim\\[2:0\\]. The step size of 1 LSB on this field is approximately 15 kHz."]
    #[inline(always)]
    pub const fn fsoffset(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are not trimmed during manufacturing and kept at 0 under normal operation. This field is hardware updated during USB osclock mode or when a WCO uses this mechanism for PLL locking the WCO. This is only available with devices that have either USB or a WCO. . This field is mapped to the least significant bits of the IMO trim imo_clk_trim\\[2:0\\]. The step size of 1 LSB on this field is approximately 15 kHz."]
    #[inline(always)]
    pub fn set_fsoffset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for ClkImoTrim2 {
    #[inline(always)]
    fn default() -> ClkImoTrim2 {
        ClkImoTrim2(0)
    }
}
#[doc = "IMO Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkImoTrim3(pub u32);
impl ClkImoTrim3 {
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
}
impl Default for ClkImoTrim3 {
    #[inline(always)]
    fn default() -> ClkImoTrim3 {
        ClkImoTrim3(0)
    }
}
#[doc = "Clock Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSelect(pub u32);
impl ClkSelect {
    #[doc = "Selects a source for clk_hf and dsi_in\\[0\\]. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub const fn hfclk_sel(&self) -> super::vals::HfclkSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::HfclkSel::from_bits(val as u8)
    }
    #[doc = "Selects a source for clk_hf and dsi_in\\[0\\]. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn set_hfclk_sel(&mut self, val: super::vals::HfclkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects clk_hf predivider value."]
    #[inline(always)]
    pub const fn hfclk_div(&self) -> super::vals::HfclkDiv {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::HfclkDiv::from_bits(val as u8)
    }
    #[doc = "Selects clk_hf predivider value."]
    #[inline(always)]
    pub fn set_hfclk_div(&mut self, val: super::vals::HfclkDiv) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Selects clock source for charge pump clock. This clock is not guaranteed to be glitch free when changing any of its sources or settings."]
    #[inline(always)]
    pub const fn pump_sel(&self) -> super::vals::PumpSel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PumpSel::from_bits(val as u8)
    }
    #[doc = "Selects clock source for charge pump clock. This clock is not guaranteed to be glitch free when changing any of its sources or settings."]
    #[inline(always)]
    pub fn set_pump_sel(&mut self, val: super::vals::PumpSel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Select clk_sys prescaler value."]
    #[inline(always)]
    pub const fn sysclk_div(&self) -> super::vals::SysclkDiv {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SysclkDiv::from_bits(val as u8)
    }
    #[doc = "Select clk_sys prescaler value."]
    #[inline(always)]
    pub fn set_sysclk_div(&mut self, val: super::vals::SysclkDiv) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for ClkSelect {
    #[inline(always)]
    fn default() -> ClkSelect {
        ClkSelect(0)
    }
}
#[doc = "Power Mode Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrControl(pub u32);
impl PwrControl {
    #[doc = "Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
    #[inline(always)]
    pub const fn power_mode(&self) -> super::vals::PowerMode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PowerMode::from_bits(val as u8)
    }
    #[doc = "Current power mode of the device. Note that this field cannot be read in all power modes on actual silicon."]
    #[inline(always)]
    pub fn set_power_mode(&mut self, val: super::vals::PowerMode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
    #[inline(always)]
    pub const fn debug_session(&self) -> super::vals::DebugSession {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DebugSession::from_bits(val as u8)
    }
    #[doc = "Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
    #[inline(always)]
    pub fn set_debug_session(&mut self, val: super::vals::DebugSession) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates whether the low power mode regulator is ready to enter DEEPSLEEP mode. 0: If DEEPSLEEP mode is requested, device will enter SLEEP mode. When low power regulators are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP works as described."]
    #[inline(always)]
    pub const fn lpm_ready(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the low power mode regulator is ready to enter DEEPSLEEP mode. 0: If DEEPSLEEP mode is requested, device will enter SLEEP mode. When low power regulators are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP works as described."]
    #[inline(always)]
    pub fn set_lpm_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the die over temperature sensor. Must be enabled when using the TEMP_HIGH interrupt."]
    #[inline(always)]
    pub const fn over_temp_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the die over temperature sensor. Must be enabled when using the TEMP_HIGH interrupt."]
    #[inline(always)]
    pub fn set_over_temp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Over-temperature threshold. 0: TEMP_HIGH condition occurs between 120C and 125C. 1: TEMP_HIGH condition occurs between 60C and 75C (used for testing)."]
    #[inline(always)]
    pub const fn over_temp_thresh(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Over-temperature threshold. 0: TEMP_HIGH condition occurs between 120C and 125C. 1: TEMP_HIGH condition occurs between 60C and 75C (used for testing)."]
    #[inline(always)]
    pub fn set_over_temp_thresh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Spare AHB readback bits that are hooked to PWR_PWRSYS_TRIM1.SPARE_TRIM\\[1:0\\] through spare logic equivalent to bitwise inversion. Engineering only."]
    #[inline(always)]
    pub const fn spare(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Spare AHB readback bits that are hooked to PWR_PWRSYS_TRIM1.SPARE_TRIM\\[1:0\\] through spare logic equivalent to bitwise inversion. Engineering only."]
    #[inline(always)]
    pub fn set_spare(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Always write 0 except as noted below. PSoC4-S0 and Streetfighter CapSense products may set this bit if Vccd is provided externally (on Vccd pin). Setting this bit turns off the active regulator and will lead to system reset (BOD) unless both Vddd and Vccd pins are supplied externally. This register bit only resets for XRES, POR, or a detected BOD."]
    #[inline(always)]
    pub const fn ext_vccd(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Always write 0 except as noted below. PSoC4-S0 and Streetfighter CapSense products may set this bit if Vccd is provided externally (on Vccd pin). Setting this bit turns off the active regulator and will lead to system reset (BOD) unless both Vddd and Vccd pins are supplied externally. This register bit only resets for XRES, POR, or a detected BOD."]
    #[inline(always)]
    pub fn set_ext_vccd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for PwrControl {
    #[inline(always)]
    fn default() -> PwrControl {
        PwrControl(0)
    }
}
#[doc = "Power DDFT Mode Selection Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrDdftSelect(pub u32);
impl PwrDdftSelect {
    #[doc = "Select signal for power DDFT output #0"]
    #[inline(always)]
    pub const fn ddft0_sel(&self) -> super::vals::Ddft0sel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Ddft0sel::from_bits(val as u8)
    }
    #[doc = "Select signal for power DDFT output #0"]
    #[inline(always)]
    pub fn set_ddft0_sel(&mut self, val: super::vals::Ddft0sel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select signal for power DDFT output #1"]
    #[inline(always)]
    pub const fn ddft1_sel(&self) -> super::vals::Ddft1sel {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Ddft1sel::from_bits(val as u8)
    }
    #[doc = "Select signal for power DDFT output #1"]
    #[inline(always)]
    pub fn set_ddft1_sel(&mut self, val: super::vals::Ddft1sel) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
}
impl Default for PwrDdftSelect {
    #[inline(always)]
    fn default() -> PwrDdftSelect {
        PwrDdftSelect(0)
    }
}
#[doc = "Power System Key&Delay Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrKeyDelay(pub u32);
impl PwrKeyDelay {
    #[doc = "Delay to wait for references to settle on wakeup from deepsleep. BOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded. The default assumes the output of the predivider is 48MHz + 3 percent. Firmware may scale this setting according to the fastest actual clock frequency that can occur when waking from DEEPSLEEP."]
    #[inline(always)]
    pub const fn wakeup_holdoff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay to wait for references to settle on wakeup from deepsleep. BOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded. The default assumes the output of the predivider is 48MHz + 3 percent. Firmware may scale this setting according to the fastest actual clock frequency that can occur when waking from DEEPSLEEP."]
    #[inline(always)]
    pub fn set_wakeup_holdoff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for PwrKeyDelay {
    #[inline(always)]
    fn default() -> PwrKeyDelay {
        PwrKeyDelay(0)
    }
}
#[doc = "Power System Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrPwrsysTrim1(pub u32);
impl PwrPwrsysTrim1 {
    #[doc = "Trims the DeepSleep reference that is used by the DeepSleep regulator and DeepSleep power comparator."]
    #[inline(always)]
    pub const fn dpslp_ref_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Trims the DeepSleep reference that is used by the DeepSleep regulator and DeepSleep power comparator."]
    #[inline(always)]
    pub fn set_dpslp_ref_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Active-Reference temperature compensation trim (repurposed from spare bits). Bits \\[7:6\\] - trim the Active-Reference IREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = +80ppm/C 10: TC = -80ppm/C 11: TC = -150ppm/C Bits \\[5:4\\] - trim the Active-Reference VREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = -50ppm/C 10: TC = -80ppm/C 11: TC = +150ppm/C"]
    #[inline(always)]
    pub const fn spare_trim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Active-Reference temperature compensation trim (repurposed from spare bits). Bits \\[7:6\\] - trim the Active-Reference IREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = +80ppm/C 10: TC = -80ppm/C 11: TC = -150ppm/C Bits \\[5:4\\] - trim the Active-Reference VREF temperature coefficient (TC). 00: TC = 0 (unchanged) 01: TC = -50ppm/C 10: TC = -80ppm/C 11: TC = +150ppm/C"]
    #[inline(always)]
    pub fn set_spare_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for PwrPwrsysTrim1 {
    #[inline(always)]
    fn default() -> PwrPwrsysTrim1 {
        PwrPwrsysTrim1(0)
    }
}
#[doc = "Reset Cause Observation Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResCause(pub u32);
impl ResCause {
    #[doc = "A WatchDog Timer reset has occurred since last power cycle."]
    #[inline(always)]
    pub const fn reset_wdt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "A WatchDog Timer reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn set_reset_wdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
    #[inline(always)]
    pub const fn reset_prot_fault(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
    #[inline(always)]
    pub fn set_reset_prot_fault(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub const fn reset_soft(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn set_reset_soft(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for ResCause {
    #[inline(always)]
    fn default() -> ResCause {
        ResCause(0)
    }
}
#[doc = "SRSS Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrssIntr(pub u32);
impl SrssIntr {
    #[doc = "WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. Clearing this bit also feeds the watch dog. Missing 2 interrupts in a row will generate brown-out reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    pub const fn wdt_match(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. Clearing this bit also feeds the watch dog. Missing 2 interrupts in a row will generate brown-out reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    pub fn set_wdt_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Regulator over-temp interrupt. This interrupt can occur when a short circuit exists on the vccd pin or when extreme loads are applied on IO-cells causing the die to overheat. Firmware is encourage to shutdown all IO cells and then go to DeepSleep mode when this interrupt occurs if protection against such conditions is desired."]
    #[inline(always)]
    pub const fn temp_high(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Regulator over-temp interrupt. This interrupt can occur when a short circuit exists on the vccd pin or when extreme loads are applied on IO-cells causing the die to overheat. Firmware is encourage to shutdown all IO cells and then go to DeepSleep mode when this interrupt occurs if protection against such conditions is desired."]
    #[inline(always)]
    pub fn set_temp_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SrssIntr {
    #[inline(always)]
    fn default() -> SrssIntr {
        SrssIntr(0)
    }
}
#[doc = "SRSS Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrssIntrMask(pub u32);
impl SrssIntrMask {
    #[doc = "Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts."]
    #[inline(always)]
    pub const fn wdt_match(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clearing this bit will not forward the interrupt to the CPU. It will not, however, disable the WDT reset generation on 2 missed interrupts."]
    #[inline(always)]
    pub fn set_wdt_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Masks REG_OVERTEMP interrupt"]
    #[inline(always)]
    pub const fn temp_high(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Masks REG_OVERTEMP interrupt"]
    #[inline(always)]
    pub fn set_temp_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SrssIntrMask {
    #[inline(always)]
    fn default() -> SrssIntrMask {
        SrssIntrMask(0)
    }
}
#[doc = "SRSS Interrupt Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrssIntrSet(pub u32);
impl SrssIntrSet {
    #[doc = "Writing 1 to this bit internally sets the overtemp interrupt. This can be observed by reading SRSS_INTR.TEMP_HIGH. This bit always reads back as zero."]
    #[inline(always)]
    pub const fn temp_high(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 to this bit internally sets the overtemp interrupt. This can be observed by reading SRSS_INTR.TEMP_HIGH. This bit always reads back as zero."]
    #[inline(always)]
    pub fn set_temp_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SrssIntrSet {
    #[inline(always)]
    fn default() -> SrssIntrSet {
        SrssIntrSet(0)
    }
}
#[doc = "Test Mode Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TstMode(pub u32);
impl TstMode {
    #[doc = "0: SWD not active 1: SWD activated (Line Reset & Connect sequence passed) (Note: this bit replaces TST_CTRL.SWD_CONNECTED and is present in all M0S8 products except TSG4)"]
    #[inline(always)]
    pub const fn swd_connected(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0: SWD not active 1: SWD activated (Line Reset & Connect sequence passed) (Note: this bit replaces TST_CTRL.SWD_CONNECTED and is present in all M0S8 products except TSG4)"]
    #[inline(always)]
    pub fn set_swd_connected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Relevant only for parts that have the alternate XRES mechanism of overloading a GPIO pin temporarily as alternate XRES during test. When set, this bit blocks the alternate XRES function, such that the pin can be used for normal I/O or for ddft/adft observation. See SAS Part-V and Part-IX for details. This register bit only resets for XRES, POR, or a detected BOD."]
    #[inline(always)]
    pub const fn block_alt_xres(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Relevant only for parts that have the alternate XRES mechanism of overloading a GPIO pin temporarily as alternate XRES during test. When set, this bit blocks the alternate XRES function, such that the pin can be used for normal I/O or for ddft/adft observation. See SAS Part-V and Part-IX for details. This register bit only resets for XRES, POR, or a detected BOD."]
    #[inline(always)]
    pub fn set_block_alt_xres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This bit is set when a XRES test mode key is shifted in. It is the value of the test_key_dft_en signal. When this bit is set, the BootROM will not yield execution to the FLASH image (same function as setting TEST_MODE bit below)."]
    #[inline(always)]
    pub const fn test_key_dft_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is set when a XRES test mode key is shifted in. It is the value of the test_key_dft_en signal. When this bit is set, the BootROM will not yield execution to the FLASH image (same function as setting TEST_MODE bit below)."]
    #[inline(always)]
    pub fn set_test_key_dft_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Setting this bit will prevent BootROM from yielding execution to Flash image. 0: Normal operation mode 1: Test mode (any test mode)"]
    #[inline(always)]
    pub const fn test_mode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit will prevent BootROM from yielding execution to Flash image. 0: Normal operation mode 1: Test mode (any test mode)"]
    #[inline(always)]
    pub fn set_test_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TstMode {
    #[inline(always)]
    fn default() -> TstMode {
        TstMode(0)
    }
}
#[doc = "Watchdog Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtCounter(pub u32);
impl WdtCounter {
    #[doc = "Current value of WDT Counter"]
    #[inline(always)]
    pub const fn counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current value of WDT Counter"]
    #[inline(always)]
    pub fn set_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for WdtCounter {
    #[inline(always)]
    fn default() -> WdtCounter {
        WdtCounter(0)
    }
}
#[doc = "Watchdog Match Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtMatch(pub u32);
impl WdtMatch {
    #[doc = "Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub const fn match_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub fn set_match_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Note that certain products may enforce a minimum value for this register through design time configuration."]
    #[inline(always)]
    pub const fn ignore_bits(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Note that certain products may enforce a minimum value for this register through design time configuration."]
    #[inline(always)]
    pub fn set_ignore_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for WdtMatch {
    #[inline(always)]
    fn default() -> WdtMatch {
        WdtMatch(0)
    }
}
