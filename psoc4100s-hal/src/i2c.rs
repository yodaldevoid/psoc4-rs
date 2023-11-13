use core::future;
use core::marker::PhantomData;
use core::task::Poll;

use embassy_hal_internal::{into_ref, PeripheralRef};
use embassy_sync::waitqueue::AtomicWaker;
use pac::gpio::vals::Dm;
use pac::hsiom::vals::IoSel;
use pac::scb::vals::*;

use crate::clocks::{PeriClkDiv, PeripheralClock};
use crate::gpio::AnyPin;
use crate::interrupt::typelevel::{Binding, Interrupt};
use crate::{interrupt, pac, peripherals, Peripheral};

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoAcknowledgeReason {
    Address,
    Data,
}

/// I2C error abort reason
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AbortReason {
    /// A bus operation was not acknowledged, e.g. due to the addressed device
    /// not being available on the bus or the device not being ready to process
    /// requests at the moment
    NoAcknowledge(NoAcknowledgeReason),
    /// The arbitration was lost, e.g. electrical problems with the clock signal
    ArbitrationLoss,
    /// Unexpected START or STOP condition
    BusError,
}

/// I2C error
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// I2C abort with error
    Abort(AbortReason),
    /// Target i2c address is out of range
    AddressOutOfRange(u8),
    /// Target i2c address is reserved
    AddressReserved(u8),
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub struct Config {
    pub frequency: u32,
    pub scl_pullup_internal: bool,
    pub sda_pullup_internal: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 100_000,
            scl_pullup_internal: false,
            sda_pullup_internal: false,
        }
    }
}

pub const RX_FIFO_SIZE: u16 = 8;
pub const TX_FIFO_SIZE: u16 = 8;

pub struct I2c<'d, T: Instance, M: Mode> {
    _phantom: PhantomData<(&'d mut T, M)>,
}

impl<'d, T: Instance + PeripheralClock> I2c<'d, T, Blocking> {
    pub fn new_blocking(
        peri: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        scl: impl Peripheral<P = impl SclPin<T>> + 'd,
        sda: impl Peripheral<P = impl SdaPin<T>> + 'd,
        config: Config,
    ) -> Self {
        into_ref!(scl, sda);
        Self::new_inner(peri, clock, scl.map_into(), sda.map_into(), config)
    }
}

impl<'d, T: Instance + PeripheralClock> I2c<'d, T, Async> {
    pub fn new_async(
        peri: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        scl: impl Peripheral<P = impl SclPin<T>> + 'd,
        sda: impl Peripheral<P = impl SdaPin<T>> + 'd,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Self {
        into_ref!(scl, sda);

        let i2c = Self::new_inner(peri, clock, scl.map_into(), sda.map_into(), config);

        // Mask all interrupts initially.
        let p = T::regs();
        p.intr_m_mask().write_value(Default::default());
        p.intr_tx_mask().write_value(Default::default());
        p.intr_rx_mask().write_value(Default::default());
        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };

