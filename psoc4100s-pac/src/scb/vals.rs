#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtrlMode {
    #[doc = "Inter-Integrated Circuits (I2C) mode."]
    I2C = 0,
    #[doc = "Serial Peripheral Interface (SPI) mode."]
    SPI = 0x01,
    #[doc = "Universal Asynchronous Receiver/Transmitter (UART) mode."]
    UART = 0x02,
    _RESERVED_3 = 0x03,
}
impl CtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlMode {
    #[inline(always)]
    fn from(val: u8) -> CtrlMode {
        CtrlMode::from_bits(val)
    }
}
impl From<CtrlMode> for u8 {
    #[inline(always)]
    fn from(val: CtrlMode) -> u8 {
        CtrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SpiCtrlMode {
    #[doc = "SPI Motorola submode. In master mode, when not transmitting data (SELECT is inactive), SCLK is stable at CPOL. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    SPI_MOTOROLA = 0,
    #[doc = "SPI Texas Instruments submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive; i.e. no pulse is generated."]
    SPI_TI = 0x01,
    #[doc = "SPI National Semiconductors submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    SPI_NS = 0x02,
    _RESERVED_3 = 0x03,
}
impl SpiCtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiCtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiCtrlMode {
    #[inline(always)]
    fn from(val: u8) -> SpiCtrlMode {
        SpiCtrlMode::from_bits(val)
    }
}
impl From<SpiCtrlMode> for u8 {
    #[inline(always)]
    fn from(val: SpiCtrlMode) -> u8 {
        SpiCtrlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UartCtrlMode {
    #[doc = "Standard UART submode."]
    UART_STD = 0,
    #[doc = "SmartCard (ISO7816) submode. Support for negative acknowledgement (NACK) on the receiver side and retransmission on the transmitter side."]
    UART_SMARTCARD = 0x01,
    #[doc = "Infrared Data Association (IrDA) submode. Return to Zero modulation scheme. In this mode, the oversampling factor should be 16, that is OVS is 15."]
    UART_IRDA = 0x02,
    _RESERVED_3 = 0x03,
}
impl UartCtrlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UartCtrlMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UartCtrlMode {
    #[inline(always)]
    fn from(val: u8) -> UartCtrlMode {
        UartCtrlMode::from_bits(val)
    }
}
impl From<UartCtrlMode> for u8 {
    #[inline(always)]
    fn from(val: UartCtrlMode) -> u8 {
        UartCtrlMode::to_bits(val)
    }
}
