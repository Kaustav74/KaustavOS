#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kaustavos_kernel::kernel::KaustavKernel;

static mut KERNEL: KaustavKernel = KaustavKernel::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        KERNEL.init();
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