        i2c
    }

    /// Calls `f` to check if we are ready or not.
    /// If not, `g` is called once the waker is set (to eg enable the required interrupts).
    async fn wait_on<F, U, G>(&mut self, mut f: F, mut g: G) -> U
    where
        F: FnMut(&mut Self) -> Poll<U>,
        G: FnMut(&mut Self),
    {
        future::poll_fn(|cx| {
            let r = f(self);

            if r.is_pending() {
                T::waker().register(cx.waker());
                g(self);
            }
            r
        })
        .await
    }

    // TODO: Move start/restart into method to reduce time between address and read.
    async fn read_async_internal(
        &mut self,
        mut read: &mut [u8],
        send_stop: bool,
    ) -> Result<(), Error> {
        let p = T::regs();

        let mut abort_reason = Ok(());

        while read.len() > 0 {
            let res = self
                .wait_on(
                    |_me| {
                        let r = p.intr_m().read();
                        p.intr_m().write_value(r);
                        // Clear the level interrupt.
                        p.intr_rx().modify(|_| {});
                        if r.i2c_bus_error() {
                            Poll::Ready(Err(Error::Abort(AbortReason::BusError)))
                        } else if r.i2c_arb_lost() {
                            Poll::Ready(Err(Error::Abort(AbortReason::ArbitrationLoss)))
                        } else if r.i2c_nack() {
                            // Always going to be an address NACK.
                            Poll::Ready(Err(Error::Abort(AbortReason::NoAcknowledge(
                                NoAcknowledgeReason::Address,
                            ))))
                        } else if !Self::rx_fifo_empty() {
                            // Resume if there's data in the RX FIFO.
                            Poll::Ready(Ok(()))
                        } else {
                            Poll::Pending
                        }
                    },
                    |_me| {
                        // Set our trigger level a little low so we can switch
                        // to manual ACKs for the last byte.
                        let trigger_level =
                            read.len().saturating_sub(2).min(RX_FIFO_SIZE as usize) as u8;
                        p.rx_fifo_ctrl()
                            .write(|r| r.set_trigger_level(trigger_level));
                        p.intr_rx_mask().write(|r| r.set_trigger(true));
                        p.intr_m_mask().write(|r| {
                            r.set_i2c_bus_error(true);
                            r.set_i2c_arb_lost(true);
                            r.set_i2c_nack(true);
                        });
                        if trigger_level != 0 {
                            p.i2c_ctrl().modify(|r| r.set_m_ready_data_ack(true));
                        } else if read.len() > 1 {
                            p.i2c_m_cmd().write(|r| r.set_m_ack(true));
                        }
                    },
                )
                .await;

            if res.is_err() {
                abort_reason = res;
                break;
            }

            let received = (Self::rx_fifo_len() as usize).min(read.len());
            for b in &mut read[..received] {
                *b = p.rx_fifo_rd().read().data() as u8;
            }
            read = &mut read[received..];
        }

        let is_nack = matches!(
            abort_reason,
            Err(Error::Abort(AbortReason::NoAcknowledge(_)))
        );
        if (abort_reason.is_ok() && send_stop) || is_nack {
            let stop_res = Self::stop();
            abort_reason.and(stop_res)
        } else if abort_reason.is_err() {
            Self::reset();
            abort_reason
        } else {
            Ok(())
        }
    }

    // TODO: Move start/restart into method to reduce time between address and write.
    async fn write_async_internal(
        &mut self,
        write: impl IntoIterator<Item = u8>,
        send_stop: bool,
    ) -> Result<(), Error> {
        let p = T::regs();

        let mut write = write.into_iter();

        let res = 'xmit: loop {
            let tx_fifo_space = Self::tx_fifo_capacity();

            for _ in 0..tx_fifo_space {
                if let Some(byte) = write.next() {
                    p.tx_fifo_wr().write(|r| r.set_data(byte as u16));
                    // This could be conditionally be done only for the last
                    // byte, but a single write is going to be quicker than a
                    // conditional branch and a possible write.
                    p.intr_tx().write(|r| r.set_underflow(true));
                } else {
                    break 'xmit Ok(());
                }
            }

            let res = self
                .wait_on(
                    |_me| {
                        let r = p.intr_m().read();
                        p.intr_m().write_value(r);
                        // Clear the level interrupt.
                        p.intr_tx().modify(|_| {});
                        if r.i2c_bus_error() {
                            Poll::Ready(Err(Error::Abort(AbortReason::BusError)))
                        } else if r.i2c_arb_lost() {
                            Poll::Ready(Err(Error::Abort(AbortReason::ArbitrationLoss)))
                        } else if r.i2c_nack() {
                            Poll::Ready(Err(Error::Abort(AbortReason::NoAcknowledge(
                                // TODO: figure out if address or data
                                NoAcknowledgeReason::Address,
                            ))))
                        } else if !Self::tx_fifo_full() {
                            // Resume if there's any space free in the TX FIFO.
                            Poll::Ready(Ok(()))
                        } else {
                            Poll::Pending
                        }
                    },
                    |_me| {
                        // Set tx "free" threshold a little high so that we get
                        // woken before the fifo completely drains to minimize
                        // transfer stalls.
                        p.tx_fifo_ctrl().write(|r| r.set_trigger_level(2));
                        critical_section::with(|_| {
                            p.intr_m_mask().write(|r| {
                                r.set_i2c_bus_error(true);
                                r.set_i2c_arb_lost(true);
                                r.set_i2c_nack(true);
                            });
                            p.intr_tx_mask().write(|r| r.set_trigger(true));
                        });
                    },
                )
                .await;
            if res.is_err() {
                break res;
            }
        };

        let res = if res.is_ok() {
            // Wait for the data to finish transmitting.
            self.wait_on(
                |_me| {
                    let intr_tx = p.intr_tx().read();
                    p.intr_tx().write_value(intr_tx);
                    let intr_m = p.intr_m().read();
                    p.intr_m().write_value(intr_m);
                    if intr_m.i2c_bus_error() {
                        Poll::Ready(Err(Error::Abort(AbortReason::BusError)))
                    } else if intr_m.i2c_arb_lost() {
                        Poll::Ready(Err(Error::Abort(AbortReason::ArbitrationLoss)))
                    } else if intr_m.i2c_nack() {
                        Poll::Ready(Err(Error::Abort(AbortReason::NoAcknowledge(
                            // TODO: figure out if address or data
                            NoAcknowledgeReason::Address,
                        ))))
                    } else if intr_tx.underflow() {
                        Poll::Ready(Ok(()))
                    } else {
                        Poll::Pending
                    }
                },
                |_me| {
                    critical_section::with(|_| {
                        p.intr_m_mask().write(|r| {
                            r.set_i2c_bus_error(true);
                            r.set_i2c_arb_lost(true);
                            r.set_i2c_nack(true);
                        });
                        p.intr_tx_mask().write(|r| r.set_underflow(true));
                    });
                },
            )
            .await
        } else {
            res
        };

        let is_nack = matches!(res, Err(Error::Abort(AbortReason::NoAcknowledge(_))));
        if (res.is_ok() && send_stop) || is_nack {
            let stop_res = Self::stop();
            res.and(stop_res)
        } else if res.is_err() {
            Self::reset();
            res
        } else {
            Ok(())
        }
    }

    pub async fn write_async(
        &mut self,
        addr: u8,
        write: impl IntoIterator<Item = u8>,
    ) -> Result<(), Error> {
        Self::start(addr, false)?;
        self.write_async_internal(write, true).await
    }

    pub async fn read_async(
        &mut self,
        addr: u8,
        read: &mut [u8],
    ) -> Result<(), Error> {
        Self::start(addr, true)?;
        self.read_async_internal(read, true).await
    }

    pub async fn write_read_async(
        &mut self,
        addr: u8,
        bytes: impl IntoIterator<Item = u8>,
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        Self::start(addr, false)?;
        self.write_async_internal(bytes, false).await?;
        Self::restart(addr, true)?;
        self.read_async_internal(buffer, true).await
    }
}

