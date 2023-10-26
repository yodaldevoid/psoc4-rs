#[doc = "Analog trim register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnaTrim(pub u32);
impl AnaTrim {
    #[doc = "Attenuation cap trimming"]
    #[inline(always)]
    pub const fn cap_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Attenuation cap trimming"]
    #[inline(always)]
    pub fn set_cap_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Attenuation cap trimming"]
    #[inline(always)]
    pub const fn trimunit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Attenuation cap trimming"]
    #[inline(always)]
    pub fn set_trimunit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for AnaTrim {
    #[inline(always)]
    fn default() -> AnaTrim {
        AnaTrim(0)
    }
}
#[doc = "Current averaging status (for debug)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AvgStat(pub u32);
impl AvgStat {
    #[doc = "the current value of the averaging accumulator"]
    #[inline(always)]
    pub const fn cur_avg_accu(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "the current value of the averaging accumulator"]
    #[inline(always)]
    pub fn set_cur_avg_accu(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "the current value of the averaging counter. Note that the value shown is updated after the sampling time and therefore runs ahead of the accumulator update."]
    #[inline(always)]
    pub const fn cur_avg_cnt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "the current value of the averaging counter. Note that the value shown is updated after the sampling time and therefore runs ahead of the accumulator update."]
    #[inline(always)]
    pub fn set_cur_avg_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for AvgStat {
    #[inline(always)]
    fn default() -> AvgStat {
        AvgStat(0)
    }
}
#[doc = "Channel configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanConfig(pub u32);
impl ChanConfig {
    #[doc = "Address of the pin to be sampled by this channel. If differential is enabled then PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. PIN_ADDR points to the even pin of a pin pair. For differential the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus."]
    #[inline(always)]
    pub const fn pin_addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Address of the pin to be sampled by this channel. If differential is enabled then PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. PIN_ADDR points to the even pin of a pin pair. For differential the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus."]
    #[inline(always)]
    pub fn set_pin_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub const fn port_addr(&self) -> super::vals::PortAddr {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::PortAddr::from_bits(val as u8)
    }
    #[doc = "Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn set_port_addr(&mut self, val: super::vals::PortAddr) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub const fn differential_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub fn set_differential_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resolution for this channel. When AVG_EN is set this bit is ignored and always a 12-bit resolution (or highest resolution allowed by wounding) is used for this channel."]
    #[inline(always)]
    pub const fn resolution(&self) -> super::vals::Resolution {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Resolution::from_bits(val as u8)
    }
    #[doc = "Resolution for this channel. When AVG_EN is set this bit is ignored and always a 12-bit resolution (or highest resolution allowed by wounding) is used for this channel."]
    #[inline(always)]
    pub fn set_resolution(&mut self, val: super::vals::Resolution) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub const fn avg_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn set_avg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub const fn sample_time_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn set_sample_time_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formating), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub const fn dsi_out_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formating), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn set_dsi_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChanConfig {
    #[inline(always)]
    fn default() -> ChanConfig {
        ChanConfig(0)
    }
}
#[doc = "Enable bits for the channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanEn(pub u32);
impl ChanEn {
    #[doc = "Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub const fn chan_en(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub fn set_chan_en(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChanEn {
    #[inline(always)]
    fn default() -> ChanEn {
        ChanEn(0)
    }
}
#[doc = "Channel result data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanResult(pub u32);
impl ChanResult {
    #[doc = "SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub const fn result(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub fn set_result(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub const fn saturate_intr_mir(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub fn set_saturate_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_RANGE_INTR register"]
    #[inline(always)]
    pub const fn range_intr_mir(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_RANGE_INTR register"]
    #[inline(always)]
    pub fn set_range_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_RESULT_VALID register"]
    #[inline(always)]
    pub const fn chan_result_valid_mir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_RESULT_VALID register"]
    #[inline(always)]
    pub fn set_chan_result_valid_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChanResult {
    #[inline(always)]
    fn default() -> ChanResult {
        ChanResult(0)
    }
}
#[doc = "Channel result data register valid bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanResultValid(pub u32);
impl ChanResultValid {
    #[doc = "If set the corresponding RESULT data is valid, i.e. was sampled during the last scan."]
    #[inline(always)]
    pub const fn chan_result_valid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If set the corresponding RESULT data is valid, i.e. was sampled during the last scan."]
    #[inline(always)]
    pub fn set_chan_result_valid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChanResultValid {
    #[inline(always)]
    fn default() -> ChanResultValid {
        ChanResultValid(0)
    }
}
#[doc = "Channel working data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanWork(pub u32);
impl ChanWork {
    #[doc = "SAR conversion working data of the channel. The data is written here right after sampling this channel."]
    #[inline(always)]
    pub const fn work(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SAR conversion working data of the channel. The data is written here right after sampling this channel."]
    #[inline(always)]
    pub fn set_work(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_WORK_VALID register"]
    #[inline(always)]
    pub const fn chan_work_valid_mir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_CHAN_WORK_VALID register"]
    #[inline(always)]
    pub fn set_chan_work_valid_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChanWork {
    #[inline(always)]
    fn default() -> ChanWork {
        ChanWork(0)
    }
}
#[doc = "Channel working data register valid bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanWorkValid(pub u32);
impl ChanWorkValid {
    #[doc = "If set the corresponding WORK data is valid, i.e. was already sampled during the current scan."]
    #[inline(always)]
    pub const fn chan_work_valid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If set the corresponding WORK data is valid, i.e. was already sampled during the current scan."]
    #[inline(always)]
    pub fn set_chan_work_valid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChanWorkValid {
    #[inline(always)]
    fn default() -> ChanWorkValid {
        ChanWorkValid(0)
    }
}
#[doc = "Analog control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "SARADC internal VREF selection."]
    #[inline(always)]
    pub const fn vref_sel(&self) -> super::vals::VrefSel {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::VrefSel::from_bits(val as u8)
    }
    #[doc = "SARADC internal VREF selection."]
    #[inline(always)]
    pub fn set_vref_sel(&mut self, val: super::vals::VrefSel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    pub const fn vref_byp_cap_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    pub fn set_vref_byp_cap_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    pub const fn neg_sel(&self) -> super::vals::NegSel {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::NegSel::from_bits(val as u8)
    }
    #[doc = "SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    pub fn set_neg_sel(&mut self, val: super::vals::NegSel) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    pub const fn sar_hw_ctrl_negvref(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    pub fn set_sar_hw_ctrl_negvref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "VREF buffer low power mode."]
    #[inline(always)]
    pub const fn pwr_ctrl_vref(&self) -> super::vals::PwrCtrlVref {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::PwrCtrlVref::from_bits(val as u8)
    }
    #[doc = "VREF buffer low power mode."]
    #[inline(always)]
    pub fn set_pwr_ctrl_vref(&mut self, val: super::vals::PwrCtrlVref) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    pub const fn spare(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    pub fn set_spare(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "SARADC internal pump: 0=disabled: pump output is VDDA, 1=enabled: pump output is boosted."]
    #[inline(always)]
    pub const fn boostpump_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SARADC internal pump: 0=disabled: pump output is VDDA, 1=enabled: pump output is boosted."]
    #[inline(always)]
    pub fn set_boostpump_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "SARADC low power mode."]
    #[inline(always)]
    pub const fn icont_lv(&self) -> super::vals::IcontLv {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::IcontLv::from_bits(val as u8)
    }
    #[doc = "SARADC low power mode."]
    #[inline(always)]
    pub fn set_icont_lv(&mut self, val: super::vals::IcontLv) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "- 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub const fn deepsleep_on(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn set_deepsleep_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "- 0: bypass clock domain synchronisation of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    pub const fn dsi_sync_config(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: bypass clock domain synchronisation of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    pub fn set_dsi_sync_config(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    pub const fn dsi_mode(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    pub fn set_dsi_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    pub const fn switch_disable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    pub fn set_switch_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Before enabling always make sure the SAR is idle (STATUS.BUSY==0) - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgaiting) on write. - 1: SAR IP enabled."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Before enabling always make sure the SAR is idle (STATUS.BUSY==0) - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgaiting) on write. - 1: SAR IP enabled."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "DFT control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DftCtrl(pub u32);
impl DftCtrl {
    #[doc = "DFT control: Control for delay circuits on sampling phase, =1 doubes the non-overlap delay"]
    #[inline(always)]
    pub const fn dly_inc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DFT control: Control for delay circuits on sampling phase, =1 doubes the non-overlap delay"]
    #[inline(always)]
    pub fn set_dly_inc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DFT control for getting higher input impedance, must be 1 (0 is deprecated)"]
    #[inline(always)]
    pub const fn hiz(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DFT control for getting higher input impedance, must be 1 (0 is deprecated)"]
    #[inline(always)]
    pub fn set_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DFT control for preamp inputs"]
    #[inline(always)]
    pub const fn dft_inc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DFT control for preamp inputs"]
    #[inline(always)]
    pub fn set_dft_inc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "DFT control for preamp outputs"]
    #[inline(always)]
    pub const fn dft_outc(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "DFT control for preamp outputs"]
    #[inline(always)]
    pub fn set_dft_outc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Usage 1: DFT bits for DAC array Usage 2: For \\[0\\]=1 (when dcen=0): Delay timing for latch enable increased by 20 percent \\[1\\]=1: comparator preamp power level increased by 25 percent"]
    #[inline(always)]
    pub const fn sel_csel_dft(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Usage 1: DFT bits for DAC array Usage 2: For \\[0\\]=1 (when dcen=0): Delay timing for latch enable increased by 20 percent \\[1\\]=1: comparator preamp power level increased by 25 percent"]
    #[inline(always)]
    pub fn set_sel_csel_dft(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Mux select signal for DAC control"]
    #[inline(always)]
    pub const fn en_csel_dft(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Mux select signal for DAC control"]
    #[inline(always)]
    pub fn set_en_csel_dft(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Delay Control Enable for latch. - 0: doubles the latch enable time. - 1: normal latch enable time (default)."]
    #[inline(always)]
    pub const fn dcen(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Delay Control Enable for latch. - 0: doubles the latch enable time. - 1: normal latch enable time (default)."]
    #[inline(always)]
    pub fn set_dcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "During deepsleep/ hibernate mode keep SARMUX active, i.e. do not open all switches (disconnect), to be used for ADFT"]
    #[inline(always)]
    pub const fn adft_override(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "During deepsleep/ hibernate mode keep SARMUX active, i.e. do not open all switches (disconnect), to be used for ADFT"]
    #[inline(always)]
    pub fn set_adft_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DftCtrl {
    #[inline(always)]
    fn default() -> DftCtrl {
        DftCtrl(0)
    }
}
#[doc = "Injection channel configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InjChanConfig(pub u32);
impl InjChanConfig {
    #[doc = "Address of the pin to be sampled by this injection channel. If differential is enabled then PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub const fn inj_pin_addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Address of the pin to be sampled by this injection channel. If differential is enabled then PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub fn set_inj_pin_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub const fn inj_port_addr(&self) -> super::vals::InjPortAddr {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::InjPortAddr::from_bits(val as u8)
    }
    #[doc = "Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn set_inj_port_addr(&mut self, val: super::vals::InjPortAddr) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub const fn inj_differential_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub fn set_inj_differential_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resolution for this channel."]
    #[inline(always)]
    pub const fn inj_resolution(&self) -> super::vals::InjResolution {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::InjResolution::from_bits(val as u8)
    }
    #[doc = "Resolution for this channel."]
    #[inline(always)]
    pub fn set_inj_resolution(&mut self, val: super::vals::InjResolution) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub const fn inj_avg_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn set_inj_avg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub const fn inj_sample_time_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn set_inj_sample_time_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub const fn inj_tailgating(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set if the SAR is not busy. If the SAR is busy, the INJ channel addressed pin is sampled at the end of the current scan. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub fn set_inj_tailgating(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub const fn inj_start_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub fn set_inj_start_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for InjChanConfig {
    #[inline(always)]
    fn default() -> InjChanConfig {
        InjChanConfig(0)
    }
}
#[doc = "Injection channel result register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InjResult(pub u32);
impl InjResult {
    #[doc = "SAR conversion result of the channel."]
    #[inline(always)]
    pub const fn inj_result(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SAR conversion result of the channel."]
    #[inline(always)]
    pub fn set_inj_result(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub const fn inj_collision_intr_mir(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn set_inj_collision_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub const fn inj_saturate_intr_mir(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn set_inj_saturate_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub const fn inj_range_intr_mir(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn set_inj_range_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub const fn inj_eoc_intr_mir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn set_inj_eoc_intr_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for InjResult {
    #[inline(always)]
    fn default() -> InjResult {
        InjResult(0)
    }
}
#[doc = "Interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn eos_intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_eos_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn overflow_intr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_overflow_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceeding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn fw_collision_intr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceeding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_fw_collision_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceeding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn dsi_collision_intr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceeding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_dsi_collision_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn inj_eoc_intr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_inj_eoc_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF (for 12-bit resolution), this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn inj_saturate_intr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF (for 12-bit resolution), this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_inj_saturate_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn inj_range_intr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_inj_range_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 && INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceeding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn inj_collision_intr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 && INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceeding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_inj_collision_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt cause register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause(pub u32);
impl IntrCause {
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn eos_masked_mir(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_eos_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn overflow_masked_mir(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_overflow_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn fw_collision_masked_mir(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_fw_collision_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn dsi_collision_masked_mir(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_dsi_collision_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn inj_eoc_masked_mir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_inj_eoc_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn inj_saturate_masked_mir(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_inj_saturate_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn inj_range_masked_mir(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_inj_range_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub const fn inj_collision_masked_mir(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn set_inj_collision_masked_mir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
    #[inline(always)]
    pub const fn saturate_masked_red(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
    #[inline(always)]
    pub fn set_saturate_masked_red(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
    #[inline(always)]
    pub const fn range_masked_red(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
    #[inline(always)]
    pub fn set_range_masked_red(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IntrCause {
    #[inline(always)]
    fn default() -> IntrCause {
        IntrCause(0)
    }
}
#[doc = "Interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMask(pub u32);
impl IntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn eos_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_eos_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn overflow_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_overflow_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn fw_collision_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_fw_collision_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn dsi_collision_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_dsi_collision_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_eoc_mask(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_eoc_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_saturate_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_saturate_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_range_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_range_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_collision_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_collision_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrMask {
    #[inline(always)]
    fn default() -> IntrMask {
        IntrMask(0)
    }
}
#[doc = "Interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMasked(pub u32);
impl IntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn eos_masked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_eos_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn overflow_masked(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_overflow_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn fw_collision_masked(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_fw_collision_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn dsi_collision_masked(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_dsi_collision_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn inj_eoc_masked(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_inj_eoc_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn inj_saturate_masked(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_inj_saturate_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn inj_range_masked(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_inj_range_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn inj_collision_masked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_inj_collision_masked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn eos_set(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_eos_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn overflow_set(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_overflow_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn fw_collision_set(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_fw_collision_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn dsi_collision_set(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_dsi_collision_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_eoc_set(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_eoc_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_saturate_set(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_saturate_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_range_set(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_range_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn inj_collision_set(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_inj_collision_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "SARMUX Firmware switch controls"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitch0(pub u32);
impl MuxSwitch0 {
    #[doc = "Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p0_vplus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p1_vplus(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p2_vplus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p2_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p3_vplus(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p3_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p4_vplus(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p4_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p5_vplus(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p5_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p6_vplus(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p6_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p7_vplus(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p7_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p0_vminus(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p1_vminus(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p2_vminus(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p2_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p3_vminus(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p3_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p4_vminus(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p4_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p5_vminus(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p5_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p6_vminus(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p6_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p7_vminus(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p7_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_vssa_vminus(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_vssa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_temp_vplus(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_temp_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vplus(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vplus(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vminus(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vminus(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vplus(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vplus(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vminus(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vminus(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p4_coreio0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p4_coreio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p5_coreio1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p5_coreio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p6_coreio2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p6_coreio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p7_coreio3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p7_coreio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for MuxSwitch0 {
    #[inline(always)]
    fn default() -> MuxSwitch0 {
        MuxSwitch0(0)
    }
}
#[doc = "SARMUX Firmware switch controls"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitch1(pub u32);
impl MuxSwitch1 {
    #[doc = "Firmware control: 0=open, 1=close switch between P4 pin and dft_inp signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p4_dft_inp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P4 pin and dft_inp signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p4_dft_inp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P5 pin and dft_inm signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_p5_dft_inm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between P5 pin and dft_inm signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_p5_dft_inm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between adft0 signal and sarbus0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_adft0_sarbus0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between adft0 signal and sarbus0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_adft0_sarbus0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Firmware control: 0=open, 1=close switch between adft1 signal and sarbus1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub const fn mux_fw_adft1_sarbus1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Firmware control: 0=open, 1=close switch between adft1 signal and sarbus1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn set_mux_fw_adft1_sarbus1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MuxSwitch1 {
    #[inline(always)]
    fn default() -> MuxSwitch1 {
        MuxSwitch1(0)
    }
}
#[doc = "SARMUX Firmware switch control clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitchClear0(pub u32);
impl MuxSwitchClear0 {
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p0_vplus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p1_vplus(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p2_vplus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p2_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p3_vplus(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p3_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_vplus(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_vplus(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_vplus(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_vplus(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p0_vminus(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p1_vminus(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p2_vminus(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p2_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p3_vminus(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p3_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_vminus(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_vminus(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_vminus(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_vminus(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_vssa_vminus(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_vssa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_temp_vplus(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_temp_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vplus(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vplus(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vminus(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vminus(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vplus(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vplus(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vminus(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vminus(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_coreio0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_coreio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_coreio1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_coreio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_coreio2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_coreio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_coreio3(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_coreio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for MuxSwitchClear0 {
    #[inline(always)]
    fn default() -> MuxSwitchClear0 {
        MuxSwitchClear0(0)
    }
}
#[doc = "SARMUX Firmware switch control clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitchClear1(pub u32);
impl MuxSwitchClear1 {
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH1"]
    #[inline(always)]
    pub const fn mux_fw_p4_dft_inp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH1"]
    #[inline(always)]
    pub fn set_mux_fw_p4_dft_inp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH1"]
    #[inline(always)]
    pub const fn mux_fw_p5_dft_inm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH1"]
    #[inline(always)]
    pub fn set_mux_fw_p5_dft_inm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH1"]
    #[inline(always)]
    pub const fn mux_fw_adft0_sarbus0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH1"]
    #[inline(always)]
    pub fn set_mux_fw_adft0_sarbus0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH1"]
    #[inline(always)]
    pub const fn mux_fw_adft1_sarbus1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to clear corresponding bit in MUX_SWITCH1"]
    #[inline(always)]
    pub fn set_mux_fw_adft1_sarbus1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MuxSwitchClear1 {
    #[inline(always)]
    fn default() -> MuxSwitchClear1 {
        MuxSwitchClear1(0)
    }
}
#[doc = "SARMUX switch hardware control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitchHwCtrl(pub u32);
impl MuxSwitchHwCtrl {
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P0 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_p0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P0 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P1 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_p1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P1 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_p1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P2 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_p2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P2 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_p2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P3 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_p3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P3 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_p3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P4 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_p4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P4 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_p4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P5 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_p5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P5 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_p5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P6 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_p6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P6 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_p6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P7 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_p7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P7 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_p7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for vssa switch."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_vssa(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for vssa switch."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_vssa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for temp switch."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_temp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for temp switch."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_temp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusa switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_amuxbusa(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusa switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_amuxbusa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusb switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_amuxbusb(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusb switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_amuxbusb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus0 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_sarbus0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus0 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_sarbus0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus1 switches."]
    #[inline(always)]
    pub const fn mux_hw_ctrl_sarbus1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus1 switches."]
    #[inline(always)]
    pub fn set_mux_hw_ctrl_sarbus1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for MuxSwitchHwCtrl {
    #[inline(always)]
    fn default() -> MuxSwitchHwCtrl {
        MuxSwitchHwCtrl(0)
    }
}
#[doc = "SARMUX switch status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxSwitchStatus(pub u32);
impl MuxSwitchStatus {
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p0_vplus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p1_vplus(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p2_vplus(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p2_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p3_vplus(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p3_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_vplus(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_vplus(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_vplus(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_vplus(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p0_vminus(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p1_vminus(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p2_vminus(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p2_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p3_vminus(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p3_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p4_vminus(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p4_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p5_vminus(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p5_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p6_vminus(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p6_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_p7_vminus(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_p7_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_vssa_vminus(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_vssa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_temp_vplus(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_temp_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vplus(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vplus(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusa_vminus(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusa_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_amuxbusb_vminus(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_amuxbusb_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vplus(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vplus(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vplus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus0_vminus(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus0_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub const fn mux_fw_sarbus1_vminus(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn set_mux_fw_sarbus1_vminus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for MuxSwitchStatus {
    #[inline(always)]
    fn default() -> MuxSwitchStatus {
        MuxSwitchStatus(0)
    }
}
#[doc = "Switch pump control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PumpCtrl(pub u32);
impl PumpCtrl {
    #[doc = "Clock select: 0=external clock, 1=internal clock (deprecated)."]
    #[inline(always)]
    pub const fn clock_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock select: 0=external clock, 1=internal clock (deprecated)."]
    #[inline(always)]
    pub fn set_clock_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0=disabled: pump output is VDDA_PUMP, 1=enabled: pump output is boosted."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "0=disabled: pump output is VDDA_PUMP, 1=enabled: pump output is boosted."]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PumpCtrl {
    #[inline(always)]
    fn default() -> PumpCtrl {
        PumpCtrl(0)
    }
}
#[doc = "Global range detect mode register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeCond(pub u32);
impl RangeCond {
    #[doc = "Range condition select."]
    #[inline(always)]
    pub const fn range_cond(&self) -> super::vals::RangeCond {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::RangeCond::from_bits(val as u8)
    }
    #[doc = "Range condition select."]
    #[inline(always)]
    pub fn set_range_cond(&mut self, val: super::vals::RangeCond) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for RangeCond {
    #[inline(always)]
    fn default() -> RangeCond {
        RangeCond(0)
    }
}
#[doc = "Range detect interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeIntr(pub u32);
impl RangeIntr {
    #[doc = "Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn range_intr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_range_intr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RangeIntr {
    #[inline(always)]
    fn default() -> RangeIntr {
        RangeIntr(0)
    }
}
#[doc = "Range detect interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeIntrMask(pub u32);
impl RangeIntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn range_mask(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_range_mask(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RangeIntrMask {
    #[inline(always)]
    fn default() -> RangeIntrMask {
        RangeIntrMask(0)
    }
}
#[doc = "Range interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeIntrMasked(pub u32);
impl RangeIntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn range_masked(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_range_masked(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RangeIntrMasked {
    #[inline(always)]
    fn default() -> RangeIntrMasked {
        RangeIntrMasked(0)
    }
}
#[doc = "Range detect interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeIntrSet(pub u32);
impl RangeIntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn range_set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_range_set(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RangeIntrSet {
    #[inline(always)]
    fn default() -> RangeIntrSet {
        RangeIntrSet(0)
    }
}
#[doc = "Global range detect threshold register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeThres(pub u32);
impl RangeThres {
    #[doc = "Low threshold for range detect."]
    #[inline(always)]
    pub const fn range_low(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Low threshold for range detect."]
    #[inline(always)]
    pub fn set_range_low(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "High threshold for range detect."]
    #[inline(always)]
    pub const fn range_high(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "High threshold for range detect."]
    #[inline(always)]
    pub fn set_range_high(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for RangeThres {
    #[inline(always)]
    fn default() -> RangeThres {
        RangeThres(0)
    }
}
#[doc = "Sample control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SampleCtrl(pub u32);
impl SampleCtrl {
    #[doc = "Conversion resolution for channels that have sub-resolution enabled (RESOLUTION=1) (otherwise resolution is 12-bit)."]
    #[inline(always)]
    pub const fn sub_resolution(&self) -> super::vals::SubResolution {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SubResolution::from_bits(val as u8)
    }
    #[doc = "Conversion resolution for channels that have sub-resolution enabled (RESOLUTION=1) (otherwise resolution is 12-bit)."]
    #[inline(always)]
    pub fn set_sub_resolution(&mut self, val: super::vals::SubResolution) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    pub const fn left_align(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    pub fn set_left_align(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output data from a single ended conversion as a signed value"]
    #[inline(always)]
    pub const fn single_ended_signed(&self) -> super::vals::SingleEndedSigned {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SingleEndedSigned::from_bits(val as u8)
    }
    #[doc = "Output data from a single ended conversion as a signed value"]
    #[inline(always)]
    pub fn set_single_ended_signed(&mut self, val: super::vals::SingleEndedSigned) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Output data from a differential conversion as a signed value"]
    #[inline(always)]
    pub const fn differential_signed(&self) -> super::vals::DifferentialSigned {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DifferentialSigned::from_bits(val as u8)
    }
    #[doc = "Output data from a differential conversion as a signed value"]
    #[inline(always)]
    pub fn set_differential_signed(&mut self, val: super::vals::DifferentialSigned) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Averaging Count for channels that have over sampling enabled (AVG_EN). A channel will be sampled back to back (1<<(AVG_CNT+1)) = \\[2..256\\] times before the result is stored and the next enabled channel is sampled (1st order accumulate and dump filter). If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3)."]
    #[inline(always)]
    pub const fn avg_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Averaging Count for channels that have over sampling enabled (AVG_EN). A channel will be sampled back to back (1<<(AVG_CNT+1)) = \\[2..256\\] times before the result is stored and the next enabled channel is sampled (1st order accumulate and dump filter). If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3)."]
    #[inline(always)]
    pub fn set_avg_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Averaging shifting: after averaging the result is shifted right to fit in the sample resolution. For averaging the sample resolution is the highest resolution allowed by wounding."]
    #[inline(always)]
    pub const fn avg_shift(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Averaging shifting: after averaging the result is shifted right to fit in the sample resolution. For averaging the sample resolution is the highest resolution allowed by wounding."]
    #[inline(always)]
    pub fn set_avg_shift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "- 0: Wait for next FW_TRIGGER (one shot) or hardware (DSI) trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    pub const fn continuous(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: Wait for next FW_TRIGGER (one shot) or hardware (DSI) trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    pub fn set_continuous(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "- 0: firmware trigger only: disable hardware (DSI) trigger. - 1: enable hardware (DSI) trigger (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    pub const fn dsi_trigger_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: firmware trigger only: disable hardware (DSI) trigger. - 1: enable hardware (DSI) trigger (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    pub fn set_dsi_trigger_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "- 0: DSI trigger signal is a pulse input, a positive edge detected on the DSI trigger signal triggers a new scan. - 1: DSI trigger signal is a level input, as long as the DSI trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    pub const fn dsi_trigger_level(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: DSI trigger signal is a pulse input, a positive edge detected on the DSI trigger signal triggers a new scan. - 1: DSI trigger signal is a level input, as long as the DSI trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    pub fn set_dsi_trigger_level(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "- 0: bypass clock domain synchronisation of the DSI trigger signal. - 1: synchronize the DSI trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub const fn dsi_sync_trigger(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "- 0: bypass clock domain synchronisation of the DSI trigger signal. - 1: synchronize the DSI trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub fn set_dsi_sync_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a pulse is send on the dsi_eos signal."]
    #[inline(always)]
    pub const fn eos_dsi_out_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a pulse is send on the dsi_eos signal."]
    #[inline(always)]
    pub fn set_eos_dsi_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SampleCtrl {
    #[inline(always)]
    fn default() -> SampleCtrl {
        SampleCtrl(0)
    }
}
#[doc = "Sample time specification ST0 and ST1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SampleTime01(pub u32);
impl SampleTime01 {
    #[doc = "Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is half a clock less than specified here. The minimum sample time is 194ns, which is 3.5 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub const fn sample_time0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is half a clock less than specified here. The minimum sample time is 194ns, which is 3.5 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub fn set_sample_time0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Sample time1"]
    #[inline(always)]
    pub const fn sample_time1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sample time1"]
    #[inline(always)]
    pub fn set_sample_time1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for SampleTime01 {
    #[inline(always)]
    fn default() -> SampleTime01 {
        SampleTime01(0)
    }
}
#[doc = "Sample time specification ST2 and ST3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SampleTime23(pub u32);
impl SampleTime23 {
    #[doc = "Sample time2"]
    #[inline(always)]
    pub const fn sample_time2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sample time2"]
    #[inline(always)]
    pub fn set_sample_time2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Sample time3"]
    #[inline(always)]
    pub const fn sample_time3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sample time3"]
    #[inline(always)]
    pub fn set_sample_time3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for SampleTime23 {
    #[inline(always)]
    fn default() -> SampleTime23 {
        SampleTime23(0)
    }
}
#[doc = "Saturate interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaturateIntr(pub u32);
impl SaturateIntr {
    #[doc = "Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF (for 12-bit resolution), this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn saturate_intr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF (for 12-bit resolution), this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_saturate_intr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SaturateIntr {
    #[inline(always)]
    fn default() -> SaturateIntr {
        SaturateIntr(0)
    }
}
#[doc = "Saturate interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaturateIntrMask(pub u32);
impl SaturateIntrMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn saturate_mask(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_saturate_mask(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SaturateIntrMask {
    #[inline(always)]
    fn default() -> SaturateIntrMask {
        SaturateIntrMask(0)
    }
}
#[doc = "Saturate interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaturateIntrMasked(pub u32);
impl SaturateIntrMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn saturate_masked(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_saturate_masked(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SaturateIntrMasked {
    #[inline(always)]
    fn default() -> SaturateIntrMasked {
        SaturateIntrMasked(0)
    }
}
#[doc = "Saturate interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SaturateIntrSet(pub u32);
impl SaturateIntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn saturate_set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_saturate_set(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SaturateIntrSet {
    #[inline(always)]
    fn default() -> SaturateIntrSet {
        SaturateIntrSet(0)
    }
}
#[doc = "Start control register (firmware trigger)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StartCtrl(pub u32);
impl StartCtrl {
    #[doc = "When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub const fn fw_trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub fn set_fw_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for StartCtrl {
    #[inline(always)]
    fn default() -> StartCtrl {
        StartCtrl(0)
    }
}
#[doc = "Current status of internal SAR registers (mostly for debug)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "current channel being sampled (channel 16 indicates the injection channel), only valid if BUSY."]
    #[inline(always)]
    pub const fn cur_chan(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "current channel being sampled (channel 16 indicates the injection channel), only valid if BUSY."]
    #[inline(always)]
    pub fn set_cur_chan(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "the current switch status, including DSI and sequencer controls, of the switch in the SARADC that shorts NEG with VREF input (see NEG_SEL)."]
    #[inline(always)]
    pub const fn sw_vref_neg(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "the current switch status, including DSI and sequencer controls, of the switch in the SARADC that shorts NEG with VREF input (see NEG_SEL)."]
    #[inline(always)]
    pub fn set_sw_vref_neg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "If high then the SAR is busy with a conversion. This bit is always high when CONTINUOUS is set. Firmware should wait for this bit to be low before putting the SAR in power down."]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If high then the SAR is busy with a conversion. This bit is always high when CONTINUOUS is set. Firmware should wait for this bit to be low before putting the SAR in power down."]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "SAR wounding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wounding(pub u32);
impl Wounding {
    #[doc = "Maximum SAR resolution allowed"]
    #[inline(always)]
    pub const fn wound_resolution(&self) -> super::vals::WoundResolution {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::WoundResolution::from_bits(val as u8)
    }
    #[doc = "Maximum SAR resolution allowed"]
    #[inline(always)]
    pub fn set_wound_resolution(&mut self, val: super::vals::WoundResolution) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Wounding {
    #[inline(always)]
    fn default() -> Wounding {
        Wounding(0)
    }
}
