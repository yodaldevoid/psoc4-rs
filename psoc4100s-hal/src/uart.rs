use core::future;
use core::marker::PhantomData;
use core::task::Poll;

use embassy_hal_internal::{into_ref, PeripheralRef};
use embassy_sync::waitqueue::AtomicWaker;
use pac::gpio::vals::Dm;
use pac::hsiom::vals::IoSel;
use pac::scb::vals::*;

use crate::clocks::{PeriClkDiv, PeripheralClock};
use crate::gpio::{sealed::Pin as _, AnyPin, Pin as _};
use crate::interrupt::typelevel::{Binding, Interrupt};
use crate::{interrupt, pac, peripherals, Peripheral};

// TODO: refactor read/write/flush as try_* functions returning `Poll` and use
// that to implement everything else.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DataBits {
    _4 = 3,
    _5 = 4,
    _6 = 5,
    _7 = 6,
    _8 = 7,
    // TODO: support word size of 9 bits
    // _9 = 8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BitDirection {
    LSB,
    MSB,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Parity {
    None,
    Even,
    Odd,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopBits {
    /// 1 stop bit
    _1 = 1,
    /// 2 stop bits
    _2 = 3,
    /// 3 stop bits
    _3 = 5,
    /// 4 stop bits
    _4 = 7,
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Config {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub bit_direction: BitDirection,
    pub stop_bits: StopBits,
    pub parity: Parity,
    /// Invert the RX pin input
    pub invert_rx: bool,
    // Invert the RTS pin
    pub invert_rts: bool,
    // Invert the CTS pin
    pub invert_cts: bool,
    // TODO: drop data on frame error
    // TODO: drop data on parity error
    pub digital_filter: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baud_rate: 115200,
            data_bits: DataBits::_8,
            bit_direction: BitDirection::LSB,
            stop_bits: StopBits::_1,
            parity: Parity::None,
            invert_rx: false,
            invert_rts: false,
            invert_cts: false,
            digital_filter: false,
        }
    }
}

/// Serial error
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// Triggered when the FIFO (or shift-register) is overflowed.
    Overrun,
    /// Triggered when a break is received
    Break,
    /// Triggered when there is a parity mismatch between what's received and
    /// our settings.
    Parity,
    /// Triggered when the received character didn't have a valid stop bit.
    Framing,
}

const TX_FIFO_SIZE: u16 = 8;

pub struct Uart<'d, T: Instance + PeripheralClock, M: Mode> {
    tx: UartTx<'d, T, M>,
    rx: UartRx<'d, T, M>,
}

pub struct UartTx<'d, T: Instance + PeripheralClock, M: Mode> {
    phantom: PhantomData<(&'d mut T, M)>,
}

pub struct UartRx<'d, T: Instance + PeripheralClock, M: Mode> {
    phantom: PhantomData<(&'d mut T, M)>,
}

impl<'d, T: Instance + PeripheralClock, M: Mode> UartTx<'d, T, M> {
    pub fn new(
        _uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        tx: impl Peripheral<P = impl TxPin<T>> + 'd,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Self {
        into_ref!(tx);
        Uart::<T, M>::init(clock, Some(tx.map_into()), None, None, None, true, config);
        Self::new_inner()
    }

    pub fn new_with_rts(
        _uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        tx: impl Peripheral<P = impl TxPin<T>> + 'd,
        rts: impl Peripheral<P = impl RtsPin<T>> + 'd,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Self {
        into_ref!(tx, rts);
        Uart::<T, M>::init(
            clock,
            Some(tx.map_into()),
            None,
            Some(rts.map_into()),
            None,
            true,
            config,
        );
        Self::new_inner()
    }

    fn new_inner() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    // TODO: break

    // TODO: only correct for byte mode
    #[inline]
    fn tx_fifo_capacity() -> u16 {
        TX_FIFO_SIZE - T::regs().tx_fifo_status().read().used()
    }
}

impl<'d, T: Instance + PeripheralClock> UartTx<'d, T, Blocking> {
    pub fn new_blocking(
        _uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        tx: impl Peripheral<P = impl TxPin<T>> + 'd,
        config: Config,
    ) -> Self {
        into_ref!(tx);
        Uart::<T, Blocking>::init(clock, Some(tx.map_into()), None, None, None, false, config);
        Self::new_inner()
    }

