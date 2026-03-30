ENTRY(_start)
SECTIONS {
    . = 0x8000; /* To musi pasować do 'call 0x8000' w boot.asm */
    .text : {
        *(.text._start) /* Wymuszamy start na początku */
        *(.text*)
    }
    .data : { *(.data*) }
    .rodata : { *(.rodata*) }
    .bss : { *(.bss*) }
}