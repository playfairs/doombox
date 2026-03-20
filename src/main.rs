#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use x86_64::instructions::hlt;

mod vga;
mod serial;
mod gdt;
mod idt;
mod memory;
mod doom_engine;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("DOOMBOX - Booting DOOM from bare metal...");
    
    gdt::init();
    idt::init();
    
    x86_64::instructions::interrupts::enable();
    
    println!("Initializing framebuffer...");
    vga::init_framebuffer();
    
    println!("Loading DOOM engine...");
    doom_engine::init();
    
    println!("Starting DOOM...");
    doom_engine::run();
    
    loop {
        hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        hlt();
    }
}
