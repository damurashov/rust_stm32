// #![no_main]
#![no_std]
#![feature(lang_items)]

use core::panic::PanicInfo;
use core::ptr;

pub union VectorEntry {
	reserved: u32,
	handler: unsafe extern "C" fn(),
}

extern "C" {
	fn nmi();
	fn hard_fault_trampoline();
	fn sv_call();
	fn pend_sv();
	fn sys_tick();
}

#[export_name = "default_exception_handler"]
pub fn default_exception_handler() -> ! {
	loop {}
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [VectorEntry; 14] = [
	VectorEntry {handler: nmi},
	VectorEntry {handler: hard_fault_trampoline},
	VectorEntry {reserved: 0},
	VectorEntry {reserved: 0},
	VectorEntry {reserved: 0},
	VectorEntry {reserved: 0},
	VectorEntry {reserved: 0},
	VectorEntry {reserved: 0},
	VectorEntry {reserved: 0},
	VectorEntry {handler: sv_call},
	VectorEntry {reserved: 0},
	VectorEntry {reserved: 0},
	VectorEntry {handler: pend_sv},
	VectorEntry {handler: sys_tick},
];

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
