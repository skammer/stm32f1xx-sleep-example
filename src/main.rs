#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;
extern crate cortex_m_rt;

use cortex_m_rt::{ExceptionFrame, entry, exception};
use hal::prelude::*;
use hal::{
    stm32,
    delay::Delay,
    gpio::{
        gpioc::PCx,
        Output,
        PushPull,
    }
};
use cortex_m::asm;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = stm32::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh).downgrade();

    bling_bling(&mut delay, &mut led);

    gpioa.pa0.into_pull_down_input(&mut gpioa.crl);

    // dp.DBGMCU.cr.modify(|_, w| {
    //     w.dbg_standby().set_bit()
    // });
    //

    // rcc.apb1.enr().modify(|_, w| {
    //     w.pwren().set_bit()
    // });

    // should to the same with pwren
    rcc.bkp.constrain(dp.BKP, &mut rcc.apb1, &mut dp.PWR);

    dp.PWR.cr.modify(|_r, w| {
        // Enable stop mode
        w.pdds().set_bit()
        // Voltage regulators to low power mode
        // .lpds().set_bit()
    });

    // Set SLEEPDEEP in cortex-m3 system control register
    cp.SCB.set_sleepdeep();

    // const SCB_SCR_SLEEPDEEP: u32 = 0x1 << 2;
    // unsafe {
    //     cp.SCB.scr.modify(|scr| scr | SCB_SCR_SLEEPDEEP);
    // }

    let standby_flag = dp.PWR.csr.read().sbf().bit();

    if standby_flag {
        dp.PWR.cr.modify(|_, w| {
            w.csbf().clear_bit()
        });
        // Clear Wakeup flag.
        dp.PWR.cr.modify(|_, w| w.cwuf().set_bit());
    }

    // Enable Wakeup Pin
    dp.PWR.csr.modify(|_, w| w.ewup().set_bit());

    loop {
        bling_bling(&mut delay, &mut led);
        asm::wfi();
    }
}

fn bling_bling(delay: &mut Delay, led: &mut PCx<Output<PushPull>>) {
    led.set_high();
    delay.delay_ms(100_u16);
    led.set_low();
    delay.delay_ms(100_u16);
    led.set_high();
    delay.delay_ms(100_u16);
    led.set_low();
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
