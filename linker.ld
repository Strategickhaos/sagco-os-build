ENTRY(_start)
SECTIONS {
    . = 0x100000;
    .text.start : { KEEP(*(.text.start)) }
    .text : { *(.text .text.*) }
    .rodata : ALIGN(4096) { *(.rodata .rodata.*) }
    .data : ALIGN(4096) { *(.data .data.*) }
    . = ALIGN(4096);
    _bss_start = .;
    .bss : { *(.bss .bss.*) *(.bss.stack) }
    _bss_end = .;
    /DISCARD/ : { *(.eh_frame) *(.note.*) }
}
