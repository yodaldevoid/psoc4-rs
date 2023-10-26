#![macro_use]
use core::future::Future;
use core::pin::Pin as FuturePin;
use core::task::{Context, Poll};

use embassy_hal_internal::{impl_peripheral, into_ref, PeripheralRef};
use embassy_sync::waitqueue::AtomicWaker;

use crate::interrupt::InterruptExt;
use crate::pac::gpio::vals::*;
use crate::{interrupt, pac, peripherals, Peripheral};

const NEW_AW: AtomicWaker = AtomicWaker::new();
const PORT0_PIN_COUNT: usize = 8;
static PORT0_WAKERS: [AtomicWaker; PORT0_PIN_COUNT] = [NEW_AW; PORT0_PIN_COUNT];
const PORT1_PIN_COUNT: usize = 8;
static PORT1_WAKERS: [AtomicWaker; PORT1_PIN_COUNT] = [NEW_AW; PORT1_PIN_COUNT];
const PORT2_PIN_COUNT: usize = 8;
static PORT2_WAKERS: [AtomicWaker; PORT2_PIN_COUNT] = [NEW_AW; PORT2_PIN_COUNT];
const PORT3_PIN_COUNT: usize = 8;
static PORT3_WAKERS: [AtomicWaker; PORT3_PIN_COUNT] = [NEW_AW; PORT3_PIN_COUNT];
const PORT4_PIN_COUNT: usize = 4;
static PORT4_WAKERS: [AtomicWaker; PORT4_PIN_COUNT] = [NEW_AW; PORT4_PIN_COUNT];

// TODO: port-wide input mode
// TODO: port-wide input buffer reference
// TODO: 1-per-port glitch filter
// TODO: port-wide slew rate

/// Represents a digital input or output level.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Level {
    Low,
    High,
}

impl From<bool> for Level {
    fn from(val: bool) -> Self {
        match val {
            true => Self::High,
            false => Self::Low,
        }
    }
}

impl From<Level> for bool {
    fn from(level: Level) -> bool {
        match level {
            Level::Low => false,
            Level::High => true,
        }
    }
}

/// Drive mode for an IO. Affects both input and output use.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DriveMode {
    HighZInput = Dm::INPUT as _,
    StrongDownWeakUp = Dm::_0_PU as _,
    WeakDownStrongUp = Dm::PD_1 as _,
    StrongDown = Dm::_0_Z as _,
    StrongUp = Dm::Z_1 as _,
    StrongDownStrongUp = Dm::_0_1 as _,
    WeakDownWeakUp = Dm::PD_PU as _,
}

impl DriveMode {
    #[inline(always)]
    const fn from_pac(val: Dm) -> Self {
        unsafe { core::mem::transmute(val) }
    }

    #[inline(always)]
    const fn to_pac(self) -> Dm {
        unsafe { core::mem::transmute(self) }
    }
}

/// Represents a pull setting for an input.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Pull {
    None,
    Up,
    Down,
}

/// A GPIO port with up to 8 pins.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Port {
    Port0 = 0,
    Port1 = 1,
    Port2 = 2,
    Port3 = 3,
    Port4 = 4,
}

pub struct Input<'d, T: Pin> {
    pin: Flex<'d, T>,
}

impl<'d, T: Pin> Input<'d, T> {
    #[inline]
    pub fn new(pin: impl Peripheral<P = T> + 'd, pull: Pull) -> Self {
        let mut pin = Flex::new(pin);
        let mode = if pull == Pull::None {
            DriveMode::HighZInput
        } else {
            pin.set_level((pull == Pull::Up).into());
            DriveMode::WeakDownWeakUp
        };
        pin.set_drive_mode(mode);
        pin.enable_input(true);
        Self { pin }
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    /// Returns current pin level
    #[inline]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }

    #[inline]
    pub async fn wait_for_high(&mut self) {
        self.pin.wait_for_high().await;
    }

    #[inline]
    pub async fn wait_for_low(&mut self) {
        self.pin.wait_for_low().await;
    }

    #[inline]
    pub async fn wait_for_rising_edge(&mut self) {
        self.pin.wait_for_rising_edge().await;
    }

    #[inline]
    pub async fn wait_for_falling_edge(&mut self) {
        self.pin.wait_for_falling_edge().await;
    }

    #[inline]
    pub async fn wait_for_any_edge(&mut self) {
        self.pin.wait_for_any_edge().await;
    }
}

/// Interrupt trigger levels.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterruptTrigger {
    High,
    Low,
    Rising,
    Falling,
    Both,
}

