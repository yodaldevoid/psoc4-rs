#[doc = "GPIO port control/configuration"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO port registers"]
    #[inline(always)]
    pub const fn prt(self, n: usize) -> Prt {
        assert!(n < 5usize);
        unsafe { Prt::from_ptr(self.ptr.add(0usize + n * 256usize) as _) }
    }
    #[doc = "Interrupt port cause register"]
    #[inline(always)]
    pub const fn intr_cause(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4096usize) as _) }
    }
}
#[doc = "GPIO port registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prt {
    ptr: *mut u8,
}
unsafe impl Send for Prt {}
unsafe impl Sync for Prt {}
impl Prt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port output data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Port IO pad state register"]
    #[inline(always)]
    pub const fn ps(self) -> crate::common::Reg<regs::Ps, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Port configuration register"]
    #[inline(always)]
    pub const fn pc(self) -> crate::common::Reg<regs::Pc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Port interrupt configuration register"]
    #[inline(always)]
    pub const fn intr_cfg(self) -> crate::common::Reg<regs::IntrCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Port interrupt status register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Port SIO configuration register"]
    #[inline(always)]
    pub const fn sio(self) -> crate::common::Reg<regs::Sio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Port configuration register 2"]
    #[inline(always)]
    pub const fn pc2(self) -> crate::common::Reg<regs::Pc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Port output data set register"]
    #[inline(always)]
    pub const fn dr_set(self) -> crate::common::Reg<regs::DrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Port output data clear register"]
    #[inline(always)]
    pub const fn dr_clr(self) -> crate::common::Reg<regs::DrClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Port output data invert register"]
    #[inline(always)]
    pub const fn dr_inv(self) -> crate::common::Reg<regs::DrInv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Reference generator configuration register"]
    #[inline(always)]
    pub const fn vrefgen(self) -> crate::common::Reg<regs::Vrefgen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
}
pub mod regs;
pub mod vals;