    pub fn new_with_rts_blocking(
        _uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        tx: impl Peripheral<P = impl TxPin<T>> + 'd,
        rts: impl Peripheral<P = impl RtsPin<T>> + 'd,
        config: Config,
    ) -> Self {
        into_ref!(tx, rts);
        Uart::<T, Blocking>::init(
            clock,
            Some(tx.map_into()),
            None,
            Some(rts.map_into()),
            None,
            false,
            config,
        );
        Self::new_inner()
    }
}

impl<'d, T: Instance + PeripheralClock> UartTx<'d, T, Async> {
    /// Calls `f` to check if we are ready or not. If not, `g` is called once
    /// the waker is set (to e.g. enable the required interrupts).
    async fn wait_on<F, U, G>(&mut self, mut f: F, mut g: G) -> U
    where
        F: FnMut(&mut Self) -> Poll<U>,
        G: FnMut(&mut Self),
    {
        future::poll_fn(|cx| {
            let r = f(self);

            if r.is_pending() {
                T::tx_waker().register(cx.waker());
                g(self);
            }
            r
        })
        .await
    }
}

impl<'d, T: Instance + PeripheralClock, M: Mode> UartRx<'d, T, M> {
    pub fn new(
        _uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        rx: impl Peripheral<P = impl RxPin<T>> + 'd,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Self {
        into_ref!(rx);
        Uart::<T, M>::init(clock, None, Some(rx.map_into()), None, None, true, config);
        Self::new_inner()
    }

    pub fn new_with_cts(
        _uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        rx: impl Peripheral<P = impl RxPin<T>> + 'd,
        cts: impl Peripheral<P = impl CtsPin<T>> + 'd,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Self {
        into_ref!(rx, cts);
        Uart::<T, M>::init(
            clock,
            None,
            Some(rx.map_into()),
            None,
            Some(cts.map_into()),
            true,
            config,
        );
        Self::new_inner()
    }

    fn new_inner() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    fn drain_fifo(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let p = T::regs();
        for (i, b) in buffer.iter_mut().enumerate() {
            let r = p.intr_rx().read();
            p.intr_rx().write_value(r);
            if r.overflow() {
                return Err(Error::Overrun);
            } else if r.parity_error() {
                return Err(Error::Parity);
            } else if r.frame_error() {
                return Err(Error::Framing);
            } else if r.break_detect() {
                return Err(Error::Break);
            } else if r.not_empty() {
                *b = p.rx_fifo_rd().read().data() as u8;
                p.intr_rx().write(|r| r.set_not_empty(true));
            } else {
                return Ok(i);
            }
        }
        Ok(buffer.len())
    }

    #[inline]
    fn rx_fifo_len() -> u16 {
        T::regs().rx_fifo_status().read().used()
    }
}

impl<'d, T: Instance + PeripheralClock> UartRx<'d, T, Blocking> {
    pub fn new_blocking(
        _uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        rx: impl Peripheral<P = impl RxPin<T>> + 'd,
        config: Config,
    ) -> Self {
        into_ref!(rx);
        Uart::<T, Blocking>::init(clock, None, Some(rx.map_into()), None, None, false, config);
        Self::new_inner()
    }

    pub fn new_with_cts_blocking(
        _uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        rx: impl Peripheral<P = impl RxPin<T>> + 'd,
        cts: impl Peripheral<P = impl CtsPin<T>> + 'd,
        config: Config,
    ) -> Self {
        into_ref!(rx, cts);
        Uart::<T, Blocking>::init(
            clock,
            None,
            Some(rx.map_into()),
            None,
            Some(cts.map_into()),
            false,
            config,
        );
        Self::new_inner()
    }
}

impl<'d, T: Instance + PeripheralClock> UartRx<'d, T, Async> {
    /// Calls `f` to check if we are ready or not. If not, `g` is called once
    /// the waker is set (to e.g. enable the required interrupts).
    async fn wait_on<F, U, G>(&mut self, mut f: F, mut g: G) -> U
    where
        F: FnMut(&mut Self) -> Poll<U>,
        G: FnMut(&mut Self),
    {
        future::poll_fn(|cx| {
            let r = f(self);

            if r.is_pending() {
                T::rx_waker().register(cx.waker());
                g(self);
            }
            r
        })
        .await
    }
}

