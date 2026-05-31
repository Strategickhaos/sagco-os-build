use core::arch::global_asm;
global_asm!(
    ".section .text.start",".global _start",".code64","_start:",
    "   lea rsp, [rip + _stack_top]","   cld",
    "   lea rdi, [rip + _bss_start]","   lea rcx, [rip + _bss_end]",
    "   sub rcx, rdi","   xor al, al","   rep stosb",
    "   call kernel_main","_halt:","   cli","   hlt","   jmp _halt",
    ".section .bss.stack",".align 16","_stack_bottom:","   .space 65536","_stack_top:",
);
unsafe extern "C" { static _bss_start: u8; static _bss_end: u8; }
