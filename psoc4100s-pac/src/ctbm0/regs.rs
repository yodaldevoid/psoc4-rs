#[doc = "Comparator status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompStat(pub u32);
impl CompStat {
    #[doc = "Opamp0 current comparator status"]
    #[inline(always)]
    pub const fn oa0_comp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 current comparator status"]
    #[inline(always)]
    pub fn set_oa0_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Opamp1 current comparator status"]
    #[inline(always)]
    pub const fn oa1_comp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 current comparator status"]
    #[inline(always)]
    pub fn set_oa1_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for CompStat {
    #[inline(always)]
    fn default() -> CompStat {
        CompStat(0)
    }
}
#[doc = "global CTB and power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtbCtrl(pub u32);
impl CtbCtrl {
    #[doc = "- 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub const fn deepsleep_on(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn set_deepsleep_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "- 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtbCtrl {
    #[inline(always)]
    fn default() -> CtbCtrl {
        CtbCtrl(0)
    }
}
#[doc = "CTB bus switch control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtbSwHwCtrl(pub u32);
impl CtbSwHwCtrl {
    #[doc = "for P22, D51 (dsi_out\\[2\\])"]
    #[inline(always)]
    pub const fn p2_hw_ctrl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "for P22, D51 (dsi_out\\[2\\])"]
    #[inline(always)]
    pub fn set_p2_hw_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for P33, D52, D62 (dsi_out\\[3\\])"]
    #[inline(always)]
    pub const fn p3_hw_ctrl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for P33, D52, D62 (dsi_out\\[3\\])"]
    #[inline(always)]
    pub fn set_p3_hw_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for CtbSwHwCtrl {
    #[inline(always)]
    fn default() -> CtbSwHwCtrl {
        CtbSwHwCtrl(0)
    }
}
#[doc = "CTB bus switch control status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtbSwStatus(pub u32);
impl CtbSwStatus {
    #[doc = "see OA0O_D51 bit in OA0_SW"]
    #[inline(always)]
    pub const fn oa0o_d51_stat(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "see OA0O_D51 bit in OA0_SW"]
    #[inline(always)]
    pub fn set_oa0o_d51_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "see OA1O_D52 bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1o_d52_stat(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "see OA1O_D52 bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1o_d52_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "see OA1O_D62 bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1o_d62_stat(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "see OA1O_D62 bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1o_d62_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CtbSwStatus {
    #[inline(always)]
    fn default() -> CtbSwStatus {
        CtbSwStatus(0)
    }
}
#[doc = "Was 'Analog DfT controls', now used as Risk Mitigation bits (RMP)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DftCtrl(pub u32);
impl DftCtrl {
    #[doc = "this bit is combined with bit 31, to form RMP\\[3:0\\], it must always be written with '3' for correct operation."]
    #[inline(always)]
    pub const fn dft_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "this bit is combined with bit 31, to form RMP\\[3:0\\], it must always be written with '3' for correct operation."]
    #[inline(always)]
    pub fn set_dft_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "this bit is combined with the 3 bits 2:0, to form RMP\\[3:0\\]"]
    #[inline(always)]
    pub const fn dft_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "this bit is combined with the 3 bits 2:0, to form RMP\\[3:0\\]"]
    #[inline(always)]
    pub fn set_dft_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DftCtrl {
    #[inline(always)]
    fn default() -> DftCtrl {
        DftCtrl(0)
    }
}
#[doc = "Interrupt request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn comp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_comp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn comp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_comp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt request mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp0_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp0_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp1_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp1_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Interrupt request masked"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn comp0_masked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_comp0_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn comp1_masked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_comp1_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Interrupt request set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp0_set(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp0_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp1_set(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp1_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Opamp0 trim control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa0compTrim(pub u32);
impl Oa0compTrim {
    #[doc = "Opamp 0 Compensation Capacitor Trim"]
    #[inline(always)]
    pub const fn oa0_comp_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Opamp 0 Compensation Capacitor Trim"]
    #[inline(always)]
    pub fn set_oa0_comp_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Oa0compTrim {
    #[inline(always)]
    fn default() -> Oa0compTrim {
        Oa0compTrim(0)
    }
}
#[doc = "Opamp0 trim control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa0offsetTrim(pub u32);
impl Oa0offsetTrim {
    #[doc = "Opamp0 offset trim"]
    #[inline(always)]
    pub const fn oa0_offset_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Opamp0 offset trim"]
    #[inline(always)]
    pub fn set_oa0_offset_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Oa0offsetTrim {
    #[inline(always)]
    fn default() -> Oa0offsetTrim {
        Oa0offsetTrim(0)
    }
}
#[doc = "Opamp0 trim control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa0slopeOffsetTrim(pub u32);
impl Oa0slopeOffsetTrim {
    #[doc = "Opamp0 slope offset drift trim"]
    #[inline(always)]
    pub const fn oa0_slope_offset_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Opamp0 slope offset drift trim"]
    #[inline(always)]
    pub fn set_oa0_slope_offset_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Oa0slopeOffsetTrim {
    #[inline(always)]
    fn default() -> Oa0slopeOffsetTrim {
        Oa0slopeOffsetTrim(0)
    }
}
#[doc = "Opamp0 switch control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa0sw(pub u32);
impl Oa0sw {
    #[doc = "Opamp0 positive terminal amuxbusa"]
    #[inline(always)]
    pub const fn oa0p_a00(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 positive terminal amuxbusa"]
    #[inline(always)]
    pub fn set_oa0p_a00(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Opamp0 positive terminal P0"]
    #[inline(always)]
    pub const fn oa0p_a20(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 positive terminal P0"]
    #[inline(always)]
    pub fn set_oa0p_a20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Opamp0 positive terminal ctbbus0"]
    #[inline(always)]
    pub const fn oa0p_a30(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 positive terminal ctbbus0"]
    #[inline(always)]
    pub fn set_oa0p_a30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Opamp0 negative terminal P1"]
    #[inline(always)]
    pub const fn oa0m_a11(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 negative terminal P1"]
    #[inline(always)]
    pub fn set_oa0m_a11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Opamp0 negative terminal Opamp0 output"]
    #[inline(always)]
    pub const fn oa0m_a81(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 negative terminal Opamp0 output"]
    #[inline(always)]
    pub fn set_oa0m_a81(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Opamp0 output sarbus0 (ctbbus2 in CTB)"]
    #[inline(always)]
    pub const fn oa0o_d51(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 output sarbus0 (ctbbus2 in CTB)"]
    #[inline(always)]
    pub fn set_oa0o_d51(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Opamp0 output switch to short 1x with 10x drive"]
    #[inline(always)]
    pub const fn oa0o_d81(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 output switch to short 1x with 10x drive"]
    #[inline(always)]
    pub fn set_oa0o_d81(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Oa0sw {
    #[inline(always)]
    fn default() -> Oa0sw {
        Oa0sw(0)
    }
}
#[doc = "Opamp0 switch control clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa0swClear(pub u32);
impl Oa0swClear {
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub const fn oa0p_a00(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn set_oa0p_a00(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub const fn oa0p_a20(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn set_oa0p_a20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub const fn oa0p_a30(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn set_oa0p_a30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub const fn oa0m_a11(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn set_oa0m_a11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub const fn oa0m_a81(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn set_oa0m_a81(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub const fn oa0o_d51(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn set_oa0o_d51(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub const fn oa0o_d81(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn set_oa0o_d81(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Oa0swClear {
    #[inline(always)]
    fn default() -> Oa0swClear {
        Oa0swClear(0)
    }
}
#[doc = "Opamp1 trim control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa1compTrim(pub u32);
impl Oa1compTrim {
    #[doc = "Opamp 1 Compensation Capacitor Trim"]
    #[inline(always)]
    pub const fn oa1_comp_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Opamp 1 Compensation Capacitor Trim"]
    #[inline(always)]
    pub fn set_oa1_comp_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Oa1compTrim {
    #[inline(always)]
    fn default() -> Oa1compTrim {
        Oa1compTrim(0)
    }
}
#[doc = "Opamp1 trim control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa1offsetTrim(pub u32);
impl Oa1offsetTrim {
    #[doc = "Opamp1 offset trim"]
    #[inline(always)]
    pub const fn oa1_offset_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Opamp1 offset trim"]
    #[inline(always)]
    pub fn set_oa1_offset_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Oa1offsetTrim {
    #[inline(always)]
    fn default() -> Oa1offsetTrim {
        Oa1offsetTrim(0)
    }
}
#[doc = "Opamp1 trim control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa1slopeOffsetTrim(pub u32);
impl Oa1slopeOffsetTrim {
    #[doc = "Opamp1 slope offset drift trim"]
    #[inline(always)]
    pub const fn oa1_slope_offset_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Opamp1 slope offset drift trim"]
    #[inline(always)]
    pub fn set_oa1_slope_offset_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Oa1slopeOffsetTrim {
    #[inline(always)]
    fn default() -> Oa1slopeOffsetTrim {
        Oa1slopeOffsetTrim(0)
    }
}
#[doc = "Opamp1 switch control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa1sw(pub u32);
impl Oa1sw {
    #[doc = "Opamp1 positive terminal amuxbusb"]
    #[inline(always)]
    pub const fn oa1p_a03(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 positive terminal amuxbusb"]
    #[inline(always)]
    pub fn set_oa1p_a03(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Opamp1 positive terminal P5"]
    #[inline(always)]
    pub const fn oa1p_a13(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 positive terminal P5"]
    #[inline(always)]
    pub fn set_oa1p_a13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Opamp1 positive terminal ctbbus1"]
    #[inline(always)]
    pub const fn oa1p_a43(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 positive terminal ctbbus1"]
    #[inline(always)]
    pub fn set_oa1p_a43(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Opamp1 negative terminal P4"]
    #[inline(always)]
    pub const fn oa1m_a22(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 negative terminal P4"]
    #[inline(always)]
    pub fn set_oa1m_a22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Opamp1 negative terminal Opamp1 output"]
    #[inline(always)]
    pub const fn oa1m_a82(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 negative terminal Opamp1 output"]
    #[inline(always)]
    pub fn set_oa1m_a82(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Opamp1 output sarbus0 (ctbbus2 in CTB)"]
    #[inline(always)]
    pub const fn oa1o_d52(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 output sarbus0 (ctbbus2 in CTB)"]
    #[inline(always)]
    pub fn set_oa1o_d52(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Opamp1 output sarbus1 (ctbbus3 in CTB)"]
    #[inline(always)]
    pub const fn oa1o_d62(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 output sarbus1 (ctbbus3 in CTB)"]
    #[inline(always)]
    pub fn set_oa1o_d62(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Opamp1 output switch to short 1x with 10x drive"]
    #[inline(always)]
    pub const fn oa1o_d82(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 output switch to short 1x with 10x drive"]
    #[inline(always)]
    pub fn set_oa1o_d82(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Oa1sw {
    #[inline(always)]
    fn default() -> Oa1sw {
        Oa1sw(0)
    }
}
#[doc = "Opamp1 switch control clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oa1swClear(pub u32);
impl Oa1swClear {
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1p_a03(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1p_a03(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1p_a13(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1p_a13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1p_a43(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1p_a43(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1m_a22(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1m_a22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1m_a82(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1m_a82(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1o_d52(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1o_d52(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1o_d62(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1o_d62(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub const fn oa1o_d82(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn set_oa1o_d82(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Oa1swClear {
    #[inline(always)]
    fn default() -> Oa1swClear {
        Oa1swClear(0)
    }
}
#[doc = "Opamp0 and resistor0 control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OaRes0ctrl(pub u32);
impl OaRes0ctrl {
    #[doc = "Opamp0 power level"]
    #[inline(always)]
    pub const fn oa0_pwr_mode(&self) -> super::vals::Oa0pwrMode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Oa0pwrMode::from_bits(val as u8)
    }
    #[doc = "Opamp0 power level"]
    #[inline(always)]
    pub fn set_oa0_pwr_mode(&mut self, val: super::vals::Oa0pwrMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Opamp0 output strenght select 0=1x, 1=10x"]
    #[inline(always)]
    pub const fn oa0_drive_str_sel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 output strenght select 0=1x, 1=10x"]
    #[inline(always)]
    pub fn set_oa0_drive_str_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Opamp0 comparator enable"]
    #[inline(always)]
    pub const fn oa0_comp_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 comparator enable"]
    #[inline(always)]
    pub fn set_oa0_comp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Opamp0 hysteresis enable (10mV)"]
    #[inline(always)]
    pub const fn oa0_hyst_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 hysteresis enable (10mV)"]
    #[inline(always)]
    pub fn set_oa0_hyst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Opamp0 bypass comparator output synchronization for DSI (trigger) output: 0=synchronize (level or pulse), 1=bypass (output async)"]
    #[inline(always)]
    pub const fn oa0_bypass_dsi_sync(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 bypass comparator output synchronization for DSI (trigger) output: 0=synchronize (level or pulse), 1=bypass (output async)"]
    #[inline(always)]
    pub fn set_oa0_bypass_dsi_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Opamp0 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA0_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
    #[inline(always)]
    pub const fn oa0_dsi_level(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA0_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
    #[inline(always)]
    pub fn set_oa0_dsi_level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Opamp0 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
    #[inline(always)]
    pub const fn oa0_compint(&self) -> super::vals::Oa0compint {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Oa0compint::from_bits(val as u8)
    }
    #[doc = "Opamp0 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
    #[inline(always)]
    pub fn set_oa0_compint(&mut self, val: super::vals::Oa0compint) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Opamp0 pump enable"]
    #[inline(always)]
    pub const fn oa0_pump_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp0 pump enable"]
    #[inline(always)]
    pub fn set_oa0_pump_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for OaRes0ctrl {
    #[inline(always)]
    fn default() -> OaRes0ctrl {
        OaRes0ctrl(0)
    }
}
#[doc = "Opamp1 and resistor1 control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OaRes1ctrl(pub u32);
impl OaRes1ctrl {
    #[doc = "Opamp1 power level: see description of OA0_PWR_MODE"]
    #[inline(always)]
    pub const fn oa1_pwr_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Opamp1 power level: see description of OA0_PWR_MODE"]
    #[inline(always)]
    pub fn set_oa1_pwr_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Opamp1 output strenght select 0=1x, 1=10x"]
    #[inline(always)]
    pub const fn oa1_drive_str_sel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 output strenght select 0=1x, 1=10x"]
    #[inline(always)]
    pub fn set_oa1_drive_str_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Opamp1 comparator enable"]
    #[inline(always)]
    pub const fn oa1_comp_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 comparator enable"]
    #[inline(always)]
    pub fn set_oa1_comp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Opamp1 hysteresis enable (10mV)"]
    #[inline(always)]
    pub const fn oa1_hyst_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 hysteresis enable (10mV)"]
    #[inline(always)]
    pub fn set_oa1_hyst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize, 1=bypass"]
    #[inline(always)]
    pub const fn oa1_bypass_dsi_sync(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize, 1=bypass"]
    #[inline(always)]
    pub fn set_oa1_bypass_dsi_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Opamp1 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA1_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
    #[inline(always)]
    pub const fn oa1_dsi_level(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA1_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
    #[inline(always)]
    pub fn set_oa1_dsi_level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Opamp1 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
    #[inline(always)]
    pub const fn oa1_compint(&self) -> super::vals::Oa1compint {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Oa1compint::from_bits(val as u8)
    }
    #[doc = "Opamp1 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
    #[inline(always)]
    pub fn set_oa1_compint(&mut self, val: super::vals::Oa1compint) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Opamp1 pump enable"]
    #[inline(always)]
    pub const fn oa1_pump_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 pump enable"]
    #[inline(always)]
    pub fn set_oa1_pump_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for OaRes1ctrl {
    #[inline(always)]
    fn default() -> OaRes1ctrl {
        OaRes1ctrl(0)
    }
}
