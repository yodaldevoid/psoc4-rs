#[doc = "Peripheral Interconnect"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Peri {
    ptr: *mut u8,
}
unsafe impl Send for Peri {}
unsafe impl Sync for Peri {}
impl Peri {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Divider command register"]
    #[inline(always)]
    pub const fn div_cmd(self) -> crate::common::Reg<regs::DivCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Programmable clock control register"]
    #[inline(always)]
    pub const fn pclk_ctl(self, n: usize) -> crate::common::Reg<regs::PclkCtl, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
    #[doc = "Divider control register (for 16.0 divider)"]
    #[inline(always)]
    pub const fn div_16_ctl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Div16ctl, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize + n * 4usize) as _) }
    }
    #[doc = "Divider control register (for 16.5 divider)"]
    #[inline(always)]
    pub const fn div_16_5_ctl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Div165ctl, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize + n * 4usize) as _) }
    }
    #[doc = "Trigger control register"]
    #[inline(always)]
    pub const fn tr_ctl(self) -> crate::common::Reg<regs::TrCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1536usize) as _) }
    }
    #[doc = "Peripheral Interconnect trigger group control registers"]
    #[inline(always)]
    pub const fn tr_group(self, n: usize) -> TrGroup {
        assert!(n < 2usize);
        unsafe { TrGroup::from_ptr(self.ptr.add(8192usize + n * 512usize) as _) }
    }
}
#[doc = "Peripheral Interconnect trigger group control registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrGroup {
    ptr: *mut u8,
}
unsafe impl Send for TrGroup {}
unsafe impl Sync for TrGroup {}
impl TrGroup {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Trigger control register"]
    #[inline(always)]
    pub const fn tr_out_ctl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::TrOutCtl, crate::common::RW> {
        assert!(n < 128usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize + n * 4usize) as _) }
    }
}
pub mod regs;