pub struct InterruptHandler<T: Instance> {
    _uart: PhantomData<T>,
}

impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let p = T::regs();

        let cause = p.intr_cause().read();
        if cause.rx() {
            p.intr_rx_mask().write_value(Default::default());
            T::rx_waker().wake();
        }
        if cause.tx() {
            p.intr_tx_mask().write_value(Default::default());
            T::tx_waker().wake();
        }
    }
}

impl<'d, T: Instance + PeripheralClock> Uart<'d, T, Blocking> {
    /// Create a new UART without hardware flow control
    pub fn new_blocking(
        uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        tx: impl Peripheral<P = impl TxPin<T>> + 'd,
        rx: impl Peripheral<P = impl RxPin<T>> + 'd,
        config: Config,
    ) -> Self {
        into_ref!(tx, rx);
        Self::new_inner(
            uart,
            clock,
            tx.map_into(),
            rx.map_into(),
            None,
            None,
            false,
            config,
        )
    }

    /// Create a new UART with hardware flow control (RTS/CTS)
    pub fn new_with_rtscts_blocking(
        uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        tx: impl Peripheral<P = impl TxPin<T>> + 'd,
        rx: impl Peripheral<P = impl RxPin<T>> + 'd,
        rts: impl Peripheral<P = impl RtsPin<T>> + 'd,
        cts: impl Peripheral<P = impl CtsPin<T>> + 'd,
        config: Config,
    ) -> Self {
        into_ref!(tx, rx, cts, rts);
        Self::new_inner(
            uart,
            clock,
            tx.map_into(),
            rx.map_into(),
            Some(rts.map_into()),
            Some(cts.map_into()),
            false,
            config,
        )
    }
}

impl<'d, T: Instance + PeripheralClock> Uart<'d, T, Async> {
    /// Create a new DMA enabled UART without hardware flow control
    pub fn new(
        uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        tx: impl Peripheral<P = impl TxPin<T>> + 'd,
        rx: impl Peripheral<P = impl RxPin<T>> + 'd,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Self {
        into_ref!(tx, rx);
        Self::new_inner(
            uart,
            clock,
            tx.map_into(),
            rx.map_into(),
            None,
            None,
            true,
            config,
        )
    }

    /// Create a new DMA enabled UART with hardware flow control (RTS/CTS)
    pub fn new_with_rtscts(
        uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        tx: impl Peripheral<P = impl TxPin<T>> + 'd,
        rx: impl Peripheral<P = impl RxPin<T>> + 'd,
        rts: impl Peripheral<P = impl RtsPin<T>> + 'd,
        cts: impl Peripheral<P = impl CtsPin<T>> + 'd,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Self {
        into_ref!(tx, rx, cts, rts);
        Self::new_inner(
            uart,
            clock,
            tx.map_into(),
            rx.map_into(),
            Some(rts.map_into()),
            Some(cts.map_into()),
            true,
            config,
        )
    }
}

