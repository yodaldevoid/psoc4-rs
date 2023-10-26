#[doc = "Flash/NVL geometry information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Geometry(pub u32);
impl Geometry {
    #[doc = "Regular flash capacity in 256 Byte multiples (chip dependent). If multiple flash macros are present, this field provides the flash capacity of all flash macros together: '0': 256 Bytes. '1': 2*256 Bytes. ... '16383': 16384*256 Bytes."]
    #[inline(always)]
    pub const fn flash(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Regular flash capacity in 256 Byte multiples (chip dependent). If multiple flash macros are present, this field provides the flash capacity of all flash macros together: '0': 256 Bytes. '1': 2*256 Bytes. ... '16383': 16384*256 Bytes."]
    #[inline(always)]
    pub fn set_flash(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Supervisory flash capacity in 256 Byte multiples (chip dependent). If multiple flash macros are present, this field provides the supervisory flash capacity of all flash macros together: '0': 256 Bytes. '1': 2*256 Bytes. ... '63': 64*256 Bytes."]
    #[inline(always)]
    pub const fn sflash(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x3f;
        val as u8
    }
    #[doc = "Supervisory flash capacity in 256 Byte multiples (chip dependent). If multiple flash macros are present, this field provides the supervisory flash capacity of all flash macros together: '0': 256 Bytes. '1': 2*256 Bytes. ... '63': 64*256 Bytes."]
    #[inline(always)]
    pub fn set_sflash(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 14usize)) | (((val as u32) & 0x3f) << 14usize);
    }
    #[doc = "Number of flash macros (chip dependent): '0': 1 flash macro '1': 2 flash macros '2': 3 flash macros '3': 4 flash macros"]
    #[inline(always)]
    pub const fn num_flash(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Number of flash macros (chip dependent): '0': 1 flash macro '1': 2 flash macros '2': 3 flash macros '3': 4 flash macros"]
    #[inline(always)]
    pub fn set_num_flash(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Page size in 64 Byte multiples (chip dependent): '0': 64 byte '1': 128 byte '2': 192 byte '3': 256 byte The page size is used to determine the number of Bytes in a page for Flash page based operations (e.g. PGM_PAGE). Note: the field name FLASH_ROW is misleading, as this field specifies the number of Bytes in a page, rather than the number of Bytes in a row. In a single plane flash macro architecture, a page consists of a single row. However, in a multi plane flash macro architecture, a page consists of multiple rows from different planes."]
    #[inline(always)]
    pub const fn flash_row(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Page size in 64 Byte multiples (chip dependent): '0': 64 byte '1': 128 byte '2': 192 byte '3': 256 byte The page size is used to determine the number of Bytes in a page for Flash page based operations (e.g. PGM_PAGE). Note: the field name FLASH_ROW is misleading, as this field specifies the number of Bytes in a page, rather than the number of Bytes in a row. In a single plane flash macro architecture, a page consists of a single row. However, in a multi plane flash macro architecture, a page consists of multiple rows from different planes."]
    #[inline(always)]
    pub fn set_flash_row(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "NVLatch size in Byte multiples (chip dependent): '0': 0 Bytes '1': 1 Byte ... '127': 127 Bytes"]
    #[inline(always)]
    pub const fn nvl(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "NVLatch size in Byte multiples (chip dependent): '0': 0 Bytes '1': 1 Byte ... '127': 127 Bytes"]
    #[inline(always)]
    pub fn set_nvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "0': SRAM busy wait loop has not been copied. '1': Busy wait loop has been written into SRAM."]
    #[inline(always)]
    pub const fn de_cpd_lp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0': SRAM busy wait loop has not been copied. '1': Busy wait loop has been written into SRAM."]
    #[inline(always)]
    pub fn set_de_cpd_lp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Geometry {
    #[inline(always)]
    fn default() -> Geometry {
        Geometry(0)
    }
}
#[doc = "SPCIF interrupt request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Timer counter value reaches '0'. Set to '1', when event is detected. Write INTR field with '1', to clear bit. Write INTR_SET field with '1', to set bit."]
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer counter value reaches '0'. Set to '1', when event is detected. Write INTR field with '1', to clear bit. Write INTR_SET field with '1', to set bit."]
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "SPCIF interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "SPCIF interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask fields."]
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask fields."]
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "SPCIF interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write INTR_SET field with '1' to set corresponding INTR field."]
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write INTR_SET field with '1' to set corresponding INTR field."]
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "NVL write data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NvlWrData(pub u32);
impl NvlWrData {
    #[doc = "Data to be written to NVLatch array"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to be written to NVLatch array"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for NvlWrData {
    #[inline(always)]
    fn default() -> NvlWrData {
        NvlWrData(0)
    }
}
