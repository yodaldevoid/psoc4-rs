#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "0': Vector Table is located at 0x0000:0000 in flash '1': Vector Table is located at 0x2000:0000 in SRAM Note that vectors for RESET and FAULT are always fetched from ROM. Value in flash/RAM is ignored for these vectors."]
    #[inline(always)]
    pub const fn vect_in_ram(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0': Vector Table is located at 0x0000:0000 in flash '1': Vector Table is located at 0x2000:0000 in SRAM Note that vectors for RESET and FAULT are always fetched from ROM. Value in flash/RAM is ignored for these vectors."]
    #[inline(always)]
    pub fn set_vect_in_ram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "DMA controller register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmacCtl(pub u32);
impl DmacCtl {
    #[doc = "Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub const fn arb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub fn set_arb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for DmacCtl {
    #[inline(always)]
    fn default() -> DmacCtl {
        DmacCtl(0)
    }
}
#[doc = "FLASH control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashCtl(pub u32);
impl FlashCtl {
    #[doc = "Amount of ROM wait states: '0': 0 wait states (fast flash: \\[0, 24\\] MHz system frequency, slow flash: \\[0, 16\\] MHz system frequency) '1': 1 wait state (fast flash: \\[24, 48\\] MHz system frequency, slow flash: \\[16, 32\\] MHz system frequency) '2': 2 wait states (slow flash: \\[32, 48\\] MHz system frequency) '3': 3 wait states (can be used to give more time for flash access if 2 wait states are not sufficient)"]
    #[inline(always)]
    pub const fn flash_ws(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Amount of ROM wait states: '0': 0 wait states (fast flash: \\[0, 24\\] MHz system frequency, slow flash: \\[0, 16\\] MHz system frequency) '1': 1 wait state (fast flash: \\[24, 48\\] MHz system frequency, slow flash: \\[16, 32\\] MHz system frequency) '2': 2 wait states (slow flash: \\[32, 48\\] MHz system frequency) '3': 3 wait states (can be used to give more time for flash access if 2 wait states are not sufficient)"]
    #[inline(always)]
    pub fn set_flash_ws(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Prefetch enable: '0': disabled. This is a desirable setting when FLASH_WS is '0' or when predictable execution time is required. '1': enabled."]
    #[inline(always)]
    pub const fn pref_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Prefetch enable: '0': disabled. This is a desirable setting when FLASH_WS is '0' or when predictable execution time is required. '1': enabled."]
    #[inline(always)]
    pub fn set_pref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1': Invalidates the content of the flash controller's buffers."]
    #[inline(always)]
    pub const fn flash_invalidate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1': Invalidates the content of the flash controller's buffers."]
    #[inline(always)]
    pub fn set_flash_invalidate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub const fn arb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub fn set_arb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for FlashCtl {
    #[inline(always)]
    fn default() -> FlashCtl {
        FlashCtl(0)
    }
}
#[doc = "MTB control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtbCtl(pub u32);
impl MtbCtl {
    #[doc = "1': Enable CPU Halt to stop MTB trace. ('HALTED' output of CM0+ can stop the trace when high/'1') '0': 'HALTED' output of CM0+ can not strop trace."]
    #[inline(always)]
    pub const fn cpu_halt_tstop_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1': Enable CPU Halt to stop MTB trace. ('HALTED' output of CM0+ can stop the trace when high/'1') '0': 'HALTED' output of CM0+ can not strop trace."]
    #[inline(always)]
    pub fn set_cpu_halt_tstop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for MtbCtl {
    #[inline(always)]
    fn default() -> MtbCtl {
        MtbCtl(0)
    }
}
#[doc = "DSI NMI pulse mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NmiMode(pub u32);
impl NmiMode {
    #[doc = "Specifies DSI NMI format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
    #[inline(always)]
    pub const fn dsi_nmi_pulse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies DSI NMI format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
    #[inline(always)]
    pub fn set_dsi_nmi_pulse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for NmiMode {
    #[inline(always)]
    fn default() -> NmiMode {
        NmiMode(0)
    }
}
#[doc = "Flash privilege register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrivFlash(pub u32);
impl PrivFlash {
    #[doc = "Indicates the limit where the privileged area of flash starts in increments of 256 Bytes. '0': Entire flash is Privileged. '1': First 256 Bytes are User accessible. Any number larger than the size of the flash indicates that the entire flash is user mode accessible. Note that Supervisory rows are always User accessible. If FLASH_PROT_LIMIT defines a non-empty privileged area, the boot ROM will assume that a system call table exists at the beginning of the Flash privileged area and use it for all SystemCalls made using SYSREQ."]
    #[inline(always)]
    pub const fn flash_prot_limit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Indicates the limit where the privileged area of flash starts in increments of 256 Bytes. '0': Entire flash is Privileged. '1': First 256 Bytes are User accessible. Any number larger than the size of the flash indicates that the entire flash is user mode accessible. Note that Supervisory rows are always User accessible. If FLASH_PROT_LIMIT defines a non-empty privileged area, the boot ROM will assume that a system call table exists at the beginning of the Flash privileged area and use it for all SystemCalls made using SYSREQ."]
    #[inline(always)]
    pub fn set_flash_prot_limit(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for PrivFlash {
    #[inline(always)]
    fn default() -> PrivFlash {
        PrivFlash(0)
    }
}
#[doc = "RAM privilege register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrivRam(pub u32);
impl PrivRam {
    #[doc = "Indicates the limit where the privileged area of SRAM starts in increments of 256 Bytes. '0': Entire SRAM is Privileged. '1': First 256 Bytes are User accessible. Any number larger than the size of the SRAM indicates that the entire SRAM is user mode accessible."]
    #[inline(always)]
    pub const fn ram_prot_limit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Indicates the limit where the privileged area of SRAM starts in increments of 256 Bytes. '0': Entire SRAM is Privileged. '1': First 256 Bytes are User accessible. Any number larger than the size of the SRAM indicates that the entire SRAM is user mode accessible."]
    #[inline(always)]
    pub fn set_ram_prot_limit(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for PrivRam {
    #[inline(always)]
    fn default() -> PrivRam {
        PrivRam(0)
    }
}
#[doc = "RAM 1 privilege register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrivRam1(pub u32);
impl PrivRam1 {
    #[doc = "See description of PRIV_RAM.RAM_PROT_LIMIT. Note that the reset value is 0x1ff, indicating that the complete RAM 1 memory capacity is User accessible."]
    #[inline(always)]
    pub const fn ram_prot_limit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "See description of PRIV_RAM.RAM_PROT_LIMIT. Note that the reset value is 0x1ff, indicating that the complete RAM 1 memory capacity is User accessible."]
    #[inline(always)]
    pub fn set_ram_prot_limit(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for PrivRam1 {
    #[inline(always)]
    fn default() -> PrivRam1 {
        PrivRam1(0)
    }
}
#[doc = "ROM privilege register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrivRom(pub u32);
impl PrivRom {
    #[doc = "Indicates the limit where the privileged area of the Boot ROM partition starts in increments of 256 Bytes. '0': Entire Boot ROM is Privileged. '1': First 256 Bytes are User accessible. ... BROM_PROT_LIMIT >= 'Boot ROM partition capacity': Entire Boot ROM partition is user mode accessible."]
    #[inline(always)]
    pub const fn brom_prot_limit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the limit where the privileged area of the Boot ROM partition starts in increments of 256 Bytes. '0': Entire Boot ROM is Privileged. '1': First 256 Bytes are User accessible. ... BROM_PROT_LIMIT >= 'Boot ROM partition capacity': Entire Boot ROM partition is user mode accessible."]
    #[inline(always)]
    pub fn set_brom_prot_limit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Indicates the limit where the privileged area of System ROM partition starts in increments of 256 Bytes. The limit is wrt. the start of the ROM memory (start of the Boot ROM partition). SROM_PROT_LIMIT * 256 Byte <= 'Boot ROM partition capacity': Entire System ROM is Privileged. SROM_PROT_LIMIT * 256 Byte > 'Boot ROM partition capacity': First SROM_PROT_LIMIT * 256 - 'Boot ROM partition capacity' Bytes are User accessible. ... SROM_PROT_LIMIT >= 'ROM capacity': Entire System ROM is user mode accessible."]
    #[inline(always)]
    pub const fn srom_prot_limit(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Indicates the limit where the privileged area of System ROM partition starts in increments of 256 Bytes. The limit is wrt. the start of the ROM memory (start of the Boot ROM partition). SROM_PROT_LIMIT * 256 Byte <= 'Boot ROM partition capacity': Entire System ROM is Privileged. SROM_PROT_LIMIT * 256 Byte > 'Boot ROM partition capacity': First SROM_PROT_LIMIT * 256 - 'Boot ROM partition capacity' Bytes are User accessible. ... SROM_PROT_LIMIT >= 'ROM capacity': Entire System ROM is user mode accessible."]
    #[inline(always)]
    pub fn set_srom_prot_limit(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for PrivRom {
    #[inline(always)]
    fn default() -> PrivRom {
        PrivRom(0)
    }
}
#[doc = "Protection control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Protection(pub u32);
impl Protection {
    #[doc = "Current protection mode; this field is available as a global signal everywhere in the system. Writes to this field are ignored when PROTECTION_LOCK is '1': 0b1xxx: BOOT 0b01xx: KILL 0b001x: PROTECTED 0b0001: OPEN 0b0000: VIRGIN (also used for DEAD mode, but then FLASH_LOCK is also set)"]
    #[inline(always)]
    pub const fn protection_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Current protection mode; this field is available as a global signal everywhere in the system. Writes to this field are ignored when PROTECTION_LOCK is '1': 0b1xxx: BOOT 0b01xx: KILL 0b001x: PROTECTED 0b0001: OPEN 0b0000: VIRGIN (also used for DEAD mode, but then FLASH_LOCK is also set)"]
    #[inline(always)]
    pub fn set_protection_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Setting this bit will force SPCIF.ADDRESS.AXA to be ignored, which prevents SM Flash from being erased or overwritten. It is used to indicate the DEAD protection mode. Writes to this field are ignored when PROTECTION_LOCK is '1'"]
    #[inline(always)]
    pub const fn flash_lock(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit will force SPCIF.ADDRESS.AXA to be ignored, which prevents SM Flash from being erased or overwritten. It is used to indicate the DEAD protection mode. Writes to this field are ignored when PROTECTION_LOCK is '1'"]
    #[inline(always)]
    pub fn set_flash_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Setting this field will block (ignore) any further writes to the PROTECTION_MODE field in this register. Once '1', this field cannot be cleared."]
    #[inline(always)]
    pub const fn protection_lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this field will block (ignore) any further writes to the PROTECTION_MODE field in this register. Once '1', this field cannot be cleared."]
    #[inline(always)]
    pub fn set_protection_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Protection {
    #[inline(always)]
    fn default() -> Protection {
        Protection(0)
    }
}
#[doc = "RAM 1 control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram1ctl(pub u32);
impl Ram1ctl {
    #[doc = "Arbitration policy (for RAM controller 1): '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub const fn arb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Arbitration policy (for RAM controller 1): '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub fn set_arb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Ram1ctl {
    #[inline(always)]
    fn default() -> Ram1ctl {
        Ram1ctl(0)
    }
}
#[doc = "RAM control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamCtl(pub u32);
impl RamCtl {
    #[doc = "Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub const fn arb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub fn set_arb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for RamCtl {
    #[inline(always)]
    fn default() -> RamCtl {
        RamCtl(0)
    }
}
#[doc = "ROM control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RomCtl(pub u32);
impl RomCtl {
    #[doc = "Amount of ROM wait states: '0': 0 wait states. Use this setting for newer, faster ROM design. Use this setting for older, slower ROM design and frequencies in the range \\[0, 24\\] MHz. '1': 1 wait state. Use this setting for older, slower ROM design and frequencies in the range <24, 48\\] MHz. CPUSSv2 supports two types of ROM memory: an older, slower design (operating at up to 24 MHz) and a newer, faster design (operating at up to 48 MHz). The older design requires 1 wait state for frequencies above 24 MHz. The newer design never requires wait states. All chips after Street Fighter will use the newer design. As a result, all chips after Street Fighter can always use 0 wait states."]
    #[inline(always)]
    pub const fn rom_ws(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Amount of ROM wait states: '0': 0 wait states. Use this setting for newer, faster ROM design. Use this setting for older, slower ROM design and frequencies in the range \\[0, 24\\] MHz. '1': 1 wait state. Use this setting for older, slower ROM design and frequencies in the range <24, 48\\] MHz. CPUSSv2 supports two types of ROM memory: an older, slower design (operating at up to 24 MHz) and a newer, faster design (operating at up to 48 MHz). The older design requires 1 wait state for frequencies above 24 MHz. The newer design never requires wait states. All chips after Street Fighter will use the newer design. As a result, all chips after Street Fighter can always use 0 wait states."]
    #[inline(always)]
    pub fn set_rom_ws(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RomCtl {
    #[inline(always)]
    fn default() -> RomCtl {
        RomCtl(0)
    }
}
#[doc = "Slave control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlCtl(pub u32);
impl SlCtl {
    #[doc = "Arbitration policy: '0': CPU priority '1': DMA priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub const fn arb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Arbitration policy: '0': CPU priority '1': DMA priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub fn set_arb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for SlCtl {
    #[inline(always)]
    fn default() -> SlCtl {
        SlCtl(0)
    }
}
#[doc = "SYSCALL control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysreq(pub u32);
impl Sysreq {
    #[doc = "Opcode of the system call being requested."]
    #[inline(always)]
    pub const fn syscall_command(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Opcode of the system call being requested."]
    #[inline(always)]
    pub fn set_syscall_command(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Disable Reset Vector fetch relocation: '0': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are redirected to ROM. '1': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are made to flash. Note that this field defaults to '0' on reset, ensuring actual reset vector fetches are always made to ROM. Note that this field does not affect DAP accesses. Flash DfT routines may set this bit to '1' to enable uninhibited read-back of programmed data in the first flash page."]
    #[inline(always)]
    pub const fn dis_reset_vect_rel(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Reset Vector fetch relocation: '0': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are redirected to ROM. '1': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are made to flash. Note that this field defaults to '0' on reset, ensuring actual reset vector fetches are always made to ROM. Note that this field does not affect DAP accesses. Flash DfT routines may set this bit to '1' to enable uninhibited read-back of programmed data in the first flash page."]
    #[inline(always)]
    pub fn set_dis_reset_vect_rel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Indicates whether the system is in privileged ('1') or user mode ('0'). Only CPU SW executing from ROM can set this field to '1' when ROM_ACCESS_EN is '1' (the CPU is executing a SystemCall NMI interrupt handler). Any other write to this field sets is to '0'. This field is used as the AHB-Lite hprot\\[1\\] signal to implement Cypress proprietary user/privileged modes. These modes are used to enable/disable access to specific MMIO registers and memory regions."]
    #[inline(always)]
    pub const fn privileged(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the system is in privileged ('1') or user mode ('0'). Only CPU SW executing from ROM can set this field to '1' when ROM_ACCESS_EN is '1' (the CPU is executing a SystemCall NMI interrupt handler). Any other write to this field sets is to '0'. This field is used as the AHB-Lite hprot\\[1\\] signal to implement Cypress proprietary user/privileged modes. These modes are used to enable/disable access to specific MMIO registers and memory regions."]
    #[inline(always)]
    pub fn set_privileged(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Indicates that executing from Boot ROM is enabled. HW sets this field to '1', on reset or when the SystemCall NMI vector is fetched from Boot ROM. HW sets this field to '0', when the CPU is NOT executing from either Boot or System ROM. This bit is used for debug purposes only."]
    #[inline(always)]
    pub const fn rom_access_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that executing from Boot ROM is enabled. HW sets this field to '1', on reset or when the SystemCall NMI vector is fetched from Boot ROM. HW sets this field to '0', when the CPU is NOT executing from either Boot or System ROM. This bit is used for debug purposes only."]
    #[inline(always)]
    pub fn set_rom_access_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Indicates the source of the write access to the SYSREQ register. '0': CPU write access. '1': DAP write access. For a SW write to the SYSREQ register, the HW will update this field based on the value of SYSCALL_REQ. SYSCALL_REQ = 0 : the current source of write access is captured. SYSCALL_REQ = 1 : the previous value (captured when SYSCALL_REQ was 0), is retained/maintained until the next SW write to this register."]
    #[inline(always)]
    pub const fn hmaster_0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the source of the write access to the SYSREQ register. '0': CPU write access. '1': DAP write access. For a SW write to the SYSREQ register, the HW will update this field based on the value of SYSCALL_REQ. SYSCALL_REQ = 0 : the current source of write access is captured. SYSCALL_REQ = 1 : the previous value (captured when SYSCALL_REQ was 0), is retained/maintained until the next SW write to this register."]
    #[inline(always)]
    pub fn set_hmaster_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "CPU/DAP writes a '1' to this field to request a SystemCall. The HMASTER_0 field indicates the source of the write access. Setting this field to '1' immediate results in a NMI. The SystemCall NMI interrupt handler sets this field to '0' after servicing the request."]
    #[inline(always)]
    pub const fn syscall_req(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "CPU/DAP writes a '1' to this field to request a SystemCall. The HMASTER_0 field indicates the source of the write access. Setting this field to '1' immediate results in a NMI. The SystemCall NMI interrupt handler sets this field to '0' after servicing the request."]
    #[inline(always)]
    pub fn set_syscall_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sysreq {
    #[inline(always)]
    fn default() -> Sysreq {
        Sysreq(0)
    }
}
#[doc = "Wounding register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wounding(pub u32);
impl Wounding {
    #[doc = "Indicates the amount of accessible RAM 0 memory capacity in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of SRAM is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible"]
    #[inline(always)]
    pub const fn ram_wound(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Indicates the amount of accessible RAM 0 memory capacity in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of SRAM is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible"]
    #[inline(always)]
    pub fn set_ram_wound(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Indicates the amount of accessible flash in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of flash is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible (used for the DEAD protection mode)"]
    #[inline(always)]
    pub const fn flash_wound(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Indicates the amount of accessible flash in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of flash is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible (used for the DEAD protection mode)"]
    #[inline(always)]
    pub fn set_flash_wound(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Wounding of RAM 1 (see description of RAM_WOUND)."]
    #[inline(always)]
    pub const fn ram1_wound(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Wounding of RAM 1 (see description of RAM_WOUND)."]
    #[inline(always)]
    pub fn set_ram1_wound(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Wounding {
    #[inline(always)]
    fn default() -> Wounding {
        Wounding(0)
    }
}
