#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Io0sel {
    #[doc = "SW controlled GPIO."]
    GPIO = 0,
    #[doc = "SW controlled 'out', DSI controlled 'oe_n'."]
    GPIO_DSI = 0x01,
    #[doc = "DSI controlled 'out' and 'oe_n'."]
    DSI_DSI = 0x02,
    #[doc = "DSI controlled 'out', SW controlled 'oe_n'."]
    DSI_GPIO = 0x03,
    #[doc = "CSD sense connection (analog mode)"]
    CSD_SENSE = 0x04,
    #[doc = "CSD shield connection (analog mode)"]
    CSD_SHIELD = 0x05,
    #[doc = "AMUXBUS A connection."]
    AMUXA = 0x06,
    #[doc = "AMUXBUS B connection. This mode is also used for CSD GPIO charging. When CSD GPIO charging is enabled in CSD_CONTROL, 'oe_n' is connected to '!csd_charge' signal (and IO pad is also still connected to AMUXBUS B)."]
    AMUXB = 0x07,
    #[doc = "Chip specific Active source 0 connection."]
    ACT_0 = 0x08,
    #[doc = "Chip specific Active source 1 connection."]
    ACT_1 = 0x09,
    #[doc = "Chip specific Active source 2 connection."]
    ACT_2 = 0x0a,
    #[doc = "Chip specific Active source 3 connection."]
    ACT_3 = 0x0b,
    #[doc = "LCD common connection. This mode provides DeepSleep functionality (provided that the LCD block exists, is enabled, and is properly configured)."]
    LCD_COM = 0x0c,
    #[doc = "LCD segment connection. This mode provides DeepSleep functionality (provided that the LCD block is enabled and properly configured)."]
    LCD_SEG = 0x0d,
    #[doc = "Chip specific DeepSleep source 2 connection."]
    DS_2 = 0x0e,
    #[doc = "Chip specific DeepSleep source 3 connection."]
    DS_3 = 0x0f,
}
impl Io0sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Io0sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Io0sel {
    #[inline(always)]
    fn from(val: u8) -> Io0sel {
        Io0sel::from_bits(val)
    }
}
impl From<Io0sel> for u8 {
    #[inline(always)]
    fn from(val: Io0sel) -> u8 {
        Io0sel::to_bits(val)
    }
}
