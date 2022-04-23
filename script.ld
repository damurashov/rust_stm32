MEMORY {
	FLASH(rx) :  ORIGIN = 0x08000000, LENGTH = 16K
	SRAM(rwx) :  ORIGIN = 0x20000000, LENGTH = 4K
}

ENTRY(reset);

EXTERN(RESET_VECTOR);
EXTERN(EXCEPTIONS);
EXTERN(malloc)

SECTIONS {
	.vector_table ORIGIN(FLASH): {
		/* Stack pointer's initial value */
		LONG(ORIGIN(SRAM) + LENGTH(SRAM));

		/* Do not eliminate (optimize out) .reset_vector symbol at linking stage */
		KEEP(*(.vector_table.reset_vector));

		KEEP(*(.vector_table.exceptions));
	} > FLASH

	.text : {
		*(.text .text.*);
	} > FLASH

	.bss : {
		. = ALIGN(4);
		_sbss = .;
		*(.bss .bss.*)
		. = ALIGN(4);
		_ebss = .;
	} > SRAM

	.rodata : {
		. = ALIGN(4);
		*(.rodata .rodata.*);
	} > FLASH

	.data : AT(ADDR(.rodata) + SIZEOF(.rodata)) {
		. = ALIGN(4);
		_sdata = .;
		*(.data .data.*);
		. = ALIGN(4);
		_edata = .;
	} > SRAM

	_sidata = LOADADDR(.data);

	/* /DISCARD/ : {
		*(.ARM.exidx .ARM.exidx.*);
	} */

	.ARM : {
		KEEP(*(.ARM*malloc))
	} > FLASH
}

PROVIDE(nmi = default_exception_handler);
PROVIDE(hard_fault = default_exception_handler);
PROVIDE(sv_call = default_exception_handler);
PROVIDE(pend_sv = default_exception_handler);
PROVIDE(sys_tick = default_exception_handler);