impl InterruptTrigger {
    #[inline(always)]
    const fn to_pac(self) -> EdgeSel {
        match self {
            InterruptTrigger::High => EdgeSel::RISING,
            InterruptTrigger::Low => EdgeSel::FALLING,
            InterruptTrigger::Rising => EdgeSel::RISING,
            InterruptTrigger::Falling => EdgeSel::FALLING,
            InterruptTrigger::Both => EdgeSel::BOTH,
        }
    }
}

// TODO: allow configuration of interrupt priority for each port
// TODO: use individual interrupts for each port?
pub(crate) unsafe fn init() {
    interrupt::IOSS_INTERRUPT_GPIO.disable();
    interrupt::IOSS_INTERRUPT_GPIO.set_priority(interrupt::Priority::P3);
    interrupt::IOSS_INTERRUPT_GPIO.enable();
}

#[cfg(feature = "rt")]
fn irq_handler<const N: usize>(port: pac::gpio::Prt, wakers: &[AtomicWaker; N]) {
    // No more than one event can be awaited per pin at any given time, so
    // we can just clear all interrupt enables for that pin without having
    // to check which event was signalled.
    let intr = port.intr().read();
    port.intr().write_value(intr);
    for pin in 0..N {
        if intr.data(pin) {
            wakers[pin as usize].wake()
        }
        port.intr_cfg()
            .modify(|r| r.set_edge_sel(pin, EdgeSel::DISABLE));
    }
}

#[cfg(feature = "rt")]
#[interrupt]
fn IOSS_INTERRUPT_GPIO() {
    let gpio = pac::GPIO;
    let cause = gpio.intr_cause().read();
    if cause & 0x01 != 0 {
        irq_handler(gpio.prt(0), &PORT0_WAKERS);
    }
    if cause & 0x02 != 0 {
        irq_handler(gpio.prt(1), &PORT1_WAKERS);
    }
    if cause & 0x04 != 0 {
        irq_handler(gpio.prt(2), &PORT2_WAKERS);
    }
    if cause & 0x08 != 0 {
        irq_handler(gpio.prt(3), &PORT3_WAKERS);
    }
    if cause & 0x10 != 0 {
        irq_handler(gpio.prt(4), &PORT4_WAKERS);
    }
}

#[must_use = "futures do nothing unless you `.await` or poll them"]
struct InputFuture<'a, T: Pin> {
    pin: PeripheralRef<'a, T>,
}

impl<'d, T: Pin> InputFuture<'d, T> {
    pub fn new(pin: impl Peripheral<P = T> + 'd, level: InterruptTrigger) -> Self {
        into_ref!(pin);
        let idx = pin.pin() as usize;
        // First, clear the INTR register bits. without this INTR will still
        // contain reports of previous edges, causing the IRQ to fire early
        // on stale state. Clearing these means that we can only detect edges
        // that occur *after* the clear happened, but since both this and the
        // alternative are fundamentally racy it's probably fine.
        //
        // The alternative being checking the current level and waiting for
        // its inverse, but that requires reading the current level and thus
        // missing anything that happened before the level was read.
        pin.prt().intr().write(|r| r.set_data(idx, true));

        // Fake level triggering by reading the current state and if the state
        // doesn't match, setting a trigger for the edge in the direction of
        // the level trigger. This is racy, but there's nothing else we can do
        // to supporting level triggers.
        let state = pin.prt().ps().read().data(idx);
        let already_triggered = match level {
            InterruptTrigger::High => state,
            InterruptTrigger::Low => !state,
            _ => false,
        };

        if !already_triggered {
            pin.prt()
                .intr_cfg()
                .write(|r| r.set_edge_sel(idx, level.to_pac()));
        }
        Self { pin }
    }
}

