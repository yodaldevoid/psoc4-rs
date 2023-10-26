use crate::pac;

const WDT_DISABLE_KEY: u32 = 0xACED8865;

pub(crate) fn init() {
    let srsslt = pac::SRSSLT;

    // Disable the watchdog so we don't reset during initialization.
    srsslt.wdt_disable_key().write_value(WDT_DISABLE_KEY);
    srsslt.srss_intr().write(|r| r.set_wdt_match(true));
}
