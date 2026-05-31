use core::panic::PanicInfo;
use crate::vga;
#[panic_handler]
fn sagco_panic(_: &PanicInfo) -> ! {
    unsafe { vga::print("\nSAGCO KERNEL PANIC\n"); }
    loop { unsafe { core::arch::asm!("cli; hlt"); } }
}
