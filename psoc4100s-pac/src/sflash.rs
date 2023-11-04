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
    #[doc = "Hibernate and DeepSleep wakeup values for PWR_KEY_DELAY"]
    #[inline(always)]
    pub const fn key_delay(self) -> crate::common::Reg<regs::KeyDelay, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(336usize) as _) }
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
    pub const fn csdv2_csd0_adc_trim(
        self,
    ) -> crate::common::Reg<regs::Csdv2csd0adcTrim, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "SAR Temperature Sensor Multiplication Factor and Offset"]
    #[inline(always)]
    pub const fn sar_temp_multiplier_offset(
        self,
    ) -> crate::common::Reg<regs::SarTempMultiplierOffset, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_tctrim_lt0_3(
        self,
    ) -> crate::common::Reg<regs::ImoTctrimLt03, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(460usize) as _) }
    }
    #[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_tctrim_lt4_7(
        self,
    ) -> crate::common::Reg<regs::ImoTctrimLt47, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(464usize) as _) }
    }
    #[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_tctrim_lt8_11(
        self,
    ) -> crate::common::Reg<regs::ImoTctrimLt811, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(468usize) as _) }
    }
    #[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_tctrim_lt12_15(
        self,
    ) -> crate::common::Reg<regs::ImoTctrimLt1215, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(472usize) as _) }
    }
    #[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_tctrim_lt16_19(
        self,
    ) -> crate::common::Reg<regs::ImoTctrimLt1619, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(476usize) as _) }
    }
    #[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_tctrim_lt20_23(
        self,
    ) -> crate::common::Reg<regs::ImoTctrimLt2023, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(480usize) as _) }
    }
    #[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_tctrim_lt24(
        self,
    ) -> crate::common::Reg<regs::ImoTctrimLt24, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(484usize) as _) }
    }
    #[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_trim_lt0_2(self) -> crate::common::Reg<regs::ImoTrimLt02, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(484usize) as _) }
    }
    #[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_trim_lt3_6(self) -> crate::common::Reg<regs::ImoTrimLt36, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(488usize) as _) }
    }
    #[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_trim_lt7_10(self) -> crate::common::Reg<regs::ImoTrimLt710, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(492usize) as _) }
    }
    #[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_trim_lt11_14(
        self,
    ) -> crate::common::Reg<regs::ImoTrimLt1114, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(496usize) as _) }
    }
    #[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_trim_lt15_18(
        self,
    ) -> crate::common::Reg<regs::ImoTrimLt1518, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(500usize) as _) }
    }
    #[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_trim_lt19_22(
        self,
    ) -> crate::common::Reg<regs::ImoTrimLt1922, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(504usize) as _) }
    }
    #[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
    #[inline(always)]
    pub const fn imo_trim_lt23_24(
        self,
    ) -> crate::common::Reg<regs::ImoTrimLt2324, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(508usize) as _) }
    }
}
pub mod regs;
