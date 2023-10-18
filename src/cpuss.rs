#[doc = "CPU Subsystem"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuss {
    ptr: *mut u8,
}
unsafe impl Send for Cpuss {}
unsafe impl Sync for Cpuss {}
impl Cpuss {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "SYSCALL control register"]
    #[inline(always)]
    pub const fn sysreq(self) -> crate::common::Reg<regs::Sysreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "SYSARG control register"]
    #[inline(always)]
    pub const fn sysarg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Protection control register"]
    #[inline(always)]
    pub const fn protection(self) -> crate::common::Reg<regs::Protection, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "ROM privilege register"]
    #[inline(always)]
    pub const fn priv_rom(self) -> crate::common::Reg<regs::PrivRom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "RAM privilege register"]
    #[inline(always)]
    pub const fn priv_ram(self) -> crate::common::Reg<regs::PrivRam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Flash privilege register"]
    #[inline(always)]
    pub const fn priv_flash(self) -> crate::common::Reg<regs::PrivFlash, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Wounding register"]
    #[inline(always)]
    pub const fn wounding(self) -> crate::common::Reg<regs::Wounding, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Interrupt multiplexer select register"]
    #[inline(always)]
    pub const fn int_sel(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "DSI interrupt pulse mode register"]
    #[inline(always)]
    pub const fn int_mode(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "DSI NMI pulse mode register"]
    #[inline(always)]
    pub const fn nmi_mode(self) -> crate::common::Reg<regs::NmiMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "FLASH control register"]
    #[inline(always)]
    pub const fn flash_ctl(self) -> crate::common::Reg<regs::FlashCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "ROM control register"]
    #[inline(always)]
    pub const fn rom_ctl(self) -> crate::common::Reg<regs::RomCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "RAM control register"]
    #[inline(always)]
    pub const fn ram_ctl(self) -> crate::common::Reg<regs::RamCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "DMA controller register"]
    #[inline(always)]
    pub const fn dmac_ctl(self) -> crate::common::Reg<regs::DmacCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "RAM 1 privilege register"]
    #[inline(always)]
    pub const fn priv_ram1(self) -> crate::common::Reg<regs::PrivRam1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "RAM 1 control register"]
    #[inline(always)]
    pub const fn ram1_ctl(self) -> crate::common::Reg<regs::Ram1ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "MTB control register"]
    #[inline(always)]
    pub const fn mtb_ctl(self) -> crate::common::Reg<regs::MtbCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "Slave control register"]
    #[inline(always)]
    pub const fn sl_ctl(self, n: usize) -> crate::common::Reg<regs::SlCtl, crate::common::RW> {
        assert!(n < 24usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
}
pub mod regs;
