#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CaptureEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl CaptureEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CaptureEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CaptureEdge {
    #[inline(always)]
    fn from(val: u8) -> CaptureEdge {
        CaptureEdge::from_bits(val)
    }
}
impl From<CaptureEdge> for u8 {
    #[inline(always)]
    fn from(val: CaptureEdge) -> u8 {
        CaptureEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CcMatchMode {
    #[doc = "Set to '1'"]
    SET = 0,
    #[doc = "Set to '0'"]
    CLEAR = 0x01,
    #[doc = "Invert"]
    INVERT = 0x02,
    #[doc = "No Change"]
    NO_CHANGE = 0x03,
}
impl CcMatchMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcMatchMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcMatchMode {
    #[inline(always)]
    fn from(val: u8) -> CcMatchMode {
        CcMatchMode::from_bits(val)
    }
}
impl From<CcMatchMode> for u8 {
    #[inline(always)]
    fn from(val: CcMatchMode) -> u8 {
        CcMatchMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CountEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl CountEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CountEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CountEdge {
    #[inline(always)]
    fn from(val: u8) -> CountEdge {
        CountEdge::from_bits(val)
    }
}
impl From<CountEdge> for u8 {
    #[inline(always)]
    fn from(val: CountEdge) -> u8 {
        CountEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mode {
    #[doc = "Timer mode"]
    TIMER = 0,
    _RESERVED_1 = 0x01,
    #[doc = "Capture mode"]
    CAPTURE = 0x02,
    #[doc = "Quadrature encoding mode"]
    QUAD = 0x03,
    #[doc = "Pulse width modulation (PWM) mode"]
    PWM = 0x04,
    #[doc = "PWM with deadtime insertion mode"]
    PWM_DT = 0x05,
    #[doc = "Pseudo random pulse width modulation"]
    PWM_PR = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OverflowMode {
    #[doc = "Set to '1'"]
    SET = 0,
    #[doc = "Set to '0'"]
    CLEAR = 0x01,
    #[doc = "Invert"]
    INVERT = 0x02,
    #[doc = "No Change"]
    NO_CHANGE = 0x03,
}
impl OverflowMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OverflowMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OverflowMode {
    #[inline(always)]
    fn from(val: u8) -> OverflowMode {
        OverflowMode::from_bits(val)
    }
}
impl From<OverflowMode> for u8 {
    #[inline(always)]
    fn from(val: OverflowMode) -> u8 {
        OverflowMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum QuadratureMode {
    #[doc = "X1 encoding (QUAD mode)"]
    X1 = 0,
    #[doc = "X2 encoding (QUAD mode)"]
    X2 = 0x01,
    #[doc = "X4 encoding (QUAD mode)"]
    X4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl QuadratureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QuadratureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QuadratureMode {
    #[inline(always)]
    fn from(val: u8) -> QuadratureMode {
        QuadratureMode::from_bits(val)
    }
}
impl From<QuadratureMode> for u8 {
    #[inline(always)]
    fn from(val: QuadratureMode) -> u8 {
        QuadratureMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReloadEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl ReloadEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReloadEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReloadEdge {
    #[inline(always)]
    fn from(val: u8) -> ReloadEdge {
        ReloadEdge::from_bits(val)
    }
}
impl From<ReloadEdge> for u8 {
    #[inline(always)]
    fn from(val: ReloadEdge) -> u8 {
        ReloadEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StartEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl StartEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StartEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StartEdge {
    #[inline(always)]
    fn from(val: u8) -> StartEdge {
        StartEdge::from_bits(val)
    }
}
impl From<StartEdge> for u8 {
    #[inline(always)]
    fn from(val: StartEdge) -> u8 {
        StartEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StopEdge {
    #[doc = "Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 0x01,
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 0x02,
    #[doc = "No edge detection, use trigger as is."]
    NO_EDGE_DET = 0x03,
}
impl StopEdge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StopEdge {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StopEdge {
    #[inline(always)]
    fn from(val: u8) -> StopEdge {
        StopEdge::from_bits(val)
    }
}
impl From<StopEdge> for u8 {
    #[inline(always)]
    fn from(val: StopEdge) -> u8 {
        StopEdge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UnderflowMode {
    #[doc = "Set to '1'"]
    SET = 0,
    #[doc = "Set to '0'"]
    CLEAR = 0x01,
    #[doc = "Invert"]
    INVERT = 0x02,
    #[doc = "No Change"]
    NO_CHANGE = 0x03,
}
impl UnderflowMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UnderflowMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UnderflowMode {
    #[inline(always)]
    fn from(val: u8) -> UnderflowMode {
        UnderflowMode::from_bits(val)
    }
}
impl From<UnderflowMode> for u8 {
    #[inline(always)]
    fn from(val: UnderflowMode) -> u8 {
        UnderflowMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UpDownMode {
    #[doc = "Count up (to PERIOD). An overflow event is generated when the counter reaches PERIOD, and is changed to a different value. A terminal count event is generated when the counter reaches PERIOD, and is changed to a different value."]
    COUNT_UP = 0,
    #[doc = "Count down (to '0'). An underflow event is generated when the counter reaches '0', and is changed to a different value. A terminal count event is generated when the counter reaches '0', and is changed to a different value."]
    COUNT_DOWN = 0x01,
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter reaches PERIOD, and is changed to a different value. An underflow event is generated when the counter reaches '0', and is changed to a different value. A terminal count event is generated when the counter reaches '0', and is changed to a different value."]
    COUNT_UPDN1 = 0x02,
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter reaches PERIOD, and is changed to a different value. An underflow event is generated when the counter reaches '0', and is changed to a different value. A terminal count event is generated when the counter reaches '0', and is changed to a different value AND when the counter reaches PERIOD, and is changed to a different value. (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    COUNT_UPDN2 = 0x03,
}
impl UpDownMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UpDownMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UpDownMode {
    #[inline(always)]
    fn from(val: u8) -> UpDownMode {
        UpDownMode::from_bits(val)
    }
}
impl From<UpDownMode> for u8 {
    #[inline(always)]
    fn from(val: UpDownMode) -> u8 {
        UpDownMode::to_bits(val)
    }
}
