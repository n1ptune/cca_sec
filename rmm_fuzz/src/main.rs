#![no_std]
#![no_main]


#[macro_use]
mod console;
mod logging;
mod panic;
mod pl011;
mod mem;
mod config;
#[allow(unused)]
mod arch;
mod fuzz;

pub use fuzz::*;
// use log::*;

extern crate alloc;
core::arch::global_asm!(include_str!("entry.asm"));



#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    pl011::pl011_init();
    logging::init();
    mem::init();
    realm_fuzz();
    done_work(0); 
    // arch::shutdown();
    loop{}
}