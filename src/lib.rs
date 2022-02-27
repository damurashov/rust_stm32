// #![no_main]
#![no_std]
#![feature(lang_items)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
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
