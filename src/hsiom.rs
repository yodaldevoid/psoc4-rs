#[doc = "High Speed IO Matrix (HSIOM)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsiom {
    ptr: *mut u8,
}
unsafe impl Send for Hsiom {}
unsafe impl Sync for Hsiom {}
impl Hsiom {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "HSIOM port registers"]
    #[inline(always)]
    pub const fn prt(self, n: usize) -> Prt {
        assert!(n < 5usize);
        unsafe { Prt::from_ptr(self.ptr.add(0usize + n * 256usize) as _) }
    }
    #[doc = "Pump control"]
    #[inline(always)]
    pub const fn pump_ctl(self) -> crate::common::Reg<regs::PumpCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8192usize) as _) }
    }
    #[doc = "AMUX splitter cell control"]
    #[inline(always)]
    pub const fn amux_split_ctl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AmuxSplitCtl, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8448usize + n * 4usize) as _) }
    }
}
#[doc = "HSIOM port registers"]
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
    #[doc = "Port selection register"]
    #[inline(always)]
    pub const fn port_sel(self) -> crate::common::Reg<regs::PortSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
