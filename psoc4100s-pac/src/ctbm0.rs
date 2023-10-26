#[doc = "Continuous Time Block Mini"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctbm0 {
    ptr: *mut u8,
}
unsafe impl Send for Ctbm0 {}
unsafe impl Sync for Ctbm0 {}
impl Ctbm0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "global CTB and power control"]
    #[inline(always)]
    pub const fn ctb_ctrl(self) -> crate::common::Reg<regs::CtbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Opamp0 and resistor0 control"]
    #[inline(always)]
    pub const fn oa_res0_ctrl(self) -> crate::common::Reg<regs::OaRes0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Opamp1 and resistor1 control"]
    #[inline(always)]
    pub const fn oa_res1_ctrl(self) -> crate::common::Reg<regs::OaRes1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Comparator status"]
    #[inline(always)]
    pub const fn comp_stat(self) -> crate::common::Reg<regs::CompStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Interrupt request register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Interrupt request set register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<regs::IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Interrupt request mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<regs::IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Interrupt request masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<regs::IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Was 'Analog DfT controls', now used as Risk Mitigation bits (RMP)"]
    #[inline(always)]
    pub const fn dft_ctrl(self) -> crate::common::Reg<regs::DftCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Opamp0 switch control"]
    #[inline(always)]
    pub const fn oa0_sw(self) -> crate::common::Reg<regs::Oa0sw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Opamp0 switch control clear"]
    #[inline(always)]
    pub const fn oa0_sw_clear(self) -> crate::common::Reg<regs::Oa0swClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Opamp1 switch control"]
    #[inline(always)]
    pub const fn oa1_sw(self) -> crate::common::Reg<regs::Oa1sw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Opamp1 switch control clear"]
    #[inline(always)]
    pub const fn oa1_sw_clear(self) -> crate::common::Reg<regs::Oa1swClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "CTB bus switch control"]
    #[inline(always)]
    pub const fn ctb_sw_hw_ctrl(self) -> crate::common::Reg<regs::CtbSwHwCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "CTB bus switch control status"]
    #[inline(always)]
    pub const fn ctb_sw_status(self) -> crate::common::Reg<regs::CtbSwStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "Opamp0 trim control"]
    #[inline(always)]
    pub const fn oa0_offset_trim(
        self,
    ) -> crate::common::Reg<regs::Oa0offsetTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3840usize) as _) }
    }
    #[doc = "Opamp0 trim control"]
    #[inline(always)]
    pub const fn oa0_slope_offset_trim(
        self,
    ) -> crate::common::Reg<regs::Oa0slopeOffsetTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3844usize) as _) }
    }
    #[doc = "Opamp0 trim control"]
    #[inline(always)]
    pub const fn oa0_comp_trim(self) -> crate::common::Reg<regs::Oa0compTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3848usize) as _) }
    }
    #[doc = "Opamp1 trim control"]
    #[inline(always)]
    pub const fn oa1_offset_trim(
        self,
    ) -> crate::common::Reg<regs::Oa1offsetTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3852usize) as _) }
    }
    #[doc = "Opamp1 trim control"]
    #[inline(always)]
    pub const fn oa1_slope_offset_trim(
        self,
    ) -> crate::common::Reg<regs::Oa1slopeOffsetTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3856usize) as _) }
    }
    #[doc = "Opamp1 trim control"]
    #[inline(always)]
    pub const fn oa1_comp_trim(self) -> crate::common::Reg<regs::Oa1compTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3860usize) as _) }
    }
}
pub mod regs;
pub mod vals;
