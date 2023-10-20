#[doc = "CSDV2 CSD0 ADC TRIM 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csdv2csd0adcTrim1(pub u32);
impl Csdv2csd0adcTrim1 {
    #[doc = "Low byte of CSDv2 Calibration"]
    #[inline(always)]
    pub const fn csd_adc_cal_lsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low byte of CSDv2 Calibration"]
    #[inline(always)]
    pub fn set_csd_adc_cal_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Csdv2csd0adcTrim1 {
    #[inline(always)]
    fn default() -> Csdv2csd0adcTrim1 {
        Csdv2csd0adcTrim1(0)
    }
}
#[doc = "CSDV2 CSD0 ADC TRIM2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csdv2csd0adcTrim2(pub u32);
impl Csdv2csd0adcTrim2 {
    #[doc = "High byte of CSDv2 Calibration"]
    #[inline(always)]
    pub const fn csd_adc_cal_msb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "High byte of CSDv2 Calibration"]
    #[inline(always)]
    pub fn set_csd_adc_cal_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Csdv2csd0adcTrim2 {
    #[inline(always)]
    fn default() -> Csdv2csd0adcTrim2 {
        Csdv2csd0adcTrim2(0)
    }
}
#[doc = "DeepSleep wakeup value for PWR_KEY_DELAY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpslpKeyDelay(pub u32);
impl DpslpKeyDelay {
    #[doc = "Delay (in 12MHz IMO clock cycles) to wait for references to settle on wakeup from hibernate/ deepsleep. PBOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded."]
    #[inline(always)]
    pub const fn wakeup_holdoff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay (in 12MHz IMO clock cycles) to wait for references to settle on wakeup from hibernate/ deepsleep. PBOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded."]
    #[inline(always)]
    pub fn set_wakeup_holdoff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for DpslpKeyDelay {
    #[inline(always)]
    fn default() -> DpslpKeyDelay {
        DpslpKeyDelay(0)
    }
}
#[doc = "Hibernate wakeup value for PWR_KEY_DELAY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HibKeyDelay(pub u32);
impl HibKeyDelay {
    #[doc = "Delay (in 12MHz IMO clock cycles) to wait for references to settle on wakeup from hibernate/ deepsleep. PBOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded."]
    #[inline(always)]
    pub const fn wakeup_holdoff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay (in 12MHz IMO clock cycles) to wait for references to settle on wakeup from hibernate/ deepsleep. PBOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded."]
    #[inline(always)]
    pub fn set_wakeup_holdoff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for HibKeyDelay {
    #[inline(always)]
    fn default() -> HibKeyDelay {
        HibKeyDelay(0)
    }
}
#[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTctrimLt(pub u32);
impl ImoTctrimLt {
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
}
impl Default for ImoTctrimLt {
    #[inline(always)]
    fn default() -> ImoTctrimLt {
        ImoTctrimLt(0)
    }
}
#[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTrimLt(pub u32);
impl ImoTrimLt {
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ImoTrimLt {
    #[inline(always)]
    fn default() -> ImoTrimLt {
        ImoTrimLt(0)
    }
}
#[doc = "SAR Temperature Sensor Multiplication Factor"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SarTempMultiplier(pub u32);
impl SarTempMultiplier {
    #[doc = "Multiplier value for SAR temperature sensor in fixed point 0.16 format. Note: this field exists in products that contain SAR (m0s8sar) only."]
    #[inline(always)]
    pub const fn temp_multiplier(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Multiplier value for SAR temperature sensor in fixed point 0.16 format. Note: this field exists in products that contain SAR (m0s8sar) only."]
    #[inline(always)]
    pub fn set_temp_multiplier(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SarTempMultiplier {
    #[inline(always)]
    fn default() -> SarTempMultiplier {
        SarTempMultiplier(0)
    }
}
#[doc = "SAR Temperature Sensor Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SarTempOffset(pub u32);
impl SarTempOffset {
    #[doc = "Offset value for SAR temperature sensor in fixed point 10.6 format. Note: this field exists in products that contain SAR (m0s8sar) only."]
    #[inline(always)]
    pub const fn temp_offset(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset value for SAR temperature sensor in fixed point 10.6 format. Note: this field exists in products that contain SAR (m0s8sar) only."]
    #[inline(always)]
    pub fn set_temp_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SarTempOffset {
    #[inline(always)]
    fn default() -> SarTempOffset {
        SarTempOffset(0)
    }
}
#[doc = "Silicon ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SiliconId(pub u32);
impl SiliconId {
    #[doc = "Silicon ID"]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Silicon ID"]
    #[inline(always)]
    pub fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for SiliconId {
    #[inline(always)]
    fn default() -> SiliconId {
        SiliconId(0)
    }
}
#[doc = "SWD pinout selector (not present in TSG4/TSG5-M)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdConfig(pub u32);
impl SwdConfig {
    #[doc = "0: Use Primary SWD location 1: Use Alternate SWD location"]
    #[inline(always)]
    pub const fn swd_select(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: Use Primary SWD location 1: Use Alternate SWD location"]
    #[inline(always)]
    pub fn set_swd_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SwdConfig {
    #[inline(always)]
    fn default() -> SwdConfig {
        SwdConfig(0)
    }
}