pub struct InterruptHandler<T: Instance> {
    _scb: PhantomData<T>,
}

impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    // Mask interrupts and wake any task waiting for this interrupt.
    unsafe fn on_interrupt() {
        let p = T::regs();
        p.i2c_ctrl().modify(|r| r.set_m_ready_data_ack(false));
        p.intr_m_mask().write_value(Default::default());
        p.intr_tx_mask().write_value(Default::default());
        p.intr_rx_mask().write_value(Default::default());

        T::waker().wake();
    }
}

pub(crate) fn set_up_i2c_pin<'d, P, T>(pin: &P, pullup: bool)
where
    P: core::ops::Deref<Target = T>,
    T: crate::gpio::Pin,
{
    let mode = if pullup { Dm::_0_PU } else { Dm::_0_Z };
    pin.prt()
        .pc()
        .modify(|r| r.set_dm(pin.pin() as usize, mode));
    pin.hsiom()
        .port_sel()
        .modify(|r| r.set_io_sel(pin.pin() as usize, IoSel::DS_2));
}

impl<'d, T: Instance + PeripheralClock + 'd, M: Mode> I2c<'d, T, M> {
    fn new_inner(
        _peri: impl Peripheral<P = T> + 'd,
        clock: impl PeriClkDiv + 'd,
        scl: PeripheralRef<'d, AnyPin>,
        sda: PeripheralRef<'d, AnyPin>,
        config: Config,
    ) -> Self {
        T::set_clock_divider(Some(&clock));

        assert!(config.frequency <= 1_000_000);
        assert!(config.frequency > 0);

        let p = T::regs();

        p.ctrl().write(|r| r.set_enabled(false));

        // Configure SCL & SDA pins.
        set_up_i2c_pin(&scl, config.scl_pullup_internal);
        set_up_i2c_pin(&sda, config.sda_pullup_internal);

        // Set FIFO watermarks to 1 to make things simpler.
        p.tx_fifo_ctrl().write(|r| r.set_clear(true));
        p.tx_fifo_ctrl().write(|r| r.set_trigger_level(1));
        p.rx_fifo_ctrl().write(|r| r.set_clear(true));
        p.rx_fifo_ctrl().write(|r| r.set_trigger_level(0));

        // Configure baudrate
        let period = (clock.frequency() + config.frequency / 2) / config.frequency;
        let (low_phase, high_phase) = if config.frequency < 100_000 {
            let low_phase = period / 2; // spend 50% of the period low
            let high_phase = period - low_phase; // spend 50% of the period high
            (low_phase, high_phase)
        } else {
            let low_phase = period * 3 / 5; // spend 60% of the period low
            let high_phase = period - low_phase; // and 40% of the period high
            (low_phase, high_phase)
        };
        let digital_filter = config.frequency >= 1_000_000;

        assert!(low_phase <= 16);
        assert!(high_phase <= 16);
        assert!(low_phase >= 8);
        assert!(high_phase >= 8);

        p.i2c_ctrl().write(|r| {
            r.set_master_mode(true);
            r.set_m_not_ready_data_nack(false);
            r.set_m_ready_data_ack(false);
            r.set_low_phase_ovs(low_phase as u8 - 1);
            r.set_high_phase_ovs(high_phase as u8 - 1);
        });
        // Configure for MSB 8-bit dataframes.
        p.tx_ctrl().write(|r| {
            // Defaults
            r.set_msb_first(true);
            r.set_data_width(7);
        });
        // Configure for MSB 8-bit dataframes and enable digital filter if needed.
        p.rx_ctrl().write(|r| {
            r.set_median(digital_filter);
            // Defaults
            r.set_msb_first(true);
            r.set_data_width(7);
        });
        // TODO: configure analog filter
        p.i2c_cfg().write(|r| {
            r.set_scl_in_filt_sel(!digital_filter);
            r.set_sda_in_filt_sel(!digital_filter);
            // Defaults
            r.set_sda_out_filt2_trim(2);
            r.set_sda_out_filt1_trim(2);
            r.set_sda_out_filt0_trim(2);
            r.set_sda_in_filt_trim(3);
        });
        p.ctrl().write(|r| {
            r.set_mode(CtrlMode::I2C);
            r.set_byte_mode(true);
            r.set_enabled(true);
        });

        Self {
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn tx_fifo_full() -> bool {
        Self::tx_fifo_capacity() == 0
    }

    #[inline]
    fn tx_fifo_capacity() -> u16 {
        TX_FIFO_SIZE - T::regs().tx_fifo_status().read().used()
    }

    #[inline]
    fn rx_fifo_empty() -> bool {
        Self::rx_fifo_len() == 0
    }

    #[inline]
    fn rx_fifo_len() -> u16 {
        T::regs().rx_fifo_status().read().used()
    }

    #[inline]
    fn reset() {
        let p = T::regs();
        p.ctrl().modify(|r| r.set_enabled(false));
        p.i2c_m_cmd().write_value(Default::default());
        p.ctrl().modify(|r| r.set_enabled(true));
    }

    fn start(addr: u8, read: bool) -> Result<(), Error> {
        if addr >= 0x80 {
            return Err(Error::AddressOutOfRange(addr));
        }

        if i2c_reserved_addr(addr) {
            return Err(Error::AddressReserved(addr));
        }

        let p = T::regs();

        p.i2c_ctrl().modify(|r| r.set_m_ready_data_ack(false));
        p.intr_m().write(|r| {
            r.set_i2c_arb_lost(true);
            r.set_i2c_bus_error(true);
            r.set_i2c_nack(true);
            r.set_i2c_ack(true);
            r.set_i2c_stop(true);
        });
        p.intr_rx().write(|r| r.set_not_empty(true));
        p.tx_fifo_ctrl().modify(|r| r.set_clear(true));
        p.tx_fifo_ctrl().modify(|r| r.set_clear(false));
        p.rx_fifo_ctrl().modify(|r| r.set_clear(true));
        p.rx_fifo_ctrl().modify(|r| r.set_clear(false));

        p.tx_fifo_wr()
            .write(|r| r.set_data((addr << 1 | read as u8) as u16));
        // Clear the TX underflow interrupt for async operation.
        p.intr_tx().write(|r| r.set_underflow(true));
        p.i2c_m_cmd().write(|r| r.set_m_start_on_idle(true));

        Ok(())
    }

    fn start_blocking(addr: u8, read: bool) -> Result<(), Error> {
        Self::start(addr, read)?;

        let p = T::regs();

        loop {
            let r = p.intr_m().read();
            p.intr_m().write_value(r);

            if r.i2c_bus_error() {
                Self::reset();
                return Err(Error::Abort(AbortReason::BusError));
            }
            if r.i2c_arb_lost() {
                Self::reset();
                return Err(Error::Abort(AbortReason::ArbitrationLoss));
            }

            if r.i2c_nack() {
                let _ = Self::stop();
                return Err(Error::Abort(AbortReason::NoAcknowledge(
                    NoAcknowledgeReason::Address,
                )));
            }
            if r.i2c_ack() {
                return Ok(());
            }
        }
    }

    fn restart(addr: u8, read: bool) -> Result<(), Error> {
        if addr >= 0x80 {
            return Err(Error::AddressOutOfRange(addr));
        }

        if i2c_reserved_addr(addr) {
            return Err(Error::AddressReserved(addr));
        }

        let p = T::regs();

        let finish_read = p.i2c_status().read().m_read();
        p.i2c_m_cmd().write(|r| {
            r.set_m_nack(finish_read);
            r.set_m_start(true);
        });

        while !finish_read && p.i2c_m_cmd().read().m_start() {}

        p.tx_fifo_wr()
            .write(|r| r.set_data((addr << 1 | read as u8) as u16));

        Ok(())
    }

    fn restart_blocking(addr: u8, read: bool) -> Result<(), Error> {
        Self::restart(addr, read)?;

        let p = T::regs();

        loop {
            let r = p.intr_m().read();
            p.intr_m().write_value(r);

            if r.i2c_bus_error() {
                Self::reset();
                return Err(Error::Abort(AbortReason::BusError));
            }
            if r.i2c_arb_lost() {
                Self::reset();
                return Err(Error::Abort(AbortReason::ArbitrationLoss));
            }

            if r.i2c_nack() {
                let _ = Self::stop();
                return Err(Error::Abort(AbortReason::NoAcknowledge(
                    NoAcknowledgeReason::Address,
                )));
            }
            if r.i2c_ack() {
                return Ok(());
            }
        }
    }

    fn stop() -> Result<(), Error> {
        let p = T::regs();

        p.i2c_m_cmd().write(|r| {
            r.set_m_stop(true);
            r.set_m_nack(true);
        });

        loop {
            let r = p.intr_m().read();
            p.intr_m().write_value(r);

            if r.i2c_bus_error() {
                Self::reset();
                return Err(Error::Abort(AbortReason::BusError));
            }
            if r.i2c_arb_lost() {
                Self::reset();
                return Err(Error::Abort(AbortReason::ArbitrationLoss));
            }

            if r.i2c_stop() {
                return Ok(());
            }
        }
    }

    fn read_blocking_internal(&mut self, read: &mut [u8], send_stop: bool) -> Result<(), Error> {
        let p = T::regs();

        let last_index = read.len() - 1;
        for (i, b) in read.iter_mut().enumerate() {
            let last_byte = i == last_index;
            loop {
                let not_empty = p.intr_rx().read().not_empty();
                let r = p.intr_m().read();
                p.intr_m().write_value(r);

                if not_empty {
                    *b = p.rx_fifo_rd().read().data() as u8;
                    p.intr_rx().write(|r| {
                        r.set_not_empty(true);
                        r.set_trigger(true);
                    });
                }

                if r.i2c_bus_error() {
                    Self::reset();
                    return Err(Error::Abort(AbortReason::BusError));
                }
                if r.i2c_arb_lost() {
                    Self::reset();
                    return Err(Error::Abort(AbortReason::ArbitrationLoss));
                }

                if not_empty {
                    if !last_byte {
                        p.i2c_m_cmd().write(|r| r.set_m_ack(true));
                    }
                    break;
                }
            }
        }

        if send_stop {
            Self::stop()
        } else {
            Ok(())
        }
    }

    fn write_blocking_internal(&mut self, write: &[u8], send_stop: bool) -> Result<(), Error> {
        let p = T::regs();

        for b in write {
            p.tx_fifo_wr().write(|r| r.set_data(*b as u16));
            loop {
                let r = p.intr_m().read();
                p.intr_m().write_value(r);

                if r.i2c_bus_error() {
                    Self::reset();
                    return Err(Error::Abort(AbortReason::BusError));
                }
                if r.i2c_arb_lost() {
                    Self::reset();
                    return Err(Error::Abort(AbortReason::ArbitrationLoss));
                }

                if r.i2c_nack() {
                    let _ = Self::stop();
                    return Err(Error::Abort(AbortReason::NoAcknowledge(
                        NoAcknowledgeReason::Data,
                    )));
                }
                if r.i2c_ack() {
                    break;
                }
            }
        }

        if send_stop {
            Self::stop()
        } else {
            Ok(())
        }
    }

    pub fn read_blocking(&mut self, address: u8, read: &mut [u8]) -> Result<(), Error> {
        Self::start_blocking(address, true)?;
        self.read_blocking_internal(read, true)
    }

    pub fn write_blocking(&mut self, address: u8, write: &[u8]) -> Result<(), Error> {
        Self::start_blocking(address, false)?;
        self.write_blocking_internal(write, true)
    }

    pub fn write_read_blocking(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Error> {
        Self::start_blocking(address, false)?;
        self.write_blocking_internal(write, false)?;
        Self::restart_blocking(address, true)?;
        self.read_blocking_internal(read, true)
    }
}

mod eh02 {
    use embedded_hal_02::blocking::i2c::{Operation, Read, Transactional, Write, WriteRead};

    use super::*;

    impl<'d, T: Instance + PeripheralClock, M: Mode> Read for I2c<'d, T, M> {
        type Error = Error;

        fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            self.read_blocking(address, buffer)
        }
    }

    impl<'d, T: Instance + PeripheralClock, M: Mode> Write for I2c<'d, T, M> {
        type Error = Error;

        fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            self.write_blocking(address, bytes)
        }
    }

    impl<'d, T: Instance + PeripheralClock, M: Mode> WriteRead for I2c<'d, T, M> {
        type Error = Error;

        fn write_read(
            &mut self,
            address: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.write_read_blocking(address, bytes, buffer)
        }
    }

    impl<'d, T: Instance + PeripheralClock, M: Mode> Transactional for I2c<'d, T, M> {
        type Error = Error;

        fn exec(
            &mut self,
            address: u8,
            operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            if let Some(mut read) = operations.first().map(|o| matches!(o, Operation::Read(_))) {
                Self::start_blocking(address, read)?;
                for i in 0..operations.len() {
                    let last = i == operations.len() - 1;
                    match &mut operations[i] {
                        Operation::Read(buf) => {
                            if !read {
                                Self::restart_blocking(address, true)?;
                            }
                            read = true;
                            self.read_blocking_internal(buf, last)?;
                        }
                        Operation::Write(buf) => {
                            if read {
                                Self::restart_blocking(address, false)?;
                            }
                            read = false;
                            self.write_blocking_internal(buf, last)?;
                        }
                    }
                }
            }
            Ok(())
        }
    }
}