impl<'d, T: Pin> Future for InputFuture<'d, T> {
    type Output = ();

    fn poll(self: FuturePin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // We need to register/re-register the waker for each poll because any
        // calls to wake will deregister the waker.
        let waker = match self.pin.port() {
            Port::Port0 => &PORT0_WAKERS[self.pin.pin() as usize],
            Port::Port1 => &PORT1_WAKERS[self.pin.pin() as usize],
            Port::Port2 => &PORT2_WAKERS[self.pin.pin() as usize],
            Port::Port3 => &PORT3_WAKERS[self.pin.pin() as usize],
            Port::Port4 => &PORT4_WAKERS[self.pin.pin() as usize],
        };
        waker.register(cx.waker());

        // Since the interrupt handler clear the INTR flags we'll check that
        // it has been cleared and unconditionally return Ready(()) if so.
        // We don't need further handshaking since only a single event wait
        // is possible for any given pin at any given time.
        let pin = self.pin.pin() as usize;
        if self.pin.prt().intr_cfg().read().edge_sel(pin) == EdgeSel::DISABLE {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub struct Output<'d, T: Pin> {
    pin: Flex<'d, T>,
}

impl<'d, T: Pin> Output<'d, T> {
    #[inline]
    pub fn new(pin: impl Peripheral<P = T> + 'd, initial_output: Level) -> Self {
        Self::new_inner(pin, initial_output, false)
    }

    #[inline]
    pub fn new_weak_drive(pin: impl Peripheral<P = T> + 'd, initial_output: Level) -> Self {
        Self::new_inner(pin, initial_output, true)
    }

    #[inline]
    fn new_inner(
        pin: impl Peripheral<P = T> + 'd,
        initial_output: Level,
        weak_drive: bool,
    ) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_level(initial_output);
        let mode = if weak_drive {
            DriveMode::WeakDownWeakUp
        } else {
            DriveMode::StrongDownStrongUp
        };
        pin.set_drive_mode(mode);
        Self { pin }
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level);
    }

    /// Is the output level high?
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high()
    }

    /// Is the output level low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.pin.is_set_low()
    }

    /// What level output is set to
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.pin.get_output_level()
    }

    /// Toggle pin output
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle()
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    /// Returns current pin level
    #[inline]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }

    #[inline]
    pub async fn wait_for_high(&mut self) {
        self.pin.wait_for_high().await;
    }

    #[inline]
    pub async fn wait_for_low(&mut self) {
        self.pin.wait_for_low().await;
    }

    #[inline]
    pub async fn wait_for_rising_edge(&mut self) {
        self.pin.wait_for_rising_edge().await;
    }

    #[inline]
    pub async fn wait_for_falling_edge(&mut self) {
        self.pin.wait_for_falling_edge().await;
    }

    #[inline]
    pub async fn wait_for_any_edge(&mut self) {
        self.pin.wait_for_any_edge().await;
    }
}

/// GPIO output open-drain.
pub struct OutputOpenDrain<'d, T: Pin> {
    pin: Flex<'d, T>,
}

impl<'d, T: Pin> OutputOpenDrain<'d, T> {
    #[inline]
    pub fn new(
        pin: impl Peripheral<P = T> + 'd,
        initial_output: Level,
        internal_pull: bool,
    ) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_level(initial_output);
        let mode = if internal_pull {
            DriveMode::StrongDownWeakUp
        } else {
            DriveMode::StrongDown
        };
        pin.set_drive_mode(mode);
        Self { pin }
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level);
    }

    /// Is the output level high?
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high()
    }

    /// Is the output level low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.pin.is_set_low()
    }

    /// What level output is set to
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.pin.get_output_level()
    }

    /// Toggle pin output
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle()
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    /// Returns current pin level
    #[inline]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }

    #[inline]
    pub async fn wait_for_high(&mut self) {
        self.pin.wait_for_high().await;
    }

    #[inline]
    pub async fn wait_for_low(&mut self) {
        self.pin.wait_for_low().await;
    }

    #[inline]
    pub async fn wait_for_rising_edge(&mut self) {
        self.pin.wait_for_rising_edge().await;
    }

    #[inline]
    pub async fn wait_for_falling_edge(&mut self) {
        self.pin.wait_for_falling_edge().await;
    }

    #[inline]
    pub async fn wait_for_any_edge(&mut self) {
        self.pin.wait_for_any_edge().await;
    }
}

