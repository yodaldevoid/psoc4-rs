#[doc = "Capsense Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csd0 {
    ptr: *mut u8,
}
unsafe impl Send for Csd0 {}
unsafe impl Sync for Csd0 {}
impl Csd0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration and Control"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Spare MMIO"]
    #[inline(always)]
    pub const fn spare(self) -> crate::common::Reg<regs::Spare, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Current Sequencer status"]
    #[inline(always)]
    pub const fn stat_seq(self) -> crate::common::Reg<regs::StatSeq, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Current status counts"]
    #[inline(always)]
    pub const fn stat_cnts(self) -> crate::common::Reg<regs::StatCnts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Result CSD/CSX accumulation counter value 1"]
    #[inline(always)]
    pub const fn result_val1(self) -> crate::common::Reg<regs::ResultVal1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "Result CSX accumulation counter value 2"]
    #[inline(always)]
    pub const fn result_val2(self) -> crate::common::Reg<regs::ResultVal2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(212usize) as _) }
    }
    #[doc = "ADC measurement"]
    #[inline(always)]
    pub const fn adc_res(self) -> crate::common::Reg<regs::AdcRes, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "CSD Interrupt Request Register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "CSD Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(self) -> crate::common::Reg<regs::IntrSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "CSD Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(self) -> crate::common::Reg<regs::IntrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(248usize) as _) }
    }
    #[doc = "CSD Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(self) -> crate::common::Reg<regs::IntrMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize) as _) }
    }
    #[doc = "High Speed Comparator configuration"]
    #[inline(always)]
    pub const fn hscmp(self) -> crate::common::Reg<regs::Hscmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Reference Generator configuration"]
    #[inline(always)]
    pub const fn ambuf(self) -> crate::common::Reg<regs::Ambuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize) as _) }
    }
    #[doc = "Reference Generator configuration"]
    #[inline(always)]
    pub const fn refgen(self) -> crate::common::Reg<regs::Refgen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(392usize) as _) }
    }
    #[doc = "CSD Comparator configuration"]
    #[inline(always)]
    pub const fn csdcmp(self) -> crate::common::Reg<regs::Csdcmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(396usize) as _) }
    }
    #[doc = "IDACA Configuration"]
    #[inline(always)]
    pub const fn idaca(self) -> crate::common::Reg<regs::Idaca, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(448usize) as _) }
    }
    #[doc = "IDACB Configuration"]
    #[inline(always)]
    pub const fn idacb(self) -> crate::common::Reg<regs::Idacb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(452usize) as _) }
    }
    #[doc = "Switch Resistance configuration"]
    #[inline(always)]
    pub const fn sw_res(self) -> crate::common::Reg<regs::SwRes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(496usize) as _) }
    }
    #[doc = "Sense clock period"]
    #[inline(always)]
    pub const fn sense_period(self) -> crate::common::Reg<regs::SensePeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Sense clock duty cycle"]
    #[inline(always)]
    pub const fn sense_duty(self) -> crate::common::Reg<regs::SenseDuty, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "HSCMP Pos input switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_hs_p_sel(self) -> crate::common::Reg<regs::SwHsPsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(640usize) as _) }
    }
    #[doc = "HSCMP Neg input switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_hs_n_sel(self) -> crate::common::Reg<regs::SwHsNsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(644usize) as _) }
    }
    #[doc = "Shielding switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_shield_sel(self) -> crate::common::Reg<regs::SwShieldSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(648usize) as _) }
    }
    #[doc = "Amuxbuffer switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_amuxbuf_sel(self) -> crate::common::Reg<regs::SwAmuxbufSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(656usize) as _) }
    }
    #[doc = "AMUXBUS bypass switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_byp_sel(self) -> crate::common::Reg<regs::SwBypSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(660usize) as _) }
    }
    #[doc = "CSDCMP Pos Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_cmp_p_sel(self) -> crate::common::Reg<regs::SwCmpPsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(672usize) as _) }
    }
    #[doc = "CSDCMP Neg Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_cmp_n_sel(self) -> crate::common::Reg<regs::SwCmpNsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(676usize) as _) }
    }
    #[doc = "Reference Generator Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_refgen_sel(self) -> crate::common::Reg<regs::SwRefgenSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(680usize) as _) }
    }
    #[doc = "Full Wave Cmod Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_fw_mod_sel(self) -> crate::common::Reg<regs::SwFwModSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(688usize) as _) }
    }
    #[doc = "Full Wave Csh_tank Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_fw_tank_sel(self) -> crate::common::Reg<regs::SwFwTankSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(692usize) as _) }
    }
    #[doc = "DSI output switch control Waveform selection"]
    #[inline(always)]
    pub const fn sw_dsi_sel(self) -> crate::common::Reg<regs::SwDsiSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(704usize) as _) }
    }
    #[doc = "Sequencer Timing"]
    #[inline(always)]
    pub const fn seq_time(self) -> crate::common::Reg<regs::SeqTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize) as _) }
    }
    #[doc = "Sequencer Initial conversion and sample counts"]
    #[inline(always)]
    pub const fn seq_init_cnt(self) -> crate::common::Reg<regs::SeqInitCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(784usize) as _) }
    }
    #[doc = "Sequencer Normal conversion and sample counts"]
    #[inline(always)]
    pub const fn seq_norm_cnt(self) -> crate::common::Reg<regs::SeqNormCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(788usize) as _) }
    }
    #[doc = "ADC Control"]
    #[inline(always)]
    pub const fn adc_ctl(self) -> crate::common::Reg<regs::AdcCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(800usize) as _) }
    }
    #[doc = "Sequencer start"]
    #[inline(always)]
    pub const fn seq_start(self) -> crate::common::Reg<regs::SeqStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(832usize) as _) }
    }
    #[doc = "Trim control"]
    #[inline(always)]
    pub const fn trim_ctrl(self) -> crate::common::Reg<regs::TrimCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3840usize) as _) }
    }
}
pub mod regs;
pub mod vals;
