#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Inttype1 {
    #[doc = "Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl Inttype1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inttype1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inttype1 {
    #[inline(always)]
    fn from(val: u8) -> Inttype1 {
        Inttype1::from_bits(val)
    }
}
impl From<Inttype1> for u8 {
    #[inline(always)]
    fn from(val: Inttype1) -> u8 {
        Inttype1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Inttype2 {
    #[doc = "Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "Rising edge"]
    RISING = 0x01,
    #[doc = "Falling edge"]
    FALLING = 0x02,
    #[doc = "Both rising and falling edges"]
    BOTH = 0x03,
}
impl Inttype2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inttype2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inttype2 {
    #[inline(always)]
    fn from(val: u8) -> Inttype2 {
        Inttype2::from_bits(val)
    }
}
impl From<Inttype2> for u8 {
    #[inline(always)]
    fn from(val: Inttype2) -> u8 {
        Inttype2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode1 {
    #[doc = "Slow operating mode (uses less power, <50uA)"]
    SLOW = 0,
    #[doc = "Fast operating mode (uses more power, <400uA)"]
    FAST = 0x01,
    #[doc = "Ultra low power operting mode (uses ~2-4uA)"]
    ULP = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode1 {
    #[inline(always)]
    fn from(val: u8) -> Mode1 {
        Mode1::from_bits(val)
    }
}
impl From<Mode1> for u8 {
    #[inline(always)]
    fn from(val: Mode1) -> u8 {
        Mode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode2 {
    #[doc = "Slow operating mode (uses less power, <50uA)"]
    SLOW = 0,
    #[doc = "Fast operating mode (uses more power, <400uA)"]
    FAST = 0x01,
    #[doc = "Ultra low power operting mode (uses ~2-4uA)"]
    ULP = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode2 {
    #[inline(always)]
    fn from(val: u8) -> Mode2 {
        Mode2::from_bits(val)
    }
}
impl From<Mode2> for u8 {
    #[inline(always)]
    fn from(val: Mode2) -> u8 {
        Mode2::to_bits(val)
    }
}
