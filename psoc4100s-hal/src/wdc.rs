use core::cell::Cell;
use portable_atomic::{AtomicU32, AtomicU8, Ordering};

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

const ALARM_COUNT: usize = 2;
const DUMMY_ALARM: AlarmState = AlarmState {
    timestamp: Cell::new(0),
    callback: Cell::new(None),
};

struct WdcTimerDriver {
    count_high: AtomicU32,
    alarms: Mutex<CriticalSectionRawMutex, [AlarmState; ALARM_COUNT]>,
    next_alarm: AtomicU8,
}

embassy_time::time_driver_impl!(static DRIVER: WdcTimerDriver = WdcTimerDriver {
    count_high: AtomicU32::new(0),
    alarms: Mutex::const_new(CriticalSectionRawMutex::new(), [DUMMY_ALARM; ALARM_COUNT]),
    next_alarm: AtomicU8::new(0),
});

impl Driver for WdcTimerDriver {
    fn now(&self) -> u64 {
        loop {
            let hi = self.count_high.load(Ordering::Relaxed);
            let lo = pac::WCO.wdt_ctrhigh().read();
            let hi2 = self.count_high.load(Ordering::Relaxed);
            if hi == hi2 {
                return (hi as u64) << 32 | (lo as u64);
            }
        }
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        let id = self
            .next_alarm
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |x| {
                if x < ALARM_COUNT as u8 {
                    Some(x + 1)
                } else {
                    None
                }
            });

        match id {
            Ok(id) => Some(AlarmHandle::new(id)),
            Err(_) => None,
        }
    }

    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        let n = alarm.id() as usize;
        critical_section::with(|cs| {
            let alarm = &self.alarms.borrow(cs)[n];
            alarm.callback.set(Some((callback, ctx)));
        })
    }

    fn set_alarm(&self, alarm: AlarmHandle, timestamp: u64) -> bool {
        let n = alarm.id() as usize;
        critical_section::with(|cs| {
            let alarm = &self.alarms.borrow(cs)[n];

            // Disable the counter in case the previous timestamp wasn't reached before the
            // alarm was changed or if the new timestamp is in the past.
            self.wait_counter_enable_acknowledged(n);
            pac::WCO.wdt_control().modify(|r| {
                r.set_wdt_enable(n, false);
                r.set_wdt_reset(n, true);
            });

            let now = self.now();
            if timestamp <= now {
                // If the alarm timestamp has passed, the alarm will not fire.
                // Return `false` to indicate that.
                alarm.timestamp.set(u64::MAX);

                false
            } else {
                alarm.timestamp.set(timestamp);

                while pac::WCO.wdt_control().read().wdt_enabled(n) {}

                // Note that we're not checking the high bits at all. This means the IRQ may
                // fire early if the alarm is more than 1.6 seconds (2^16 / 4e5 Hz) in the
                // future. This is OK, since on IRQ fire it is checked if the alarm time has
                // passed.
                let diff = timestamp - now;
                pac::WCO
                    .wdt_match()
                    .modify(|r| r.set_wdt_match(n, diff as u16));
                pac::WCO
                    .wdt_config()
                    .modify(|r| r.set_wdt_mode(n, WdtMode::INT));

                // We don't want to enable the timer before the previous reset completes.
                while pac::WCO.wdt_control().read().wdt_reset(n) {}

                self.wait_counter_enable_acknowledged(n);
                pac::WCO.wdt_control().modify(|r| r.set_wdt_enable(n, true));

                true
            }
        })
    }
}

impl WdcTimerDriver {
    fn check_alarm(&self, n: usize) {
        if pac::WCO.wdt_control().read().wdt_int(n) {
            critical_section::with(|cs| {
                let alarm = &self.alarms.borrow(cs)[n];
                let timestamp = alarm.timestamp.get();
                if timestamp <= self.now() {
                    // Disarm the alarm.
                    pac::WCO.wdt_match().modify(|r| r.set_wdt_match(n, 0xFFFF));
                    pac::WCO.wdt_control().modify(|r| {
                        r.set_wdt_enable(n, false);
                        // Reset the timer for the next alarm.
                        r.set_wdt_reset(n, true);
                    });

                    alarm.timestamp.set(0);

                    // Call after clearing alarm so the callback can set another alarm.
                    if let Some((f, ctx)) = alarm.callback.get() {
                        f(ctx);
                    }
                }
            });

            // Clear the IRQ.
            pac::WCO.wdt_control().modify(|r| r.set_wdt_int(n, true));
            pac::WCO.wdt_control().read();
        }
    }

    fn check_timer(&self) {
        if pac::WCO.wdt_control().read().wdt_int(2) {
            self.count_high.fetch_add(1, Ordering::AcqRel);

            // Clear the IRQ.
            pac::WCO.wdt_control().modify(|r| r.set_wdt_int(2, true));
            pac::WCO.wdt_control().read();
        }
    }

    fn wait_counter_enable_acknowledged(&self, n: usize) {
        loop {
            let r = pac::WCO.wdt_control().read();
            if r.wdt_enable(n) == r.wdt_enabled(n) {
                break;
            }
        }
    }
}

pub(crate) unsafe fn init() {
    critical_section::with(|cs| {
        for n in 0..ALARM_COUNT {
            DRIVER.alarms.borrow(cs)[n].timestamp.set(u64::MAX);
        }
    });

    pac::WCO.wdt_clken().write(|r| {
        r.set_clk_wco_en_for_wdt(false);
        r.set_clk_ilo_en_for_wdt(false);
    });

    pac::WCO.wdt_match().write(|r| {
        r.set_wdt_match(0, 0xFFFF);
        r.set_wdt_match(1, 0xFFFF);
    });

    let use_wco = cfg!(feature = "time-driver-wdc-wco");
    pac::WCO.wdt_config().write(|r| {
        r.set_wdt_mode(0, WdtMode::INT);
        r.set_wdt_mode(1, WdtMode::INT);
        r.set_wdt_mode2(WdtMode2::INT);
        r.set_wdt_bits2(31);
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

    pac::WCO.wdt_control().write(|r| r.set_wdt_enable(2, true));
    while !pac::WCO.wdt_control().read().wdt_enabled(2) {}

    interrupt::WCO_INTERRUPT.enable();
}

#[cfg(feature = "rt")]
#[interrupt]
fn WCO_INTERRUPT() {
    for n in 0..ALARM_COUNT {
        DRIVER.check_alarm(n);
    }
    DRIVER.check_timer();
}