impl<'d, T: Instance + PeripheralClock + 'd, M: Mode> Uart<'d, T, M> {
    fn new_inner(
        _uart: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        mut tx: PeripheralRef<'d, AnyPin>,
        mut rx: PeripheralRef<'d, AnyPin>,
        mut rts: Option<PeripheralRef<'d, AnyPin>>,
        mut cts: Option<PeripheralRef<'d, AnyPin>>,
        has_irq: bool,
        config: Config,
    ) -> Self {
        Self::init(
            clock,
            Some(tx.reborrow()),
            Some(rx.reborrow()),
            rts.as_mut().map(|x| x.reborrow()),
            cts.as_mut().map(|x| x.reborrow()),
            has_irq,
            config,
        );

        Self {
            tx: UartTx::new_inner(),
            rx: UartRx::new_inner(),
        }
    }

    fn init(
        clock: impl PeriClkDiv + 'd,
        tx: Option<PeripheralRef<'_, AnyPin>>,
        rx: Option<PeripheralRef<'_, AnyPin>>,
        rts: Option<PeripheralRef<'_, AnyPin>>,
        cts: Option<PeripheralRef<'_, AnyPin>>,
        has_irq: bool,
        config: Config,
    ) {
        T::set_clock_divider(Some(&clock));

        assert!(config.baud_rate > 0);

        let ovs = clock.frequency() / config.baud_rate;
        assert!(ovs >= 8);
        assert!(ovs < 16);
        let ovs = (ovs - 1) as u8;

        let p = T::regs();

        p.ctrl().write(|r| r.set_enabled(false));

        if let Some(pin) = tx {
            pin.prt()
                .pc()
                .modify(|r| r.set_dm(pin.pin() as usize, Dm::_0_1));
            pin.hsiom()
                .port_sel()
                .modify(|r| r.set_io_sel(pin.pin() as usize, T::IO_SEL));
        }
        if let Some(pin) = rx {
            pin.prt()
                .pc()
                .modify(|r| r.set_dm(pin.pin() as usize, Dm::INPUT));
            pin.hsiom()
                .port_sel()
                .modify(|r| r.set_io_sel(pin.pin() as usize, T::IO_SEL));
        }
        if let Some(pin) = rts {
            pin.prt()
                .pc()
                .modify(|r| r.set_dm(pin.pin() as usize, Dm::_0_1));
            pin.hsiom()
                .port_sel()
                .modify(|r| r.set_io_sel(pin.pin() as usize, T::IO_SEL));
        }
        if let Some(ref pin) = cts {
            pin.prt()
                .pc()
                .modify(|r| r.set_dm(pin.pin() as usize, Dm::INPUT));
            pin.hsiom()
                .port_sel()
                .modify(|r| r.set_io_sel(pin.pin() as usize, T::IO_SEL));
        }

        p.uart_ctrl().write(|r| {
            r.set_mode(UartCtrlMode::UART_STD);
        });

        p.uart_tx_ctrl().write(|r| {
            r.set_parity_enabled(config.parity != Parity::None);
            r.set_parity(config.parity == Parity::Odd);
            r.set_stop_bits(config.stop_bits as u8);
        });
        p.uart_rx_ctrl().write(|r| {
            r.set_polarity(config.invert_rx);
            r.set_parity_enabled(config.parity != Parity::None);
            r.set_parity(config.parity == Parity::Odd);
            r.set_stop_bits(config.stop_bits as u8);
            // Default values
            r.set_break_width(10);
        });
        p.uart_flow_ctrl().write(|r| {
            r.set_cts_enabled(cts.is_some());
            r.set_cts_polarity(config.invert_cts);
            r.set_rts_polarity(config.invert_rts);
            // TODO: RTS trigger level
        });

        p.tx_ctrl().write(|r| {
            r.set_msb_first(config.bit_direction == BitDirection::MSB);
            r.set_data_width(config.data_bits as u8);
        });
        p.rx_ctrl().write(|r| {
            r.set_msb_first(config.bit_direction == BitDirection::MSB);
            r.set_data_width(config.data_bits as u8);
            r.set_median(config.digital_filter);
        });

        p.tx_fifo_ctrl().write(|r| r.set_clear(true));
        // Set TX trigger threshold a little high so that we get
        // woken before the FIFO completely drains to minimize
        // transfer stalls.
        p.tx_fifo_ctrl().write(|r| r.set_trigger_level(2));
        p.rx_fifo_ctrl().write(|r| r.set_clear(true));
        p.rx_fifo_ctrl().write(|r| r.set_trigger_level(0));

        if has_irq {
            // Mask all interrupts initially.
            let p = T::regs();
            p.intr_m_mask().write_value(Default::default());
            p.intr_tx_mask().write_value(Default::default());
            p.intr_rx_mask().write_value(Default::default());
            T::Interrupt::unpend();
            unsafe { T::Interrupt::enable() };
        }

        p.ctrl().write(|r| {
            r.set_mode(CtrlMode::UART);
            // TODO: support word size of 9 bits
            // r.set_byte_mode(config.data_bits != DataBits::_9);
            r.set_byte_mode(true);
            r.set_ovs(ovs);
            r.set_enabled(true);
        });
    }
}

