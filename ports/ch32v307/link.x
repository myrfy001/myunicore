MEMORY {
    FLASH : ORIGIN = 0x00000000, LENGTH = 128K
    RAM : ORIGIN = 0x20000000, LENGTH = 32K
}

OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS {
    . = 0x80000000;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    } > FLASH
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    } > FLASH
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    } > RAM
    .bss : ALIGN(8) {
        _bss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    } > RAM
    .eh_frame (INFO) : { KEEP(*(.eh_frame)) }
    .eh_frame_hdr (INFO) : { *(.eh_frame_hdr) }

    _end = ALIGN(8);
}