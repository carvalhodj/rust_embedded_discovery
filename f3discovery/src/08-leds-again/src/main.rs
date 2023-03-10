#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // TODO initialize GPIOE
    rcc.ahbenr.write(|f| f.iopeen().set_bit());

    gpioe.moder.write(|w| {
        w.moder8().bits(1);
        w.moder9().bits(1);
        w.moder10().bits(1);
        w.moder11().bits(1);
        w.moder12().bits(1);
        w.moder13().bits(1);
        w.moder14().bits(1);
        w.moder15().bits(1)
    });

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    aux8::bkpt();

    loop {}
}