impl<'d, T: Instance + PeripheralClock, M: Mode> Uart<'d, T, M> {
    /// Split the Uart into a transmitter and receiver, which is particularly
    /// useful when having two tasks correlating to transmitting and receiving.
    pub fn split(self) -> (UartTx<'d, T, M>, UartRx<'d, T, M>) {
        (self.tx, self.rx)
    }
}

mod eio {
    use super::*;

    impl embedded_io::Error for Error {
        fn kind(&self) -> embedded_io::ErrorKind {
            embedded_io::ErrorKind::Other
        }
    }

    impl<'d, T, M> embedded_io::ErrorType for UartRx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;
    }

    impl<'d, T, M> embedded_io::ErrorType for UartTx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;
    }

    impl<'d, T, M> embedded_io::ErrorType for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;
    }

    impl<'d, T, M> embedded_io::Read for UartRx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            loop {
                let bytes_read = self.drain_fifo(buf)?;
                if bytes_read != 0 {
                    return Ok(bytes_read);
                }
            }
        }
    }

    impl<'d, T, M> embedded_io::ReadReady for UartRx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn read_ready(&mut self) -> Result<bool, Self::Error> {
            Ok(T::regs().intr_rx().read().not_empty())
        }
    }

    impl<'d, T, M> embedded_io::Write for UartTx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            let p = T::regs();
            for (i, word) in buf.iter().enumerate() {
                loop {
                    p.intr_tx().write(|r| r.set_not_full(true));
                    if p.intr_tx().read().not_full() {
                        p.tx_fifo_wr().write(|r| r.set_data(*word as u16));
                        break;
                    } else if i != 0 {
                        return Ok(i);
                    }
                }
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            let p = T::regs();
            loop {
                p.intr_tx().write(|r| r.set_uart_done(true));
                if p.intr_tx().read().uart_done() {
                    return Ok(());
                }
            }
        }
    }

    impl<'d, T, M> embedded_io::WriteReady for UartTx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn write_ready(&mut self) -> Result<bool, Self::Error> {
            let p = T::regs();
            p.intr_tx().write(|r| r.set_not_full(true));
            Ok(p.intr_tx().read().not_full())
        }
    }

    impl<'d, T, M> embedded_io::Read for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            embedded_io::Read::read(&mut self.rx, buf)
        }
    }

    impl<'d, T, M> embedded_io::ReadReady for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn read_ready(&mut self) -> Result<bool, Self::Error> {
            embedded_io::ReadReady::read_ready(&mut self.rx)
        }
    }

    impl<'d, T, M> embedded_io::Write for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            embedded_io::Write::write(&mut self.tx, buf)
        }

        fn flush(&mut self) -> Result<(), Self::Error> {
            embedded_io::Write::flush(&mut self.tx)
        }
    }

    impl<'d, T, M> embedded_io::WriteReady for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn write_ready(&mut self) -> Result<bool, Self::Error> {
            embedded_io::WriteReady::write_ready(&mut self.tx)
        }
    }
}

#[cfg(feature = "nightly")]
mod eio_async {
    use super::*;

    use embedded_io_async::ReadExactError;

    impl<'d, T> embedded_io_async::Read for UartRx<'d, T, Async>
    where
        T: Instance + PeripheralClock,
    {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            if buf.is_empty() {
                return Ok(0);
            }

            let p = T::regs();

            self.wait_on(
                |_me| {
                    let r = p.intr_rx().read();
                    p.intr_rx().write_value(r);
                    if r.overflow() {
                        Poll::Ready(Err(Error::Overrun))
                    } else if r.parity_error() {
                        Poll::Ready(Err(Error::Parity))
                    } else if r.frame_error() {
                        Poll::Ready(Err(Error::Framing))
                    } else if r.break_detect() {
                        Poll::Ready(Err(Error::Break))
                    } else if r.not_empty() {
                        Poll::Ready(Ok(()))
                    } else {
                        Poll::Pending
                    }
                },
                |_me| {
                    p.intr_rx_mask().write(|r| {
                        r.set_break_detect(true);
                        r.set_parity_error(true);
                        r.set_frame_error(true);
                        r.set_overflow(true);
                        r.set_not_empty(true);
                    });
                },
            )
            .await?;

            let received = (Self::rx_fifo_len() as usize).min(buf.len());
            for word in &mut buf[..received] {
                *word = p.rx_fifo_rd().read().data() as u8;
            }

            p.intr_rx().write(|r| r.set_not_empty(true));

            Ok(received)
        }

