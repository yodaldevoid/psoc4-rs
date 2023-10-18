#[doc = "LCD Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub const fn ls_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn set_ls_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub const fn hs_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn set_hs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HS/LS Mode selection"]
    #[inline(always)]
    pub const fn lcd_mode(&self) -> super::vals::LcdMode {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::LcdMode::from_bits(val as u8)
    }
    #[doc = "HS/LS Mode selection"]
    #[inline(always)]
    pub fn set_lcd_mode(&mut self, val: super::vals::LcdMode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LCD driving waveform type configuration."]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::Type {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Type::from_bits(val as u8)
    }
    #[doc = "LCD driving waveform type configuration."]
    #[inline(always)]
    pub fn set_type_(&mut self, val: super::vals::Type) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Driving mode configuration"]
    #[inline(always)]
    pub const fn op_mode(&self) -> super::vals::OpMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::OpMode::from_bits(val as u8)
    }
    #[doc = "Driving mode configuration"]
    #[inline(always)]
    pub fn set_op_mode(&mut self, val: super::vals::OpMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PWM bias selection"]
    #[inline(always)]
    pub const fn bias(&self) -> super::vals::Bias {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Bias::from_bits(val as u8)
    }
    #[doc = "PWM bias selection"]
    #[inline(always)]
    pub fn set_bias(&mut self, val: super::vals::Bias) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    pub const fn com_num(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    pub fn set_com_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "LS enable status bit. This bit is a copy of LS_EN that is synchronized to the low speed clock domain and back to the system clock domain. Firmware can use this bit to observe whether LS_EN has taken effect in the low speed clock domain. Firmware should never change the configuration for the LS generator without ensuring this bit is 0. The following procedure should be followed to disable the LS generator: 1. If LS_EN=0 we are done. Exit the procedure. 2. Check that LS_EN_STAT=1. If not, wait until it is. This will catch the case of a recent enable (LS_EN=1) that has not taken effect yet. 3. Set LS_EN=0. 4. Wait until LS_EN_STAT=0."]
    #[inline(always)]
    pub const fn ls_en_stat(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LS enable status bit. This bit is a copy of LS_EN that is synchronized to the low speed clock domain and back to the system clock domain. Firmware can use this bit to observe whether LS_EN has taken effect in the low speed clock domain. Firmware should never change the configuration for the LS generator without ensuring this bit is 0. The following procedure should be followed to disable the LS generator: 1. If LS_EN=0 we are done. Exit the procedure. 2. Check that LS_EN_STAT=1. If not, wait until it is. This will catch the case of a recent enable (LS_EN=1) that has not taken effect yet. 3. Set LS_EN=0. 4. Wait until LS_EN_STAT=0."]
    #[inline(always)]
    pub fn set_ls_en_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
#[doc = "LCD Divider Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Divider(pub u32);
impl Divider {
    #[doc = "Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub const fn subfr_div(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub fn set_subfr_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub const fn dead_div(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub fn set_dead_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Divider {
    #[inline(always)]
    fn default() -> Divider {
        Divider(0)
    }
}
#[doc = "ID & Revision"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "The ID of LCD controller peripheral is 0xF0F0"]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The ID of LCD controller peripheral is 0xF0F0"]
    #[inline(always)]
    pub fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The version number is 0x0001"]
    #[inline(always)]
    pub const fn revision(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "The version number is 0x0001"]
    #[inline(always)]
    pub fn set_revision(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
