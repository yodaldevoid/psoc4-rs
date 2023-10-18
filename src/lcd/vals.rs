#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bias {
    #[doc = "1/2 Bias"]
    HALF = 0,
    #[doc = "1/3 Bias"]
    THIRD = 0x01,
    #[doc = "1/4 Bias (not supported by LS generator)"]
    FOURTH = 0x02,
    #[doc = "1/5 Bias (not supported by LS generator)"]
    FIFTH = 0x03,
}
impl Bias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bias {
    #[inline(always)]
    fn from(val: u8) -> Bias {
        Bias::from_bits(val)
    }
}
impl From<Bias> for u8 {
    #[inline(always)]
    fn from(val: Bias) -> u8 {
        Bias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LcdMode {
    #[doc = "Select Low Speed (32kHz) Generator (Works in Active, Sleep and DeepSleep power modes)."]
    LS = 0,
    #[doc = "Select High Speed (system clock) Generator (Works in Active and Sleep power modes only)."]
    HS = 0x01,
}
impl LcdMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LcdMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LcdMode {
    #[inline(always)]
    fn from(val: u8) -> LcdMode {
        LcdMode::from_bits(val)
    }
}
impl From<LcdMode> for u8 {
    #[inline(always)]
    fn from(val: LcdMode) -> u8 {
        LcdMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OpMode {
    #[doc = "PWM Mode"]
    PWM = 0,
    #[doc = "Digital Correlation Mode"]
    CORRELATION = 0x01,
}
impl OpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpMode {
    #[inline(always)]
    fn from(val: u8) -> OpMode {
        OpMode::from_bits(val)
    }
}
impl From<OpMode> for u8 {
    #[inline(always)]
    fn from(val: OpMode) -> u8 {
        OpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    #[doc = "Type A - Each frame addresses each COM pin only once with a balanced (DC=0) waveform."]
    TYPE_A = 0,
    #[doc = "Type B - Each frame addresses each COM pin twice in sequence with a positive and negative waveform that together are balanced (DC=0)."]
    TYPE_B = 0x01,
}
impl Type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Type {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Type {
    #[inline(always)]
    fn from(val: u8) -> Type {
        Type::from_bits(val)
    }
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(val: Type) -> u8 {
        Type::to_bits(val)
    }
}