        async fn read_exact(
            &mut self,
            mut buf: &mut [u8],
        ) -> Result<(), ReadExactError<Self::Error>> {
            let p = T::regs();

            while !buf.is_empty() {
                self.wait_on(
                    |_me| {
                        let r = p.intr_rx().read();
                        p.intr_rx().write_value(r);
                        if r.overflow() {
                            Poll::Ready(Err(Error::Overrun))
                        } else if r.parity_error() {
                            Poll::Ready(Err(Error::Parity))
                        } else if r.frame_error() {
                            Poll::Ready(Err(Error::Framing))
                        } else if r.break_detect() {
                            Poll::Ready(Err(Error::Break))
                        } else if r.not_empty() {
                            Poll::Ready(Ok(()))
                        } else {
                            Poll::Pending
                        }
                    },
                    |_me| {
                        p.intr_rx_mask().write(|r| {
                            r.set_break_detect(true);
                            r.set_parity_error(true);
                            r.set_frame_error(true);
                            r.set_overflow(true);
                            r.set_not_empty(true);
                        });
                    },
                )
                .await?;

                let received = (Self::rx_fifo_len() as usize).min(buf.len());
                for word in &mut buf[..received] {
                    *word = p.rx_fifo_rd().read().data() as u8;
                }
                buf = &mut buf[received..];

                p.intr_rx().write(|r| r.set_not_empty(true));
            }

            Ok(())
        }
    }

    impl<'d, T> embedded_io_async::Write for UartTx<'d, T, Async>
    where
        T: Instance + PeripheralClock,
    {
        async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            if buf.is_empty() {
                return Ok(0);
            }

            let p = T::regs();

            self.wait_on(
                |_me| {
                    p.intr_tx().write(|r| r.set_not_full(true));
                    let r = p.intr_tx().read();
                    p.intr_tx().write_value(r);
                    if r.not_full() {
                        Poll::Ready(())
                    } else {
                        Poll::Pending
                    }
                },
                |_me| p.intr_tx_mask().write(|r| r.set_not_full(true)),
            )
            .await;

            let words_to_write = (Self::tx_fifo_capacity() as usize).min(buf.len());
            for word in &buf[..words_to_write] {
                p.tx_fifo_wr().write(|r| r.set_data(*word as u16));
            }
            Ok(words_to_write)
        }

        async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
            let p = T::regs();

            let mut write = buf.iter();

            'xmit: loop {
                let tx_fifo_space = Self::tx_fifo_capacity();
                for _ in 0..tx_fifo_space {
                    if let Some(word) = write.next() {
                        p.tx_fifo_wr().write(|r| r.set_data(*word as u16));
                    } else {
                        break 'xmit;
                    }
                }

                self.wait_on(
                    |_me| {
                        p.intr_tx().write(|r| r.set_not_full(true));
                        let r = p.intr_tx().read();
                        p.intr_tx().write_value(r);
                        if r.not_full() {
                            Poll::Ready(())
                        } else {
                            Poll::Pending
                        }
                    },
                    |_me| {
                        p.intr_tx_mask().write(|r| r.set_trigger(true));
                    },
                )
                .await;
            }
            Ok(())
        }

        async fn flush(&mut self) -> Result<(), Self::Error> {
            let p = T::regs();
            self.wait_on(
                |_me| {
                    p.intr_tx().write(|r| r.set_uart_done(true));
                    let r = p.intr_tx().read();
                    p.intr_tx().write_value(r);
                    if r.uart_done() {
                        Poll::Ready(Ok(()))
                    } else {
                        Poll::Pending
                    }
                },
                |_me| {
                    p.intr_tx_mask().write(|r| r.set_uart_done(true));
                },
            )
            .await
        }
    }

    impl<'d, T> embedded_io_async::Read for Uart<'d, T, Async>
    where
        T: Instance + PeripheralClock,
    {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            embedded_io_async::Read::read(&mut self.rx, buf).await
        }
    }

    impl<'d, T> embedded_io_async::Write for Uart<'d, T, Async>
    where
        T: Instance + PeripheralClock,
    {
        async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            embedded_io_async::Write::write(&mut self.tx, buf).await
        }

        async fn flush(&mut self) -> Result<(), Self::Error> {
            embedded_io_async::Write::flush(&mut self.tx).await
        }
    }
}

