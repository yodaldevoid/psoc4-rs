#[doc = "Serial Communications Block (SPI/UART/I2C)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scb {
    ptr: *mut u8,
}
unsafe impl Send for Scb {}
unsafe impl Sync for Scb {}
impl Scb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Generic control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Generic status register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Command/response control register."]
    #[inline(always)]
    pub const fn cmd_resp_ctrl(self) -> crate::common::Reg<regs::CmdRespCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Command/response status register."]
    #[inline(always)]
    pub const fn cmd_resp_status(
        self,
    ) -> crate::common::Reg<regs::CmdRespStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "SPI control register."]
    #[inline(always)]
    pub const fn spi_ctrl(self) -> crate::common::Reg<regs::SpiCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "SPI status register."]
    #[inline(always)]
    pub const fn spi_status(self) -> crate::common::Reg<regs::SpiStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "UART control register."]
    #[inline(always)]
    pub const fn uart_ctrl(self) -> crate::common::Reg<regs::UartCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "UART transmitter control register."]
    #[inline(always)]
    pub const fn uart_tx_ctrl(self) -> crate::common::Reg<regs::UartTxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "UART receiver control register."]
    #[inline(always)]
    pub const fn uart_rx_ctrl(self) -> crate::common::Reg<regs::UartRxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "UART receiver status register."]
    #[inline(always)]
    pub const fn uart_rx_status(self) -> crate::common::Reg<regs::UartRxStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "UART flow control register"]
    #[inline(always)]
    pub const fn uart_flow_ctrl(self) -> crate::common::Reg<regs::UartFlowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "I2C control register."]
    #[inline(always)]
    pub const fn i2c_ctrl(self) -> crate::common::Reg<regs::I2cCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "I2C status register."]
    #[inline(always)]
    pub const fn i2c_status(self) -> crate::common::Reg<regs::I2cStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "I2C master command register."]
    #[inline(always)]
    pub const fn i2c_m_cmd(self) -> crate::common::Reg<regs::I2cMcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "I2C slave command register."]
    #[inline(always)]
    pub const fn i2c_s_cmd(self) -> crate::common::Reg<regs::I2cScmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "I2C configuration register."]
    #[inline(always)]
    pub const fn i2c_cfg(self) -> crate::common::Reg<regs::I2cCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "Transmitter control register."]
    #[inline(always)]
    pub const fn tx_ctrl(self) -> crate::common::Reg<regs::TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Transmitter FIFO control register."]
    #[inline(always)]
    pub const fn tx_fifo_ctrl(self) -> crate::common::Reg<regs::TxFifoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "Transmitter FIFO status register."]
    #[inline(always)]
    pub const fn tx_fifo_status(self) -> crate::common::Reg<regs::TxFifoStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[doc = "Transmitter FIFO write register."]
    #[inline(always)]
    pub const fn tx_fifo_wr(self) -> crate::common::Reg<regs::TxFifoWr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(576usize) as _) }
    }
    #[doc = "Receiver control register."]
    #[inline(always)]
    pub const fn rx_ctrl(self) -> crate::common::Reg<regs::RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize) as _) }
    }
    #[doc = "Receiver FIFO control register."]
    #[inline(always)]
    pub const fn rx_fifo_ctrl(self) -> crate::common::Reg<regs::RxFifoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(772usize) as _) }
    }
    #[doc = "Receiver FIFO status register."]
    #[inline(always)]
    pub const fn rx_fifo_status(self) -> crate::common::Reg<regs::RxFifoStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(776usize) as _) }
    }
    #[doc = "Slave address and mask register."]
    #[inline(always)]
    pub const fn rx_match(self) -> crate::common::Reg<regs::RxMatch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(784usize) as _) }
    }
    #[doc = "Receiver FIFO read register."]
    #[inline(always)]
    pub const fn rx_fifo_rd(self) -> crate::common::Reg<regs::RxFifoRd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(832usize) as _) }
    }
    #[doc = "Receiver FIFO read register."]
    #[inline(always)]
    pub const fn rx_fifo_rd_silent(
        self,
    ) -> crate::common::Reg<regs::RxFifoRdSilent, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(836usize) as _) }
    }
    #[doc = "Active clocked interrupt signal register"]
    #[inline(always)]
    pub const fn intr_cause(self) -> crate::common::Reg<regs::IntrCause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3584usize) as _) }
    }
    #[doc = "Externally clocked I2C interrupt request register"]
    #[inline(always)]
    pub const fn intr_i2c_ec(self) -> crate::common::Reg<regs::IntrI2cEc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3712usize) as _) }
    }
    #[doc = "Externally clocked I2C interrupt mask register"]
    #[inline(always)]
    pub const fn intr_i2c_ec_mask(
        self,
    ) -> crate::common::Reg<regs::IntrI2cEcMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3720usize) as _) }
    }
    #[doc = "Externally clocked I2C interrupt masked register"]
    #[inline(always)]
    pub const fn intr_i2c_ec_masked(
        self,
    ) -> crate::common::Reg<regs::IntrI2cEcMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3724usize) as _) }
    }
    #[doc = "Externally clocked SPI interrupt request register"]
    #[inline(always)]
    pub const fn intr_spi_ec(self) -> crate::common::Reg<regs::IntrSpiEc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3776usize) as _) }
    }
    #[doc = "Externally clocked SPI interrupt mask register"]
    #[inline(always)]
    pub const fn intr_spi_ec_mask(
        self,
    ) -> crate::common::Reg<regs::IntrSpiEcMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3784usize) as _) }
    }
    #[doc = "Externally clocked SPI interrupt masked register"]
    #[inline(always)]
    pub const fn intr_spi_ec_masked(
        self,
    ) -> crate::common::Reg<regs::IntrSpiEcMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3788usize) as _) }
    }
    #[doc = "Master interrupt request register."]
    #[inline(always)]
    pub const fn intr_m(self) -> crate::common::Reg<regs::IntrM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3840usize) as _) }
    }
    #[doc = "Master interrupt set request register"]
    #[inline(always)]
    pub const fn intr_m_set(self) -> crate::common::Reg<regs::IntrMset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3844usize) as _) }
    }
    #[doc = "Master interrupt mask register."]
    #[inline(always)]
    pub const fn intr_m_mask(self) -> crate::common::Reg<regs::IntrMmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3848usize) as _) }
    }
    #[doc = "Master interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_m_masked(self) -> crate::common::Reg<regs::IntrMmasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3852usize) as _) }
    }
    #[doc = "Slave interrupt request register."]
    #[inline(always)]
    pub const fn intr_s(self) -> crate::common::Reg<regs::IntrS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3904usize) as _) }
    }
    #[doc = "Slave interrupt set request register."]
    #[inline(always)]
    pub const fn intr_s_set(self) -> crate::common::Reg<regs::IntrSset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3908usize) as _) }
    }
    #[doc = "Slave interrupt mask register."]
    #[inline(always)]
    pub const fn intr_s_mask(self) -> crate::common::Reg<regs::IntrSmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3912usize) as _) }
    }
    #[doc = "Slave interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_s_masked(self) -> crate::common::Reg<regs::IntrSmasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3916usize) as _) }
    }
    #[doc = "Transmitter interrupt request register."]
    #[inline(always)]
    pub const fn intr_tx(self) -> crate::common::Reg<regs::IntrTx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3968usize) as _) }
    }
    #[doc = "Transmitter interrupt set request register"]
    #[inline(always)]
    pub const fn intr_tx_set(self) -> crate::common::Reg<regs::IntrTxSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3972usize) as _) }
    }
    #[doc = "Transmitter interrupt mask register."]
    #[inline(always)]
    pub const fn intr_tx_mask(self) -> crate::common::Reg<regs::IntrTxMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3976usize) as _) }
    }
    #[doc = "Transmitter interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_tx_masked(self) -> crate::common::Reg<regs::IntrTxMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3980usize) as _) }
    }
    #[doc = "Receiver interrupt request register."]
    #[inline(always)]
    pub const fn intr_rx(self) -> crate::common::Reg<regs::IntrRx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4032usize) as _) }
    }
    #[doc = "Receiver interrupt set request register."]
    #[inline(always)]
    pub const fn intr_rx_set(self) -> crate::common::Reg<regs::IntrRxSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4036usize) as _) }
    }
    #[doc = "Receiver interrupt mask register."]
    #[inline(always)]
    pub const fn intr_rx_mask(self) -> crate::common::Reg<regs::IntrRxMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4040usize) as _) }
    }
    #[doc = "Receiver interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_rx_masked(self) -> crate::common::Reg<regs::IntrRxMasked, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4044usize) as _) }
    }
}
pub mod regs;
pub mod vals;
