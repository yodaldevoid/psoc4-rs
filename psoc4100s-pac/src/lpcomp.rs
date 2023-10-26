#[doc = "Low Power Comparators"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpcomp {
    ptr: *mut u8,
}
unsafe impl Send for Lpcomp {}
unsafe impl Sync for Lpcomp {}
impl Lpcomp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ID & Revision"]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "LPCOMP Configuration Register"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "LPCOMP Interrupt request register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "LPCOMP Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<regs::IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "LPCOMP Interrupt request mask"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<regs::IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "LPCOMP Interrupt request masked"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<regs::IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "LPCOMP Trim Register"]
    #[inline(always)]
    pub const fn trim1(self) -> crate::common::Reg<regs::Trim1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65280usize) as _) }
    }
    #[doc = "LPCOMP Trim Register"]
    #[inline(always)]
    pub const fn trim2(self) -> crate::common::Reg<regs::Trim2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65284usize) as _) }
    }
    #[doc = "LPCOMP Trim Register"]
    #[inline(always)]
    pub const fn trim3(self) -> crate::common::Reg<regs::Trim3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65288usize) as _) }
    }
    #[doc = "LPCOMP Trim Register"]
    #[inline(always)]
    pub const fn trim4(self) -> crate::common::Reg<regs::Trim4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(65292usize) as _) }
    }
}
pub mod regs;
pub mod vals;