/// GPIO output open-source.
pub struct OutputOpenSource<'d, T: Pin> {
    pin: Flex<'d, T>,
}

impl<'d, T: Pin> OutputOpenSource<'d, T> {
    #[inline]
    pub fn new(pin: impl Peripheral<P = T> + 'd, initial_output: Level) -> Self {
        Self::new_inner(pin, initial_output, false)
    }

    #[inline]
    pub fn new_internal_pull(pin: impl Peripheral<P = T> + 'd, initial_output: Level) -> Self {
        Self::new_inner(pin, initial_output, true)
    }

    #[inline]
    fn new_inner(
        pin: impl Peripheral<P = T> + 'd,
        initial_output: Level,
        internal_pull: bool,
    ) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_level(initial_output);
        let mode = if internal_pull {
            DriveMode::WeakDownStrongUp
        } else {
            DriveMode::StrongUp
        };
        pin.set_drive_mode(mode);
        Self { pin }
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level);
    }

    /// Is the output level high?
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high()
    }

    /// Is the output level low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.pin.is_set_low()
    }

    /// What level output is set to
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.pin.get_output_level()
    }

    /// Toggle pin output
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle()
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    /// Returns current pin level
    #[inline]
    pub fn get_level(&self) -> Level {
        self.pin.get_level()
    }

    #[inline]
    pub async fn wait_for_high(&mut self) {
        self.pin.wait_for_high().await;
    }

    #[inline]
    pub async fn wait_for_low(&mut self) {
        self.pin.wait_for_low().await;
    }

    #[inline]
    pub async fn wait_for_rising_edge(&mut self) {
        self.pin.wait_for_rising_edge().await;
    }

    #[inline]
    pub async fn wait_for_falling_edge(&mut self) {
        self.pin.wait_for_falling_edge().await;
    }

    #[inline]
    pub async fn wait_for_any_edge(&mut self) {
        self.pin.wait_for_any_edge().await;
    }
}

/// GPIO flexible pin.
///
/// This pin can be either an input or output pin. The output level register bit will remain
/// set while not in output mode, so the pin's level will be 'remembered' when it is not in output
/// mode.
pub struct Flex<'d, T: Pin> {
    pin: PeripheralRef<'d, T>,
}

impl<'d, T: Pin> Flex<'d, T> {
    #[inline]
    pub fn new(pin: impl Peripheral<P = T> + 'd) -> Self {
        into_ref!(pin);
        pin.prt()
            .pc()
            .modify(|r| r.set_dm(pin.pin() as usize, Dm::INPUT));
        Self { pin }
    }

    /// Set the pin's drive mode.
    #[inline]
    pub fn set_drive_mode(&mut self, mode: DriveMode) {
        let idx = self.pin.pin() as usize;
        self.pin.prt().pc().modify(|r| r.set_dm(idx, mode.to_pac()));
    }

    #[inline]
    pub fn drive_mode(&self) -> DriveMode {
        let idx = self.pin.pin() as usize;
        DriveMode::from_pac(self.pin.prt().pc().read().dm(idx))
    }

    #[inline]
    pub fn enable_input(&mut self, enable: bool) {
        let idx = self.pin.pin() as usize;
        self.pin.prt().pc2().modify(|r| r.set_inp_dis(idx, !enable));
    }

    #[inline]
    pub fn is_input_enabled(&self) -> bool {
        !self.pin.prt().pc2().read().inp_dis(self.pin.pin() as usize)
    }

