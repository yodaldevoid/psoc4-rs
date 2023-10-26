use core::cell::Cell;
use portable_atomic::{AtomicBool, Ordering};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::driver::{AlarmHandle, Driver};
use pac::wco::vals::*;

use crate::clocks::{sys_clk_freq, wait_cycles, ILO_SLOW_FREQ_HZ, WCO_FREQ_HZ};
use crate::interrupt::InterruptExt;
use crate::{interrupt, pac};

struct AlarmState {
    timestamp: Cell<u64>,
    callback: Cell<Option<(fn(*mut ()), *mut ())>>,
}
unsafe impl Send for AlarmState {}

const DUMMY_ALARM: AlarmState = AlarmState {
    timestamp: Cell::new(0),
    callback: Cell::new(None),
};

struct WdcTimerDriver {
    alarm: Mutex<CriticalSectionRawMutex, AlarmState>,
    alarm_allocated: AtomicBool,
}

embassy_time::time_driver_impl!(static DRIVER: WdcTimerDriver = WdcTimerDriver {
    alarm: Mutex::const_new(CriticalSectionRawMutex::new(), DUMMY_ALARM),
    alarm_allocated: AtomicBool::new(false),
});

impl Driver for WdcTimerDriver {
    fn now(&self) -> u64 {
        loop {
            let hi = pac::WCO.wdt_ctrhigh().read();
            let lo = pac::WCO.wdt_ctrlow().read().0;
            let hi2 = pac::WCO.wdt_ctrhigh().read();
            if hi == hi2 {
                return (hi as u64) << 32 | (lo as u64);
            }
        }
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        if self.alarm_allocated.swap(true, Ordering::AcqRel) {
            None
        } else {
            Some(AlarmHandle::new(0))
        }
    }

    fn set_alarm_callback(&self, _: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        critical_section::with(|cs| {
            let alarm = &self.alarm.borrow(cs);
            alarm.callback.set(Some((callback, ctx)));
        })
    }

    fn set_alarm(&self, _: AlarmHandle, timestamp: u64) -> bool {
        critical_section::with(|cs| {
            let alarm = &self.alarm.borrow(cs);
            alarm.timestamp.set(timestamp);

            // TODO: if the timestamp is less-than or equal to 3 clock cycles
            // into the future, the alarm might be missed.

            // Arm it.
            pac::WCO.wdt_config().modify(|r| {
                r.set_wdt_mode0(WdtMode::INT);
                r.set_wdt_mode1(WdtMode::NOTHING);
            });

            let now = self.now();
            if timestamp <= now {
                // If the alarm timestamp has passed, the alarm will not fire.
                // Disarm the alarm and return `false` to indicate that.
                pac::WCO.wdt_config().modify(|r| {
                    r.set_wdt_mode0(WdtMode::NOTHING);
                    r.set_wdt_mode1(WdtMode::NOTHING);
                });

                alarm.timestamp.set(u64::MAX);

                false
            } else {
                true
            }
        })
    }
}

impl WdcTimerDriver {
    fn check_alarm(&self) {
        critical_section::with(|cs| {
            let alarm = &self.alarm.borrow(cs);
            let timestamp = alarm.timestamp.get();
            if timestamp <= self.now() {
                // Disarm the alarm.
                pac::WCO.wdt_config().modify(|r| {
                    r.set_wdt_mode0(WdtMode::NOTHING);
                    r.set_wdt_mode1(WdtMode::NOTHING);
                });

                alarm.timestamp.set(u64::MAX);

                // Call after clearing alarm so the callback can set another
                // alarm.
                if let Some((f, ctx)) = alarm.callback.get() {
                    f(ctx);
                }
            } else {
                // If we have not reached the timestamp, set the next alarm and
                // continue waiting.
                pac::WCO.wdt_config().modify(|r| {
                    if r.wdt_mode0() == WdtMode::NOTHING {
                        r.set_wdt_mode0(WdtMode::INT);
                        r.set_wdt_mode1(WdtMode::NOTHING);
                    } else {
                        r.set_wdt_mode0(WdtMode::NOTHING);
                        r.set_wdt_mode1(WdtMode::INT);
                    }
                });
            }
        });

        // Clear the IRQ.
        pac::WCO.wdt_control().modify(|r| {
            r.set_wdt_int0(true);
            r.set_wdt_int1(true);
        });
        pac::WCO.wdt_control().read();
    }
}

pub(crate) unsafe fn init() {
    critical_section::with(|cs| {
        DRIVER.alarm.borrow(cs).timestamp.set(u64::MAX);
    });

    pac::WCO.wdt_clken().write(|r| {
        r.set_clk_wco_en_for_wdt(false);
        r.set_clk_ilo_en_for_wdt(false);
    });

    pac::WCO.wdt_match().write(|r| {
        r.set_wdt_match0(0xFFFF);
        r.set_wdt_match1(0xFFFF);
    });

    let use_wco = cfg!(feature = "time-driver-wdc-wco");
    pac::WCO.wdt_config().write(|r| {
        r.set_wdt_cascade0_1(true);
        r.set_wdt_cascade1_2(true);
        r.set_lfclk_sel(use_wco as u8);
    });

    pac::WCO.wdt_clken().write(|r| {
        r.set_clk_wco_en_for_wdt(use_wco);
        r.set_clk_ilo_en_for_wdt(!use_wco);
    });

    // Wait 4 LFCLK cycles.
    let sysclk_cycles = if use_wco {
        4 * sys_clk_freq() / WCO_FREQ_HZ
    } else {
        4 * sys_clk_freq() / ILO_SLOW_FREQ_HZ
    };
    wait_cycles(sysclk_cycles);

    pac::WCO.wdt_control().write(|r| {
        r.set_wdt_enable0(true);
        r.set_wdt_enable1(true);
        r.set_wdt_enable2(true);
    });
    loop {
        let r = pac::WCO.wdt_control().read();
        if r.wdt_enabled0() && r.wdt_enabled1() && r.wdt_enabled2() {
            break;
        }
    }

    interrupt::WCO_INTERRUPT.enable();
}

#[cfg(feature = "rt")]
#[interrupt]
fn WCO_INTERRUPT() {
    DRIVER.check_alarm()
}
