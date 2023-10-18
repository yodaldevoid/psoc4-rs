#[doc = "Programmable IO configuration"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prgio {
    ptr: *mut u8,
}
unsafe impl Send for Prgio {}
unsafe impl Sync for Prgio {}
impl Prgio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Programmable IO port registers"]
    #[inline(always)]
    pub const fn prt(self, n: usize) -> Prt {
        assert!(n < 2usize);
        unsafe { Prt::from_ptr(self.ptr.add(0usize + n * 256usize) as _) }
    }
}
#[doc = "Programmable IO port registers"]
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
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Synchronization control register"]
    #[inline(always)]
    pub const fn sync_ctl(self) -> crate::common::Reg<regs::SyncCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "LUT component input selection"]
    #[inline(always)]
    pub const fn lut_sel(self, n: usize) -> crate::common::Reg<regs::LutSel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize + n * 4usize) as _) }
    }
    #[doc = "LUT component control register"]
    #[inline(always)]
    pub const fn lut_ctl(self, n: usize) -> crate::common::Reg<regs::LutCtl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize + n * 4usize) as _) }
    }
    #[doc = "Data unit component input selection"]
    #[inline(always)]
    pub const fn du_sel(self) -> crate::common::Reg<regs::DuSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "Data unit component control register"]
    #[inline(always)]
    pub const fn du_ctl(self) -> crate::common::Reg<regs::DuCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
}
pub mod regs;