mod eh02 {
    use super::*;

    use embedded_hal_02::blocking::serial::write::Default as WriteBlockingDefault;
    use embedded_hal_02::serial::{Read as ReadNb, Write as WriteNb};

    impl<'d, T, M> ReadNb<u8> for UartRx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;
        fn read(&mut self) -> Result<u8, nb::Error<Self::Error>> {
            let mut b = 0;
            let bytes_read = self
                .drain_fifo(core::slice::from_mut(&mut b))
                .map_err(nb::Error::Other)?;
            if bytes_read == 0 {
                Err(nb::Error::WouldBlock)
            } else {
                Ok(b)
            }
        }
    }

    impl<'d, T, M> WriteNb<u8> for UartTx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;

        fn write(&mut self, word: u8) -> Result<(), nb::Error<Self::Error>> {
            let p = T::regs();
            p.intr_tx().write(|r| r.set_not_full(true));
            if p.intr_tx().read().not_full() {
                p.tx_fifo_wr().write(|r| r.set_data(word as u16));
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }

        fn flush(&mut self) -> Result<(), nb::Error<Self::Error>> {
            let p = T::regs();
            p.intr_tx().write(|r| r.set_uart_done(true));
            if p.intr_tx().read().uart_done() {
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }
    }

    impl<'d, T, M> WriteBlockingDefault<u8> for UartTx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
    }

    impl<'d, T, M> ReadNb<u8> for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;

        fn read(&mut self) -> Result<u8, nb::Error<Self::Error>> {
            ReadNb::read(&mut self.rx)
        }
    }

    impl<'d, T, M> WriteNb<u8> for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;

        fn write(&mut self, word: u8) -> Result<(), nb::Error<Self::Error>> {
            WriteNb::write(&mut self.tx, word)
        }

        fn flush(&mut self) -> Result<(), nb::Error<Self::Error>> {
            WriteNb::flush(&mut self.tx)
        }
    }

    impl<'d, T, M> WriteBlockingDefault<u8> for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
    }
}

#[cfg(feature = "unstable-traits")]
mod eh1 {
    use super::*;

    impl embedded_hal_nb::serial::Error for Error {
        fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
            match *self {
                Self::Framing => embedded_hal_nb::serial::ErrorKind::FrameFormat,
                Self::Break => embedded_hal_nb::serial::ErrorKind::Other,
                Self::Overrun => embedded_hal_nb::serial::ErrorKind::Overrun,
                Self::Parity => embedded_hal_nb::serial::ErrorKind::Parity,
            }
        }
    }

    impl<'d, T, M> embedded_hal_nb::serial::ErrorType for UartRx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;
    }

    impl<'d, T, M> embedded_hal_nb::serial::ErrorType for UartTx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;
    }

    impl<'d, T, M> embedded_hal_nb::serial::ErrorType for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        type Error = Error;
    }

    impl<'d, T, M> embedded_hal_nb::serial::Read for UartRx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn read(&mut self) -> nb::Result<u8, Self::Error> {
            embedded_hal_02::serial::Read::read(self)
        }
    }

    impl<'d, T, M> embedded_hal_nb::serial::Write for UartTx<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
            embedded_hal_02::serial::Write::write(self, word)
        }

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            embedded_hal_02::serial::Write::flush(self)
        }
    }

    impl<'d, T, M> embedded_hal_nb::serial::Read for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn read(&mut self) -> Result<u8, nb::Error<Self::Error>> {
            embedded_hal_nb::serial::Read::read(&mut self.rx)
        }
    }

    impl<'d, T, M> embedded_hal_nb::serial::Write for Uart<'d, T, M>
    where
        T: Instance + PeripheralClock,
        M: Mode,
    {
        fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
            embedded_hal_nb::serial::Write::write(&mut self.tx, word)
        }

        fn flush(&mut self) -> nb::Result<(), Self::Error> {
            embedded_hal_nb::serial::Write::flush(&mut self.tx)
        }
    }
}

