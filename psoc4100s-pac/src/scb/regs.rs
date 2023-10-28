#[doc = "Command/response control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRespCtrl(pub u32);
impl CmdRespCtrl {
    #[doc = "I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub const fn base_rd_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn set_base_rd_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "I2C/SPI write base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub const fn base_wr_addr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "I2C/SPI write base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn set_base_wr_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
}
impl Default for CmdRespCtrl {
    #[inline(always)]
    fn default() -> CmdRespCtrl {
        CmdRespCtrl(0)
    }
}
#[doc = "Command/response status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRespStatus(pub u32);
impl CmdRespStatus {
    #[doc = "I2C/SPI read current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximum memory buffer address). The field is used to determine how many bytes have been read (# bytes = CURR_RD_ADDR - CMD_RESP_CTRL.BASE_RD_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub const fn curr_rd_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "I2C/SPI read current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximum memory buffer address). The field is used to determine how many bytes have been read (# bytes = CURR_RD_ADDR - CMD_RESP_CTRL.BASE_RD_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub fn set_curr_rd_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "I2C/SPI write current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximum memory buffer address). The field is used to determine how many bytes have been written (# bytes = CURR_WR_ADDR - CMD_RESP_CTRL.BASE_WR_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub const fn curr_wr_addr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "I2C/SPI write current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximum memory buffer address). The field is used to determine how many bytes have been written (# bytes = CURR_WR_ADDR - CMD_RESP_CTRL.BASE_WR_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub fn set_curr_wr_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Indicates whether there is an ongoing bus transfer to the IP. '0': no ongoing bus transfer. '1': ongoing bus transfer. For SPI, the field is '1' when the slave is selected. For I2C, the field is set to '1' at a I2C START/RESTART. In case of an address match, the field is set to '0' on a I2C STOP. In case of NO address match, the field is set to '0' after the failing address match."]
    #[inline(always)]
    pub const fn cmd_resp_ec_bus_busy(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether there is an ongoing bus transfer to the IP. '0': no ongoing bus transfer. '1': ongoing bus transfer. For SPI, the field is '1' when the slave is selected. For I2C, the field is set to '1' at a I2C START/RESTART. In case of an address match, the field is set to '0' on a I2C STOP. In case of NO address match, the field is set to '0' after the failing address match."]
    #[inline(always)]
    pub fn set_cmd_resp_ec_bus_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Indicates whether the CURR_RD_ADDR and CURR_WR_ADDR fields in this register are reliable (when CMD_RESP_EC_BUSY is '0') or not reliable (when CMD_RESP_EC_BUSY is '1'). Note: - When there is no ongoing bus transfer, CMD_RESP_EC_BUSY is '0' (reliable). - When there is a ongoing bus transfer, CMD_RESP_EC_BUSY is '0' (reliable), when the CURR_RD_ADDR and CURR_WR_ADDR are not being updated by the HW. - When there is a ongoing bus transfer, CMD_RESP_EC_BUSY is '1' (not reliable), when the CURR_RD_ADDR or CURR_WR_ADDR are being updated by the HW. Note that this update lasts one I2C clock cycle, or two SPI clock cycles."]
    #[inline(always)]
    pub const fn cmd_resp_ec_busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the CURR_RD_ADDR and CURR_WR_ADDR fields in this register are reliable (when CMD_RESP_EC_BUSY is '0') or not reliable (when CMD_RESP_EC_BUSY is '1'). Note: - When there is no ongoing bus transfer, CMD_RESP_EC_BUSY is '0' (reliable). - When there is a ongoing bus transfer, CMD_RESP_EC_BUSY is '0' (reliable), when the CURR_RD_ADDR and CURR_WR_ADDR are not being updated by the HW. - When there is a ongoing bus transfer, CMD_RESP_EC_BUSY is '1' (not reliable), when the CURR_RD_ADDR or CURR_WR_ADDR are being updated by the HW. Note that this update lasts one I2C clock cycle, or two SPI clock cycles."]
    #[inline(always)]
    pub fn set_cmd_resp_ec_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CmdRespStatus {
    #[inline(always)]
    fn default() -> CmdRespStatus {
        CmdRespStatus(0)
    }
}
#[doc = "Generic control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn ovs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_ovs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    pub const fn ec_am_mode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn set_ec_am_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    pub const fn ec_op_mode(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn set_ec_op_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    pub const fn ez_mode(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn set_ez_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    pub const fn byte_mode(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    pub fn set_byte_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    pub const fn cmd_resp_mode(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    pub fn set_cmd_resp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    pub const fn addr_accept(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    pub fn set_addr_accept(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    pub const fn block(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    pub fn set_block(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::CtrlMode {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::CtrlMode::from_bits(val as u8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::CtrlMode) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
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
#[doc = "I2C configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2cCfg(pub u32);
impl I2cCfg {
    #[doc = "Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub const fn sda_in_filt_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn set_sda_in_filt_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub const fn sda_in_filt_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn set_sda_in_filt_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. With s8iom0s8v1p2 I/Os, trim bits should be programmed to 3 to suppress glitches below 50ns."]
    #[inline(always)]
    pub const fn scl_in_filt_trim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. With s8iom0s8v1p2 I/Os, trim bits should be programmed to 3 to suppress glitches below 50ns."]
    #[inline(always)]
    pub fn set_scl_in_filt_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub const fn scl_in_filt_sel(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn set_scl_in_filt_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub const fn sda_out_filt0_trim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn set_sda_out_filt0_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub const fn sda_out_filt1_trim(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn set_sda_out_filt1_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub const fn sda_out_filt2_trim(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn set_sda_out_filt2_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
    #[inline(always)]
    pub const fn sda_out_filt_sel(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
    #[inline(always)]
    pub fn set_sda_out_filt_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for I2cCfg {
    #[inline(always)]
    fn default() -> I2cCfg {
        I2cCfg(0)
    }
}
#[doc = "I2C control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2cCtrl(pub u32);
impl I2cCtrl {
    #[doc = "Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\] with input signal median filtering and \\[4, 15\\] without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and <= 16 IP clock cycles."]
    #[inline(always)]
    pub const fn high_phase_ovs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\] with input signal median filtering and \\[4, 15\\] without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and <= 16 IP clock cycles."]
    #[inline(always)]
    pub fn set_high_phase_ovs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\] with input signal median filtering and \\[6, 15\\] without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and <= 16 IP clock cycles."]
    #[inline(always)]
    pub const fn low_phase_ovs(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\] with input signal median filtering and \\[6, 15\\] without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and <= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and <= 16 IP clock cycles."]
    #[inline(always)]
    pub fn set_low_phase_ovs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
    #[inline(always)]
    pub const fn m_ready_data_ack(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
    #[inline(always)]
    pub fn set_m_ready_data_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub const fn m_not_ready_data_nack(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn set_m_not_ready_data_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
    #[inline(always)]
    pub const fn s_general_ignore(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
    #[inline(always)]
    pub fn set_s_general_ignore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub const fn s_ready_addr_ack(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub fn set_s_ready_addr_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub const fn s_ready_data_ack(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub fn set_s_ready_data_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
    #[inline(always)]
    pub const fn s_not_ready_addr_nack(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
    #[inline(always)]
    pub fn set_s_not_ready_addr_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub const fn s_not_ready_data_nack(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn set_s_not_ready_data_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    pub const fn loopback(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    pub fn set_loopback(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave mode enabled ('1') or not ('0')."]
    #[inline(always)]
    pub const fn slave_mode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Slave mode enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn set_slave_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
    #[inline(always)]
    pub const fn master_mode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
    #[inline(always)]
    pub fn set_master_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I2cCtrl {
    #[inline(always)]
    fn default() -> I2cCtrl {
        I2cCtrl(0)
    }
}
#[doc = "I2C master command register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2cMcmd(pub u32);
impl I2cMcmd {
    #[doc = "When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub const fn m_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn set_m_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub const fn m_start_on_idle(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn set_m_start_on_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub const fn m_ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn set_m_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub const fn m_nack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn set_m_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. I2C_M_CMD.M_START has a higher priority than this command: in situations where both a STOP and a REPEATED START could be transmitted, M_START takes precedence over M_STOP."]
    #[inline(always)]
    pub const fn m_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. I2C_M_CMD.M_START has a higher priority than this command: in situations where both a STOP and a REPEATED START could be transmitted, M_START takes precedence over M_STOP."]
    #[inline(always)]
    pub fn set_m_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for I2cMcmd {
    #[inline(always)]
    fn default() -> I2cMcmd {
        I2cMcmd(0)
    }
}
#[doc = "I2C slave command register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2cScmd(pub u32);
impl I2cScmd {
    #[doc = "When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode)."]
    #[inline(always)]
    pub const fn s_ack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode)."]
    #[inline(always)]
    pub fn set_s_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    pub const fn s_nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    pub fn set_s_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for I2cScmd {
    #[inline(always)]
    fn default() -> I2cScmd {
        I2cScmd(0)
    }
}
#[doc = "I2C status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2cStatus(pub u32);
impl I2cStatus {
    #[doc = "I2C bus is busy. The bus is considered busy ('1'), from the time a START is detected or from the time the SCL line is '0'. The bus is considered idle ('0'), from the time a STOP is detected. If the IP is disabled, BUS_BUSY is '0'. After enabling the IP, it takes time for the BUS_BUSY to detect a busy bus. This time is the maximum high time of the SCL line. For a 100 kHz interface frequency, this maximum high time may last roughly 5 us (half a bit period). For single master systems, BUS_BUSY does not have to be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START (no bus collisions). For multi-master systems, BUS_BUSY can be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START_ON_IDLE (to prevent bus collisions)."]
    #[inline(always)]
    pub const fn bus_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C bus is busy. The bus is considered busy ('1'), from the time a START is detected or from the time the SCL line is '0'. The bus is considered idle ('0'), from the time a STOP is detected. If the IP is disabled, BUS_BUSY is '0'. After enabling the IP, it takes time for the BUS_BUSY to detect a busy bus. This time is the maximum high time of the SCL line. For a 100 kHz interface frequency, this maximum high time may last roughly 5 us (half a bit period). For single master systems, BUS_BUSY does not have to be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START (no bus collisions). For multi-master systems, BUS_BUSY can be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START_ON_IDLE (to prevent bus collisions)."]
    #[inline(always)]
    pub fn set_bus_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_EZ_ADDR or CURR_EZ_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_EZ_ADDR and CURR_EZ_ADDR are reliable."]
    #[inline(always)]
    pub const fn i2c_ec_busy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_EZ_ADDR or CURR_EZ_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_EZ_ADDR and CURR_EZ_ADDR are reliable."]
    #[inline(always)]
    pub fn set_i2c_ec_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C slave read transfer ('1') or I2C slave write transfer ('0'). When the I2C slave is inactive/idle or receiving START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub const fn s_read(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C slave read transfer ('1') or I2C slave write transfer ('0'). When the I2C slave is inactive/idle or receiving START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub fn set_s_read(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "I2C master read transfer ('1') or I2C master write transfer ('0'). When the I2C master is inactive/idle or transmitting START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub const fn m_read(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "I2C master read transfer ('1') or I2C master write transfer ('0'). When the I2C master is inactive/idle or transmitting START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub fn set_m_read(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C slave current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when I2C_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub const fn curr_ez_addr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "I2C slave current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when I2C_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn set_curr_ez_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "I2C slave base EZ address. Address as provided by an I2C write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub const fn base_ez_addr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "I2C slave base EZ address. Address as provided by an I2C write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn set_base_ez_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for I2cStatus {
    #[inline(always)]
    fn default() -> I2cStatus {
        I2cStatus(0)
    }
}
#[doc = "Active clocked interrupt signal register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrCause(pub u32);
impl IntrCause {
    #[doc = "Master interrupt active ('interrupt_master'): INTR_M_MASKED != 0."]
    #[inline(always)]
    pub const fn m(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master interrupt active ('interrupt_master'): INTR_M_MASKED != 0."]
    #[inline(always)]
    pub fn set_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave interrupt active ('interrupt_slave'): INTR_S_MASKED != 0."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Slave interrupt active ('interrupt_slave'): INTR_S_MASKED != 0."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter interrupt active ('interrupt_tx'): INTR_TX_MASKED != 0."]
    #[inline(always)]
    pub const fn tx(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter interrupt active ('interrupt_tx'): INTR_TX_MASKED != 0."]
    #[inline(always)]
    pub fn set_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receiver interrupt active ('interrupt_rx'): INTR_RX_MASKED != 0."]
    #[inline(always)]
    pub const fn rx(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver interrupt active ('interrupt_rx'): INTR_RX_MASKED != 0."]
    #[inline(always)]
    pub fn set_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Externally clock I2C interrupt active ('interrupt_i2c_ec'): INTR_I2C_EC_MASKED != 0."]
    #[inline(always)]
    pub const fn i2c_ec(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Externally clock I2C interrupt active ('interrupt_i2c_ec'): INTR_I2C_EC_MASKED != 0."]
    #[inline(always)]
    pub fn set_i2c_ec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Externally clocked SPI interrupt active ('interrupt_spi_ec'): INTR_SPI_EC_MASKED != 0."]
    #[inline(always)]
    pub const fn spi_ec(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Externally clocked SPI interrupt active ('interrupt_spi_ec'): INTR_SPI_EC_MASKED != 0."]
    #[inline(always)]
    pub fn set_spi_ec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for IntrCause {
    #[inline(always)]
    fn default() -> IntrCause {
        IntrCause(0)
    }
}
#[doc = "Externally clocked I2C interrupt request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrI2cEc(pub u32);
impl IntrI2cEc {
    #[doc = "Wake up request. Active on incoming slave request (with address match). Only used when EC_AM is '1'."]
    #[inline(always)]
    pub const fn wake_up(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake up request. Active on incoming slave request (with address match). Only used when EC_AM is '1'."]
    #[inline(always)]
    pub fn set_wake_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "STOP detection. Activated on the end of a every transfer (I2C STOP). Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub const fn ez_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection. Activated on the end of a every transfer (I2C STOP). Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn set_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub const fn ez_write_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn set_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub const fn ez_read_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn set_ez_read_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntrI2cEc {
    #[inline(always)]
    fn default() -> IntrI2cEc {
        IntrI2cEc(0)
    }
}
#[doc = "Externally clocked I2C interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrI2cEcMask(pub u32);
impl IntrI2cEcMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn wake_up(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_wake_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn ez_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn ez_write_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn ez_read_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_ez_read_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntrI2cEcMask {
    #[inline(always)]
    fn default() -> IntrI2cEcMask {
        IntrI2cEcMask(0)
    }
}
#[doc = "Externally clocked I2C interrupt masked register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrI2cEcMasked(pub u32);
impl IntrI2cEcMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn wake_up(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_wake_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn ez_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn ez_write_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn ez_read_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_ez_read_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntrI2cEcMasked {
    #[inline(always)]
    fn default() -> IntrI2cEcMasked {
        IntrI2cEcMasked(0)
    }
}
#[doc = "Master interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrM(pub u32);
impl IntrM {
    #[doc = "I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
    #[inline(always)]
    pub const fn i2c_arb_lost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
    #[inline(always)]
    pub fn set_i2c_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub const fn i2c_nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn set_i2c_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub const fn i2c_ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn set_i2c_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C master STOP. Set to '1', when the master has transmitted a STOP."]
    #[inline(always)]
    pub const fn i2c_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C master STOP. Set to '1', when the master has transmitted a STOP."]
    #[inline(always)]
    pub fn set_i2c_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "I2C master bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    pub const fn i2c_bus_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "I2C master bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    pub fn set_i2c_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SPI master transfer done event: all data frames in the transmit FIFO are sent and the transmit FIFO and shift register are empty."]
    #[inline(always)]
    pub const fn spi_done(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SPI master transfer done event: all data frames in the transmit FIFO are sent and the transmit FIFO and shift register are empty."]
    #[inline(always)]
    pub fn set_spi_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for IntrM {
    #[inline(always)]
    fn default() -> IntrM {
        IntrM(0)
    }
}
#[doc = "Master interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMmask(pub u32);
impl IntrMmask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_arb_lost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_bus_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn spi_done(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_spi_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for IntrMmask {
    #[inline(always)]
    fn default() -> IntrMmask {
        IntrMmask(0)
    }
}
#[doc = "Master interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMmasked(pub u32);
impl IntrMmasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_arb_lost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_bus_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn spi_done(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_spi_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for IntrMmasked {
    #[inline(always)]
    fn default() -> IntrMmasked {
        IntrMmasked(0)
    }
}
#[doc = "Master interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrMset(pub u32);
impl IntrMset {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_arb_lost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_bus_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn spi_done(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_spi_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for IntrMset {
    #[inline(always)]
    fn default() -> IntrMset {
        IntrMset(0)
    }
}
#[doc = "Receiver interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrRx(pub u32);
impl IntrRx {
    #[doc = "More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in SCB_RX_FIFO_CTL. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "More entries in the RX FIFO than the value specified by TRIGGER_LEVEL in SCB_RX_FIFO_CTL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO is not empty. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn not_empty(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO is not empty. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_not_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub const fn blocked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn set_blocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    pub const fn frame_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    pub fn set_frame_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    pub const fn parity_error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    pub fn set_parity_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn baud_detect(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_baud_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn break_detect(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_break_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for IntrRx {
    #[inline(always)]
    fn default() -> IntrRx {
        IntrRx(0)
    }
}
#[doc = "Receiver interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrRxMask(pub u32);
impl IntrRxMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn not_empty(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_not_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn blocked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_blocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn frame_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_frame_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn parity_error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_parity_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn baud_detect(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_baud_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn break_detect(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_break_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for IntrRxMask {
    #[inline(always)]
    fn default() -> IntrRxMask {
        IntrRxMask(0)
    }
}
#[doc = "Receiver interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrRxMasked(pub u32);
impl IntrRxMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn not_empty(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_not_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn blocked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_blocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn frame_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_frame_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn parity_error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_parity_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn baud_detect(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_baud_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn break_detect(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_break_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for IntrRxMasked {
    #[inline(always)]
    fn default() -> IntrRxMasked {
        IntrRxMasked(0)
    }
}
#[doc = "Receiver interrupt set request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrRxSet(pub u32);
impl IntrRxSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub const fn not_empty(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn set_not_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub const fn blocked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn set_blocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub const fn frame_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn set_frame_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub const fn parity_error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn set_parity_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub const fn baud_detect(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn set_baud_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub const fn break_detect(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt status register."]
    #[inline(always)]
    pub fn set_break_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for IntrRxSet {
    #[inline(always)]
    fn default() -> IntrRxSet {
        IntrRxSet(0)
    }
}
#[doc = "Slave interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrS(pub u32);
impl IntrS {
    #[doc = "I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub const fn i2c_arb_lost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn set_i2c_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub const fn i2c_nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn set_i2c_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub const fn i2c_ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn set_i2c_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
    #[inline(always)]
    pub const fn i2c_write_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
    #[inline(always)]
    pub fn set_i2c_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
    #[inline(always)]
    pub const fn i2c_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
    #[inline(always)]
    pub fn set_i2c_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    pub const fn i2c_start(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    pub fn set_i2c_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub const fn i2c_addr_match(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn set_i2c_addr_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub const fn i2c_general(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn set_i2c_general(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub const fn i2c_bus_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn set_i2c_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SPI slave deselected after a write EZ SPI transfer occurred."]
    #[inline(always)]
    pub const fn spi_ez_write_stop(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SPI slave deselected after a write EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn set_spi_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SPI slave deselected after any EZ SPI transfer occurred."]
    #[inline(always)]
    pub const fn spi_ez_stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SPI slave deselected after any EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn set_spi_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub const fn spi_bus_error(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn set_spi_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for IntrS {
    #[inline(always)]
    fn default() -> IntrS {
        IntrS(0)
    }
}
#[doc = "Slave interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSmask(pub u32);
impl IntrSmask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_arb_lost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_write_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_start(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_addr_match(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_addr_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_general(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_general(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_bus_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn spi_ez_write_stop(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_spi_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn spi_ez_stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_spi_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn spi_bus_error(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_spi_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for IntrSmask {
    #[inline(always)]
    fn default() -> IntrSmask {
        IntrSmask(0)
    }
}
#[doc = "Slave interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSmasked(pub u32);
impl IntrSmasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_arb_lost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_write_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_start(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_addr_match(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_addr_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_general(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_general(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn i2c_bus_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_i2c_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn spi_ez_write_stop(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_spi_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn spi_ez_stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_spi_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn spi_bus_error(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_spi_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for IntrSmasked {
    #[inline(always)]
    fn default() -> IntrSmasked {
        IntrSmasked(0)
    }
}
#[doc = "Externally clocked SPI interrupt request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSpiEc(pub u32);
impl IntrSpiEc {
    #[doc = "Wake up request. Active on incoming slave request when externally clocked selection is '1'. Only used when EC_AM is '1'."]
    #[inline(always)]
    pub const fn wake_up(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake up request. Active on incoming slave request when externally clocked selection is '1'. Only used when EC_AM is '1'."]
    #[inline(always)]
    pub fn set_wake_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "STOP detection. Activated on the end of a every transfer (SPI deselection). Only available in EZ and CMD_RESP mode and when EC_OP is '1'."]
    #[inline(always)]
    pub const fn ez_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection. Activated on the end of a every transfer (SPI deselection). Only available in EZ and CMD_RESP mode and when EC_OP is '1'."]
    #[inline(always)]
    pub fn set_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    pub const fn ez_write_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    pub fn set_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    pub const fn ez_read_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    pub fn set_ez_read_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntrSpiEc {
    #[inline(always)]
    fn default() -> IntrSpiEc {
        IntrSpiEc(0)
    }
}
#[doc = "Externally clocked SPI interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSpiEcMask(pub u32);
impl IntrSpiEcMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn wake_up(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_wake_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn ez_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn ez_write_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn ez_read_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_ez_read_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntrSpiEcMask {
    #[inline(always)]
    fn default() -> IntrSpiEcMask {
        IntrSpiEcMask(0)
    }
}
#[doc = "Externally clocked SPI interrupt masked register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSpiEcMasked(pub u32);
impl IntrSpiEcMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn wake_up(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_wake_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn ez_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn ez_write_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn ez_read_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_ez_read_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntrSpiEcMasked {
    #[inline(always)]
    fn default() -> IntrSpiEcMasked {
        IntrSpiEcMasked(0)
    }
}
#[doc = "Slave interrupt set request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrSset(pub u32);
impl IntrSset {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_arb_lost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_ack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_write_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_start(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_addr_match(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_addr_match(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_general(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_general(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn i2c_bus_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_i2c_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn spi_ez_write_stop(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_spi_ez_write_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn spi_ez_stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_spi_ez_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn spi_bus_error(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_spi_bus_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for IntrSset {
    #[inline(always)]
    fn default() -> IntrSset {
        IntrSset(0)
    }
}
#[doc = "Transmitter interrupt request register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrTx(pub u32);
impl IntrTx {
    #[doc = "Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX FIFO is not full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn not_full(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO is not full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_not_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Attempt to write to a full TX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to write to a full TX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub const fn blocked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn set_blocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn uart_nack(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_uart_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO; i.e. EMPTY is '1'. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn uart_done(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO; i.e. EMPTY is '1'. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_uart_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is useful when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub const fn uart_arb_lost(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is useful when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn set_uart_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for IntrTx {
    #[inline(always)]
    fn default() -> IntrTx {
        IntrTx(0)
    }
}
#[doc = "Transmitter interrupt mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrTxMask(pub u32);
impl IntrTxMask {
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn not_full(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_not_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn blocked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_blocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn uart_nack(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_uart_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn uart_done(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_uart_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn uart_arb_lost(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_uart_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for IntrTxMask {
    #[inline(always)]
    fn default() -> IntrTxMask {
        IntrTxMask(0)
    }
}
#[doc = "Transmitter interrupt masked request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrTxMasked(pub u32);
impl IntrTxMasked {
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn not_full(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_not_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn blocked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_blocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn uart_nack(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_uart_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn uart_done(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_uart_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub const fn uart_arb_lost(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn set_uart_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for IntrTxMasked {
    #[inline(always)]
    fn default() -> IntrTxMasked {
        IntrTxMasked(0)
    }
}
#[doc = "Transmitter interrupt set request register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntrTxSet(pub u32);
impl IntrTxSet {
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn not_full(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_not_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn blocked(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_blocked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn uart_nack(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_uart_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn uart_done(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_uart_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub const fn uart_arb_lost(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn set_uart_arb_lost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for IntrTxSet {
    #[inline(always)]
    fn default() -> IntrTxSet {
        IntrTxSet(0)
    }
}
#[doc = "Receiver control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxCtrl(pub u32);
impl RxCtrl {
    #[doc = "Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    pub const fn data_width(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    pub fn set_data_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub const fn msb_first(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn set_msb_first(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptibility to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
    #[inline(always)]
    pub const fn median(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptibility to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
    #[inline(always)]
    pub fn set_median(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for RxCtrl {
    #[inline(always)]
    fn default() -> RxCtrl {
        RxCtrl(0)
    }
}
#[doc = "Receiver FIFO control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxFifoCtrl(pub u32);
impl RxFifoCtrl {
    #[doc = "Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
    #[inline(always)]
    pub const fn trigger_level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
    #[inline(always)]
    pub fn set_trigger_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
    #[inline(always)]
    pub const fn freeze(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
    #[inline(always)]
    pub fn set_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for RxFifoCtrl {
    #[inline(always)]
    fn default() -> RxFifoCtrl {
        RxFifoCtrl(0)
    }
}
#[doc = "Receiver FIFO read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxFifoRd(pub u32);
impl RxFifoRd {
    #[doc = "Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\] are used. This register has a side effect when read by software: a data frame is removed from the FIFO. This may be undesirable during debug; i.e. a read during debug should NOT have a side effect. To this end, the IP uses the AHB-Lite 'hmaster\\[0\\]' input signal. When this signal is '1' in the address cycle of a bus transfer, a read transfer will not have a side effect. As a result, a read from this register will not remove a data frame from the FIFO. As a result, a read from this register behaves as a read from the SCB_RX_FIFO_RD_SILENT register. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO; i.e. behavior is similar to that of a POP operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\] are used. This register has a side effect when read by software: a data frame is removed from the FIFO. This may be undesirable during debug; i.e. a read during debug should NOT have a side effect. To this end, the IP uses the AHB-Lite 'hmaster\\[0\\]' input signal. When this signal is '1' in the address cycle of a bus transfer, a read transfer will not have a side effect. As a result, a read from this register will not remove a data frame from the FIFO. As a result, a read from this register behaves as a read from the SCB_RX_FIFO_RD_SILENT register. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RxFifoRd {
    #[inline(always)]
    fn default() -> RxFifoRd {
        RxFifoRd(0)
    }
}
#[doc = "Receiver FIFO read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxFifoRdSilent(pub u32);
impl RxFifoRdSilent {
    #[doc = "Data read from the receiver FIFO. Reading a data frame will NOT remove the data frame from the FIFO; i.e. behavior is similar to that of a PEEK operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\] are used. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data read from the receiver FIFO. Reading a data frame will NOT remove the data frame from the FIFO; i.e. behavior is similar to that of a PEEK operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\] are used. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RxFifoRdSilent {
    #[inline(always)]
    fn default() -> RxFifoRdSilent {
        RxFifoRdSilent(0)
    }
}
#[doc = "Receiver FIFO status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxFifoStatus(pub u32);
impl RxFifoStatus {
    #[doc = "Amount of entries in the receiver FIFO. The value of this field ranges from 0 to FF_DATA_NR (EZ_DATA_NR/2)."]
    #[inline(always)]
    pub const fn used(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Amount of entries in the receiver FIFO. The value of this field ranges from 0 to FF_DATA_NR (EZ_DATA_NR/2)."]
    #[inline(always)]
    pub fn set_used(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Indicates whether the RX shift registers holds a (partial) valid data frame ('1') or not ('0'). The shift register can be considered the bottom of the RX FIFO (the data frame is not included in the USED field of the RX FIFO). The shift register is a working register and holds the data frame that is currently being received (when the protocol state machine is receiving a data frame)."]
    #[inline(always)]
    pub const fn sr_valid(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the RX shift registers holds a (partial) valid data frame ('1') or not ('0'). The shift register can be considered the bottom of the RX FIFO (the data frame is not included in the USED field of the RX FIFO). The shift register is a working register and holds the data frame that is currently being received (when the protocol state machine is receiving a data frame)."]
    #[inline(always)]
    pub fn set_sr_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FIFO read pointer: FIFO location from which a data frame is read."]
    #[inline(always)]
    pub const fn rd_ptr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "FIFO read pointer: FIFO location from which a data frame is read."]
    #[inline(always)]
    pub fn set_rd_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "FIFO write pointer: FIFO location at which a new data frame is written by the hardware."]
    #[inline(always)]
    pub const fn wr_ptr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "FIFO write pointer: FIFO location at which a new data frame is written by the hardware."]
    #[inline(always)]
    pub fn set_wr_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for RxFifoStatus {
    #[inline(always)]
    fn default() -> RxFifoStatus {
        RxFifoStatus(0)
    }
}
#[doc = "Slave address and mask register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxMatch(pub u32);
impl RxMatch {
    #[doc = "Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Slave device address. In UART multi-processor mode, all 8 bits are used. In I2C slave mode, only bits 7 down to 1 are used. This reflects the organization of the first transmitted byte in a I2C transfer: the first 7 bits represent the address of the addressed slave, and the last 1 bit is a read/write indicator ('0': write, '1': read)."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR & MASK) == ('slave address' & MASK))."]
    #[inline(always)]
    pub const fn mask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Slave device address mask. This field is a mask that specifies which of the ADDR field bits in the ADDR field take part in the matching of the slave address: MATCH = ((ADDR & MASK) == ('slave address' & MASK))."]
    #[inline(always)]
    pub fn set_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for RxMatch {
    #[inline(always)]
    fn default() -> RxMatch {
        RxMatch(0)
    }
}
#[doc = "SPI control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiCtrl(pub u32);
impl SpiCtrl {
    #[doc = "Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily separated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always separated by slave deselection: independent of the availability of TX FIFO data frames, data frames are send out with slave deselection."]
    #[inline(always)]
    pub const fn continuous(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily separated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always separated by slave deselection: independent of the availability of TX FIFO data frames, data frames are send out with slave deselection."]
    #[inline(always)]
    pub fn set_continuous(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub const fn select_precede(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub fn set_select_precede(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Only applicable in SPI Motorola submode. Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK."]
    #[inline(always)]
    pub const fn cpha(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Only applicable in SPI Motorola submode. Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK."]
    #[inline(always)]
    pub fn set_cpha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates the clock polarity. Only used in SPI Motorola submode. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
    #[inline(always)]
    pub const fn cpol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the clock polarity. Only used in SPI Motorola submode. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
    #[inline(always)]
    pub fn set_cpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    pub const fn late_miso_sample(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    pub fn set_late_miso_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
    #[inline(always)]
    pub const fn sclk_continuous(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
    #[inline(always)]
    pub fn set_sclk_continuous(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). only SPI_SELECT\\[0\\] is used in slave mode. For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Instruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
    #[inline(always)]
    pub const fn ssel_polarity0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). only SPI_SELECT\\[0\\] is used in slave mode. For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Instruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
    #[inline(always)]
    pub fn set_ssel_polarity0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave select polarity. SSEL_POLARITY1 applies to the outgoing SPI slave select signal 1 (master mode)."]
    #[inline(always)]
    pub const fn ssel_polarity1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Slave select polarity. SSEL_POLARITY1 applies to the outgoing SPI slave select signal 1 (master mode)."]
    #[inline(always)]
    pub fn set_ssel_polarity1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Slave select polarity. SSEL_POLARITY2 applies to the outgoing SPI slave select signal 2 (master mode)."]
    #[inline(always)]
    pub const fn ssel_polarity2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Slave select polarity. SSEL_POLARITY2 applies to the outgoing SPI slave select signal 2 (master mode)."]
    #[inline(always)]
    pub fn set_ssel_polarity2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Slave select polarity. SSEL_POLARITY3 applies to the outgoing SPI slave select signal 3 (master mode)."]
    #[inline(always)]
    pub const fn ssel_polarity3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Slave select polarity. SSEL_POLARITY3 applies to the outgoing SPI slave select signal 3 (master mode)."]
    #[inline(always)]
    pub fn set_ssel_polarity3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    pub const fn loopback(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    pub fn set_loopback(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::SpiCtrlMode {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::SpiCtrlMode::from_bits(val as u8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::SpiCtrlMode) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Selects one of the four outgoing SPI slave select signals: - 0: Slave 0, SPI_SELECT\\[0\\]. - 1: Slave 1, SPI_SELECT\\[1\\]. - 2: Slave 2, SPI_SELECT\\[2\\]. - 3: Slave 3, SPI_SELECT\\[3\\]. Only used in master mode. The IP should be disabled when changes are made to this field. only SPI_SELECT\\[0\\] is used in slave mode."]
    #[inline(always)]
    pub const fn slave_select(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Selects one of the four outgoing SPI slave select signals: - 0: Slave 0, SPI_SELECT\\[0\\]. - 1: Slave 1, SPI_SELECT\\[1\\]. - 2: Slave 2, SPI_SELECT\\[2\\]. - 3: Slave 3, SPI_SELECT\\[3\\]. Only used in master mode. The IP should be disabled when changes are made to this field. only SPI_SELECT\\[0\\] is used in slave mode."]
    #[inline(always)]
    pub fn set_slave_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
    #[inline(always)]
    pub const fn master_mode(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
    #[inline(always)]
    pub fn set_master_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SpiCtrl {
    #[inline(always)]
    fn default() -> SpiCtrl {
        SpiCtrl(0)
    }
}
#[doc = "SPI status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiStatus(pub u32);
impl SpiStatus {
    #[doc = "SPI bus is busy. The bus is considered busy ('1') during an ongoing transaction. For Motorola and National submodes, the busy bit is '1', when the slave selection (low active) is activated. For TI submode, the busy bit is '1' from the time the preceding/coinciding slave select (high active) is activated for the first transmitted data frame, till the last MOSI/MISO bit of the last data frame is transmitted."]
    #[inline(always)]
    pub const fn bus_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI bus is busy. The bus is considered busy ('1') during an ongoing transaction. For Motorola and National submodes, the busy bit is '1', when the slave selection (low active) is activated. For TI submode, the busy bit is '1' from the time the preceding/coinciding slave select (high active) is activated for the first transmitted data frame, till the last MOSI/MISO bit of the last data frame is transmitted."]
    #[inline(always)]
    pub fn set_bus_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_ADDR or CURR_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_ADDR and CURR_ADDR are reliable."]
    #[inline(always)]
    pub const fn spi_ec_busy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_ADDR or CURR_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_ADDR and CURR_ADDR are reliable."]
    #[inline(always)]
    pub fn set_spi_ec_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when SPI_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub const fn curr_ez_addr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SPI current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when SPI_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn set_curr_ez_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "SPI base EZ address. Address as provided by a SPI write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub const fn base_ez_addr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "SPI base EZ address. Address as provided by a SPI write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn set_base_ez_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for SpiStatus {
    #[inline(always)]
    fn default() -> SpiStatus {
        SpiStatus(0)
    }
}
#[doc = "Generic status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Indicates whether the externally clocked logic is potentially accessing the EZ memory (this is only possible in EZ mode). This bit can be used by SW to determine whether it is safe to issue a SW access to the EZ memory (without bus wait states (a blocked SW access) or bus errors being generated). Note that the INTR_TX.BLOCKED and INTR_RX.BLOCKED interrupt causes are used to indicate whether a SW access was actually blocked by externally clocked logic."]
    #[inline(always)]
    pub const fn ec_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the externally clocked logic is potentially accessing the EZ memory (this is only possible in EZ mode). This bit can be used by SW to determine whether it is safe to issue a SW access to the EZ memory (without bus wait states (a blocked SW access) or bus errors being generated). Note that the INTR_TX.BLOCKED and INTR_RX.BLOCKED interrupt causes are used to indicate whether a SW access was actually blocked by externally clocked logic."]
    #[inline(always)]
    pub fn set_ec_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Transmitter control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCtrl(pub u32);
impl TxCtrl {
    #[doc = "Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
    #[inline(always)]
    pub const fn data_width(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
    #[inline(always)]
    pub fn set_data_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub const fn msb_first(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn set_msb_first(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for TxCtrl {
    #[inline(always)]
    fn default() -> TxCtrl {
        TxCtrl(0)
    }
}
#[doc = "Transmitter FIFO control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxFifoCtrl(pub u32);
impl TxFifoCtrl {
    #[doc = "Trigger level. When the transmitter FIFO has less entries than the number of this field, a transmitter trigger event INTR_TX.TRIGGER is generated."]
    #[inline(always)]
    pub const fn trigger_level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger level. When the transmitter FIFO has less entries than the number of this field, a transmitter trigger event INTR_TX.TRIGGER is generated."]
    #[inline(always)]
    pub fn set_trigger_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "When '1', the transmitter FIFO and transmitter shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', the transmitter FIFO and transmitter shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When '1', hardware reads from the transmitter FIFO do not remove FIFO entries. Freeze will not advance the TX FIFO read pointer."]
    #[inline(always)]
    pub const fn freeze(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', hardware reads from the transmitter FIFO do not remove FIFO entries. Freeze will not advance the TX FIFO read pointer."]
    #[inline(always)]
    pub fn set_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for TxFifoCtrl {
    #[inline(always)]
    fn default() -> TxFifoCtrl {
        TxFifoCtrl(0)
    }
}
#[doc = "Transmitter FIFO status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxFifoStatus(pub u32);
impl TxFifoStatus {
    #[doc = "Amount of entries in the transmitter FIFO. The value of this field ranges from 0 to FF_DATA_NR (EZ_DATA_NR/2)."]
    #[inline(always)]
    pub const fn used(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Amount of entries in the transmitter FIFO. The value of this field ranges from 0 to FF_DATA_NR (EZ_DATA_NR/2)."]
    #[inline(always)]
    pub fn set_used(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Indicates whether the TX shift registers holds a valid data frame ('1') or not ('0'). The shift register can be considered the top of the TX FIFO (the data frame is not included in the USED field of the TX FIFO). The shift register is a working register and holds the data frame that is currently transmitted (when the protocol state machine is transmitting a data frame) or the data frame that is transmitted next (when the protocol state machine is not transmitting a data frame)."]
    #[inline(always)]
    pub const fn sr_valid(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether the TX shift registers holds a valid data frame ('1') or not ('0'). The shift register can be considered the top of the TX FIFO (the data frame is not included in the USED field of the TX FIFO). The shift register is a working register and holds the data frame that is currently transmitted (when the protocol state machine is transmitting a data frame) or the data frame that is transmitted next (when the protocol state machine is not transmitting a data frame)."]
    #[inline(always)]
    pub fn set_sr_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "FIFO read pointer: FIFO location from which a data frame is read by the hardware."]
    #[inline(always)]
    pub const fn rd_ptr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "FIFO read pointer: FIFO location from which a data frame is read by the hardware."]
    #[inline(always)]
    pub fn set_rd_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "FIFO write pointer: FIFO location at which a new data frame is written."]
    #[inline(always)]
    pub const fn wr_ptr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "FIFO write pointer: FIFO location at which a new data frame is written."]
    #[inline(always)]
    pub fn set_wr_ptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxFifoStatus {
    #[inline(always)]
    fn default() -> TxFifoStatus {
        TxFifoStatus(0)
    }
}
#[doc = "Transmitter FIFO write register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxFifoWr(pub u32);
impl TxFifoWr {
    #[doc = "Data frame written into the transmitter FIFO. Behavior is similar to that of a PUSH operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\] are used. A write to a full TX FIFO sets INTR_TX.OVERFLOW to '1'."]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data frame written into the transmitter FIFO. Behavior is similar to that of a PUSH operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\] are used. A write to a full TX FIFO sets INTR_TX.OVERFLOW to '1'."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for TxFifoWr {
    #[inline(always)]
    fn default() -> TxFifoWr {
        TxFifoWr(0)
    }
}
#[doc = "UART control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartCtrl(pub u32);
impl UartCtrl {
    #[doc = "Local loopback control (does NOT affect the information on the pins). When '0', the transmitter TX line 'uart_tx_out' is connected to the TX pin and the receiver RX line 'uart_rx_in' is connected to the RX pin. When '1', the transmitter TX line 'uart_tx_out' is connected to the receiver RX line 'uart_rx_in'. A similar connections scheme is followed for 'uart_rts_out' and 'uart_cts_in'. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    pub const fn loopback(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Local loopback control (does NOT affect the information on the pins). When '0', the transmitter TX line 'uart_tx_out' is connected to the TX pin and the receiver RX line 'uart_rx_in' is connected to the RX pin. When '1', the transmitter TX line 'uart_tx_out' is connected to the receiver RX line 'uart_rx_in'. A similar connections scheme is followed for 'uart_rts_out' and 'uart_cts_in'. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    pub fn set_loopback(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::UartCtrlMode {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::UartCtrlMode::from_bits(val as u8)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::UartCtrlMode) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for UartCtrl {
    #[inline(always)]
    fn default() -> UartCtrl {
        UartCtrl(0)
    }
}
#[doc = "UART flow control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartFlowCtrl(pub u32);
impl UartFlowCtrl {
    #[doc = "Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal 'uart_rts_out' is activated. By setting this field to '0', flow control is effectively SW disabled (may be useful for debug purposes)."]
    #[inline(always)]
    pub const fn trigger_level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal 'uart_rts_out' is activated. By setting this field to '0', flow control is effectively SW disabled (may be useful for debug purposes)."]
    #[inline(always)]
    pub fn set_trigger_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Polarity of the RTS output signal 'uart_rts_out': '0': RTS is low/'0' active; 'uart_rts_out' is '0' when active and 'uart_rts_out' is '1' when inactive. '1': RTS is high/'1' active; 'uart_rts_out' is '1' when active and 'uart_rts_out' is '0' when inactive. During IP reset (Hibernate system power mode), 'uart_rts_out' is '1'. This represents an inactive state assuming a low/'0' active polarity."]
    #[inline(always)]
    pub const fn rts_polarity(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of the RTS output signal 'uart_rts_out': '0': RTS is low/'0' active; 'uart_rts_out' is '0' when active and 'uart_rts_out' is '1' when inactive. '1': RTS is high/'1' active; 'uart_rts_out' is '1' when active and 'uart_rts_out' is '0' when inactive. During IP reset (Hibernate system power mode), 'uart_rts_out' is '1'. This represents an inactive state assuming a low/'0' active polarity."]
    #[inline(always)]
    pub fn set_rts_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Polarity of the CTS input signal 'uart_cts_in': '0': CTS is low/'0' active; 'uart_cts_in' is '0' when active and 'uart_cts_in' is '1' when inactive. '1': CTS is high/'1' active; 'uart_cts_in' is '1' when active and 'uart_cts_in' is '0' when inactive."]
    #[inline(always)]
    pub const fn cts_polarity(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Polarity of the CTS input signal 'uart_cts_in': '0': CTS is low/'0' active; 'uart_cts_in' is '0' when active and 'uart_cts_in' is '1' when inactive. '1': CTS is high/'1' active; 'uart_cts_in' is '1' when active and 'uart_cts_in' is '0' when inactive."]
    #[inline(always)]
    pub fn set_cts_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable use of CTS input signal 'uart_cts_in' by the UART transmitter: '0': Disabled. The UART transmitter ignores 'uart_cts_in', and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses 'uart_cts_in' to qualify the transmission of data. It transmits when 'uart_cts_in' is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', 'uart_cts_in' is connected to 'uart_rts_out' in the IP (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    pub const fn cts_enabled(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable use of CTS input signal 'uart_cts_in' by the UART transmitter: '0': Disabled. The UART transmitter ignores 'uart_cts_in', and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses 'uart_cts_in' to qualify the transmission of data. It transmits when 'uart_cts_in' is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', 'uart_cts_in' is connected to 'uart_rts_out' in the IP (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    pub fn set_cts_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for UartFlowCtrl {
    #[inline(always)]
    fn default() -> UartFlowCtrl {
        UartFlowCtrl(0)
    }
}
#[doc = "UART receiver control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartRxCtrl(pub u32);
impl UartRxCtrl {
    #[doc = "Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period. If STOP_BITS is '1', stop bits error detection is NOT performed. If STOP_BITS is in \\[2, 7\\], stop bits error detection is performed and the associated interrupt cause INTR_RX.FRAME_ERROR is set to '1' if an error is detected. In other words, the receiver supports data frames with a 1 bit period stop bit sequence, but requires at least 1.5 bit period stop bit sequences to detect errors. This limitation is due to possible transmitter and receiver clock skew that prevents the design from doing reliable stop bit detection for short (1 bit period) stop bit sequences. Note that in case of a stop bits error, the successive data frames may get lost as the receiver needs to resynchronize its start bit detection. The amount of lost data frames depends on both the amount of stop bits, the idle ('1') time between data frames and the data frame value."]
    #[inline(always)]
    pub const fn stop_bits(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period. If STOP_BITS is '1', stop bits error detection is NOT performed. If STOP_BITS is in \\[2, 7\\], stop bits error detection is performed and the associated interrupt cause INTR_RX.FRAME_ERROR is set to '1' if an error is detected. In other words, the receiver supports data frames with a 1 bit period stop bit sequence, but requires at least 1.5 bit period stop bit sequences to detect errors. This limitation is due to possible transmitter and receiver clock skew that prevents the design from doing reliable stop bit detection for short (1 bit period) stop bit sequences. Note that in case of a stop bits error, the successive data frames may get lost as the receiver needs to resynchronize its start bit detection. The amount of lost data frames depends on both the amount of stop bits, the idle ('1') time between data frames and the data frame value."]
    #[inline(always)]
    pub fn set_stop_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Parity bit. When '0', the receiver expects an even parity. When '1', the receiver expects an odd parity. Only applicable in standard UART and SmartCard submodes."]
    #[inline(always)]
    pub const fn parity(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Parity bit. When '0', the receiver expects an even parity. When '1', the receiver expects an odd parity. Only applicable in standard UART and SmartCard submodes."]
    #[inline(always)]
    pub fn set_parity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Parity checking enabled ('1') or not ('0'). Only applicable in standard UART submode. In SmartCard submode, parity checking is always enabled through hardware. In IrDA submode, parity checking is always disabled through hardware."]
    #[inline(always)]
    pub const fn parity_enabled(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Parity checking enabled ('1') or not ('0'). Only applicable in standard UART submode. In SmartCard submode, parity checking is always enabled through hardware. In IrDA submode, parity checking is always disabled through hardware."]
    #[inline(always)]
    pub fn set_parity_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Inverts incoming RX line signal 'uart_rx_in'. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
    #[inline(always)]
    pub const fn polarity(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Inverts incoming RX line signal 'uart_rx_in'. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
    #[inline(always)]
    pub fn set_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Behavior when a parity check fails. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost. Only applicable in standard UART and SmartCard submodes (negatively acknowledged SmartCard data frames may be dropped with this field)."]
    #[inline(always)]
    pub const fn drop_on_parity_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Behavior when a parity check fails. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost. Only applicable in standard UART and SmartCard submodes (negatively acknowledged SmartCard data frames may be dropped with this field)."]
    #[inline(always)]
    pub fn set_drop_on_parity_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Behavior when an error is detected in a start or stop period. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    pub const fn drop_on_frame_error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Behavior when an error is detected in a start or stop period. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    pub fn set_drop_on_frame_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH should indicate a 9-bit data frame. In multi-processor mode, the 9th received bit of a data frame separates addresses (bit is '1') from data (bit is '0'). A received address is matched with RX_MATCH.DATA and RX_MATCH.MASK. In the case of a match, subsequent received data are sent to the RX FIFO. In the case of NO match, subsequent received data are dropped."]
    #[inline(always)]
    pub const fn mp_mode(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH should indicate a 9-bit data frame. In multi-processor mode, the 9th received bit of a data frame separates addresses (bit is '1') from data (bit is '0'). A received address is matched with RX_MATCH.DATA and RX_MATCH.MASK. In the case of a match, subsequent received data are sent to the RX FIFO. In the case of NO match, subsequent received data are dropped."]
    #[inline(always)]
    pub fn set_mp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minimum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
    #[inline(always)]
    pub const fn lin_mode(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minimum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
    #[inline(always)]
    pub fn set_lin_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'. This functionality is intended for wake up from DeepSleep when receiving a data frame. The transition from idle ('1') to START ('0') on the RX line is used to wake up the CPU. The transition detection (and the associated wake up functionality) is performed by the GPIO2 IP. The woken up CPU will enable the SCB's UART receiver functionality. Once enabled, it is assumed that the START bit is ongoing (the CPU wakeup and SCB enable time should be less than the START bit period). The SCB will synchronize to a '0' to '1' transition, which indicates the first data frame bit is received (first data frame bit should be '1'). After synchronization to the first data frame bit, the SCB will resume normal UART functionality: subsequent data frames will be synchronized on the receipt of a START bit."]
    #[inline(always)]
    pub const fn skip_start(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'. This functionality is intended for wake up from DeepSleep when receiving a data frame. The transition from idle ('1') to START ('0') on the RX line is used to wake up the CPU. The transition detection (and the associated wake up functionality) is performed by the GPIO2 IP. The woken up CPU will enable the SCB's UART receiver functionality. Once enabled, it is assumed that the START bit is ongoing (the CPU wakeup and SCB enable time should be less than the START bit period). The SCB will synchronize to a '0' to '1' transition, which indicates the first data frame bit is received (first data frame bit should be '1'). After synchronization to the first data frame bit, the SCB will resume normal UART functionality: subsequent data frames will be synchronized on the receipt of a START bit."]
    #[inline(always)]
    pub fn set_skip_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break. During a break the transmitted/received line value is '0'. This feature is useful for standard UART submode and LIN submode ('break field' detection). Once, the break is detected, the INTR_RX.BREAK_DETECT bit is set to '1'. Note that break detection precedes baud rate detection, which is used to synchronize/refine the receiver clock to the transmitter clock. As a result, break detection operates with an unsynchronized/unrefined receiver clock. Therefore, the receiver's definition of a bit period is imprecise and the setting of this field should take this imprecision into account. The LIN standard also accounts for this imprecision: a LIN start bit followed by 8 data bits allows for up to 9 consecutive '0' bit periods during regular transmission, whereas the LIN break detection should be at least 13 consecutive '0' bit periods. This provides for a margin of 4 bit periods. Therefore, the default value of this field is set to 10, representing a minimal break field with of 10+1 = 11 bit periods; a value in between the 9 consecutive bit periods of a regular transmission and the 13 consecutive bit periods of a break field. This provides for slight imprecisions of the receiver clock wrt. the transmitter clock. There should not be a need to program this field to any value other than its default value."]
    #[inline(always)]
    pub const fn break_width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break. During a break the transmitted/received line value is '0'. This feature is useful for standard UART submode and LIN submode ('break field' detection). Once, the break is detected, the INTR_RX.BREAK_DETECT bit is set to '1'. Note that break detection precedes baud rate detection, which is used to synchronize/refine the receiver clock to the transmitter clock. As a result, break detection operates with an unsynchronized/unrefined receiver clock. Therefore, the receiver's definition of a bit period is imprecise and the setting of this field should take this imprecision into account. The LIN standard also accounts for this imprecision: a LIN start bit followed by 8 data bits allows for up to 9 consecutive '0' bit periods during regular transmission, whereas the LIN break detection should be at least 13 consecutive '0' bit periods. This provides for a margin of 4 bit periods. Therefore, the default value of this field is set to 10, representing a minimal break field with of 10+1 = 11 bit periods; a value in between the 9 consecutive bit periods of a regular transmission and the 13 consecutive bit periods of a break field. This provides for slight imprecisions of the receiver clock wrt. the transmitter clock. There should not be a need to program this field to any value other than its default value."]
    #[inline(always)]
    pub fn set_break_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for UartRxCtrl {
    #[inline(always)]
    fn default() -> UartRxCtrl {
        UartRxCtrl(0)
    }
}
#[doc = "UART receiver status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartRxStatus(pub u32);
impl UartRxStatus {
    #[doc = "Amount of peripheral clock periods that constitute the transmission of a 0x55 data frame (sent least significant bit first) as determined by the receiver. BR_COUNTER / 8 is the amount of peripheral clock periods that constitute a bit period. This field has valid data when INTR_RX.BAUD_DETECT is set to '1'."]
    #[inline(always)]
    pub const fn br_counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Amount of peripheral clock periods that constitute the transmission of a 0x55 data frame (sent least significant bit first) as determined by the receiver. BR_COUNTER / 8 is the amount of peripheral clock periods that constitute a bit period. This field has valid data when INTR_RX.BAUD_DETECT is set to '1'."]
    #[inline(always)]
    pub fn set_br_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for UartRxStatus {
    #[inline(always)]
    fn default() -> UartRxStatus {
        UartRxStatus(0)
    }
}
#[doc = "UART transmitter control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartTxCtrl(pub u32);
impl UartTxCtrl {
    #[doc = "Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
    #[inline(always)]
    pub const fn stop_bits(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
    #[inline(always)]
    pub fn set_stop_bits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity. Only applicable in standard UART and SmartCard submodes."]
    #[inline(always)]
    pub const fn parity(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity. Only applicable in standard UART and SmartCard submodes."]
    #[inline(always)]
    pub fn set_parity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware"]
    #[inline(always)]
    pub const fn parity_enabled(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware"]
    #[inline(always)]
    pub fn set_parity_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
    #[inline(always)]
    pub const fn retry_on_nack(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
    #[inline(always)]
    pub fn set_retry_on_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for UartTxCtrl {
    #[inline(always)]
    fn default() -> UartTxCtrl {
        UartTxCtrl(0)
    }
}
