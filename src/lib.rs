// #![no_main]
#![no_std]
#![feature(lang_items)]

use core::panic::PanicInfo;
use core::ptr;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {

	extern "C" {
		static mut _sbss: u8;
		static mut _ebss: u8;
		static mut _edata: u8;
		static mut _sdata: u8;
		static _sidata: u8;
	}

	let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
	ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

	let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
	ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

	extern "Rust" {
		fn main() -> !;
	}

	main()
}

#[macro_export]
macro_rules! entry_point {
	($entry_point:path) => {
		pub fn __main() -> ! {
			let f: fn() -> ! = $entry_point;
			f()
		}
	}
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