    #[inline]
    pub fn is_output_enabled(&self) -> bool {
        self.drive_mode() != DriveMode::HighZInput
    }

    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.prt().ps().read().data(self.pin.pin() as usize)
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// Returns current pin level.
    #[inline]
    pub fn get_level(&self) -> Level {
        self.is_high().into()
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin
            .prt()
            .dr_set()
            .write(|r| r.set_data(self.pin.pin() as usize, true))
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin
            .prt()
            .dr_clr()
            .write(|r| r.set_data(self.pin.pin() as usize, true))
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::Low => self.set_low(),
            Level::High => self.set_high(),
        }
    }

    /// Is the output level high?
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.prt().dr().read().data(self.pin.pin() as usize)
    }

    /// Is the output level low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    /// What level output is set to
    #[inline]
    pub fn get_output_level(&self) -> Level {
        self.is_set_high().into()
    }

    /// Toggle pin output
    #[inline]
    pub fn toggle(&mut self) {
        self.pin
            .prt()
            .dr_inv()
            .write(|r| r.set_data(self.pin.pin() as usize, true))
    }

    #[inline]
    pub async fn wait_for_high(&mut self) {
        InputFuture::new(&mut self.pin, InterruptTrigger::High).await;
    }

    #[inline]
    pub async fn wait_for_low(&mut self) {
        InputFuture::new(&mut self.pin, InterruptTrigger::Low).await;
    }

    #[inline]
    pub async fn wait_for_rising_edge(&mut self) {
        InputFuture::new(&mut self.pin, InterruptTrigger::Rising).await;
    }

    #[inline]
    pub async fn wait_for_falling_edge(&mut self) {
        InputFuture::new(&mut self.pin, InterruptTrigger::Falling).await;
    }

    #[inline]
    pub async fn wait_for_any_edge(&mut self) {
        InputFuture::new(&mut self.pin, InterruptTrigger::Both).await;
    }
}

impl<'d, T: Pin> Drop for Flex<'d, T> {
    #[inline]
    fn drop(&mut self) {
        let pin = self.pin.pin() as usize;
        let port = self.pin.prt();
        // TODO: reset SWD pins to SWD?
        port.pc().modify(|r| r.set_dm(pin, Dm::OFF));
        port.intr_cfg().modify(|r| {
            r.set_edge_sel(pin, EdgeSel::DISABLE);
            if r.flt_sel() == pin as u8 {
                r.set_flt_edge_sel(EdgeSel::DISABLE);
            }
        });
        port.pc2().modify(|r| r.set_inp_dis(pin, false));
        port.dr_clr().write(|r| r.set_data(pin, true));
    }
}

pub(crate) mod sealed {
    use super::*;

    pub trait Pin: Sized {
        fn pin_port(&self) -> u8;

        #[inline]
        fn _pin(&self) -> u8 {
            self.pin_port() & 0x07
        }

        #[inline]
        fn _port(&self) -> Port {
            match self.pin_port() >> 3 {
                1 => Port::Port1,
                2 => Port::Port2,
                3 => Port::Port3,
                4 => Port::Port4,
                _ => Port::Port0,
            }
        }

        fn prt(&self) -> pac::gpio::Prt {
            crate::pac::GPIO.prt(self._port() as _)
        }
    }
}

pub trait Pin: Peripheral<P = Self> + Into<AnyPin> + sealed::Pin + Sized + 'static {
    /// Degrade to a generic pin struct
    fn degrade(self) -> AnyPin {
        AnyPin {
            pin_port: self.pin_port(),
        }
    }

    /// Returns the pin number within a port
    #[inline]
    fn pin(&self) -> u8 {
        self._pin()
    }

    /// Returns the port of this pin
    #[inline]
    fn port(&self) -> Port {
        self._port()
    }
}

pub struct AnyPin {
    pin_port: u8,
}

impl_peripheral!(AnyPin);

impl Pin for AnyPin {}
impl sealed::Pin for AnyPin {
    fn pin_port(&self) -> u8 {
        self.pin_port
    }
}

// ==========================

macro_rules! impl_pin {
    ($name:ident, $port:expr, $pin_num:expr) => {
        impl Pin for peripherals::$name {}
        impl sealed::Pin for peripherals::$name {
            #[inline]
            fn pin_port(&self) -> u8 {
                ($port as u8) * 8 + $pin_num
            }
        }

        impl From<peripherals::$name> for crate::gpio::AnyPin {
            fn from(val: peripherals::$name) -> Self {
                crate::gpio::Pin::degrade(val)
            }
        }
    };
}

