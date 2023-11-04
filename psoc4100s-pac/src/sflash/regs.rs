#[doc = "CSDV2 CSD0 ADC TRIM 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csdv2csd0adcTrim(pub u32);
impl Csdv2csd0adcTrim {
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
    #[doc = "High byte of CSDv2 Calibration"]
    #[inline(always)]
    pub const fn csd_adc_cal_msb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High byte of CSDv2 Calibration"]
    #[inline(always)]
    pub fn set_csd_adc_cal_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Csdv2csd0adcTrim {
    #[inline(always)]
    fn default() -> Csdv2csd0adcTrim {
        Csdv2csd0adcTrim(0)
    }
}
#[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTctrimLt03(pub u32);
impl ImoTctrimLt03 {
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim0(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim3(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for ImoTctrimLt03 {
    #[inline(always)]
    fn default() -> ImoTctrimLt03 {
        ImoTctrimLt03(0)
    }
}
#[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTctrimLt1215(pub u32);
impl ImoTctrimLt1215 {
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim12(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize13(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim13(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize14(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim14(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize15(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim15(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for ImoTctrimLt1215 {
    #[inline(always)]
    fn default() -> ImoTctrimLt1215 {
        ImoTctrimLt1215(0)
    }
}
#[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTctrimLt1619(pub u32);
impl ImoTctrimLt1619 {
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim16(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize17(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim17(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize18(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim18(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize19(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim19(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for ImoTctrimLt1619 {
    #[inline(always)]
    fn default() -> ImoTctrimLt1619 {
        ImoTctrimLt1619(0)
    }
}
#[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTctrimLt2023(pub u32);
impl ImoTctrimLt2023 {
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize20(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim20(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize21(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim21(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize22(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize22(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim22(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim22(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize23(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim23(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for ImoTctrimLt2023 {
    #[inline(always)]
    fn default() -> ImoTctrimLt2023 {
        ImoTctrimLt2023(0)
    }
}
#[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTctrimLt24(pub u32);
impl ImoTctrimLt24 {
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize24(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim24(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
}
impl Default for ImoTctrimLt24 {
    #[inline(always)]
    fn default() -> ImoTctrimLt24 {
        ImoTctrimLt24(0)
    }
}
#[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTctrimLt47(pub u32);
impl ImoTctrimLt47 {
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim4(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim5(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim6(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim7(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for ImoTctrimLt47 {
    #[inline(always)]
    fn default() -> ImoTctrimLt47 {
        ImoTctrimLt47(0)
    }
}
#[doc = "IMO TempCo Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTctrimLt811(pub u32);
impl ImoTctrimLt811 {
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim8(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize9(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim9(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize10(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim10(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub const fn stepsize11(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "IMO trim stepsize bits. These bits are determined at manufacturing time to adjust for process variation. They are used to tune the stepsize of the FSOFFSET and OFFSET trims."]
    #[inline(always)]
    pub fn set_stepsize11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub const fn tctrim11(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "IMO temperature compesation trim. These bits are determined at manufacturing time to adjust for temperature dependence. This bits are dependent on frequency and need to be changed using the Cypress provided frequency change algorithm."]
    #[inline(always)]
    pub fn set_tctrim11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for ImoTctrimLt811 {
    #[inline(always)]
    fn default() -> ImoTctrimLt811 {
        ImoTctrimLt811(0)
    }
}
#[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTrimLt02(pub u32);
impl ImoTrimLt02 {
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ImoTrimLt02 {
    #[inline(always)]
    fn default() -> ImoTrimLt02 {
        ImoTrimLt02(0)
    }
}
#[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTrimLt1114(pub u32);
impl ImoTrimLt1114 {
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset11(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset12(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset13(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ImoTrimLt1114 {
    #[inline(always)]
    fn default() -> ImoTrimLt1114 {
        ImoTrimLt1114(0)
    }
}
#[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTrimLt1518(pub u32);
impl ImoTrimLt1518 {
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset15(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset16(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset17(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset17(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset18(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset18(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ImoTrimLt1518 {
    #[inline(always)]
    fn default() -> ImoTrimLt1518 {
        ImoTrimLt1518(0)
    }
}
#[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTrimLt1922(pub u32);
impl ImoTrimLt1922 {
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset19(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset19(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset20(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset20(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset21(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset21(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset22(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset22(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ImoTrimLt1922 {
    #[inline(always)]
    fn default() -> ImoTrimLt1922 {
        ImoTrimLt1922(0)
    }
}
#[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTrimLt2324(pub u32);
impl ImoTrimLt2324 {
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset23(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset23(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset24(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for ImoTrimLt2324 {
    #[inline(always)]
    fn default() -> ImoTrimLt2324 {
        ImoTrimLt2324(0)
    }
}
#[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTrimLt36(pub u32);
impl ImoTrimLt36 {
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ImoTrimLt36 {
    #[inline(always)]
    fn default() -> ImoTrimLt36 {
        ImoTrimLt36(0)
    }
}
#[doc = "IMO Frequency Trim Register (SRSS-Lite)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImoTrimLt710(pub u32);
impl ImoTrimLt710 {
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset8(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset9(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub const fn offset10(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting (IMO_TRIM2) and stored in SFLASH."]
    #[inline(always)]
    pub fn set_offset10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ImoTrimLt710 {
    #[inline(always)]
    fn default() -> ImoTrimLt710 {
        ImoTrimLt710(0)
    }
}
#[doc = "Hibernate and DeepSleep wakeup values for PWR_KEY_DELAY"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeyDelay(pub u32);
impl KeyDelay {
    #[doc = "Delay (in 12MHz IMO clock cycles) to wait for references to settle on wakeup from hibernate/ deepsleep. PBOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded."]
    #[inline(always)]
    pub const fn hib_wakeup_holdoff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay (in 12MHz IMO clock cycles) to wait for references to settle on wakeup from hibernate/ deepsleep. PBOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded."]
    #[inline(always)]
    pub fn set_hib_wakeup_holdoff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Delay (in 12MHz IMO clock cycles) to wait for references to settle on wakeup from hibernate/ deepsleep. PBOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded."]
    #[inline(always)]
    pub const fn dpslp_wakeup_holdoff(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay (in 12MHz IMO clock cycles) to wait for references to settle on wakeup from hibernate/ deepsleep. PBOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded."]
    #[inline(always)]
    pub fn set_dpslp_wakeup_holdoff(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for KeyDelay {
    #[inline(always)]
    fn default() -> KeyDelay {
        KeyDelay(0)
    }
}
#[doc = "SAR Temperature Sensor Multiplication Factor and Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SarTempMultiplierOffset(pub u32);
impl SarTempMultiplierOffset {
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
    #[doc = "Offset value for SAR temperature sensor in fixed point 10.6 format. Note: this field exists in products that contain SAR (m0s8sar) only."]
    #[inline(always)]
    pub const fn temp_offset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset value for SAR temperature sensor in fixed point 10.6 format. Note: this field exists in products that contain SAR (m0s8sar) only."]
    #[inline(always)]
    pub fn set_temp_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SarTempMultiplierOffset {
    #[inline(always)]
    fn default() -> SarTempMultiplierOffset {
        SarTempMultiplierOffset(0)
    }
}
#[doc = "Silicon ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SiliconId(pub u32);
impl SiliconId {
    #[doc = "Silicon ID"]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Silicon ID"]
    #[inline(always)]
    pub fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
