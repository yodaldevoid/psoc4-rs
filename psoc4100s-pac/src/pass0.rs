#[doc = "DSAB configuration"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsab {
    ptr: *mut u8,
}
unsafe impl Send for Dsab {}
unsafe impl Sync for Dsab {}
impl Dsab {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "global DSAB control"]
    #[inline(always)]
    pub const fn dsab_ctrl(self) -> crate::common::Reg<regs::DsabCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "DFT bits"]
    #[inline(always)]
    pub const fn dsab_dft(self) -> crate::common::Reg<regs::DsabDft, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
}
#[doc = "PASS top-level MMIO (DSABv2, INTR)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pass0 {
    ptr: *mut u8,
}
unsafe impl Send for Pass0 {}
unsafe impl Sync for Pass0 {}
impl Pass0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(self) -> crate::common::Reg<regs::IntrCause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "DFT control register"]
    #[inline(always)]
    pub const fn dft_ctrl(self) -> crate::common::Reg<regs::DftCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "PASS Control"]
    #[inline(always)]
    pub const fn pass_ctrl(self) -> crate::common::Reg<regs::PassCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "DSAB configuration"]
    #[inline(always)]
    pub const fn dsab(self) -> Dsab {
        unsafe { Dsab::from_ptr(self.ptr.add(3584usize) as _) }
    }
    #[doc = "DSAB Trim bits"]
    #[inline(always)]
    pub const fn dsab_trim(self) -> crate::common::Reg<regs::DsabTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3840usize) as _) }
    }
}
pub mod regs;
