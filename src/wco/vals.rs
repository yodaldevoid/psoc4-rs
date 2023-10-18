#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WdtMode0 {
    #[doc = "Do nothing"]
    NOTHING = 0,
    #[doc = "Assert WDT_INTx"]
    INT = 0x01,
    #[doc = "Assert WDT Reset - Not Supported - here for backwards compatibility"]
    RESET = 0x02,
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt. Not supported - here for Backwards compatibility."]
    INT_THEN_RESET = 0x03,
}
impl WdtMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtMode0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtMode0 {
    #[inline(always)]
    fn from(val: u8) -> WdtMode0 {
        WdtMode0::from_bits(val)
    }
}
impl From<WdtMode0> for u8 {
    #[inline(always)]
    fn from(val: WdtMode0) -> u8 {
        WdtMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WdtMode1 {
    #[doc = "Do nothing"]
    NOTHING = 0,
    #[doc = "Assert WDT_INTx"]
    INT = 0x01,
    #[doc = "Assert WDT Reset - Not Supported - here for backwards compatibility"]
    RESET = 0x02,
    #[doc = "Assert WDT_INTx, assert WDT Reset after 3rd unhandled interrupt - Not supported - here for backwards compatibility."]
    INT_THEN_RESET = 0x03,
}
impl WdtMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtMode1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtMode1 {
    #[inline(always)]
    fn from(val: u8) -> WdtMode1 {
        WdtMode1::from_bits(val)
    }
}
impl From<WdtMode1> for u8 {
    #[inline(always)]
    fn from(val: WdtMode1) -> u8 {
        WdtMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum WdtMode2 {
    #[doc = "Free running counter with no interrupt requests"]
    NOTHING = 0,
    #[doc = "Free running counter with interrupt request when a specified bit in CTR2 toggles (see WDT_BITS2)"]
    INT = 0x01,
}
impl WdtMode2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdtMode2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdtMode2 {
    #[inline(always)]
    fn from(val: u8) -> WdtMode2 {
        WdtMode2::from_bits(val)
    }
}
impl From<WdtMode2> for u8 {
    #[inline(always)]
    fn from(val: WdtMode2) -> u8 {
        WdtMode2::to_bits(val)
    }
}
