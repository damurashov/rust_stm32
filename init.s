    .section .text.hard_fault_trampoline
    .global hard_fault_trampoline
    .type hard_fault_trampoline, %function
hard_fault_trampoline:
    mrs r0, MSP /* Load the 1st arg - stack pointer value */
    b hard_fault /* Proceed w/ the user implementation of hardfault handler, if there's one. See "script.ld" and "lib.rs" */

.section .rodata
.word EXC_RETURN

    .section .text.reset_trampoline
    .global reset_trampoline
    .type reset_trampoline, %function
reset_trampoline:
    /* Copy MSP to PSP */
    mrs r0, MSP
    msr PSP, r0
    /* Use PSP stack pointer instead of default MSP */
    movs R0, #0x2
    msr CONTROL, r0
    isb  /* Apply instruction barrier to force use of the new stack pointer */
    b reset
