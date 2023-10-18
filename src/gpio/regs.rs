#[doc = "Port output data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "IO pad 0 output data."]
    #[inline(always)]
    pub const fn data0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 0 output data."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO pad 1 output data."]
    #[inline(always)]
    pub const fn data1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 1 output data."]
    #[inline(always)]
    pub fn set_data1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IO pad 2 output data."]
    #[inline(always)]
    pub const fn data2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 2 output data."]
    #[inline(always)]
    pub fn set_data2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IO pad 3 output data."]
    #[inline(always)]
    pub const fn data3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 3 output data."]
    #[inline(always)]
    pub fn set_data3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IO pad 4 output data."]
    #[inline(always)]
    pub const fn data4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 4 output data."]
    #[inline(always)]
    pub fn set_data4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO pad 5 output data."]
    #[inline(always)]
    pub const fn data5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 5 output data."]
    #[inline(always)]
    pub fn set_data5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IO pad 6 output data."]
    #[inline(always)]
    pub const fn data6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 6 output data."]
    #[inline(always)]
    pub fn set_data6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IO pad 7 output data."]
    #[inline(always)]
    pub const fn data7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 7 output data."]
    #[inline(always)]
    pub fn set_data7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
#[doc = "Port output data clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DrClr(pub u32);
impl DrClr {
    #[doc = "IO pad i: '0': Output state DR.DATA\\[i\\] not affected. '1': Output state DR.DATA\\[i\\] set to '0'."]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IO pad i: '0': Output state DR.DATA\\[i\\] not affected. '1': Output state DR.DATA\\[i\\] set to '0'."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DrClr {
    #[inline(always)]
    fn default() -> DrClr {
        DrClr(0)
    }
}
#[doc = "Port output data invert register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DrInv(pub u32);
impl DrInv {
    #[doc = "IO pad i: '0': Output state DR.DATA\\[i\\] not affected. '1': Output state DR.DATA\\[i\\] inverted ('0' => '1', '1' => '0')."]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IO pad i: '0': Output state DR.DATA\\[i\\] not affected. '1': Output state DR.DATA\\[i\\] inverted ('0' => '1', '1' => '0')."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DrInv {
    #[inline(always)]
    fn default() -> DrInv {
        DrInv(0)
    }
}
#[doc = "Port output data set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DrSet(pub u32);
impl DrSet {
    #[doc = "IO pad i: '0': Output state DR.DATA\\[i\\] not affected. '1': Output state DR.DATA\\[i\\] set to '1'."]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IO pad i: '0': Output state DR.DATA\\[i\\] not affected. '1': Output state DR.DATA\\[i\\] set to '1'."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DrSet {
    #[inline(always)]
    fn default() -> DrSet {
        DrSet(0)
    }
}
#[doc = "Port interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub const fn data0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub const fn data1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn set_data1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt pending on IO pad 2. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub const fn data2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt pending on IO pad 2. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn set_data2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt pending on IO pad 3. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub const fn data3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt pending on IO pad 3. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn set_data3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt pending on IO pad 4. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub const fn data4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt pending on IO pad 4. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn set_data4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt pending on IO pad 5. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub const fn data5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt pending on IO pad 5. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn set_data5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt pending on IO pad 6. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub const fn data6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt pending on IO pad 6. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn set_data6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt pending on IO pad 7. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub const fn data7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt pending on IO pad 7. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn set_data7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Deglitched interrupt pending (selected by FLT_SEL)."]
    #[inline(always)]
    pub const fn flt_data(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Deglitched interrupt pending (selected by FLT_SEL)."]
    #[inline(always)]
    pub fn set_flt_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "`"]
    #[inline(always)]
    pub const fn ps_data0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "`"]
    #[inline(always)]
    pub fn set_ps_data0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ps_data1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ps_data1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ps_data2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ps_data2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ps_data3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ps_data3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ps_data4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ps_data4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ps_data5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ps_data5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ps_data6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ps_data6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ps_data7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ps_data7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This is a duplicate of the contents of the PS register, provided here to allow reading of both pin state and interrupt state of the port in a single read operation."]
    #[inline(always)]
    pub const fn ps_flt_data(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "This is a duplicate of the contents of the PS register, provided here to allow reading of both pin state and interrupt state of the port in a single read operation."]
    #[inline(always)]
    pub fn set_ps_flt_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Port interrupt configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCfg(pub u32);
