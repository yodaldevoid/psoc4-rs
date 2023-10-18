#[doc = "Timer/Counter/PWM Counter Module"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt {
    ptr: *mut u8,
}
unsafe impl Send for Cnt {}
unsafe impl Sync for Cnt {}
impl Cnt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Counter control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::CntCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Counter status register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Counter count register"]
    #[inline(always)]
    pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Counter compare/capture register"]
    #[inline(always)]
    pub const fn cc(self) -> crate::common::Reg<regs::Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Counter buffered compare/capture register"]
    #[inline(always)]
    pub const fn cc_buff(self) -> crate::common::Reg<regs::CcBuff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Counter period register"]
    #[inline(always)]
    pub const fn period(self) -> crate::common::Reg<regs::Period, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Counter buffered period register"]
    #[inline(always)]
    pub const fn period_buff(self) -> crate::common::Reg<regs::PeriodBuff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Counter trigger control register 0"]
    #[inline(always)]
    pub const fn tr_ctrl0(self) -> crate::common::Reg<regs::TrCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Counter trigger control register 1"]
    #[inline(always)]
    pub const fn tr_ctrl1(self) -> crate::common::Reg<regs::TrCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Counter trigger control register 2"]
    #[inline(always)]
    pub const fn tr_ctrl2(self) -> crate::common::Reg<regs::TrCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Interrupt request register."]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Interrupt set request register."]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<regs::IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Interrupt mask register."]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<regs::IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<regs::IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
}
#[doc = "Timer/Counter/PWM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcpwm {
    ptr: *mut u8,
}
unsafe impl Send for Tcpwm {}
unsafe impl Sync for Tcpwm {}
impl Tcpwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TCPWM control register 0."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::TcpwmCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "TCPWM command register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "TCPWM Counter interrupt cause register."]
    #[inline(always)]
    pub const fn intr_cause(self) -> crate::common::Reg<regs::IntrCause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Timer/Counter/PWM Counter Module"]
    #[inline(always)]
    pub const fn cnt(self, n: usize) -> Cnt {
        assert!(n < 5usize);
        unsafe { Cnt::from_ptr(self.ptr.add(256usize + n * 64usize) as _) }
    }
}
pub mod regs;
pub mod vals;
