#![no_std]
#![no_main]

use core::panic::PanicInfo;

// PSCI function IDs
const PSCI_SYSTEM_OFF: u32 = 0x84000008;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    // 调用 PSCI SYSTEM_OFF
    // x0 = function ID
    // 使用 HVC #0 进行 PSCI 调用
    core::arch::asm!(
        "mov x0, {psci_fn}",
        "hvc #0",
        psci_fn = in(reg) PSCI_SYSTEM_OFF as u64,
        options(noreturn)
    )
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
