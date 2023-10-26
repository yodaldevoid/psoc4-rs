#[doc = "LPCOMP Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Operating mode for the comparator"]
    #[inline(always)]
    pub const fn mode1(&self) -> super::vals::Mode1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mode1::from_bits(val as u8)
    }
    #[doc = "Operating mode for the comparator"]
    #[inline(always)]
    pub fn set_mode1(&mut self, val: super::vals::Mode1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
    #[inline(always)]
    pub const fn hyst1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
    #[inline(always)]
    pub fn set_hyst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn filter1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_filter1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub const fn inttype1(&self) -> super::vals::Inttype1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Inttype1::from_bits(val as u8)
    }
    #[doc = "Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn set_inttype1(&mut self, val: super::vals::Inttype1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Current output value of the comparator."]
    #[inline(always)]
    pub const fn out1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Current output value of the comparator."]
    #[inline(always)]
    pub fn set_out1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable comparator #1"]
    #[inline(always)]
    pub const fn enable1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable comparator #1"]
    #[inline(always)]
    pub fn set_enable1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Operating mode for the comparator"]
    #[inline(always)]
    pub const fn mode2(&self) -> super::vals::Mode2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Mode2::from_bits(val as u8)
    }
    #[doc = "Operating mode for the comparator"]
    #[inline(always)]
    pub fn set_mode2(&mut self, val: super::vals::Mode2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
    #[inline(always)]
    pub const fn hyst2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
    #[inline(always)]
    pub fn set_hyst2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn filter2(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_filter2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub const fn inttype2(&self) -> super::vals::Inttype2 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Inttype2::from_bits(val as u8)
    }
    #[doc = "Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn set_inttype2(&mut self, val: super::vals::Inttype2) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Current output value of the comparator."]
    #[inline(always)]
    pub const fn out2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Current output value of the comparator."]
    #[inline(always)]
    pub fn set_out2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable comparator #2"]
    #[inline(always)]
    pub const fn enable2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable comparator #2"]
    #[inline(always)]
    pub fn set_enable2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub const fn dsi_bypass1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn set_dsi_bypass1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Opamp1 comparator DSI (trigger) out level : 0=pulse, 1=level"]
    #[inline(always)]
    pub const fn dsi_level1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp1 comparator DSI (trigger) out level : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn set_dsi_level1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Opamp2 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub const fn dsi_bypass2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp2 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn set_dsi_bypass2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Opamp2 comparator DSI (trigger) out level : 0=pulse, 1=level"]
    #[inline(always)]
    pub const fn dsi_level2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Opamp2 comparator DSI (trigger) out level : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn set_dsi_level2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "ID & Revision"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "the ID of LPCOMP peripheral is 0xE0E0"]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "the ID of LPCOMP peripheral is 0xE0E0"]
    #[inline(always)]
    pub fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "the version number is 0x0001"]
    #[inline(always)]
    pub const fn revision(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "the version number is 0x0001"]
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
#[doc = "LPCOMP Interrupt request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Comparator 1 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn comp1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_comp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 2 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn comp2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_comp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "LPCOMP Interrupt request mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp1_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp1_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp2_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp2_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "LPCOMP Interrupt request masked"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn comp1_masked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_comp1_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn comp2_masked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_comp2_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "LPCOMP Interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn comp2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_comp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "LPCOMP Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trim1(pub u32);
impl Trim1 {
    #[doc = "Trim A for Comparator #1"]
    #[inline(always)]
    pub const fn comp1_trima(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim A for Comparator #1"]
    #[inline(always)]
    pub fn set_comp1_trima(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Trim1 {
    #[inline(always)]
    fn default() -> Trim1 {
        Trim1(0)
    }
}
#[doc = "LPCOMP Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trim2(pub u32);
impl Trim2 {
    #[doc = "Trim B for Comparator #1"]
    #[inline(always)]
    pub const fn comp1_trimb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim B for Comparator #1"]
    #[inline(always)]
    pub fn set_comp1_trimb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Trim2 {
    #[inline(always)]
    fn default() -> Trim2 {
        Trim2(0)
    }
}
#[doc = "LPCOMP Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trim3(pub u32);
impl Trim3 {
    #[doc = "Trim A for Comparator #2"]
    #[inline(always)]
    pub const fn comp2_trima(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim A for Comparator #2"]
    #[inline(always)]
    pub fn set_comp2_trima(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Trim3 {
    #[inline(always)]
    fn default() -> Trim3 {
        Trim3(0)
    }
}
#[doc = "LPCOMP Trim Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trim4(pub u32);
impl Trim4 {
    #[doc = "Trim B for Comparator #2"]
    #[inline(always)]
    pub const fn comp2_trimb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim B for Comparator #2"]
    #[inline(always)]
    pub fn set_comp2_trimb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Trim4 {
    #[inline(always)]
    fn default() -> Trim4 {
        Trim4(0)
    }
}
