//! This example shows how to communicate using I2C with external chips.
//!
//! Example written for the [`MCP23017 16-Bit I2C I/O Expander with Serial Interface`] chip.
//! (https://www.microchip.com/en-us/product/mcp23017)

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_hal_1::i2c::I2c;
use psoc4100s_hal::clocks::*;
use psoc4100s_hal::i2c::{self, Config};
use {defmt_rtt as _, panic_probe as _};

#[allow(dead_code)]
mod mcp23017 {
    pub const ADDR: u8 = 0x20; // default addr

    pub const IODIRA: u8 = 0x00;
    pub const IPOLA: u8 = 0x02;
    pub const GPINTENA: u8 = 0x04;
    pub const DEFVALA: u8 = 0x06;
    pub const INTCONA: u8 = 0x08;
    pub const IOCONA: u8 = 0x0A;
    pub const GPPUA: u8 = 0x0C;
    pub const INTFA: u8 = 0x0E;
    pub const INTCAPA: u8 = 0x10;
    pub const GPIOA: u8 = 0x12;
    pub const OLATA: u8 = 0x14;
    pub const IODIRB: u8 = 0x01;
    pub const IPOLB: u8 = 0x03;
    pub const GPINTENB: u8 = 0x05;
    pub const DEFVALB: u8 = 0x07;
    pub const INTCONB: u8 = 0x09;
    pub const IOCONB: u8 = 0x0B;
    pub const GPPUB: u8 = 0x0D;
    pub const INTFB: u8 = 0x0F;
    pub const INTCAPB: u8 = 0x11;
    pub const GPIOB: u8 = 0x13;
    pub const OLATB: u8 = 0x15;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let (imo, p) = psoc4100s_hal::init(Default::default());

    let scl = p.PIN_3_0;
    let sda = p.PIN_3_1;

    info!("set up I2C source clock");
    let mut i2c_clk_src = imo.peripheral_divider_16_bit::<0>();
    let mut div_config = PeriClkDiv16Config::default();
    // Divide the HF clock (24 MHz) by 12 (2 MHz) to be slow enough for the I2C
    // peripheral.
    div_config.integer = 11;
    i2c_clk_src.configure(Some(div_config));

    info!("set up I2C");
    let mut i2c = i2c::I2c::new_blocking(p.SCB1, &i2c_clk_src, scl, sda, Config::default());

    use mcp23017::*;

    info!("init MCP23017 config for IXPANDO");
    // init - a outputs, b inputs
    i2c.write(ADDR, &[IODIRA, 0x00]).unwrap();
    i2c.write(ADDR, &[IODIRB, 0xFF]).unwrap();
    i2c.write(ADDR, &[GPPUB, 0xFF]).unwrap(); // pullups

    let mut val = 0xAA;
    loop {
        i2c.write(mcp23017::ADDR, &[GPIOA, val]).unwrap();

        let mut portb = [0];
        i2c.write_read(mcp23017::ADDR, &[GPIOB], &mut portb)
            .unwrap();
        info!("portb = 0x{:02X}", portb[0]);

        val = !val;

        Timer::after_secs(1).await;
    }
}
