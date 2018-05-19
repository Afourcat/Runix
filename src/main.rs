/*
** Alexandre
** Fourcat
** 2018
** main.rs
** main file of the kernel
*/


#![feature(lang_items)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros))]
#![cfg(test)]
extern crate std;

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! 
{
	loop{}
}

extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;
#[macro_use]
mod vga_buffer;

//Linux
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello World{}", "!");
	println!("\n\n\n\n\n\n\n\n");
	loop {}
}