impl IntrCfg {
    #[doc = "Sets which edge will trigger an IRQ for IO pad 0."]
    #[inline(always)]
    pub const fn edge0_sel(&self) -> super::vals::Edge0sel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Edge0sel::from_bits(val as u8)
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 0."]
    #[inline(always)]
    pub fn set_edge0_sel(&mut self, val: super::vals::Edge0sel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 1."]
    #[inline(always)]
    pub const fn edge1_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 1."]
    #[inline(always)]
    pub fn set_edge1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 2."]
    #[inline(always)]
    pub const fn edge2_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 2."]
    #[inline(always)]
    pub fn set_edge2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 3."]
    #[inline(always)]
    pub const fn edge3_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 3."]
    #[inline(always)]
    pub fn set_edge3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 4."]
    #[inline(always)]
    pub const fn edge4_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 4."]
    #[inline(always)]
    pub fn set_edge4_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 5."]
    #[inline(always)]
    pub const fn edge5_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 5."]
    #[inline(always)]
    pub fn set_edge5_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 6."]
    #[inline(always)]
    pub const fn edge6_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 6."]
    #[inline(always)]
    pub fn set_edge6_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 7."]
    #[inline(always)]
    pub const fn edge7_sel(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Sets which edge will trigger an IRQ for IO pad 7."]
    #[inline(always)]
    pub fn set_edge7_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Same for the glitch filtered pin (selected by FLT_SEL)."]
    #[inline(always)]
    pub const fn flt_edge_sel(&self) -> super::vals::FltEdgeSel {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::FltEdgeSel::from_bits(val as u8)
    }
    #[doc = "Same for the glitch filtered pin (selected by FLT_SEL)."]
    #[inline(always)]
    pub fn set_flt_edge_sel(&mut self, val: super::vals::FltEdgeSel) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub const fn flt_sel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn set_flt_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
}
impl Default for IntrCfg {
    #[inline(always)]
    fn default() -> IntrCfg {
        IntrCfg(0)
    }
}
#[doc = "Port configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc(pub u32);
impl Pc {
    #[doc = "The GPIO drive mode for IO pad 0. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the HSIOM is properly configured (HSIOM_PRT_SELx) before turning the IO on here to avoid producing glitches on the bus."]
    #[inline(always)]
    pub const fn dm0(&self) -> super::vals::Dm0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Dm0::from_bits(val as u8)
    }
    #[doc = "The GPIO drive mode for IO pad 0. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the HSIOM is properly configured (HSIOM_PRT_SELx) before turning the IO on here to avoid producing glitches on the bus."]
    #[inline(always)]
    pub fn set_dm0(&mut self, val: super::vals::Dm0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The GPIO drive mode for IO pad 1."]
    #[inline(always)]
    pub const fn dm1(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pad 1."]
    #[inline(always)]
    pub fn set_dm1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "The GPIO drive mode for IO pad 2."]
    #[inline(always)]
    pub const fn dm2(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pad 2."]
    #[inline(always)]
    pub fn set_dm2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "The GPIO drive mode for IO pad 3."]
    #[inline(always)]
    pub const fn dm3(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pad 3."]
    #[inline(always)]
    pub fn set_dm3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "The GPIO drive mode for IO pad 4."]
    #[inline(always)]
    pub const fn dm4(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pad 4."]
    #[inline(always)]
    pub fn set_dm4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "The GPIO drive mode for IO pad 5."]
    #[inline(always)]
    pub const fn dm5(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pad 5."]
    #[inline(always)]
    pub fn set_dm5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "The GPIO drive mode for IO pad 6."]
    #[inline(always)]
    pub const fn dm6(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pad 6."]
    #[inline(always)]
    pub fn set_dm6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "The GPIO drive mode for IO pad 7."]
    #[inline(always)]
    pub const fn dm7(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "The GPIO drive mode for IO pad 7."]
    #[inline(always)]
    pub fn set_dm7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "The GPIO cells include a VTRIP_SEL signal to alter the input buffer voltage. Note: this bit is ignored for SIO ports, the VTRIP_SEL settings in the SIO register are used instead (a separate VTRIP_SEL is provided for each pin pair). 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer."]
    #[inline(always)]
    pub const fn port_vtrip_sel(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "The GPIO cells include a VTRIP_SEL signal to alter the input buffer voltage. Note: this bit is ignored for SIO ports, the VTRIP_SEL settings in the SIO register are used instead (a separate VTRIP_SEL is provided for each pin pair). 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer."]
    #[inline(always)]
    pub fn set_port_vtrip_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "This field controls the output edge rate of all pins on the port: '0': fast. '1': slow."]
    #[inline(always)]
    pub const fn port_slow(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the output edge rate of all pins on the port: '0': fast. '1': slow."]
    #[inline(always)]
    pub fn set_port_slow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This field is used to improve the hysteresis (to 10 percent of vddio) of the selectable trip point input buffer. The voltage reference comes from the VREFGEN block and is only available when using the VREFGEN block: '0': <= 2.2 V input signaling Voltage. '1': > 2.2 V input signaling Voltage."]
    #[inline(always)]
    pub const fn port_hyst_trim(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This field is used to improve the hysteresis (to 10 percent of vddio) of the selectable trip point input buffer. The voltage reference comes from the VREFGEN block and is only available when using the VREFGEN block: '0': <= 2.2 V input signaling Voltage. '1': > 2.2 V input signaling Voltage."]
    #[inline(always)]
    pub fn set_port_hyst_trim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Slew control. Only used in the O_Z drive mode (mode 4: strong pull down, open drain): This field is intended for I2C functionality. See BROS 001-70428 for more details."]
    #[inline(always)]
    pub const fn port_slew_ctl(&self) -> super::vals::PortSlewCtl {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::PortSlewCtl::from_bits(val as u8)
    }
    #[doc = "Slew control. Only used in the O_Z drive mode (mode 4: strong pull down, open drain): This field is intended for I2C functionality. See BROS 001-70428 for more details."]
    #[inline(always)]
    pub fn set_port_slew_ctl(&mut self, val: super::vals::PortSlewCtl) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "This field selects the input buffer reference. The size (1 or 2 bits) and functionality is dependent on the IO cell. For GPIOv2 IO cells, bit PORT_IB_MODE_SEL\\[1\\] is not used (GPIOv2 IO cell replaces GPIO IO cell): '0'/'2': CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') '1'/'3': vcchib. For GPIO_OVTv2 and SIOv2 IO cells: '0': CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') '1': vcchib. '2': OVT. '3': Reference (possibly from reference generator cell). For SIO IO cell, this field is present but not used as the SIO IO cell does not provide input buffer mode select functionality (SIOv2 IO cell will replace SIO IO cell, as soon as it is available)."]
    #[inline(always)]
    pub const fn port_ib_mode_sel(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "This field selects the input buffer reference. The size (1 or 2 bits) and functionality is dependent on the IO cell. For GPIOv2 IO cells, bit PORT_IB_MODE_SEL\\[1\\] is not used (GPIOv2 IO cell replaces GPIO IO cell): '0'/'2': CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') '1'/'3': vcchib. For GPIO_OVTv2 and SIOv2 IO cells: '0': CMOS input buffer (PORT_VTRIP_SEL is '0'), LVTTL input buffer (PORT_VTRIP_SEL is '1') '1': vcchib. '2': OVT. '3': Reference (possibly from reference generator cell). For SIO IO cell, this field is present but not used as the SIO IO cell does not provide input buffer mode select functionality (SIOv2 IO cell will replace SIO IO cell, as soon as it is available)."]
    #[inline(always)]
    pub fn set_port_ib_mode_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Pc {
    #[inline(always)]
    fn default() -> Pc {
        Pc(0)
    }
}
#[doc = "Port configuration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc2(pub u32);
impl Pc2 {
    #[doc = "Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
    #[inline(always)]
    pub const fn inp_dis0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
    #[inline(always)]
    pub fn set_inp_dis0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Disables the input buffer for IO pad 1."]
    #[inline(always)]
    pub const fn inp_dis1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the input buffer for IO pad 1."]
    #[inline(always)]
    pub fn set_inp_dis1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Disables the input buffer for IO pad 2."]
    #[inline(always)]
    pub const fn inp_dis2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the input buffer for IO pad 2."]
    #[inline(always)]
    pub fn set_inp_dis2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Disables the input buffer for IO pad 3."]
    #[inline(always)]
    pub const fn inp_dis3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the input buffer for IO pad 3."]
    #[inline(always)]
    pub fn set_inp_dis3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Disables the input buffer for IO pad 4."]
    #[inline(always)]
    pub const fn inp_dis4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the input buffer for IO pad 4."]
    #[inline(always)]
    pub fn set_inp_dis4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Disables the input buffer for IO pad 5."]
    #[inline(always)]
    pub const fn inp_dis5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the input buffer for IO pad 5."]
    #[inline(always)]
    pub fn set_inp_dis5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Disables the input buffer for IO pad 6."]
    #[inline(always)]
    pub const fn inp_dis6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the input buffer for IO pad 6."]
    #[inline(always)]
    pub fn set_inp_dis6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Disables the input buffer for IO pad 7."]
    #[inline(always)]
    pub const fn inp_dis7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Disables the input buffer for IO pad 7."]
    #[inline(always)]
    pub fn set_inp_dis7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Pc2 {
    #[inline(always)]
    fn default() -> Pc2 {
        Pc2(0)
    }
}
#[doc = "Port IO pad state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ps(pub u32);
impl Ps {
    #[doc = "IO pad 0 state: 1: Logic high, if the pin voltage is above the input buffer threshold, logic high. 0: Logic low, if the pin voltage is below that threshold, logic low. If the drive mode for the pin is set to high Z Analog, the pin state will read 0 independent of the voltage on the pin."]
    #[inline(always)]
    pub const fn data0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 0 state: 1: Logic high, if the pin voltage is above the input buffer threshold, logic high. 0: Logic low, if the pin voltage is below that threshold, logic low. If the drive mode for the pin is set to high Z Analog, the pin state will read 0 independent of the voltage on the pin."]
    #[inline(always)]
    pub fn set_data0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO pad 1 state."]
    #[inline(always)]
    pub const fn data1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 1 state."]
    #[inline(always)]
    pub fn set_data1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IO pad 2 state."]
    #[inline(always)]
    pub const fn data2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 2 state."]
    #[inline(always)]
    pub fn set_data2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IO pad 3 state."]
    #[inline(always)]
    pub const fn data3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 3 state."]
    #[inline(always)]
    pub fn set_data3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IO pad 4 state."]
    #[inline(always)]
    pub const fn data4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 4 state."]
    #[inline(always)]
    pub fn set_data4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO pad 5 state."]
    #[inline(always)]
    pub const fn data5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 5 state."]
    #[inline(always)]
    pub fn set_data5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IO pad 6 state."]
    #[inline(always)]
    pub const fn data6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 6 state."]
    #[inline(always)]
    pub fn set_data6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IO pad 7 state."]
    #[inline(always)]
    pub const fn data7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IO pad 7 state."]
    #[inline(always)]
    pub fn set_data7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Reads of this register return the logical state of the filtered pin."]
    #[inline(always)]
    pub const fn flt_data(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Reads of this register return the logical state of the filtered pin."]
    #[inline(always)]
    pub fn set_flt_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Ps {
    #[inline(always)]
    fn default() -> Ps {
        Ps(0)
    }
}
#[doc = "Port SIO configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sio(pub u32);
impl Sio {
    #[doc = "Selects output buffer mode: 0: unregulated output buffer 1: regulated output buffer"]
    #[inline(always)]
    pub const fn pair_vreg01_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selects output buffer mode: 0: unregulated output buffer 1: regulated output buffer"]
    #[inline(always)]
    pub fn set_pair_vreg01_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects input buffer mode: 0: singled ended input buffer 1: differential input buffer"]
    #[inline(always)]
    pub const fn pair_ibuf01_sel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Selects input buffer mode: 0: singled ended input buffer 1: differential input buffer"]
    #[inline(always)]
    pub fn set_pair_ibuf01_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1') '0': trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL) Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
    #[inline(always)]
    pub const fn pair_vtrip01_sel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1') '0': trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL) Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
    #[inline(always)]
    pub fn set_pair_vtrip01_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Selects reference voltage Vref for trip-point of input buffer: 0: trip-point reference of SRSS internal reference Vref (1.2V) 1: trip-point reference of SRSS internal reference Vref (1.2V) 2: trip-point reference of AMUXBUS_A 3: trip-point reference of AMUXBUS_B Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
    #[inline(always)]
    pub const fn pair_vref01_sel(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Selects reference voltage Vref for trip-point of input buffer: 0: trip-point reference of SRSS internal reference Vref (1.2V) 1: trip-point reference of SRSS internal reference Vref (1.2V) 2: trip-point reference of AMUXBUS_A 3: trip-point reference of AMUXBUS_B Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
    #[inline(always)]
    pub fn set_pair_vref01_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Selects regulated Voh output level and trip point of input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). 0: Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V 1: Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V 2: Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V 3: Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V 4: Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V 5: Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V 6: Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V 7: Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on VOH is limited to Vddio - 400mV"]
    #[inline(always)]
    pub const fn pair_voh01_sel(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Selects regulated Voh output level and trip point of input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). 0: Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V 1: Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V 2: Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V 3: Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V 4: Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V 5: Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V 6: Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V 7: Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on VOH is limited to Vddio - 400mV"]
    #[inline(always)]
    pub fn set_pair_voh01_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_vreg23_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_vreg23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_ibuf23_sel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_ibuf23_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_vtrip23_sel(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_vtrip23_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_vref23_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_vref23_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_voh23_sel(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_voh23_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_vreg45_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_vreg45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_ibuf45_sel(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_ibuf45_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_vtrip45_sel(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_vtrip45_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_vref45_sel(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_vref45_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_voh45_sel(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_voh45_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_vreg67_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_vreg67_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_ibuf67_sel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_ibuf67_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_vtrip67_sel(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_vtrip67_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_vref67_sel(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_vref67_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub const fn pair_voh67_sel(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "See corresponding definition for IO pads 0 and 1."]
    #[inline(always)]
    pub fn set_pair_voh67_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Sio {
    #[inline(always)]
    fn default() -> Sio {
        Sio(0)
    }
}
#[doc = "Reference generator configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrefgen(pub u32);
impl Vrefgen {
    #[doc = "Reference selection. A reference Voltage vinref is created using a Voltage vddio: '0': vinref = (0 * 13 + 184)/600 * vddio = 184/600 * vddio. '1': vinref = (1 * 13 + 184)/600 * vddio = 197/600 * vddio. '2': vinref = (2 * 13 + 184)/600 * vddio = 210/600 * vddio. ... '31': vinref = (31 * 13 + 184)/600 * vddio = 587/600 * vddio."]
    #[inline(always)]
    pub const fn ref_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Reference selection. A reference Voltage vinref is created using a Voltage vddio: '0': vinref = (0 * 13 + 184)/600 * vddio = 184/600 * vddio. '1': vinref = (1 * 13 + 184)/600 * vddio = 197/600 * vddio. '2': vinref = (2 * 13 + 184)/600 * vddio = 210/600 * vddio. ... '31': vinref = (31 * 13 + 184)/600 * vddio = 587/600 * vddio."]
    #[inline(always)]
    pub fn set_ref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Reference generator enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn vrefgen_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Reference generator enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_vrefgen_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Vrefgen {
    #[inline(always)]
    fn default() -> Vrefgen {
        Vrefgen(0)
    }
}
