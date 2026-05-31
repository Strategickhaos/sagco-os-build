#![no_std]
#![no_main]
mod arch;
mod vga;
mod panic;
const DNA: u64 = 0xa364ca9f90356c85;
const FP:  u64 = 0x6a6db9183a5c1112;
#[repr(C)]
pub struct Mmap { pub dsz: u64, pub cnt: u64, pub ptr: *const u8 }
#[no_mangle]
pub extern "sysv64" fn kernel_main(_mmap: *const Mmap, dna: u64, fp: u64) -> ! {
    unsafe {
        vga::clear();
        if dna != DNA || fp != FP { vga::print("DNA MISMATCH\n"); loop { core::arch::asm!("cli; hlt"); } }
        vga::banner("==============================================\n");
        vga::banner("  SAGCO-OS KERNEL  |  INV-001\n");
        vga::banner("  Strategickhaos DAO LLC  |  Dom (Me10101)\n");
        vga::banner("==============================================\n");
        vga::print("\nDNA: "); vga::hex(dna);
        vga::print("\nFP:  "); vga::hex(fp);
        vga::print("\nGATE: PASS\n\n");
        vga::banner("SAGCO KERNEL ONLINE\n");
        vga::print("\nNEXT: kernel/mm/pmm.rs\nNEXT: kernel/src/idt.rs\n");
    }
    loop { unsafe { core::arch::asm!("cli; hlt"); } }
}
