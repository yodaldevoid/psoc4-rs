use core::arch::asm;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::pac;
use crate::pac::srsslt::vals::*;

struct Clocks {
    hf: AtomicU32,
    sys: AtomicU32,
    pump: AtomicU32,
}

static CLOCKS: Clocks = Clocks {
    hf: AtomicU32::new(0),
    sys: AtomicU32::new(0),
    pump: AtomicU32::new(0),
};

#[non_exhaustive]
pub struct ClockConfig {
    // Clock sources
    pub imo: Option<ImoConfig>,
    // TODO: ext_clk: Option<ExtClkConfig>,
    pub ilo: Option<IloConfig>,
    // TODO: wco: Option<WcoConfig>,

    // Derived clocks
    pub hf_clk: HfClkConfig,
    pub sys_clk: SysClkConfig,
    pub pump_clk: PumpClkConfig,
    // TODO: WDC source

    // Peripheral clock dividers
    // TODO: phase alignment for clock dividers
    pub peri_clk_div_16: [Option<PeriClkDiv16Config>; 6],
    pub peri_clk_div_16_5: [Option<PeriClkDiv16_5Config>; 3],

    // Peripheral clocks
    pub scb0_clk: Option<(PeriClkDivType, u8)>,
    pub scb1_clk: Option<(PeriClkDivType, u8)>,
    pub scb2_clk: Option<(PeriClkDivType, u8)>,
    pub csd_clk: Option<(PeriClkDivType, u8)>,
    pub tcpwm0_clk: Option<(PeriClkDivType, u8)>,
    pub tcpwm1_clk: Option<(PeriClkDivType, u8)>,
    pub tcpwm2_clk: Option<(PeriClkDivType, u8)>,
    pub tcpwm3_clk: Option<(PeriClkDivType, u8)>,
    pub tcpwm4_clk: Option<(PeriClkDivType, u8)>,
    pub prgio2_clk: Option<(PeriClkDivType, u8)>,
    pub prgio3_clk: Option<(PeriClkDivType, u8)>,
    pub lcd_clk: Option<(PeriClkDivType, u8)>,
    pub pass0_sar_clk: Option<(PeriClkDivType, u8)>,
}