#[cfg(feature = "unstable-traits")]
mod eh1 {
    use embedded_hal_1::i2c::{
        Error as I2cError, ErrorKind, ErrorType, I2c as I2cHal, NoAcknowledgeSource, Operation,
    };

    use super::*;

    impl I2cError for Error {
        fn kind(&self) -> ErrorKind {
            match *self {
                Self::Abort(AbortReason::BusError) => ErrorKind::Bus,
                Self::Abort(AbortReason::ArbitrationLoss) => ErrorKind::ArbitrationLoss,
                Self::Abort(AbortReason::NoAcknowledge(NoAcknowledgeReason::Address)) => {
                    ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address)
                }
                Self::Abort(AbortReason::NoAcknowledge(NoAcknowledgeReason::Data)) => {
                    ErrorKind::NoAcknowledge(NoAcknowledgeSource::Data)
                }
                Self::AddressOutOfRange(_) => ErrorKind::Other,
                Self::AddressReserved(_) => ErrorKind::Other,
            }
        }
    }

    impl<'d, T: Instance + PeripheralClock, M: Mode> ErrorType for I2c<'d, T, M> {
        type Error = Error;
    }

    impl<'d, T: Instance + PeripheralClock, M: Mode> I2cHal for I2c<'d, T, M> {
        fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
            self.read_blocking(address, read)
        }

        fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
            self.write_blocking(address, write)
        }

        fn write_read(
            &mut self,
            address: u8,
            write: &[u8],
            read: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.write_read_blocking(address, write, read)
        }

        fn transaction(
            &mut self,
            address: u8,
            operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            if let Some(mut read) = operations.first().map(|o| matches!(o, Operation::Read(_))) {
                Self::start_blocking(address, read)?;
                for i in 0..operations.len() {
                    let last = i == operations.len() - 1;
                    match &mut operations[i] {
                        Operation::Read(buf) => {
                            if !read {
                                Self::restart_blocking(address, true)?;
                            }
                            read = true;
                            self.read_blocking_internal(buf, last)?;
                        }
                        Operation::Write(buf) => {
                            if read {
                                Self::restart_blocking(address, false)?;
                            }
                            read = false;
                            self.write_blocking_internal(buf, last)?;
                        }
                    }
                }
            }
            Ok(())
        }
    }
}

