#[doc = "Flash Control Interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spcif {
    ptr: *mut u8,
}
unsafe impl Send for Spcif {}
unsafe impl Sync for Spcif {}
impl Spcif {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash/NVL geometry information"]
    #[inline(always)]
    pub const fn geometry(self) -> crate::common::Reg<regs::Geometry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "NVL write data register"]
    #[inline(always)]
    pub const fn nvl_wr_data(self) -> crate::common::Reg<regs::NvlWrData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "SPCIF interrupt request register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2032usize) as _) }
    }
    #[doc = "SPCIF interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<regs::IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2036usize) as _) }
    }
    #[doc = "SPCIF interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<regs::IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2040usize) as _) }
    }
    #[doc = "SPCIF interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<regs::IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2044usize) as _) }
    }
}
pub mod regs;