impl_pin!(PIN_0_0, Port::Port0, 0);
impl_pin!(PIN_0_1, Port::Port0, 1);
impl_pin!(PIN_0_2, Port::Port0, 2);
impl_pin!(PIN_0_3, Port::Port0, 3);
impl_pin!(PIN_0_4, Port::Port0, 4);
impl_pin!(PIN_0_5, Port::Port0, 5);
impl_pin!(PIN_0_6, Port::Port0, 6);
impl_pin!(PIN_0_7, Port::Port0, 7);
impl_pin!(PIN_1_0, Port::Port1, 0);
impl_pin!(PIN_1_1, Port::Port1, 1);
impl_pin!(PIN_1_2, Port::Port1, 2);
impl_pin!(PIN_1_3, Port::Port1, 3);
impl_pin!(PIN_1_4, Port::Port1, 4);
impl_pin!(PIN_1_5, Port::Port1, 5);
impl_pin!(PIN_1_6, Port::Port1, 6);
impl_pin!(PIN_1_7, Port::Port1, 7);
impl_pin!(PIN_2_0, Port::Port2, 0);
impl_pin!(PIN_2_1, Port::Port2, 1);
impl_pin!(PIN_2_2, Port::Port2, 2);
impl_pin!(PIN_2_3, Port::Port2, 3);
impl_pin!(PIN_2_4, Port::Port2, 4);
impl_pin!(PIN_2_5, Port::Port2, 5);
impl_pin!(PIN_2_6, Port::Port2, 6);
impl_pin!(PIN_2_7, Port::Port2, 7);
impl_pin!(PIN_3_0, Port::Port3, 0);
impl_pin!(PIN_3_1, Port::Port3, 1);
impl_pin!(PIN_3_2, Port::Port3, 2);
impl_pin!(PIN_3_3, Port::Port3, 3);
impl_pin!(PIN_3_4, Port::Port3, 4);
impl_pin!(PIN_3_5, Port::Port3, 5);
impl_pin!(PIN_3_6, Port::Port3, 6);
impl_pin!(PIN_3_7, Port::Port3, 7);
impl_pin!(PIN_4_0, Port::Port4, 0);
impl_pin!(PIN_4_1, Port::Port4, 1);
impl_pin!(PIN_4_2, Port::Port4, 2);
impl_pin!(PIN_4_3, Port::Port4, 3);

// ====================

mod eh02 {
    use core::convert::Infallible;

    use super::*;

