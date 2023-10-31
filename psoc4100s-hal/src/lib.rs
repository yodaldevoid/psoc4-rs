#![no_std]
#![cfg_attr(feature = "nightly", feature(async_fn_in_trait))]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

pub mod clocks;
pub mod gpio;
pub mod watchdog;
#[cfg(any(feature = "time-driver-wdc-ilo", feature = "time-driver-wdc-wco"))]
pub mod wdc;

// Reexports
pub use embassy_hal_internal::{into_ref, Peripheral, PeripheralRef};
#[cfg(feature = "unstable-pac")]
pub use psoc4100s_pac as pac;
#[cfg(not(feature = "unstable-pac"))]
pub(crate) use psoc4100s_pac as pac;

#[cfg(feature = "rt")]
pub use crate::pac::NVIC_PRIO_BITS;

embassy_hal_internal::interrupt_mod!(
    IOSS_INTERRUPTS_GPIO_0,
    IOSS_INTERRUPTS_GPIO_1,
    IOSS_INTERRUPTS_GPIO_2,
    IOSS_INTERRUPTS_GPIO_3,
    IOSS_INTERRUPT_GPIO,
    LPCOMP_INTERRUPT,
    SRSS_INTERRUPT_WDT,
    SCB_0_INTERRUPT,
    SCB_1_INTERRUPT,
    SCB_2_INTERRUPT,
    PASS_0_INTERRUPT_CTBS,
    WCO_INTERRUPT,
    CPUSS_INTERRUPT_SPCIF,
    CSD_INTERRUPT,
    TCPWM_INTERRUPTS_0,
    TCPWM_INTERRUPTS_1,
    TCPWM_INTERRUPTS_2,
    TCPWM_INTERRUPTS_3,
    TCPWM_INTERRUPTS_4,
    PASS_0_INTERRUPT_SAR,
);

/// Macro to bind interrupts to handlers.
///
/// This defines the right interrupt handlers, and creates a unit struct (like `struct Irqs;`)
/// and implements the right [`Binding`]s for it. You can pass this struct to drivers to
/// prove at compile-time that the right interrupts have been bound.
// developer note: this macro can't be in `embassy-hal-internal` due to the use of `$crate`.
#[macro_export]
macro_rules! bind_interrupts {
    ($vis:vis struct $name:ident { $($irq:ident => $($handler:ty),*;)* }) => {
            #[derive(Copy, Clone)]
            $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[no_mangle]
            unsafe extern "C" fn $irq() {
                $(
                    <$handler as $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$irq>>::on_interrupt();
                )*
            }

            $(
                unsafe impl $crate::interrupt::typelevel::Binding<$crate::interrupt::typelevel::$irq, $handler> for $name {}
            )*
        )*
    };
}

embassy_hal_internal::peripherals! {
    PIN_0_0,
    PIN_0_1,
    PIN_0_2,
    PIN_0_3,
    PIN_0_4,
    PIN_0_5,
    PIN_0_6,
    PIN_0_7,
    PIN_1_0,
    PIN_1_1,
    PIN_1_2,
    PIN_1_3,
    PIN_1_4,
    PIN_1_5,
    PIN_1_6,
    PIN_1_7,
    PIN_2_0,
    PIN_2_1,
    PIN_2_2,
    PIN_2_3,
    PIN_2_4,
    PIN_2_5,
    PIN_2_6,
    PIN_2_7,
    PIN_3_0,
    PIN_3_1,
    PIN_3_2,
    PIN_3_3,
    PIN_3_4,
    PIN_3_5,
    PIN_3_6,
    PIN_3_7,
    PIN_4_0,
    PIN_4_1,
    PIN_4_2,
    PIN_4_3,

    SCB0,
    SCB1,
    SCB2,

    PRGIO0,
    PRGIO1,

    TCPWM,

    LCD,

    FLASH,

    WATCHDOG,

    // TODO: CapSense/IDACs
    // TODO: SAR ADC
    // TODO: temperature sensor
    // TODO: Comparators/LPC
    // TODO: OpAmps/CTB
    // TODO: tigger mux
}

pub mod config {
    use crate::clocks::{ClockConfig, ImoFreq};

    #[non_exhaustive]
    pub struct Config {
        pub clocks: ClockConfig,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                clocks: ClockConfig::imo(ImoFreq::_24_MHz),
            }
        }
    }

    impl Config {
        pub fn new(clocks: ClockConfig) -> Self {
            Self { clocks }
        }
    }
}

pub fn init(config: config::Config) -> Peripherals {
    // Do this first, so that it panics if user is calling `init` a second time
    // before doing anything important.
    let peripherals = Peripherals::take();

    unsafe {
        // TODO: This might need to get moved to pre_init if initialization of static
        // variables takes too long. That said, with reset values and wrost-case ILO
        // inaccuracy, we have ~2.1 seconds to get to this point.
        watchdog::init();
        clocks::init(config.clocks);
        #[cfg(any(feature = "time-driver-wdc-ilo", feature = "time-driver-wdc-wco"))]
        wdc::init();
        //dma::init();
        gpio::init();
    }

    peripherals
}