#[cfg(all(feature = "unstable-traits", feature = "nightly"))]
mod nightly {
    use embedded_hal_1::i2c::Operation;

    use super::*;

    impl<'d, T> embedded_hal_async::i2c::I2c for I2c<'d, T, Async>
    where
        T: Instance + PeripheralClock + 'd,
    {
        async fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
            self.read_async(address, read).await
        }

        async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
            self.write_async(address, write.iter().copied()).await
        }

        async fn write_read(&mut self, address: u8, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
            self.write_read_async(address, write.iter().copied(), read).await
        }

        async fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
            if let Some(mut read) = operations.first().map(|o| matches!(o, Operation::Read(_))) {
                Self::start(address, read)?;
                for i in 0..operations.len() {
                    let last = i == operations.len() - 1;
                    match &mut operations[i] {
                        Operation::Read(buf) => {
                            if !read {
                                Self::restart(address, true)?;
                            }
                            read = true;
                            self.read_async_internal(buf, last).await?;
                        }
                        Operation::Write(buf) => {
                            if read {
                                Self::restart(address, false)?;
                            }
                            read = false;
                            self.write_async_internal(buf.iter().copied(), last).await?;
                        }
                    }
                }
            }
            Ok(())
        }
    }
}

pub fn i2c_reserved_addr(addr: u8) -> bool {
    ((addr & 0x78) == 0 || (addr & 0x78) == 0x78) && addr != 0
}

