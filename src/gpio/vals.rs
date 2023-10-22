#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dm {
    #[doc = "Mode 0 (analog mode): Output buffer off (high Z). Input buffer off."]
    OFF = 0,
    #[doc = "Mode 1: Output buffer off (high Z). Input buffer on."]
    INPUT = 0x01,
    #[doc = "Mode 2: Strong pull down ('0'), weak/resistive pull up (PU). Input buffer on."]
    _0_PU = 0x02,
    #[doc = "Mode 3: Weak/resistive pull down (PD), strong pull up ('1'). Input buffer on."]
    PD_1 = 0x03,
    #[doc = "Mode 4: Strong pull down ('0'), open drain (pull up off). Input buffer on."]
    _0_Z = 0x04,
    #[doc = "Mode 5: Open drain (pull down off), strong pull up ('1'). Input buffer on."]
    Z_1 = 0x05,
    #[doc = "Mode 6: Strong pull down ('0'), strong pull up ('1'). Input buffer on."]
    _0_1 = 0x06,
    #[doc = "Mode 7: Weak/resistive pull down (PD), weak/resistive pull up (PU). Input buffer on."]
    PD_PU = 0x07,
}
impl Dm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dm {
    #[inline(always)]
    fn from(val: u8) -> Dm {
        Dm::from_bits(val)
    }
}
impl From<Dm> for u8 {
    #[inline(always)]
    fn from(val: Dm) -> u8 {
        Dm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum EdgeSel {
    #[doc = "Disabled"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl EdgeSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EdgeSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EdgeSel {
    #[inline(always)]
    fn from(val: u8) -> EdgeSel {
        EdgeSel::from_bits(val)
    }
}
impl From<EdgeSel> for u8 {
    #[inline(always)]
    fn from(val: EdgeSel) -> u8 {
        EdgeSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PortSlewCtl {
    #[doc = "HS mode (100pf < Cb < 400pF, 1.71<VDDD<5.5, Vext>3.0) FS mode (10pf<Cb<400pf,1.71<VDDD<5.5) (20-160ns)"]
    PORT_SLEW_CTL_0 = 0,
    #[doc = "HS mode (Cb<100pf,1.71<VDDD<5.5,Vext>2.8,F=1.7MHz) (10-80ns) FS+ Mode (Vext>2.8,1.71<VDDD<5.5) (20-120ns)"]
    PORT_SLEW_CTL_1 = 0x01,
    #[doc = "HS mode (100pf<Cb<400pf, 1.71<VDDD<5.5,Vext<3.3) (20-160ns)"]
    PORT_SLEW_CTL_2 = 0x02,
    #[doc = "HS mode (Cb<100pf,1.71<VDDD<5.5,Vext<=2.8,F=1.7MHz) (10-80ns) FS+ mode (Vext<=2.8,1.71<VDDD<5.5) (20-120ns)"]
    PORT_SLEW_CTL_3 = 0x03,
}
impl PortSlewCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PortSlewCtl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PortSlewCtl {
    #[inline(always)]
    fn from(val: u8) -> PortSlewCtl {
        PortSlewCtl::from_bits(val)
    }
}
impl From<PortSlewCtl> for u8 {
    #[inline(always)]
    fn from(val: PortSlewCtl) -> u8 {
        PortSlewCtl::to_bits(val)
    }
}
