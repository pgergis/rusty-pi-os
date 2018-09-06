#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(panic_implementation)]

#[macro_use]
extern crate core;
extern crate pi;
extern crate stack_vec;

// use pi::gpio::Gpio;
// use pi::timer;
use pi::uart::MiniUart;
use std::fmt::Write;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

#[no_mangle]
pub extern "C" fn kmain() {
    // FIXME: Start the shell.

    let mut myuart = MiniUart::new();
    loop {
        let b = myuart.read_byte();
        myuart.write_byte(b);
        myuart.write_str("<-");
    }

}

// #[no_mangle]
// pub unsafe extern "C" fn kmain() {

//     let mut light1 = Gpio::new(16).into_output();
//     let mut light2 = Gpio::new(20).into_output();
//     let mut light3 = Gpio::new(21).into_output();

//     loop {
//         light1.set();
//         timer::spin_sleep_ms(2000);
//         light2.set();
//         timer::spin_sleep_ms(2000);
//         light3.set();
//         timer::spin_sleep_ms(2000);
//         light1.clear(); light2.clear(); light3.clear();
//         timer::spin_sleep_ms(500);
//         light1.set(); light2.set(); light3.set();
//         timer::spin_sleep_ms(500);
//         light1.clear(); light2.clear(); light3.clear();
//         timer::spin_sleep_ms(500);
//         light1.set(); light2.set(); light3.set();
//         timer::spin_sleep_ms(3000);
//         light1.clear(); light2.clear(); light3.clear();
//         timer::spin_sleep_ms(1000);
//     }
// }
