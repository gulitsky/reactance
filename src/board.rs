use super::pac;

fn rcc_reset(rcc: &pac::RCC) {
    // Reset all peripherals
    rcc.ahb1rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.ahb1rstr.reset();
    rcc.ahb2rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.ahb2rstr.reset();
    rcc.ahb3rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.ahb3rstr.reset();

    rcc.apb1rstr1.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.apb1rstr1.reset();
    rcc.apb1rstr2.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.apb1rstr2.reset();
    rcc.apb2rstr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    rcc.apb2rstr.reset();
}

fn pwr_setup(pwr: &pac::PWR) {
    // Go to range 1 boost mode for high performance
    pwr.cr1.modify(|_, w| unsafe { w.vos().bits(0b01) });
    while pwr.sr2.read().vosf().bit_is_clear() {}
    pwr.cr5.modify(|_, w| w.r1mode().clear_bit());
}

fn rcc_pll_setup(rcc: &pac::RCC, flash: &pac::FLASH) {
    rcc.cr.modify(|_, w| w.hsion().set_bit());
    while rcc.cr.read().hsirdy().bit_is_clear() {}
    rcc.icscr.modify(|_, w| unsafe { w.hsitrim().bits(0x40) });
    rcc.cfgr.modify(|_, w| unsafe { w.sw().bits(0b01) });
    while rcc.cfgr.read().sws().bits() != 0b01 {}
}
