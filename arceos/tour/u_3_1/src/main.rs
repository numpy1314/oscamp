#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use core::{mem, str};
use std::os::arceos::modules::axhal::mem::phys_to_virt;

/// Physical address for GICv2 on aarch64-qemu-virt-hv
const GICV2_BASE: usize = 0x0800_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Starting GICv2 device access test...");
    
    // Makesure that we can access GICv2 device region.
    let va = phys_to_virt(GICV2_BASE.into()).as_usize();
    println!("Physical address: {:#X}, Virtual address: {:#X}", GICV2_BASE, va);
    
    let ptr = va as *const u32;
    unsafe {
        println!("Attempting to read from GICv2 device...");
        let value = *ptr;
        println!("Try to access dev region [{:#X}], got {:#X}", va, value);
        
        // 尝试读取多个寄存器
        for i in 0..4 {
            let reg_ptr = (va + i * 4) as *const u32;
            let reg_value = *reg_ptr;
            println!("Register {} (offset {:#X}): {:#X}", i, i * 4, reg_value);
        }
    }
    
    // 添加设备直通内存区域访问成功消息
    println!("Successfully accessed device memory region at physical address {:#X}", GICV2_BASE);
    
    // 正常退出程序
    println!("Test completed successfully. Exiting...");
}