#![no_std]
#![no_main]

extern crate panic_halt;

mod vfd;


use riscv_rt::entry;
use gd32vf103_hal as hal;
use hal::prelude::*;
use hal::pac as pac;
use pac::TIMER4;
use hal::delay;
use hal::ctimer;
use hal::spi::{Spi, Mode, Polarity, Phase};
use hal::pwm::Pwm;
use hal::rcu;
use hal::timer::Timer;
use hal::gpio::gpiob::*;
use hal::gpio::{Alternate, PushPull};
use embedded_hal::blocking::delay::DelayMs;


// extern crate stepper_rs;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcu.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcu.apb2);
    // let mut afen = dp.APB2.split(&mut rcu.apb2);
    let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.ctl1);
    let clocks = rcu::Strict::new().freeze(&mut rcu.cfg);
    let ctimer = ctimer::CoreTimer::new(dp.CTIMER);;
    let mut delay = delay::Delay::new(clocks, ctimer);
    // let mut timer4 = dp.TIMER4;


    // Next goal is to set up a simple PWM on PA0. This is the VFD filament wire.
    // Datasheet says 50/60 Hz, so it probably can take ~48 to 64 ish...
    // Actually, we're good to push it over 10KHz. 
    // Timer 4, channel 0. This runs off the CK_TIMER clock
    //

    pc13.set_high().unwrap();
    delay.delay_ms(1000 as u32);

    let mut pwm4ch0 = Some(gpioa.pa0.into_alternate_push_pull(&mut gpioa.ctl0));
    let mut pwm4ch1 = Some(gpioa.pa1.into_alternate_push_pull(&mut gpioa.ctl0));
    let mut pwm4ch2 = Some(gpioa.pa2.into_alternate_push_pull(&mut gpioa.ctl0));
    let mut pwm4ch3 = Some(gpioa.pa3.into_alternate_push_pull(&mut gpioa.ctl0));

    // Goal is to get PWM on PA1, green. timer4_ch1


    // pa1 is green
    // pa2 is blue

    let mut pwm = Pwm::pwm(4000.hz(), clocks, dp.TIMER4, &mut rcu.apb1, &mut rcu.apb2, pwm4ch0, pwm4ch1, pwm4ch2, pwm4ch3);
    // let mut pwm = Pwm::pwm(1000.hz(), clocks, dp.TIMER4, &mut rcu.apb1, &mut rcu.apb2, pwm4ch1);
    pwm.init();
    pwm.set_duty(0,pwm.get_max_duty()/2);
    pwm.set_duty(1,20);
    pwm.set_duty(2,20);
    pwm.set_duty(3,2);

    // let sck = Some(gpiob.pb13.into_alternate_push_pull(&mut gpiob.ctl1));
    // let mosi = Some(gpiob.pb15.into_alternate_push_pull(&mut gpiob.ctl1));
    let sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.ctl1);
    let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.ctl1);
    let miso = gpiob.pb14.into_floating_input(&mut gpiob.ctl1);
    let nss = gpiob.pb12.into_alternate_push_pull(&mut gpiob.ctl1);
    // let vfd_strobe = gpiob.pb14.into_push_pull_output(&mut gpiob.ctl1);
    let vfd_strobe = gpiob.pb8.into_push_pull_output(&mut gpiob.ctl1);
    let vfd_blank = gpiob.pb9.into_push_pull_output(&mut gpiob.ctl1);
    let spi_mode = Mode {
        // polarity: Polarity::IdleHigh,
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
        // phase: Phase::CaptureOnSecondTransition,
        };
    let vfd_spi = Spi::spi1(
                            dp.SPI1,
                            // (sck, None, mosi, None),
                            (sck, miso, mosi, nss),
                            spi_mode,
                            500.khz(),
                            clocks,
                            &mut rcu.apb1
                            );
    let mut vfd = vfd::VFD::new(vfd_spi, vfd_blank, vfd_strobe);

    let mut data : u32 = 0;
    vfd.clock_out_data_direct(0x123456889a);
    // delay.delay_ms(1000 as u32);
    // vfd.clock_out_data(0x0f0f0f0f, 0);
    // vfd.clock_out_data(0x00f0f0f0, 1);
    vfd.clock_out_data(0x0fffffff, 2);
    // delay.delay_ms(1000 as u32);

    vfd.write_str(0,0,"MILES:");
    // delay.delay_ms(1000 as u32);
    vfd.write_u32(0, 6, 1);
    // vfd.write_str(0,0,"FFFFFFFFFF");
    // delay.delay_ms(1000 as u32);
        // vfd.update_display();
    loop {
        vfd.update_display();
        delay.delay_ms(3 as u32);
    }

}
