    .section .text.hard_fault_trampoline
    .global hard_fault_trampoline
    .type hard_fault_trampoline, %function
hard_fault_trampoline:
    mrs r0, MSP /* Load the 1st arg - stack pointer value */
    b hard_fault /* Proceed w/ the user implementation of hardfault handler, if there's one. See "script.ld" and "lib.rs" */
