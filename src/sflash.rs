#[doc = "Supervisory Flash"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sflash {
    ptr: *mut u8,
}
unsafe impl Send for Sflash {}
unsafe impl Sync for Sflash {}
impl Sflash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Silicon ID"]
    #[inline(always)]
    pub const fn silicon_id(self) -> crate::common::Reg<regs::SiliconId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(324usize) as _) }
    }
    #[doc = "Hibernate wakeup value for PWR_KEY_DELAY"]
    #[inline(always)]
    pub const fn hib_key_delay(self) -> crate::common::Reg<regs::HibKeyDelay, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(336usize) as _) }
    }
    #[doc = "DeepSleep wakeup value for PWR_KEY_DELAY"]
    #[inline(always)]
    pub const fn dpslp_key_delay(
        self,
    ) -> crate::common::Reg<regs::DpslpKeyDelay, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(338usize) as _) }
    }
    #[doc = "SWD pinout selector (not present in TSG4/TSG5-M)"]
    #[inline(always)]
    pub const fn swd_config(self) -> crate::common::Reg<regs::SwdConfig, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(340usize) as _) }
    }
    #[doc = "Listen Window Length"]
    #[inline(always)]
    pub const fn swd_listen(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(344usize) as _) }
    }
    #[doc = "Flash Image Start Address"]
    #[inline(always)]
    pub const fn flash_start(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(348usize) as _) }
    }
    #[doc = "CSDV2 CSD0 ADC TRIM 1"]
    #[inline(always)]
    pub const fn csdv2_csd0_adc_trim1(
        self,
    ) -> crate::common::Reg<regs::Csdv2csd0adcTrim1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "CSDV2 CSD0 ADC TRIM2"]
    #[inline(always)]
    pub const fn csdv2_csd0_adc_trim2(
        self,
    ) -> crate::common::Reg<regs::Csdv2csd0adcTrim2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "SAR Temperature Sensor Multiplication Factor"]
    #[inline(always)]
    pub const fn sar_temp_multiplier(
        self,
    ) -> crate::common::Reg<regs::SarTempMultiplier, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "SAR Temperature Sensor Offset"]
    #[inline(always)]
    pub const fn sar_temp_offset(
        self,
    ) -> crate::common::Reg<regs::SarTempOffset, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(358usize) as _) }
    }
    #[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_tctrim_lt(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ImoTctrimLt, crate::common::R> {
        assert!(n < 25usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(460usize + n * 1usize) as _) }
    }
    #[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_trim_lt(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ImoTrimLt, crate::common::R> {
        assert!(n < 25usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(485usize + n * 1usize) as _) }
    }
}
pub mod regs;
