use core::arch::asm;
use core::marker::PhantomData;

use embassy_hal_internal::{into_ref, PeripheralRef};

use crate::gpio::sealed::Pin as _;
use crate::gpio::{AnyPin, Pin};
use crate::pac::gpio::vals::Dm;
use crate::pac::hsiom::vals::IoSel;
use crate::pac::srsslt::vals::*;
use crate::{pac, peripherals, Peripheral};

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

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PllClkConfig {
    pub fb_div: u8,
    // TODO: actually 6 bits
    pub ref_clk_div: u8,
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
    const fn div(&self) -> u32 {
        match self {
            HfClkDiv::_1 => 1,
            HfClkDiv::_2 => 2,
            HfClkDiv::_4 => 4,
            HfClkDiv::_8 => 8,
        }
    }

    #[inline(always)]
    const fn to_pac(self) -> HfclkDiv {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    const fn from_pac(div: HfclkDiv) -> Self {
        unsafe { core::mem::transmute(div) }
    }
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
    const fn div(&self) -> u32 {
        match self {
            SysClkDiv::_1 => 1,
            SysClkDiv::_2 => 2,
            SysClkDiv::_4 => 4,
            SysClkDiv::_8 => 8,
        }
    }

    #[inline(always)]
    const fn to_pac(self) -> SysclkDiv {
        unsafe { core::mem::transmute(self) }
    }

    #[inline(always)]
    const fn from_pac(div: SysclkDiv) -> Self {
        unsafe { core::mem::transmute(div) }
    }
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
    const fn from_pac(div: PumpSel) -> Self {
        unsafe { core::mem::transmute(div) }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PeriClkDiv16Config {
    pub integer: u16,
}

impl Default for PeriClkDiv16Config {
    fn default() -> Self {
        Self { integer: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PeriClkDiv16_5Config {
    pub integer: u16,
    // TODO: only 5 bits
    pub frac: u8,
}

impl Default for PeriClkDiv16_5Config {
    fn default() -> Self {
        Self {
            integer: 0,
            frac: 0,
        }
    }
}

pub(crate) mod sealed {
    pub trait HfClockSource {
        fn frequency(&self) -> u32;
    }

    pub trait ExtClkPinPin {}

    pub trait PeriClkDiv {
        type Config;

        const SEL_TYPE: u8;
        const SEL_DIV: u8;
    }

    pub trait PeripheralClock {
        fn peri_clock_index() -> u8;
    }
}

pub trait HfClockSource: sealed::HfClockSource {
    fn set_hf_clock_divider(&mut self, div: HfClkDiv) {
        let sys_clk_div = SysClkDiv::from_pac(pac::SRSSLT.clk_select().read().sysclk_div());
        let new_sys_clk_freq = self.frequency() / div.div() / sys_clk_div.div();
        let current_sys_clk_freq = self.sys_clk_freq();

        if new_sys_clk_freq > current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }

        pac::SRSSLT
            .clk_select()
            .modify(|r| r.set_hfclk_div(div.to_pac()));

        if new_sys_clk_freq < current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }
    }

    fn set_sys_clock_divider(&self, div: SysClkDiv) {
        let new_sys_clk_freq = self.hf_clk_freq() / div.div();
        let current_sys_clk_freq = self.sys_clk_freq();

        if new_sys_clk_freq > current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }

        pac::SRSSLT
            .clk_select()
            .modify(|r| r.set_sysclk_div(div.to_pac()));

        if new_sys_clk_freq < current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }
    }

    fn hf_clk_freq(&self) -> u32 {
        let hf_clk_div = HfClkDiv::from_pac(pac::SRSSLT.clk_select().read().hfclk_div());
        self.frequency() / hf_clk_div.div()
    }

    fn sys_clk_freq(&self) -> u32 {
        let sys_clk_div = SysClkDiv::from_pac(pac::SRSSLT.clk_select().read().sysclk_div());
        self.hf_clk_freq() / sys_clk_div.div()
    }

    // TODO: set pump clock source
    fn pump_clk_freq(&self) -> u32 {
        let src = PumpClkSrc::from_pac(pac::SRSSLT.clk_select().read().pump_sel());
        match src {
            PumpClkSrc::Gnd => 0,
            PumpClkSrc::Imo => self.frequency(),
            PumpClkSrc::HfClk => self.hf_clk_freq(),
        }
    }

    // TODO: move to trait only implemented for the correct values of N
    fn peripheral_divider_16_bit<'a, const N: u8>(&'a self) -> PeriClkDiv16<'a, N> {
        PeriClkDiv16 {
            hf_clk_freq: self.hf_clk_freq(),
            _phantom: PhantomData,
        }
    }

    // TODO: move to trait only implemented for the correct values of N
    fn peripheral_divider_16_5_bit<'a, const N: u8>(&'a self) -> PeriClkDiv16_5<'a, N> {
        PeriClkDiv16_5 {
            hf_clk_freq: self.hf_clk_freq(),
            _phantom: PhantomData,
        }
    }
}

