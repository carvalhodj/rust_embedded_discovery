#![no_main]
#![no_std]

// use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprintln, ITM, RegisterBlock};

// // Print the current contents of odr
// fn iprint_odr(itm: &mut ITM) {
//     const GPIOE_ODR: u32 = 0x4800_1014;

//     unsafe {
//         iprintln!(
//             &mut itm.stim[0],
//             "ODR = 0x{:04x}",
//             ptr::read_volatile(GPIOE_ODR as *const u16)
//         );
//     }
// }


#[entry]
fn main() -> ! {

    let gpioe = aux7::init().1;

    gpioe.bsrr.write(|w| w.bs9().set_bit());
    gpioe.bsrr.write(|w| w.bs8().set_bit());

    gpioe.bsrr.write(|w| w.br9().set_bit());
    gpioe.bsrr.write(|w| w.br8().set_bit());
    // let mut itm = aux7::init().0;

    // unsafe {
    //     // A magic address!
    //     const GPIOE_BSRR: u32 = 0x48001018;

    //     // Print the initial contents of ODR
    //     iprint_odr(&mut itm);

    //     // Turn on the "North" LED (red)
    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
    //     iprint_odr(&mut itm);

    //     // Turn on the "East" LED (green)
    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 8);
    //     iprint_odr(&mut itm);

    //     // Turn off the "North" LED
    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
    //     iprint_odr(&mut itm);

    //     // Turn off the "East" LED
    //     ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (8 + 16));
    //     iprint_odr(&mut itm);
    // }

    // unsafe {
    //     ptr::read_volatile(0x4800_1800 as *const u32);
    // }

    loop {}
}