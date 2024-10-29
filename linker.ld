ENTRY(_start)

SECTIONS
{
    /* Start loading at address 0x80000, typical for Raspberry Pi */
    . = 0x80000;

    /* Code (text) section */
    .text : {
        *(.text._start)    /* Entry point */
        *(.text*)          /* All other text/code */
    }

    /* Read-only data section */
    .rodata : {
        *(.rodata*)        /* Read-only data */
    }

    /* Initialized data section */
    .data : {
        *(.data*)          /* Initialized data */
    }

    /* Uninitialized data section (BSS) */
    .bss : {
        *(.bss*)           /* Uninitialized data */
    }

    /* Stack setup */
    . = ALIGN(512);
    _end = .;
    . = 0x200000;
    __stack_top = .;
}