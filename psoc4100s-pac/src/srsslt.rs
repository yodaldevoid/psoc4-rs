#[doc = "System Resources Lite Subsystem"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srsslt {
    ptr: *mut u8,
}
unsafe impl Send for Srsslt {}
unsafe impl Sync for Srsslt {}
impl Srsslt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power Mode Control"]
    #[inline(always)]
    pub const fn pwr_control(self) -> crate::common::Reg<regs::PwrControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Power System Key&Delay Register"]
    #[inline(always)]
    pub const fn pwr_key_delay(self) -> crate::common::Reg<regs::PwrKeyDelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Power DDFT Mode Selection Register"]
    #[inline(always)]
    pub const fn pwr_ddft_select(
        self,
    ) -> crate::common::Reg<regs::PwrDdftSelect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Test Mode Control Register"]
    #[inline(always)]
    pub const fn tst_mode(self) -> crate::common::Reg<regs::TstMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Clock Select Register"]
    #[inline(always)]
    pub const fn clk_select(self) -> crate::common::Reg<regs::ClkSelect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "ILO Configuration"]
    #[inline(always)]
    pub const fn clk_ilo_config(self) -> crate::common::Reg<regs::ClkIloConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "IMO Configuration"]
    #[inline(always)]
    pub const fn clk_imo_config(self) -> crate::common::Reg<regs::ClkImoConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Clock DFT Mode Selection Register"]
    #[inline(always)]
    pub const fn clk_dft_select(self) -> crate::common::Reg<regs::ClkDftSelect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Watchdog Disable Key Register"]
    #[inline(always)]
    pub const fn wdt_disable_key(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Watchdog Counter Register"]
    #[inline(always)]
    pub const fn wdt_counter(self) -> crate::common::Reg<regs::WdtCounter, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Watchdog Match Register"]
    #[inline(always)]
    pub const fn wdt_match(self) -> crate::common::Reg<regs::WdtMatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "SRSS Interrupt Register"]
    #[inline(always)]
    pub const fn srss_intr(self) -> crate::common::Reg<regs::SrssIntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "SRSS Interrupt Set Register"]
    #[inline(always)]
    pub const fn srss_intr_set(self) -> crate::common::Reg<regs::SrssIntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "SRSS Interrupt Mask Register"]
    #[inline(always)]
    pub const fn srss_intr_mask(self) -> crate::common::Reg<regs::SrssIntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Reset Cause Observation Register"]
    #[inline(always)]
    pub const fn res_cause(self) -> crate::common::Reg<regs::ResCause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "IMO Frequency Select Register"]
    #[inline(always)]
    pub const fn clk_imo_select(self) -> crate::common::Reg<regs::ClkImoSelect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3848usize) as _) }
    }
    #[doc = "IMO Trim Register"]
    #[inline(always)]
    pub const fn clk_imo_trim1(self) -> crate::common::Reg<regs::ClkImoTrim1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3852usize) as _) }
    }
    #[doc = "IMO Trim Register"]
    #[inline(always)]
    pub const fn clk_imo_trim2(self) -> crate::common::Reg<regs::ClkImoTrim2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3856usize) as _) }
    }
    #[doc = "Power System Trim Register"]
    #[inline(always)]
    pub const fn pwr_pwrsys_trim1(
        self,
    ) -> crate::common::Reg<regs::PwrPwrsysTrim1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3860usize) as _) }
    }
    #[doc = "IMO Trim Register"]
    #[inline(always)]
    pub const fn clk_imo_trim3(self) -> crate::common::Reg<regs::ClkImoTrim3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3864usize) as _) }
    }
}
pub mod regs;
pub mod vals;
