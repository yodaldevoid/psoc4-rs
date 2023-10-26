#[doc = "Counter compare/capture register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc = "In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
    #[inline(always)]
    pub const fn cc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
    #[inline(always)]
    pub fn set_cc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cc {
    #[inline(always)]
    fn default() -> Cc {
        Cc(0)
    }
}
#[doc = "Counter buffered compare/capture register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcBuff(pub u32);
impl CcBuff {
    #[doc = "Additional buffer for counter CC register."]
    #[inline(always)]
    pub const fn cc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Additional buffer for counter CC register."]
    #[inline(always)]
    pub fn set_cc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CcBuff {
    #[inline(always)]
    fn default() -> CcBuff {
        CcBuff(0)
    }
}
#[doc = "TCPWM command register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub const fn counter_capture(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn set_counter_capture(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub const fn counter_reload(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn set_counter_reload(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub const fn counter_stop(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn set_counter_stop(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub const fn counter_start(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn set_counter_start(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        Cmd(0)
    }
}
#[doc = "Counter control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CntCtrl(pub u32);
impl CntCtrl {
    #[doc = "Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub const fn auto_reload_cc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn set_auto_reload_cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub const fn auto_reload_period(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn set_auto_reload_period(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub const fn pwm_sync_kill(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub fn set_pwm_sync_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub const fn pwm_stop_on_kill(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn set_pwm_stop_on_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    pub const fn generic(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    pub fn set_generic(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Determines counter direction."]
    #[inline(always)]
    pub const fn up_down_mode(&self) -> super::vals::UpDownMode {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::UpDownMode::from_bits(val as u8)
    }
    #[doc = "Determines counter direction."]
    #[inline(always)]
    pub fn set_up_down_mode(&mut self, val: super::vals::UpDownMode) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub const fn one_shot(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub fn set_one_shot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\] and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    pub const fn quadrature_mode(&self) -> super::vals::QuadratureMode {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::QuadratureMode::from_bits(val as u8)
    }
    #[doc = "In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\] and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    pub fn set_quadrature_mode(&mut self, val: super::vals::QuadratureMode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Counter mode."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Counter mode."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for CntCtrl {
    #[inline(always)]
    fn default() -> CntCtrl {
        CntCtrl(0)
    }
}
#[doc = "Counter count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Counter(pub u32);
impl Counter {
    #[doc = "16-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub const fn counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "16-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub fn set_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Counter {
    #[inline(always)]
    fn default() -> Counter {
        Counter(0)
    }
}
#[doc = "Interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Terminal count event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn cc_match(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter matches CC register event. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_cc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "TCPWM Counter interrupt cause register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause(pub u32);
impl IntrCause {
    #[doc = "Counters interrupt signal active. If the counter is disabled through CTRL.COUNTER_ENABLED, the associated interrupt field is immediately set to '0'."]
    #[inline(always)]
    pub const fn counter_int(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Counters interrupt signal active. If the counter is disabled through CTRL.COUNTER_ENABLED, the associated interrupt field is immediately set to '0'."]
    #[inline(always)]
    pub fn set_counter_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn cc_match(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_cc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn cc_match(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_cc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrMasked {
    #[inline(always)]
    fn default() -> IntrMasked {
        IntrMasked(0)
    }
}
#[doc = "Interrupt set request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSet(pub u32);
impl IntrSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn tc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_tc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn cc_match(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_cc_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntrSet {
    #[inline(always)]
    fn default() -> IntrSet {
        IntrSet(0)
    }
}
#[doc = "Counter period register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Period(pub u32);
impl Period {
    #[doc = "Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    pub const fn period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    pub fn set_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Period {
    #[inline(always)]
    fn default() -> Period {
        Period(0)
    }
}
#[doc = "Counter buffered period register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PeriodBuff(pub u32);
impl PeriodBuff {
    #[doc = "Additional buffer for counter PERIOD register."]
    #[inline(always)]
    pub const fn period(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Additional buffer for counter PERIOD register."]
    #[inline(always)]
    pub fn set_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PeriodBuff {
    #[inline(always)]
    fn default() -> PeriodBuff {
        PeriodBuff(0)
    }
}
#[doc = "Counter status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
    #[inline(always)]
    pub const fn down(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
    #[inline(always)]
    pub fn set_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
    #[inline(always)]
    pub const fn generic(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
    #[inline(always)]
    pub fn set_generic(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "When '0', the counter is NOT running. When '1', the counter is running."]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When '0', the counter is NOT running. When '1', the counter is running."]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "TCPWM control register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcpwmCtrl(pub u32);
impl TcpwmCtrl {
    #[doc = "Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out')."]
    #[inline(always)]
    pub const fn counter_enabled(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out')."]
    #[inline(always)]
    pub fn set_counter_enabled(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for TcpwmCtrl {
    #[inline(always)]
    fn default() -> TcpwmCtrl {
        TcpwmCtrl(0)
    }
}
#[doc = "Counter trigger control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCtrl0(pub u32);
impl TrCtrl0 {
    #[doc = "Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger 1 is always '1'. Input trigger 2 is the first external trigger line (tcpwm.tr_in\\[0\\]). In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub const fn capture_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger 1 is always '1'. Input trigger 2 is the first external trigger line (tcpwm.tr_in\\[0\\]). In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn set_capture_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub const fn count_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub fn set_count_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with the value in the TCPWM_CNTn_PERIOD register."]
    #[inline(always)]
    pub const fn reload_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with the value in the TCPWM_CNTn_PERIOD register."]
    #[inline(always)]
    pub fn set_reload_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub const fn stop_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn set_stop_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub const fn start_sel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn set_start_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TrCtrl0 {
    #[inline(always)]
    fn default() -> TrCtrl0 {
        TrCtrl0(0)
    }
}
#[doc = "Counter trigger control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCtrl1(pub u32);
impl TrCtrl1 {
    #[doc = "A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    pub const fn capture_edge(&self) -> super::vals::CaptureEdge {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CaptureEdge::from_bits(val as u8)
    }
    #[doc = "A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    pub fn set_capture_edge(&mut self, val: super::vals::CaptureEdge) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub const fn count_edge(&self) -> super::vals::CountEdge {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CountEdge::from_bits(val as u8)
    }
    #[doc = "A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub fn set_count_edge(&mut self, val: super::vals::CountEdge) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub const fn reload_edge(&self) -> super::vals::ReloadEdge {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::ReloadEdge::from_bits(val as u8)
    }
    #[doc = "A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub fn set_reload_edge(&mut self, val: super::vals::ReloadEdge) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub const fn stop_edge(&self) -> super::vals::StopEdge {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::StopEdge::from_bits(val as u8)
    }
    #[doc = "A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub fn set_stop_edge(&mut self, val: super::vals::StopEdge) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub const fn start_edge(&self) -> super::vals::StartEdge {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::StartEdge::from_bits(val as u8)
    }
    #[doc = "A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub fn set_start_edge(&mut self, val: super::vals::StartEdge) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for TrCtrl1 {
    #[inline(always)]
    fn default() -> TrCtrl1 {
        TrCtrl1(0)
    }
}
#[doc = "Counter trigger control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCtrl2(pub u32);
impl TrCtrl2 {
    #[doc = "Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub const fn cc_match_mode(&self) -> super::vals::CcMatchMode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CcMatchMode::from_bits(val as u8)
    }
    #[doc = "Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub fn set_cc_match_mode(&mut self, val: super::vals::CcMatchMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    pub const fn overflow_mode(&self) -> super::vals::OverflowMode {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::OverflowMode::from_bits(val as u8)
    }
    #[doc = "Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn set_overflow_mode(&mut self, val: super::vals::OverflowMode) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    pub const fn underflow_mode(&self) -> super::vals::UnderflowMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::UnderflowMode::from_bits(val as u8)
    }
    #[doc = "Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    pub fn set_underflow_mode(&mut self, val: super::vals::UnderflowMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for TrCtrl2 {
    #[inline(always)]
    fn default() -> TrCtrl2 {
        TrCtrl2(0)
    }
}