pub struct Imo {
    _private: (),
}

impl Imo {
    // safety: This must be called only when it is known that the IMO clock is
    // the HF clock source.
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }

    #[inline]
    pub fn set_frequency(&mut self, freq: ImoFreq) {
        let hf_clk_div = HfClkDiv::from_pac(pac::SRSSLT.clk_select().read().hfclk_div());
        let sys_clk_div = SysClkDiv::from_pac(pac::SRSSLT.clk_select().read().sysclk_div());
        let new_sys_clk_freq = freq.hz() / hf_clk_div.div() / sys_clk_div.div();
        let current_sys_clk_freq = self.sys_clk_freq();

        if new_sys_clk_freq > current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }

        configure_imo(Some(ImoConfig { freq }));

        if new_sys_clk_freq < current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }
    }

    #[inline]
    pub fn switch_to_external_clock<'p, T: Pin>(
        self,
        freq: u32,
        pin: ExtClkPin<'p, T>,
    ) -> ExtClkImo {
        let hf_clk_div = HfClkDiv::from_pac(pac::SRSSLT.clk_select().read().hfclk_div());
        let sys_clk_div = SysClkDiv::from_pac(pac::SRSSLT.clk_select().read().sysclk_div());
        let new_sys_clk_freq = freq / hf_clk_div.div() / sys_clk_div.div();
        let current_sys_clk_freq = self.sys_clk_freq();

        if new_sys_clk_freq > current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }

        pac::SRSSLT
            .clk_select()
            .modify(|r| r.set_hfclk_sel(HfclkSel::EXTCLK));

        if new_sys_clk_freq < current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }

        ExtClkImo {
            freq,
            _pin: pin.map_into(),
        }
    }

    // TODO: PLL
}

impl sealed::HfClockSource for Imo {
    #[inline]
    fn frequency(&self) -> u32 {
        // Assumes this can only be called when the IMO is enabled.
        imo_freq().unwrap().hz()
    }
}

impl HfClockSource for Imo {}

pub struct ExtClkImo<'p> {
    freq: u32,
    _pin: ExtClkPin<'p, AnyPin>,
}

impl<'p> ExtClkImo<'p> {
    #[inline]
    pub fn set_imo_frequency(&self, freq: ImoFreq) {
        configure_imo(Some(ImoConfig { freq }));
    }

    #[inline]
    pub fn disable_imo(self) -> ExtClk<'p> {
        configure_imo(None);

        ExtClk {
            freq: self.freq,
            _pin: self._pin,
        }
    }

    #[inline]
    // TODO: return pin
    pub fn switch_to_imo(self) -> Imo {
        let hf_clk_div = HfClkDiv::from_pac(pac::SRSSLT.clk_select().read().hfclk_div());
        let sys_clk_div = SysClkDiv::from_pac(pac::SRSSLT.clk_select().read().sysclk_div());
        let new_sys_clk_freq = imo_freq().unwrap().hz() / hf_clk_div.div() / sys_clk_div.div();
        let current_sys_clk_freq = self.sys_clk_freq();

        if new_sys_clk_freq > current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }

        pac::SRSSLT
            .clk_select()
            .modify(|r| r.set_hfclk_sel(HfclkSel::IMO));

        if new_sys_clk_freq < current_sys_clk_freq {
            set_wait_states(new_sys_clk_freq);
        }

        Imo::new()
    }
}

impl<'p> sealed::HfClockSource for ExtClkImo<'p> {
    #[inline]
    fn frequency(&self) -> u32 {
        self.freq
    }
}

impl<'p> HfClockSource for ExtClkImo<'p> {}

pub struct ExtClk<'p> {
    freq: u32,
    _pin: ExtClkPin<'p, AnyPin>,
}

impl<'p> ExtClk<'p> {
    #[inline]
    pub fn enable_imo(self, freq: ImoFreq) -> ExtClkImo<'p> {
        configure_imo(Some(ImoConfig { freq }));

        ExtClkImo {
            freq: self.freq,
            _pin: self._pin,
        }
    }
}

impl<'p> sealed::HfClockSource for ExtClk<'p> {
    #[inline]
    fn frequency(&self) -> u32 {
        self.freq
    }
}

impl<'p> HfClockSource for ExtClk<'p> {}

pub trait ExtClkPinPin: sealed::ExtClkPinPin + Pin {}