mod sealed {
    use super::*;

    pub trait Mode {}

    pub trait Instance {
        type Interrupt: interrupt::typelevel::Interrupt;
        const IO_SEL: IoSel;

        fn regs() -> crate::pac::scb::Scb;
        fn tx_waker() -> &'static AtomicWaker;
        fn rx_waker() -> &'static AtomicWaker;
    }
    pub trait TxPin<T: Instance> {}
    pub trait RxPin<T: Instance> {}
    pub trait CtsPin<T: Instance> {}
    pub trait RtsPin<T: Instance> {}
}

pub trait Mode: sealed::Mode {}

macro_rules! impl_mode {
    ($name:ident) => {
        impl sealed::Mode for $name {}
        impl Mode for $name {}
    };
}

pub struct Blocking;
pub struct Async;

impl_mode!(Blocking);
impl_mode!(Async);

pub trait Instance: sealed::Instance {}

macro_rules! impl_instance {
    ($inst:ident, $irq:ident, $io_sel:ident) => {
        impl sealed::Instance for peripherals::$inst {
            type Interrupt = crate::interrupt::typelevel::$irq;
            // TODO: somehow move to pins
            const IO_SEL: IoSel = IoSel::$io_sel;

            #[inline]
            fn regs() -> crate::pac::scb::Scb {
                pac::$inst
            }

            #[inline]
            fn tx_waker() -> &'static AtomicWaker {
                static WAKER: AtomicWaker = AtomicWaker::new();

                &WAKER
            }

            #[inline]
            fn rx_waker() -> &'static AtomicWaker {
                static WAKER: AtomicWaker = AtomicWaker::new();

                &WAKER
            }
        }
        impl Instance for peripherals::$inst {}
    };
}

impl_instance!(SCB0, SCB_0_INTERRUPT, ACT_1);
impl_instance!(SCB1, SCB_1_INTERRUPT, ACT_1);
impl_instance!(SCB2, SCB_2_INTERRUPT, ACT_3);

pub trait TxPin<T: Instance>: sealed::TxPin<T> + crate::gpio::Pin {}
pub trait RxPin<T: Instance>: sealed::RxPin<T> + crate::gpio::Pin {}
pub trait CtsPin<T: Instance>: sealed::CtsPin<T> + crate::gpio::Pin {}
pub trait RtsPin<T: Instance>: sealed::RtsPin<T> + crate::gpio::Pin {}

macro_rules! impl_pin {
    ($pin:ident, $instance:ident, $function:ident) => {
        impl sealed::$function<peripherals::$instance> for peripherals::$pin {}
        impl $function<peripherals::$instance> for peripherals::$pin {}
    };
}

impl_pin!(PIN_0_4, SCB1, RxPin);
impl_pin!(PIN_0_4, SCB2, RxPin);
impl_pin!(PIN_0_5, SCB1, TxPin);
impl_pin!(PIN_0_5, SCB2, TxPin);
impl_pin!(PIN_0_6, SCB1, CtsPin);
impl_pin!(PIN_0_6, SCB2, TxPin);
impl_pin!(PIN_0_7, SCB1, RtsPin);
impl_pin!(PIN_1_0, SCB0, RxPin);
impl_pin!(PIN_1_1, SCB0, TxPin);
impl_pin!(PIN_1_2, SCB0, CtsPin);
impl_pin!(PIN_1_3, SCB0, RtsPin);
impl_pin!(PIN_3_0, SCB1, RxPin);
impl_pin!(PIN_3_1, SCB1, TxPin);
impl_pin!(PIN_3_2, SCB1, CtsPin);
impl_pin!(PIN_3_3, SCB1, RtsPin);
impl_pin!(PIN_4_0, SCB0, RxPin);
impl_pin!(PIN_4_1, SCB0, TxPin);
impl_pin!(PIN_4_2, SCB0, CtsPin);
impl_pin!(PIN_4_3, SCB0, RtsPin);
