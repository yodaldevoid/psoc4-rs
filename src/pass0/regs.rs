#[doc = "DFT control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DftCtrl(pub u32);
impl DftCtrl {
    #[doc = "Close the switch to connect the DSAB ADFT resistor to the AMUXBUS"]
    #[inline(always)]
    pub const fn dsab_adft_res_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Close the switch to connect the DSAB ADFT resistor to the AMUXBUS"]
    #[inline(always)]
    pub fn set_dsab_adft_res_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for DftCtrl {
    #[inline(always)]
    fn default() -> DftCtrl {
        DftCtrl(0)
    }
}
#[doc = "global DSAB control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DsabCtrl(pub u32);
impl DsabCtrl {
    #[doc = "DSAB DAC control field Nominal DSAB Output Current = CURRENT_SEL * 0.075 uA In products with SRSS-LITE, this setting impacts the CTB(m) offset. A value of 0x20 is used during factory trim and is required to maintain low offsets across temperature variation. If a different setting is used then a periodic re-trim of CTB(m) offset should be performed."]
    #[inline(always)]
    pub const fn current_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "DSAB DAC control field Nominal DSAB Output Current = CURRENT_SEL * 0.075 uA In products with SRSS-LITE, this setting impacts the CTB(m) offset. A value of 0x20 is used during factory trim and is required to maintain low offsets across temperature variation. If a different setting is used then a periodic re-trim of CTB(m) offset should be performed."]
    #[inline(always)]
    pub fn set_current_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn sel_out(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_sel_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "This field (along with SEL_OUT and ENABLED) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables."]
    #[inline(always)]
    pub const fn ref_swap_en(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This field (along with SEL_OUT and ENABLED) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables."]
    #[inline(always)]
    pub fn set_ref_swap_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "0 - DSAB PTAT generator is powered from DSAB regulator: VDDA must be at least 2.4V 1 - DSAB PTAT generator is pwoered directly from VDDA: VDDA cannot exceed 4.0V"]
    #[inline(always)]
    pub const fn bypass_mode_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "0 - DSAB PTAT generator is powered from DSAB regulator: VDDA must be at least 2.4V 1 - DSAB PTAT generator is pwoered directly from VDDA: VDDA cannot exceed 4.0V"]
    #[inline(always)]
    pub fn set_bypass_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Risk mitigation control 1 - Force start the startup circuit"]
    #[inline(always)]
    pub const fn startup_rm(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Risk mitigation control 1 - Force start the startup circuit"]
    #[inline(always)]
    pub fn set_startup_rm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This field (along with SEL_OUT and REF_SWAP_EN) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables. In SRSSLT devices, in active mode, this bit is overridden to '1', that is - it is always enabled in active mode."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This field (along with SEL_OUT and REF_SWAP_EN) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables. In SRSSLT devices, in active mode, this bit is overridden to '1', that is - it is always enabled in active mode."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DsabCtrl {
    #[inline(always)]
    fn default() -> DsabCtrl {
        DsabCtrl(0)
    }
}
#[doc = "DFT bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DsabDft(pub u32);
impl DsabDft {
    #[doc = "- 0: DSAB DFT disabled - 1: DSAB DFT enabled (connect output to amuxbus) 0001 - PTAT<0> 0010 - PTAT<1> 0011 - PTAT<1:0> 0100 - PTAT<2> 0111 - PTAT<2:0> 1000 - PTAT<3> 1111 - PTAT<3:0> 1001 - DSAB Reg Out"]
    #[inline(always)]
    pub const fn en_dft(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "- 0: DSAB DFT disabled - 1: DSAB DFT enabled (connect output to amuxbus) 0001 - PTAT<0> 0010 - PTAT<1> 0011 - PTAT<1:0> 0100 - PTAT<2> 0111 - PTAT<2:0> 1000 - PTAT<3> 1111 - PTAT<3:0> 1001 - DSAB Reg Out"]
    #[inline(always)]
    pub fn set_en_dft(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for DsabDft {
    #[inline(always)]
    fn default() -> DsabDft {
        DsabDft(0)
    }
}
#[doc = "DSAB Trim bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DsabTrim(pub u32);
impl DsabTrim {
    #[doc = "1111=lowest, 0000=highest"]
    #[inline(always)]
    pub const fn ibias_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "1111=lowest, 0000=highest"]
    #[inline(always)]
    pub fn set_ibias_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Risk mitigation bits"]
    #[inline(always)]
    pub const fn dsab_rmb_bits(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Risk mitigation bits"]
    #[inline(always)]
    pub fn set_dsab_rmb_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for DsabTrim {
    #[inline(always)]
    fn default() -> DsabTrim {
        DsabTrim(0)
    }
}
#[doc = "Interrupt cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause(pub u32);
impl IntrCause {
    #[doc = "CTB0 interrupt pending"]
    #[inline(always)]
    pub const fn ctb0_int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CTB0 interrupt pending"]
    #[inline(always)]
    pub fn set_ctb0_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CTB1 interrupt pending"]
    #[inline(always)]
    pub const fn ctb1_int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CTB1 interrupt pending"]
    #[inline(always)]
    pub fn set_ctb1_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTB2 interrupt pending"]
    #[inline(always)]
    pub const fn ctb2_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CTB2 interrupt pending"]
    #[inline(always)]
    pub fn set_ctb2_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CTB3 interrupt pending"]
    #[inline(always)]
    pub const fn ctb3_int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CTB3 interrupt pending"]
    #[inline(always)]
    pub fn set_ctb3_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntrCause {
    #[inline(always)]
    fn default() -> IntrCause {
        IntrCause(0)
    }
}
#[doc = "PASS Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PassCtrl(pub u32);
impl PassCtrl {
    #[doc = "- 0: Pump clk is clk_hf/2 - 1: Pump clk is selected from PMPCLK_SRC"]
    #[inline(always)]
    pub const fn pmpclk_byp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: Pump clk is clk_hf/2 - 1: Pump clk is selected from PMPCLK_SRC"]
    #[inline(always)]
    pub fn set_pmpclk_byp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "- 0: Pump clk is clk_hf - 1: Pump clk is direct from SRSS"]
    #[inline(always)]
    pub const fn pmpclk_src(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: Pump clk is clk_hf - 1: Pump clk is direct from SRSS"]
    #[inline(always)]
    pub fn set_pmpclk_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Risk mitigation bits"]
    #[inline(always)]
    pub const fn rmb_bits(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Risk mitigation bits"]
    #[inline(always)]
    pub fn set_rmb_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for PassCtrl {
    #[inline(always)]
    fn default() -> PassCtrl {
        PassCtrl(0)
    }
}