impl ExtClkPinPin for peripherals::PIN_0_6 {}
impl sealed::ExtClkPinPin for peripherals::PIN_0_6 {}

pub struct ExtClkPin<'d, T: Pin> {
    pin: PeripheralRef<'d, AnyPin>,
    _phantom: PhantomData<T>,
}

impl<'d, T: Pin> ExtClkPin<'d, T> {
    pub fn new<P: ExtClkPinPin>(pin: impl Peripheral<P = P> + 'd) -> ExtClkPin<'d, P> {
        into_ref!(pin);

        pin.hsiom()
            .port_sel()
            .modify(|r| r.set_io_sel(pin.pin() as usize, IoSel::ACT_0));
        pin.prt()
            .pc()
            .modify(|r| r.set_dm(pin.pin() as usize, Dm::INPUT));

        ExtClkPin {
            pin: pin.map_into(),
            _phantom: PhantomData,
        }
    }

    fn map_into(self) -> ExtClkPin<'d, AnyPin> {
        unsafe { core::mem::transmute(self) }
    }
}

impl<'d, T: Pin> Drop for ExtClkPin<'d, T> {
    fn drop(&mut self) {
        self.pin
            .prt()
            .pc()
            .modify(|r| r.set_dm(self.pin.pin() as usize, Dm::OFF));
        self.pin
            .hsiom()
            .port_sel()
            .modify(|r| r.set_io_sel(self.pin.pin() as usize, IoSel::GPIO));
    }
}

use sealed::PeriClkDiv as _;
pub trait PeriClkDiv: sealed::PeriClkDiv {
    fn frequency(&self) -> u32;
}

impl<T: sealed::PeriClkDiv> sealed::PeriClkDiv for &T {
    type Config = T::Config;

    const SEL_TYPE: u8 = T::SEL_TYPE;
    const SEL_DIV: u8 = T::SEL_DIV;
}

impl<T: sealed::PeriClkDiv> sealed::PeriClkDiv for &mut T {
    type Config = T::Config;

    const SEL_TYPE: u8 = T::SEL_TYPE;
    const SEL_DIV: u8 = T::SEL_DIV;
}

impl<T: PeriClkDiv + sealed::PeriClkDiv> PeriClkDiv for &T {
    fn frequency(&self) -> u32 {
        T::frequency(self)
    }
}

impl<T: PeriClkDiv + sealed::PeriClkDiv> PeriClkDiv for &mut T {
    fn frequency(&self) -> u32 {
        T::frequency(self)
    }
}

pub trait PeriClkDivExt: PeriClkDiv {
    fn configure(&mut self, config: Option<Self::Config>);
}

impl<T: PeriClkDivExt + PeriClkDiv + sealed::PeriClkDiv> PeriClkDivExt for &mut T {
    fn configure(&mut self, config: Option<Self::Config>) {
        T::configure(self, config)
    }
}

pub struct PeriClkDiv16<'a, const N: u8> {
    hf_clk_freq: u32,
    _phantom: PhantomData<&'a mut ()>,
}

impl<'a, const N: u8> sealed::PeriClkDiv for PeriClkDiv16<'a, N> {
    type Config = PeriClkDiv16Config;

    const SEL_TYPE: u8 = 1;
    const SEL_DIV: u8 = N;
}

impl<'a, const N: u8> PeriClkDiv for PeriClkDiv16<'a, N> {
    #[inline]
    fn frequency(&self) -> u32 {
        let r = pac::PERI.div_16_ctl(Self::SEL_DIV as usize).read();
        if r.en() {
            self.hf_clk_freq / (r.int16_div() as u32 + 1)
        } else {
            0
        }
    }
}

impl<'a, const N: u8> PeriClkDivExt for PeriClkDiv16<'a, N> {
    #[inline]
    fn configure(&mut self, config: Option<Self::Config>) {
        if let Some(config) = config {
            pac::PERI
                .div_16_ctl(Self::SEL_DIV as usize)
                .write(|r| r.set_int16_div(config.integer));
            pac::PERI.div_cmd().write(|r| {
                r.set_sel_type(Self::SEL_TYPE);
                r.set_sel_div(Self::SEL_DIV);
                r.set_enable(true);
                // Defaults
                r.set_pa_sel_type(3);
                r.set_pa_sel_div(63);
            });
            while pac::PERI.div_cmd().read().enable() {}
        } else {
            pac::PERI.div_cmd().write(|r| {
                r.set_sel_type(Self::SEL_TYPE);
                r.set_sel_div(Self::SEL_DIV);
                r.set_disable(true);
                // Defaults
                r.set_pa_sel_type(3);
                r.set_pa_sel_div(63);
            });
            while pac::PERI.div_cmd().read().disable() {}
        }
    }
}

