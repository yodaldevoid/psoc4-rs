#[doc = "32KHz Oscillator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wco {
    ptr: *mut u8,
}
unsafe impl Send for Wco {}
unsafe impl Sync for Wco {}
impl Wco {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "WCO Configuration Register"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "WCO Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "WCO DPLL Register"]
    #[inline(always)]
    pub const fn dpll(self) -> crate::common::Reg<regs::Dpll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Watchdog Counters 0/1"]
    #[inline(always)]
    pub const fn wdt_ctrlow(self) -> crate::common::Reg<regs::WdtCtrlow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Watchdog Counter 2"]
    #[inline(always)]
    pub const fn wdt_ctrhigh(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "Watchdog counter match values"]
    #[inline(always)]
    pub const fn wdt_match(self) -> crate::common::Reg<regs::WdtMatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[doc = "Watchdog Counters Configuration"]
    #[inline(always)]
    pub const fn wdt_config(self) -> crate::common::Reg<regs::WdtConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize) as _) }
    }
    #[doc = "Watchdog Counters Control"]
    #[inline(always)]
    pub const fn wdt_control(self) -> crate::common::Reg<regs::WdtControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(528usize) as _) }
    }
    #[doc = "Watchdog Counters Clock Enable"]
    #[inline(always)]
    pub const fn wdt_clken(self) -> crate::common::Reg<regs::WdtClken, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(532usize) as _) }
    }
    #[doc = "WCO Trim Register"]
    #[inline(always)]
    pub const fn trim(self) -> crate::common::Reg<regs::Trim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3840usize) as _) }
    }
}
pub mod regs;
pub mod vals;
