use super::pac;

fn clocks_setup(flash: &pac::FLASH, pwr: &pac::PWR, rcc: &pac::RCC) {
    // Enable power controller clock
    rcc.apb1enr1.modify(|_, w| w.pwren().set_bit());
    // Disable USB Type-C dead battery pull-down behavior
    pwr.cr3.modify(|_, w| w.ucpd1_dbdis().set_bit());
    // Go to power range 1
    pwr.cr1.modify(|_, w| unsafe { w.vos().bits(0b01) });
    while pwr.sr2.read().vosf().bit_is_set() {}

    // Set flash latency and enable prefetch
    flash
        .acr
        .modify(|_, w| unsafe { w.latency().bits(4).prften().set_bit() });
    while flash.acr.read().latency() != 4 {}

    // The system clock must be divided by 2 using the AHB prescaler before switching
    // to a higher system frequency
    rcc.cfgr.modify(|_, w| unsafe { w.hpre().bits(0b1000) });
    // Go to boost mode for high performance
    pwr.cr5.modify(|_, w| w.r1mode().clear_bit());

    // Ensure HSE oscillator is on and stable
    rcc.cr.modify(|_, w| w.hseon().set_bit());
    while rcc.cr.read().hserdy().bit_is_clear() {}

    // Configure and enable PLL: 24 MHz / 6 * 85 / 2 = 170 MHz
    rcc.pllcfgr.modify(|_, w| unsafe {
        // HSE is the PLL source
        w.pllsrc()
            .bits(0b11)
            // NOTE: 0b0000 is 1, 0b0001 is 2 and so on
            // M divider is 6
            .pllm()
            .bits(6 - 1)
            // N multiplier is 85
            .plln()
            .bits(85)
            // Enable PLL P output
            .pllpen()
            .set_bit()
            // Enable PLL Q output
            .pllqen()
            .set_bit()
            // Q divider is 2
            .pllq()
            .bits(0)
            // Enable PLL R output
            .pllren()
            .set_bit()
            // R divider is 2
            .pllr()
            .bits(0)
            // P divider is 2
            .pllpdiv()
            .bits(2)
    });
    rcc.cr.modify(|_, w| w.pllon().set_bit());
    while rcc.cr.read().pllrdy().bit_is_set() {}
    // Switch system clock source to PLL
    rcc.cfgr.modify(|_, w| unsafe { w.sw().bits(0b11) });
    while rcc.cfgr.read().sws() != 0b11 {}
    // Set AHB prescaler back to 1
    rcc.cfgr.modify(|_, w| unsafe { w.hpre().bits(0b0000) });
}

fn cordic_setup(rcc: &pac::RCC, _cordic: &pac::CORDIC) {
    // Enable CORDIC clock
    rcc.ahb1enr.modify(|_, w| w.cordicen().set_bit());
}

pub fn init() {
    let dp = unsafe { pac::Peripherals::steal() };
    clocks_setup(&dp.FLASH, &dp.PWR, &dp.RCC);
    cordic_setup(&dp.RCC, &dp.CORDIC);
}