    impl<'d, T: Pin> embedded_hal_02::digital::v2::InputPin for Input<'d, T> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::InputPin for Output<'d, T> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::OutputPin for Output<'d, T> {
        type Error = Infallible;

        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::StatefulOutputPin for Output<'d, T> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::ToggleableOutputPin for Output<'d, T> {
        type Error = Infallible;
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::InputPin for OutputOpenDrain<'d, T> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::OutputPin for OutputOpenDrain<'d, T> {
        type Error = Infallible;

        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::StatefulOutputPin for OutputOpenDrain<'d, T> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::ToggleableOutputPin for OutputOpenDrain<'d, T> {
        type Error = Infallible;
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::InputPin for OutputOpenSource<'d, T> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::OutputPin for OutputOpenSource<'d, T> {
        type Error = Infallible;

        #[inline]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        #[inline]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::StatefulOutputPin for OutputOpenSource<'d, T> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::ToggleableOutputPin for OutputOpenSource<'d, T> {
        type Error = Infallible;
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::InputPin for Flex<'d, T> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::OutputPin for Flex<'d, T> {
        type Error = Infallible;

        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::StatefulOutputPin for Flex<'d, T> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_02::digital::v2::ToggleableOutputPin for Flex<'d, T> {
        type Error = Infallible;
        #[inline]
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }
}

#[cfg(feature = "unstable-traits")]
mod eh1 {
    use core::convert::Infallible;

    use super::*;

    impl<'d, T: Pin> embedded_hal_1::digital::ErrorType for Input<'d, T> {
        type Error = Infallible;
    }

    impl<'d, T: Pin> embedded_hal_1::digital::InputPin for Input<'d, T> {
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::ErrorType for Output<'d, T> {
        type Error = Infallible;
    }

    impl<'d, T: Pin> embedded_hal_1::digital::OutputPin for Output<'d, T> {
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::StatefulOutputPin for Output<'d, T> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::ToggleableOutputPin for Output<'d, T> {
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::InputPin for Output<'d, T> {
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::ErrorType for OutputOpenDrain<'d, T> {
        type Error = Infallible;
    }

    impl<'d, T: Pin> embedded_hal_1::digital::OutputPin for OutputOpenDrain<'d, T> {
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::StatefulOutputPin for OutputOpenDrain<'d, T> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::ToggleableOutputPin for OutputOpenDrain<'d, T> {
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::InputPin for OutputOpenDrain<'d, T> {
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::ErrorType for OutputOpenSource<'d, T> {
        type Error = Infallible;
    }

    impl<'d, T: Pin> embedded_hal_1::digital::OutputPin for OutputOpenSource<'d, T> {
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::StatefulOutputPin for OutputOpenSource<'d, T> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::ToggleableOutputPin for OutputOpenSource<'d, T> {
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::InputPin for OutputOpenSource<'d, T> {
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::ErrorType for Flex<'d, T> {
        type Error = Infallible;
    }

    impl<'d, T: Pin> embedded_hal_1::digital::InputPin for Flex<'d, T> {
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_high())
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::OutputPin for Flex<'d, T> {
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_high())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(self.set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::StatefulOutputPin for Flex<'d, T> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_high())
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(self.is_set_low())
        }
    }

    impl<'d, T: Pin> embedded_hal_1::digital::ToggleableOutputPin for Flex<'d, T> {
        fn toggle(&mut self) -> Result<(), Self::Error> {
            Ok(self.toggle())
        }
    }

    #[cfg(feature = "nightly")]
    impl<'d, T: Pin> embedded_hal_async::digital::Wait for Flex<'d, T> {
        async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
            self.wait_for_high().await;
            Ok(())
        }

        async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
            self.wait_for_low().await;
            Ok(())
        }

        async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_rising_edge().await;
            Ok(())
        }

        async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_falling_edge().await;
            Ok(())
        }

        async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_any_edge().await;
            Ok(())
        }
    }

    #[cfg(feature = "nightly")]
    impl<'d, T: Pin> embedded_hal_async::digital::Wait for Input<'d, T> {
        async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
            self.wait_for_high().await;
            Ok(())
        }

        async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
            self.wait_for_low().await;
            Ok(())
        }

        async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_rising_edge().await;
            Ok(())
        }

        async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_falling_edge().await;
            Ok(())
        }

        async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_any_edge().await;
            Ok(())
        }
    }

    #[cfg(feature = "nightly")]
    impl<'d, T: Pin> embedded_hal_async::digital::Wait for Output<'d, T> {
        async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
            self.wait_for_high().await;
            Ok(())
        }

        async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
            self.wait_for_low().await;
            Ok(())
        }

        async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_rising_edge().await;
            Ok(())
        }

        async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_falling_edge().await;
            Ok(())
        }

        async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_any_edge().await;
            Ok(())
        }
    }

    #[cfg(feature = "nightly")]
    impl<'d, T: Pin> embedded_hal_async::digital::Wait for OutputOpenDrain<'d, T> {
        async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
            self.wait_for_high().await;
            Ok(())
        }

        async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
            self.wait_for_low().await;
            Ok(())
        }

        async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_rising_edge().await;
            Ok(())
        }

        async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_falling_edge().await;
            Ok(())
        }

        async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_any_edge().await;
            Ok(())
        }
    }

    #[cfg(feature = "nightly")]
    impl<'d, T: Pin> embedded_hal_async::digital::Wait for OutputOpenSource<'d, T> {
        async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
            self.wait_for_high().await;
            Ok(())
        }

        async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
            self.wait_for_low().await;
            Ok(())
        }

        async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_rising_edge().await;
            Ok(())
        }

        async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_falling_edge().await;
            Ok(())
        }

        async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
            self.wait_for_any_edge().await;
            Ok(())
        }
    }
}
