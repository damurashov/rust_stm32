MEMORY {
    FLASH : ORIGIN = 0x08000000, LENGTH = 16K
    SRAM :  ORIGIN = 0x20000000, LENGTH = 4K
}

ENTRY(reset);

EXTERN(RESET_VECTOR);

SECTIONS {
    .vector_table ORIGIN(FLASH): {
        /* Stack pointer's initial value */
        LONG(ORIGIN(SRAM) + LENGTH(SRAM));

        /* Do not eliminate (optimize out) .reset_vector symbol at linking stage */
        KEEP(*(.vector_table.reset_vector));
    } > FLASH

    .text : {
        *(.text .text.*);
    } > FLASH

    .data : {
        $(.data .data.*);
    } > SRAM

    .rodata : {
        $(.rodata .rodata.*);
    } > FLASH

    .bss : {
        $(.bss .bss.*)
    } > SRAM

    /DISCARD/ : {
        *(.ARM.exidx .ARM.exidx.*);
    }
}