mod sealed {
    use embassy_sync::waitqueue::AtomicWaker;

    use crate::interrupt;

    pub trait Instance {
        type Interrupt: interrupt::typelevel::Interrupt;

        fn regs() -> crate::pac::scb::Scb;
        fn waker() -> &'static AtomicWaker;
    }

    pub trait Mode {}

    pub trait SdaPin<T: Instance> {}
    pub trait SclPin<T: Instance> {}
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
    ($type:ident, $irq:ident) => {
        impl sealed::Instance for peripherals::$type {
            type Interrupt = crate::interrupt::typelevel::$irq;

            #[inline]
            fn regs() -> pac::scb::Scb {
                pac::$type
            }

            #[inline]
            fn waker() -> &'static AtomicWaker {
                static WAKER: AtomicWaker = AtomicWaker::new();

                &WAKER
            }
        }
        impl Instance for peripherals::$type {}
    };
}

impl_instance!(SCB0, SCB_0_INTERRUPT);
impl_instance!(SCB1, SCB_1_INTERRUPT);
impl_instance!(SCB2, SCB_2_INTERRUPT);

pub trait SdaPin<T: Instance>: sealed::SdaPin<T> + crate::gpio::Pin {}
pub trait SclPin<T: Instance>: sealed::SclPin<T> + crate::gpio::Pin {}

macro_rules! impl_pin {
    ($pin:ident, $instance:ident, $function:ident) => {
        impl sealed::$function<peripherals::$instance> for peripherals::$pin {}
        impl $function<peripherals::$instance> for peripherals::$pin {}
    };
}

impl_pin!(PIN_0_0, SCB2, SclPin);
impl_pin!(PIN_0_1, SCB2, SdaPin);
impl_pin!(PIN_0_4, SCB1, SclPin);
impl_pin!(PIN_0_5, SCB1, SdaPin);
impl_pin!(PIN_1_0, SCB0, SclPin);
impl_pin!(PIN_1_1, SCB0, SdaPin);
impl_pin!(PIN_1_2, SCB2, SclPin);
impl_pin!(PIN_1_3, SCB2, SdaPin);
impl_pin!(PIN_2_0, SCB1, SclPin);
impl_pin!(PIN_2_1, SCB1, SdaPin);
impl_pin!(PIN_3_0, SCB1, SclPin);
impl_pin!(PIN_3_1, SCB1, SdaPin);
impl_pin!(PIN_4_0, SCB0, SclPin);
impl_pin!(PIN_4_1, SCB0, SdaPin);
