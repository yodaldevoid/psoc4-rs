#[doc = "SAR ADC with Sequencer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sar0 {
    ptr: *mut u8,
}
unsafe impl Send for Sar0 {}
unsafe impl Sync for Sar0 {}
impl Sar0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Analog control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Sample control register."]
    #[inline(always)]
    pub const fn sample_ctrl(self) -> crate::common::Reg<regs::SampleCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Sample time specification ST0 and ST1"]
    #[inline(always)]
    pub const fn sample_time01(self) -> crate::common::Reg<regs::SampleTime01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Sample time specification ST2 and ST3"]
    #[inline(always)]
    pub const fn sample_time23(self) -> crate::common::Reg<regs::SampleTime23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Global range detect threshold register."]
    #[inline(always)]
    pub const fn range_thres(self) -> crate::common::Reg<regs::RangeThres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Global range detect mode register."]
    #[inline(always)]
    pub const fn range_cond(self) -> crate::common::Reg<regs::RangeCond, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Enable bits for the channels"]
    #[inline(always)]
    pub const fn chan_en(self) -> crate::common::Reg<regs::ChanEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Start control register (firmware trigger)."]
    #[inline(always)]
    pub const fn start_ctrl(self) -> crate::common::Reg<regs::StartCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "DFT control register."]
    #[inline(always)]
    pub const fn dft_ctrl(self) -> crate::common::Reg<regs::DftCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Channel configuration register."]
    #[inline(always)]
    pub const fn chan_config(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ChanConfig, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize + n * 4usize) as _) }
    }
    #[doc = "Channel working data register"]
    #[inline(always)]
    pub const fn chan_work(self, n: usize) -> crate::common::Reg<regs::ChanWork, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
    #[doc = "Channel result data register"]
    #[inline(always)]
    pub const fn chan_result(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ChanResult, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize + n * 4usize) as _) }
    }
    #[doc = "Channel working data register valid bits"]
    #[inline(always)]
    pub const fn chan_work_valid(
        self,
    ) -> crate::common::Reg<regs::ChanWorkValid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Channel result data register valid bits"]
    #[inline(always)]
    pub const fn chan_result_valid(
        self,
    ) -> crate::common::Reg<regs::ChanResultValid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "Current status of internal SAR registers (mostly for debug)"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[doc = "Current averaging status (for debug)"]
    #[inline(always)]
    pub const fn avg_stat(self) -> crate::common::Reg<regs::AvgStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize) as _) }
    }
    #[doc = "Interrupt request register."]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(528usize) as _) }
    }
    #[doc = "Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<regs::IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(532usize) as _) }
    }
    #[doc = "Interrupt mask register."]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<regs::IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(536usize) as _) }
    }
    #[doc = "Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<regs::IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(540usize) as _) }
    }
    #[doc = "Saturate interrupt request register."]
    #[inline(always)]
    pub const fn saturate_intr(self) -> crate::common::Reg<regs::SaturateIntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize) as _) }
    }
    #[doc = "Saturate interrupt set request register"]
    #[inline(always)]
    pub const fn saturate_intr_set(
        self,
    ) -> crate::common::Reg<regs::SaturateIntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(548usize) as _) }
    }
    #[doc = "Saturate interrupt mask register."]
    #[inline(always)]
    pub const fn saturate_intr_mask(
        self,
    ) -> crate::common::Reg<regs::SaturateIntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(552usize) as _) }
    }
    #[doc = "Saturate interrupt masked request register"]
    #[inline(always)]
    pub const fn saturate_intr_masked(
        self,
    ) -> crate::common::Reg<regs::SaturateIntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(556usize) as _) }
    }
    #[doc = "Range detect interrupt request register."]
    #[inline(always)]
    pub const fn range_intr(self) -> crate::common::Reg<regs::RangeIntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(560usize) as _) }
    }
    #[doc = "Range detect interrupt set request register"]
    #[inline(always)]
    pub const fn range_intr_set(self) -> crate::common::Reg<regs::RangeIntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(564usize) as _) }
    }
    #[doc = "Range detect interrupt mask register."]
    #[inline(always)]
    pub const fn range_intr_mask(
        self,
    ) -> crate::common::Reg<regs::RangeIntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(568usize) as _) }
    }
    #[doc = "Range interrupt masked request register"]
    #[inline(always)]
    pub const fn range_intr_masked(
        self,
    ) -> crate::common::Reg<regs::RangeIntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(572usize) as _) }
    }
    #[doc = "Interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(self) -> crate::common::Reg<regs::IntrCause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(576usize) as _) }
    }
    #[doc = "Injection channel configuration register."]
    #[inline(always)]
    pub const fn inj_chan_config(
        self,
    ) -> crate::common::Reg<regs::InjChanConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(640usize) as _) }
    }
    #[doc = "Injection channel result register"]
    #[inline(always)]
    pub const fn inj_result(self) -> crate::common::Reg<regs::InjResult, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(656usize) as _) }
    }
    #[doc = "SARMUX Firmware switch controls"]
    #[inline(always)]
    pub const fn mux_switch0(self) -> crate::common::Reg<regs::MuxSwitch0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize) as _) }
    }
    #[doc = "SARMUX Firmware switch control clear"]
    #[inline(always)]
    pub const fn mux_switch_clear0(
        self,
    ) -> crate::common::Reg<regs::MuxSwitchClear0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(772usize) as _) }
    }
    #[doc = "SARMUX Firmware switch controls"]
    #[inline(always)]
    pub const fn mux_switch1(self) -> crate::common::Reg<regs::MuxSwitch1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(776usize) as _) }
    }
    #[doc = "SARMUX Firmware switch control clear"]
    #[inline(always)]
    pub const fn mux_switch_clear1(
        self,
    ) -> crate::common::Reg<regs::MuxSwitchClear1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(780usize) as _) }
    }
    #[doc = "SARMUX switch hardware control"]
    #[inline(always)]
    pub const fn mux_switch_hw_ctrl(
        self,
    ) -> crate::common::Reg<regs::MuxSwitchHwCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(832usize) as _) }
    }
    #[doc = "SARMUX switch status"]
    #[inline(always)]
    pub const fn mux_switch_status(
        self,
    ) -> crate::common::Reg<regs::MuxSwitchStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(840usize) as _) }
    }
    #[doc = "Switch pump control"]
    #[inline(always)]
    pub const fn pump_ctrl(self) -> crate::common::Reg<regs::PumpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(896usize) as _) }
    }
    #[doc = "Analog trim register."]
    #[inline(always)]
    pub const fn ana_trim(self) -> crate::common::Reg<regs::AnaTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3840usize) as _) }
    }
    #[doc = "SAR wounding register"]
    #[inline(always)]
    pub const fn wounding(self) -> crate::common::Reg<regs::Wounding, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3844usize) as _) }
    }
}
pub mod regs;
pub mod vals;
