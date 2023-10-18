#![no_std]
#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - GPIO P0"]
    IOSS_INTERRUPTS_GPIO_0 = 0,
    #[doc = "1 - GPIO P1"]
    IOSS_INTERRUPTS_GPIO_1 = 1,
    #[doc = "2 - GPIO P2"]
    IOSS_INTERRUPTS_GPIO_2 = 2,
    #[doc = "3 - GPIO P3"]
    IOSS_INTERRUPTS_GPIO_3 = 3,
    #[doc = "4 - GPIO All Ports"]
    IOSS_INTERRUPT_GPIO = 4,
    #[doc = "5 - LPCOMP trigger interrupt"]
    LPCOMP_INTERRUPT = 5,
    #[doc = "6 - WDT"]
    SRSS_INTERRUPT_WDT = 6,
    #[doc = "7 - SCB #0"]
    SCB_0_INTERRUPT = 7,
    #[doc = "8 - SCB #1"]
    SCB_1_INTERRUPT = 8,
    #[doc = "9 - SCB #2"]
    SCB_2_INTERRUPT = 9,
    #[doc = "10 - CTBm Interrupt (all CTBms)"]
    PASS_0_INTERRUPT_CTBS = 10,
    #[doc = "11 - WCO WDT Interrupt"]
    WCO_INTERRUPT = 11,
    #[doc = "12 - SPCIF interrupt"]
    CPUSS_INTERRUPT_SPCIF = 12,
    #[doc = "13 - CSD #0 (Primarily Capsense)"]
    CSD_INTERRUPT = 13,
    #[doc = "14 - TCPWM #0, Counter #0"]
    TCPWM_INTERRUPTS_0 = 14,
    #[doc = "15 - TCPWM #0, Counter #1"]
    TCPWM_INTERRUPTS_1 = 15,
    #[doc = "16 - TCPWM #0, Counter #2"]
    TCPWM_INTERRUPTS_2 = 16,
    #[doc = "17 - TCPWM #0, Counter #3"]
    TCPWM_INTERRUPTS_3 = 17,
    #[doc = "18 - TCPWM #0, Counter #4"]
    TCPWM_INTERRUPTS_4 = 18,
    #[doc = "19 - SAR"]
    PASS_0_INTERRUPT_SAR = 19,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "Peripheral Interconnect"]
pub const PERI: peri::Peri = unsafe { peri::Peri::from_ptr(0x4001_0000 as usize as _) };
#[doc = "High Speed IO Matrix (HSIOM)"]
pub const HSIOM: hsiom::Hsiom = unsafe { hsiom::Hsiom::from_ptr(0x4002_0000 as usize as _) };
#[doc = "System Resources Lite Subsystem"]
pub const SRSSLT: srsslt::Srsslt = unsafe { srsslt::Srsslt::from_ptr(0x4003_0000 as usize as _) };
#[doc = "GPIO port control/configuration"]
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004_0000 as usize as _) };
#[doc = "Programmable IO configuration"]
pub const PRGIO: prgio::Prgio = unsafe { prgio::Prgio::from_ptr(0x4005_0000 as usize as _) };
#[doc = "Timer/Counter/PWM"]
pub const TCPWM: tcpwm::Tcpwm = unsafe { tcpwm::Tcpwm::from_ptr(0x4006_0000 as usize as _) };
#[doc = "32KHz Oscillator"]
pub const WCO: wco::Wco = unsafe { wco::Wco::from_ptr(0x4007_0000 as usize as _) };
#[doc = "Serial Communications Block (SPI/UART/I2C)"]
pub const SCB0: scb0::Scb0 = unsafe { scb0::Scb0::from_ptr(0x4008_0000 as usize as _) };
pub const SCB1: scb0::Scb0 = unsafe { scb0::Scb0::from_ptr(0x4009_0000 as usize as _) };
pub const SCB2: scb0::Scb0 = unsafe { scb0::Scb0::from_ptr(0x400a_0000 as usize as _) };
#[doc = "LCD Controller Block"]
pub const LCD: lcd::Lcd = unsafe { lcd::Lcd::from_ptr(0x400b_0000 as usize as _) };
#[doc = "Capsense Controller"]
pub const CSD0: csd0::Csd0 = unsafe { csd0::Csd0::from_ptr(0x400c_0000 as usize as _) };
#[doc = "Low Power Comparators"]
pub const LPCOMP: lpcomp::Lpcomp = unsafe { lpcomp::Lpcomp::from_ptr(0x400d_0000 as usize as _) };
#[doc = "CPU Subsystem"]
pub const CPUSS: cpuss::Cpuss = unsafe { cpuss::Cpuss::from_ptr(0x4010_0000 as usize as _) };
#[doc = "Flash Control Interface"]
pub const SPCIF: spcif::Spcif = unsafe { spcif::Spcif::from_ptr(0x4011_0000 as usize as _) };
#[doc = "Continuous Time Block Mini"]
pub const CTBM0: ctbm0::Ctbm0 = unsafe { ctbm0::Ctbm0::from_ptr(0x4030_0000 as usize as _) };
#[doc = "SAR ADC with Sequencer"]
pub const SAR0: sar0::Sar0 = unsafe { sar0::Sar0::from_ptr(0x403a_0000 as usize as _) };
#[doc = "PASS top-level MMIO (DSABv2, INTR)"]
pub const PASS0: pass0::Pass0 = unsafe { pass0::Pass0::from_ptr(0x403f_0000 as usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod common;
pub mod cpuss;
pub mod csd0;
pub mod ctbm0;
pub mod gpio;
pub mod hsiom;
pub mod lcd;
pub mod lpcomp;
pub mod pass0;
pub mod peri;
pub mod prgio;
pub mod sar0;
pub mod scb0;
pub mod spcif;
pub mod srsslt;
pub mod tcpwm;
pub mod wco;
