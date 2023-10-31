#[doc = "AMUX splitter cell control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AmuxSplitCtl(pub u32);
impl AmuxSplitCtl {
    #[doc = "T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub const fn switch_aa_sl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn set_switch_aa_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub const fn switch_aa_sr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn set_switch_aa_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub const fn switch_aa_s0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn set_switch_aa_s0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub const fn switch_bb_sl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub fn set_switch_bb_sl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub const fn switch_bb_sr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub fn set_switch_bb_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub const fn switch_bb_s0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub fn set_switch_bb_s0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for AmuxSplitCtl {
    #[inline(always)]
    fn default() -> AmuxSplitCtl {
        AmuxSplitCtl(0)
    }
}
#[doc = "Port selection register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortSel(pub u32);
impl PortSel {
    #[doc = "Selects connection for IO pad 0 route."]
    #[inline(always)]
    pub const fn io_sel(&self, n: usize) -> super::vals::IoSel {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x0f;
        super::vals::IoSel::from_bits(val as u8)
    }
    #[doc = "Selects connection for IO pad 0 route."]
    #[inline(always)]
    pub fn set_io_sel(&mut self, n: usize, val: super::vals::IoSel) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
    }
}
impl Default for PortSel {
    #[inline(always)]
    fn default() -> PortSel {
        PortSel(0)
    }
}
#[doc = "Pump control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PumpCtl(pub u32);
impl PumpCtl {
    #[doc = "Clock select: '0': External clock. '1': Internal clock (deprecated)."]
    #[inline(always)]
    pub const fn clock_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock select: '0': External clock. '1': Internal clock (deprecated)."]
    #[inline(always)]
    pub fn set_clock_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Pump enabled: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Pump enabled: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PumpCtl {
    #[inline(always)]
    fn default() -> PumpCtl {
        PumpCtl(0)
    }
}