pub struct PeriClkDiv16_5<'a, const N: u8> {
    hf_clk_freq: u32,
    _phantom: PhantomData<&'a mut ()>,
}

impl<'a, const N: u8> sealed::PeriClkDiv for PeriClkDiv16_5<'a, N> {
    type Config = PeriClkDiv16_5Config;

    const SEL_TYPE: u8 = 2;
    const SEL_DIV: u8 = N;
}

impl<'a, const N: u8> PeriClkDiv for PeriClkDiv16_5<'a, N> {
    #[inline]
    fn frequency(&self) -> u32 {
        let r = pac::PERI.div_16_5_ctl(Self::SEL_DIV as usize).read();
        if r.en() {
            (self.hf_clk_freq << 5) / (((r.int16_div() as u32 + 1) << 5) + r.frac5_div() as u32)
        } else {
            0
        }
    }
}

impl<'a, const N: u8> PeriClkDivExt for PeriClkDiv16_5<'a, N> {
    #[inline]
    fn configure(&mut self, config: Option<Self::Config>) {
        if let Some(config) = config {
            pac::PERI.div_16_5_ctl(Self::SEL_DIV as usize).write(|r| {
                r.set_int16_div(config.integer);
                r.set_frac5_div(config.frac);
            });
            pac::PERI.div_cmd().write(|r| {
                r.set_sel_type(Self::SEL_TYPE);
                r.set_sel_div(Self::SEL_DIV);
                r.set_enable(true);
                // Defaults
                r.set_pa_sel_type(3);
                r.set_pa_sel_div(63);
            });
            while pac::PERI.div_cmd().read().enable() {}
        } else {
            pac::PERI.div_cmd().write(|r| {
                r.set_sel_type(Self::SEL_TYPE);
                r.set_sel_div(Self::SEL_DIV);
                r.set_disable(true);
                // Defaults
                r.set_pa_sel_type(3);
                r.set_pa_sel_div(63);
            });
            while pac::PERI.div_cmd().read().enable() {}
        }
    }
}

pub trait PeripheralClock: sealed::PeripheralClock {
    #[inline]
    fn set_clock_divider<T: PeriClkDiv>(clk_div: Option<&T>) {
        let (sel_type, sel_div) = clk_div
            .map(|_| (T::SEL_TYPE, T::SEL_DIV))
            .unwrap_or((3, 63));
        crate::pac::PERI
            .pclk_ctl(Self::peri_clock_index() as usize)
            .write(|r| {
                r.set_sel_type(sel_type);
                r.set_sel_div(sel_div);
            });
    }
}

macro_rules! impl_peri_clk {
    ($name:ident, $index:expr) => {
        impl PeripheralClock for peripherals::$name {}
        impl sealed::PeripheralClock for peripherals::$name {
            #[inline]
            fn peri_clock_index() -> u8 {
                $index as u8
            }
        }
    };
}

impl_peri_clk!(SCB0, 0);
impl_peri_clk!(SCB1, 1);
impl_peri_clk!(SCB2, 2);
// impl_peri_clk!(CSD, 3);
impl_peri_clk!(TCPWM0, 4);
impl_peri_clk!(TCPWM1, 5);
impl_peri_clk!(TCPWM2, 6);
impl_peri_clk!(TCPWM3, 7);
impl_peri_clk!(TCPWM4, 8);
impl_peri_clk!(PRGIO2, 9);
impl_peri_clk!(PRGIO3, 10);
impl_peri_clk!(LCD, 11);
// impl_peri_clk!(PASS0SAR, 12);

fn configure_imo(config: Option<ImoConfig>) {
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
            srsslt.clk_imo_trim2().write_value(Default::default());
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

fn set_wait_states(sys_clk_freq: u32) {
    let wait_states = if sys_clk_freq <= 16_000_000 {
        0
    } else if sys_clk_freq <= 32_000_000 {
        1
    } else {
        3
    };
    pac::CPUSS.flash_ctl().modify(|r| {
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

fn imo_freq() -> Option<ImoFreq> {
    if pac::SRSSLT.clk_imo_config().read().enable() {
        Some(ImoFreq::from_pac(
            pac::SRSSLT.clk_imo_select().read().freq(),
        ))
    } else {
        None
    }
}

// If the WDC clocked by the ILO has been selected for the embassy-time driver,
// don't allow the user to disable the ILO.
#[cfg(not(feature = "time-driver-wdc-ilo"))]
/// Requires WDT to be disabled in order to disable.
pub fn ilo_enable(enable: bool) {
    pac::SRSSLT.clk_ilo_config().write(|r| r.set_enable(enable));
}

// TODO: WCO enable and trimming
// TODO: select WDC source between ILO and WCO

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