impl ClockConfig {
    pub fn imo(freq: ImoFreq) -> Self {
        Self {
            imo: Some(ImoConfig { freq }),
            // ext_clk: None,
            ilo: Some(IloConfig),
            // wco: None,
            hf_clk: HfClkConfig {
                src: HfClkSrc::Imo,
                div: HfClkDiv::_4,
            },
            sys_clk: SysClkConfig { div: SysClkDiv::_1 },
            pump_clk: PumpClkConfig {
                src: PumpClkSrc::Gnd,
            },
            peri_clk_div_16: [None; 6],
            peri_clk_div_16_5: [None; 3],
            scb0_clk: None,
            scb1_clk: None,
            scb2_clk: None,
            csd_clk: None,
            tcpwm0_clk: None,
            tcpwm1_clk: None,
            tcpwm2_clk: None,
            tcpwm3_clk: None,
            tcpwm4_clk: None,
            prgio2_clk: None,
            prgio3_clk: None,
            lcd_clk: None,
            pass0_sar_clk: None,
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImoFreq {
    _24_MHz = Freq::_24_MHZ as _,
    _28_MHz = Freq::_28_MHZ as _,
    _32_MHz = Freq::_32_MHZ as _,
    _36_MHz = Freq::_36_MHZ as _,
    _40_MHz = Freq::_40_MHZ as _,
    _44_MHz = Freq::_44_MHZ as _,
    _48_MHz = Freq::_48_MHZ as _,
}

impl ImoFreq {
    pub const fn hz(&self) -> u32 {
        match self {
            Self::_24_MHz => 24_000_000,
            Self::_28_MHz => 28_000_000,
            Self::_32_MHz => 32_000_000,
            Self::_36_MHz => 36_000_000,
            Self::_40_MHz => 40_000_000,
            Self::_44_MHz => 44_000_000,
            Self::_48_MHz => 48_000_000,
        }
    }

    #[inline(always)]
    const fn from_pac(val: Freq) -> ImoFreq {
        unsafe { core::mem::transmute(val) }
    }

    #[inline(always)]
    const fn to_pac(self) -> Freq {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    const fn index(&self) -> usize {
        match self {
            Self::_24_MHz => 0,
            Self::_28_MHz => 4,
            Self::_32_MHz => 8,
            Self::_36_MHz => 12,
            Self::_40_MHz => 16,
            Self::_44_MHz => 20,
            Self::_48_MHz => 24,
        }
    }

    #[inline(always)]
    const fn intermediate(&self) -> Self {
        match self {
            Self::_32_MHz => Self::_28_MHz,
            Self::_36_MHz => Self::_32_MHz,
            Self::_40_MHz => Self::_36_MHz,
            Self::_44_MHz => Self::_40_MHz,
            Self::_48_MHz => Self::_44_MHz,
            _ => Self::_24_MHz,
        }
    }
}

pub struct ImoConfig {
    pub freq: ImoFreq,
    // TODO: lock IMO to WCO
}

// TODO: pin
// TODO: frequency
// struct ExtClkConfig;

pub struct IloConfig;

// TODO: trimming
// struct WcoConfig;

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PllClkConfig {
    pub fb_div: u8,
    // TODO: actually 6 bits
    pub ref_clk_div: u8,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfClkSrc {
    Imo,
    ImoPll(PllClkConfig),
    ExtClk,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfClkDiv {
    _1 = HfclkDiv::NO_DIV as _,
    _2 = HfclkDiv::DIV_BY_2 as _,
    _4 = HfclkDiv::DIV_BY_4 as _,
    _8 = HfclkDiv::DIV_BY_8 as _,
}

impl HfClkDiv {
    #[inline(always)]
    const fn to_pac(self) -> HfclkDiv {
        unsafe { core::mem::transmute(self) }
    }
}

pub struct HfClkConfig {
    pub src: HfClkSrc,
    pub div: HfClkDiv,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysClkDiv {
    _1 = SysclkDiv::NO_DIV as _,
    _2 = SysclkDiv::DIV_BY_2 as _,
    _4 = SysclkDiv::DIV_BY_4 as _,
    _8 = SysclkDiv::DIV_BY_8 as _,
}

impl SysClkDiv {
    #[inline(always)]
    const fn to_pac(self) -> SysclkDiv {
        unsafe { core::mem::transmute(self) }
    }
}

pub struct SysClkConfig {
    pub div: SysClkDiv,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PumpClkSrc {
    Gnd = PumpSel::GND as _,
    Imo = PumpSel::IMO as _,
    HfClk = PumpSel::HFCLK as _,
}

impl PumpClkSrc {
    #[inline(always)]
    const fn to_pac(self) -> PumpSel {
        unsafe { core::mem::transmute(self) }
    }
}

pub struct PumpClkConfig {
    pub src: PumpClkSrc,
}

#[derive(Clone, Copy, Debug)]
pub struct PeriClkDiv16Config {
    integer: u16,
}

impl Default for PeriClkDiv16Config {
    fn default() -> Self {
        Self { integer: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PeriClkDiv16_5Config {
    integer: u16,
    // TODO: only 5 bits
    frac: u8,
}

impl Default for PeriClkDiv16_5Config {
    fn default() -> Self {
        Self {
            integer: 0,
            frac: 0,
        }
    }
}

#[allow(non_camel_case_types)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeriClkDivType {
    _16 = 1,
    _16_5 = 2,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeriClk {
    Scb0 = 0,
    Scb1 = 1,
    Scb2 = 2,
    Csd = 3,
    Tcpwm0 = 4,
    Tcpwm1 = 5,
    Tcpwm2 = 6,
    Tcpwm3 = 7,
    Tcpwm4 = 8,
    Prgio2 = 9,
    Prgio3 = 10,
    Lcd = 11,
    Pass0Sar = 12,
}

// TODO: record clock frequencies for peripheral calculations
/// safety: must be called exactly once at bootup
pub(crate) unsafe fn init(config: ClockConfig) {
    // Clock configuration order selected to prevent turning off our current
    // system clock during initial configuration.

    let srsslt = pac::SRSSLT;
    let peri = pac::PERI;

    // Requires WDT to be disabled in order to disable.
    #[cfg(not(feature = "time-driver-wdc-ilo"))]
    srsslt
        .clk_ilo_config()
        .write(|r| r.set_enable(config.ilo.is_some()));
    // Ignore the user configuration if the WDC clocked by the ILO has been
    // selected for the embassy-time driver.
    #[cfg(feature = "time-driver-wdc-ilo")]
    srsslt.clk_ilo_config().write(|r| r.set_enable(true));

    // TODO: configure WCO

    // TODO: configure WDC source

    // TODO: configure external clock

    // TODO: configure PLL

    // Pre-calculate the IMO frequency after applying settings to calculate the
    // eventual system clock frequency.
    let imo_freq = config.imo.as_ref().map(|c| c.freq.hz()).unwrap_or(0);

    let hf_clk_div = 1 << config.hf_clk.div as u8;
    let (hf_clk_sel, hf_clk_freq) = match config.hf_clk.src {
        HfClkSrc::Imo => (HfclkSel::IMO, imo_freq / hf_clk_div),
        _ => todo!(),
    };
    CLOCKS.hf.store(hf_clk_freq, Ordering::Relaxed);

    let sys_clk_freq = hf_clk_freq / (1 << config.sys_clk.div as u8);

    // TODO: read/calculate the current frequency. This calculation assumes we
    // are performing startup initialization.
    let current_sys_clk_freq = 24_000_000 / 4;
    if sys_clk_freq > current_sys_clk_freq {
        set_wait_states(sys_clk_freq);
    }

    srsslt.clk_select().write(|r| {
        r.set_hfclk_sel(hf_clk_sel);
        r.set_hfclk_div(config.hf_clk.div.to_pac());
        r.set_pump_sel(config.pump_clk.src.to_pac());
        r.set_sysclk_div(config.sys_clk.div.to_pac());
    });

    configure_imo(&config.imo);

    if sys_clk_freq < current_sys_clk_freq {
        set_wait_states(sys_clk_freq);
    }
    CLOCKS.sys.store(sys_clk_freq, Ordering::Relaxed);

    let pump_freq = match config.pump_clk.src {
        PumpClkSrc::Gnd => 0,
        PumpClkSrc::Imo => imo_freq,
        PumpClkSrc::HfClk => hf_clk_freq,
    };
    CLOCKS.pump.store(pump_freq, Ordering::Relaxed);

    // TODO: Calculate frequencies
    for (n, clk_div) in config.peri_clk_div_16.iter().enumerate() {
        if let Some(config) = clk_div {
            peri.div_16_ctl(n as usize)
                .write(|r| r.set_int16_div(config.integer));
            peri.div_cmd().write(|r| {
                r.set_sel_type(PeriClkDivType::_16 as u8);
                r.set_sel_div(n as u8);
                r.set_enable(true);
            });
        } else {
            peri.div_cmd().write(|r| {
                r.set_sel_type(PeriClkDivType::_16 as u8);
                r.set_sel_div(n as u8);
                r.set_disable(true);
            });
        }
    }
    for (n, clk_div) in config.peri_clk_div_16_5.iter().enumerate() {
        if let Some(config) = clk_div {
            peri.div_16_5_ctl(n as usize).write(|r| {
                r.set_int16_div(config.integer);
                r.set_frac5_div(config.frac);
            });
            peri.div_cmd().write(|r| {
                r.set_sel_type(PeriClkDivType::_16_5 as u8);
                r.set_sel_div(n as u8);
                r.set_enable(true);
            });
        } else {
            peri.div_cmd().write(|r| {
                r.set_sel_type(PeriClkDivType::_16_5 as u8);
                r.set_sel_div(n as u8);
                r.set_disable(true);
            });
        }
    }

    // TODO: Calculate frequencies
    configure_peripheral_clock(PeriClk::Scb0, config.scb0_clk);
    configure_peripheral_clock(PeriClk::Scb1, config.scb1_clk);
    configure_peripheral_clock(PeriClk::Scb2, config.scb2_clk);
    configure_peripheral_clock(PeriClk::Csd, config.csd_clk);
    configure_peripheral_clock(PeriClk::Tcpwm0, config.tcpwm0_clk);
    configure_peripheral_clock(PeriClk::Tcpwm1, config.tcpwm1_clk);
    configure_peripheral_clock(PeriClk::Tcpwm2, config.tcpwm2_clk);
    configure_peripheral_clock(PeriClk::Tcpwm3, config.tcpwm3_clk);
    configure_peripheral_clock(PeriClk::Tcpwm4, config.tcpwm4_clk);
    configure_peripheral_clock(PeriClk::Prgio2, config.prgio2_clk);
    configure_peripheral_clock(PeriClk::Prgio3, config.prgio3_clk);
    configure_peripheral_clock(PeriClk::Lcd, config.lcd_clk);
    // TODO: fractional divider not supported for SAR
    configure_peripheral_clock(PeriClk::Pass0Sar, config.pass0_sar_clk);
}

#[inline(always)]
fn configure_imo(config: &Option<ImoConfig>) {
    // TODO: if current frequency is desired frequency, return early
    let desired_freq = config.as_ref().map(|c| c.freq.hz()).unwrap_or(0);
    let current_freq = imo_freq().map(|c| c.hz()).unwrap_or(0);
    if desired_freq == current_freq {
        return;
    }

    critical_section::with(|_| {
        // TODO: if IMO is locked to WCO, unlock

        let srsslt = pac::SRSSLT;

        if let Some(config) = config {
            srsslt.clk_imo_config().write(|r| r.set_enable(true));

            // Change the IMO frequency to 24 MHz.
            srsslt.clk_imo_select().write(|r| r.set_freq(Freq::_24_MHZ));

            let sflash = pac::SFLASH;
            let freq_index = config.freq.index();
            let course_trim = sflash.imo_trim_lt(freq_index).read().0;
            let temp_comp = sflash.imo_tctrim_lt(freq_index).read().0;

            // Read the course trim value for a desired frequency from SFLASH and
            // load it into CLK_IMO_TRIM1.
            srsslt.clk_imo_trim1().write(|r| r.0 = course_trim as u32);
            // Clear CLK_IMO_TRIM2.
            srsslt.clk_imo_trim2().write(|_| {});
            // Read the temperature compensation value for a desired frequency from
            // SFLASH and load it into CLK_IMO_TRIM3.
            srsslt.clk_imo_trim3().write(|r| r.0 = temp_comp as u32);

            // Wait (a minimum of) 50 IMO cycles. The SYSCLK frequency can only be
            // at max 2x the IMO frequency, so waiting double the required number
            // of cycles _should_ be safe.
            wait_cycles(100);

            if config.freq != ImoFreq::_24_MHz {
                // TODO: skip the intermediate frequency if switching to 28 MHz
                // Switch to an intermediate frequency.
                srsslt
                    .clk_imo_select()
                    .write(|r| r.set_freq(config.freq.intermediate().to_pac()));
                // Wait (a minimum of) 50 IMO cycles. The SYSCLK frequency can only
                // be at max 2x the IMO frequency, so waiting double the required
                // number of cycles _should_ be safe.
                wait_cycles(100);
                // Switch to the desired frequency.
                srsslt
                    .clk_imo_select()
                    .write(|r| r.set_freq(config.freq.to_pac()));
            }

            // TODO: restore WCO lock
        } else {
            srsslt.clk_imo_config().write(|r| r.set_enable(false));
        }
    })
}

#[inline(always)]
fn configure_peripheral_clock(clk: PeriClk, clk_src: Option<(PeriClkDivType, u8)>) {
    let peri = pac::PERI;
    let (sel_type, sel_div) = clk_src.map(|(t, n)| (t as u8, n)).unwrap_or((3, 63));
    peri.pclk_ctl(clk as usize).write(|r| {
        r.set_sel_type(sel_type);
        r.set_sel_div(sel_div);
    })
}

fn set_wait_states(sys_clk_freq: u32) {
    let cpuss = pac::CPUSS;
    let wait_states = if sys_clk_freq <= 16_000_000 {
        0
    } else if sys_clk_freq <= 32_000_000 {
        1
    } else {
        3
    };
    cpuss.flash_ctl().modify(|r| {
        r.set_flash_ws(wait_states);
        r.set_pref_en(wait_states != 0);
    })
}

pub(crate) fn wait_cycles(cycles: u32) {
    // We are okay using any 32 bit register
    unsafe {
        asm!(
            "ADDS {0}, {0}, #2",
            "LSRS {0}, {0}, #2",
            "BEQ 1f",
            "0:",
            "SUBS {0}, {0}, #1",
            "BNE 0b",
            "NOP",
            "NOP",
            "1:",
            "NOP",
            "NOP",
            "NOP",
            "NOP",
            inout(reg) cycles => _,
            options(nomem),
        );
    }
}

pub fn imo_freq() -> Option<ImoFreq> {
    if pac::SRSSLT.clk_imo_config().read().enable() {
        Some(ImoFreq::from_pac(
            pac::SRSSLT.clk_imo_select().read().freq(),
        ))
    } else {
        None
    }
}

pub fn hf_clk_freq() -> u32 {
    CLOCKS.hf.load(Ordering::Relaxed)
}

pub fn sys_clk_freq() -> u32 {
    CLOCKS.sys.load(Ordering::Relaxed)
}

pub fn pump_clk_freq() -> u32 {
    CLOCKS.pump.load(Ordering::Relaxed)
}

pub const ILO_AVG_FREQ_HZ: u32 = 40_000;
pub const ILO_FAST_FREQ_HZ: u32 = 64_000;
pub const ILO_SLOW_FREQ_HZ: u32 = 24_000;
pub const WCO_FREQ_HZ: u32 = 32_768;

pub fn ilo_avg_freq() -> u32 {
    if pac::SRSSLT.clk_ilo_config().read().enable() {
        ILO_AVG_FREQ_HZ
    } else {
        0
    }
}

pub fn ilo_fast_freq() -> u32 {
    if pac::SRSSLT.clk_ilo_config().read().enable() {
        ILO_FAST_FREQ_HZ
    } else {
        0
    }
}

pub fn ilo_slow_freq() -> u32 {
    if pac::SRSSLT.clk_ilo_config().read().enable() {
        ILO_SLOW_FREQ_HZ
    } else {
        0
    }
}

pub fn wco_freq() -> u32 {
    if pac::WCO.config().read().ip_enable() {
        WCO_FREQ_HZ
    } else {
        0
    }
}
