extern "C" {
    fn IOSS_INTERRUPTS_GPIO_0();
    fn IOSS_INTERRUPTS_GPIO_1();
    fn IOSS_INTERRUPTS_GPIO_2();
    fn IOSS_INTERRUPTS_GPIO_3();
    fn IOSS_INTERRUPT_GPIO();
    fn LPCOMP_INTERRUPT();
    fn SRSS_INTERRUPT_WDT();
    fn SCB_0_INTERRUPT();
    fn SCB_1_INTERRUPT();
    fn SCB_2_INTERRUPT();
    fn PASS_0_INTERRUPT_CTBS();
    fn WCO_INTERRUPT();
    fn CPUSS_INTERRUPT_SPCIF();
    fn CSD_INTERRUPT();
    fn TCPWM_INTERRUPTS_0();
    fn TCPWM_INTERRUPTS_1();
    fn TCPWM_INTERRUPTS_2();
    fn TCPWM_INTERRUPTS_3();
    fn TCPWM_INTERRUPTS_4();
    fn PASS_0_INTERRUPT_SAR();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 20] = [
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_0,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_1,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_2,
    },
    Vector {
        _handler: IOSS_INTERRUPTS_GPIO_3,
    },
    Vector {
        _handler: IOSS_INTERRUPT_GPIO,
    },
    Vector {
        _handler: LPCOMP_INTERRUPT,
    },
    Vector {
        _handler: SRSS_INTERRUPT_WDT,
    },
    Vector {
        _handler: SCB_0_INTERRUPT,
    },
    Vector {
        _handler: SCB_1_INTERRUPT,
    },
    Vector {
        _handler: SCB_2_INTERRUPT,
    },
    Vector {
        _handler: PASS_0_INTERRUPT_CTBS,
    },
    Vector {
        _handler: WCO_INTERRUPT,
    },
    Vector {
        _handler: CPUSS_INTERRUPT_SPCIF,
    },
    Vector {
        _handler: CSD_INTERRUPT,
    },
    Vector {
        _handler: TCPWM_INTERRUPTS_0,
    },
    Vector {
        _handler: TCPWM_INTERRUPTS_1,
    },
    Vector {
        _handler: TCPWM_INTERRUPTS_2,
    },
    Vector {
        _handler: TCPWM_INTERRUPTS_3,
    },
    Vector {
        _handler: TCPWM_INTERRUPTS_4,
    },
    Vector {
        _handler: PASS_0_INTERRUPT_SAR,
    },
];
