#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Oa0compint {
    #[doc = "Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl Oa0compint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oa0compint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oa0compint {
    #[inline(always)]
    fn from(val: u8) -> Oa0compint {
        Oa0compint::from_bits(val)
    }
}
impl From<Oa0compint> for u8 {
    #[inline(always)]
    fn from(val: Oa0compint) -> u8 {
        Oa0compint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Oa0pwrMode {
    #[doc = "Off"]
    OFF = 0,
    #[doc = "Low compensation setting (smallest cap, highest GBW). For gain=10: PM=60deg @ Cload=50pF for the output to pin driver and 10pF for the internal only driver"]
    LOW = 0x01,
    #[doc = "Medium compensation setting. For gain=4: PM=60deg @ Cload=50pF for the output to pin driver and 10pF for the internal only driver."]
    MEDIUM = 0x02,
    #[doc = "Highest compensation (largest cap, lowest GBW). For gain=1: PM=60deg @ Cload=50pF for the output to pin driver and 10pF for the internal only driver"]
    HIGH = 0x03,
}
impl Oa0pwrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oa0pwrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oa0pwrMode {
    #[inline(always)]
    fn from(val: u8) -> Oa0pwrMode {
        Oa0pwrMode::from_bits(val)
    }
}
impl From<Oa0pwrMode> for u8 {
    #[inline(always)]
    fn from(val: Oa0pwrMode) -> u8 {
        Oa0pwrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Oa1compint {
    #[doc = "Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl Oa1compint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oa1compint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oa1compint {
    #[inline(always)]
    fn from(val: u8) -> Oa1compint {
        Oa1compint::from_bits(val)
    }
}
impl From<Oa1compint> for u8 {
    #[inline(always)]
    fn from(val: Oa1compint) -> u8 {
        Oa1compint::to_bits(val)
    }
}
