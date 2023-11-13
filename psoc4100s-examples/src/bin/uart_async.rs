//! This example shows how to use an SCB in UART (Universal asynchronous
//! receiver-transmitter) mode.
//!
//! No specific hardware is specified in this example.

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embedded_io_async::{Read, Write};
use psoc4100s_hal::bind_interrupts;
use psoc4100s_hal::clocks::*;
use psoc4100s_hal::peripherals::SCB0;
use psoc4100s_hal::uart;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    SCB_0_INTERRUPT => uart::InterruptHandler<SCB0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let (imo, p) = psoc4100s_hal::init(Default::default());

    let tx = p.PIN_4_1;
    let rx = p.PIN_4_0;
    let rts = p.PIN_4_3;
    let cts = p.PIN_4_2;

    info!("Set up UART source clock");
    let mut uart_clk_src = imo.peripheral_divider_16_bit::<0>();
    let mut div_config = PeriClkDiv16Config::default();
    // Divide the HF clock (24 MHz) by 26 (923 kHz) to be slow enough for the UART
    // peripheral at 115200 baud.
    div_config.integer = 25;
    uart_clk_src.configure(Some(div_config));

    let config = uart::Config::default();
    let mut uart =
        uart::Uart::new_with_rtscts(p.SCB0, uart_clk_src, tx, rx, rts, cts, Irqs, config);

    uart.write_all("Hello World!\r\n".as_bytes()).await.unwrap();

    loop {
        // Echo input.
        let mut buf = [0; 32];
        let rx_bytes = uart.read(&mut buf).await.unwrap();
        uart.write(&buf[..rx_bytes]).await.unwrap();
    }
}
