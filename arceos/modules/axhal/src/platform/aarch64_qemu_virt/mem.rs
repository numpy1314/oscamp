use crate::mem::MemRegion;
use page_table_entry::{aarch64::A64PTE, GenericPTE, MappingFlags};

/// Returns platform-specific memory regions.
pub(crate) fn platform_regions() -> impl Iterator<Item = MemRegion> {
    crate::mem::default_free_regions().chain(crate::mem::default_mmio_regions())
}

pub(crate) unsafe fn init_boot_page_table(
    boot_pt_l0: *mut [A64PTE; 512],
    boot_pt_l1: *mut [A64PTE; 512],
) {
    let boot_pt_l0 = &mut *boot_pt_l0;
    let boot_pt_l1 = &mut *boot_pt_l1;
    
    #[cfg(feature = "el2")]
    {
        use core::ptr::write_volatile;
        
        let uart_base = 0x0900_0000 as *mut u8;
        let msg1 = b"PT init start\r\n";
        for &byte in msg1 {
            unsafe { core::ptr::write_volatile(uart_base, byte); }
        }
        
        // L0[0]: 指向 L1
        let l1_paddr = pa!(boot_pt_l1.as_ptr() as usize).as_usize();
        boot_pt_l0[0] = A64PTE::new_table(pa!(l1_paddr));
        
        let msg2 = b"L0[0] set\r\n";
        for &byte in msg2 {
            unsafe { core::ptr::write_volatile(uart_base, byte); }
        }
        
        // L0[511]: 也指向 L1
        boot_pt_l0[511] = A64PTE::new_table(pa!(l1_paddr));
        
        let msg3 = b"L0[511] set\r\n";
        for &byte in msg3 {
            unsafe { core::ptr::write_volatile(uart_base, byte); }
        }
        
        // L1[0]: Device memory block (0x0 ~ 0x40000000)
        // 1GB block descriptor: bits [47:30] = output address bits [47:30]
        //let l1_0_desc: u64 = (0 << 30) | (0 << 2) | (0b00 << 6) | (0b00 << 8) | (1 << 10) | (1 << 1) | (1 << 0);
        let l1_0_desc: u64 = (0x401);
        unsafe { write_volatile(&mut boot_pt_l1[0] as *mut _ as *mut u64, l1_0_desc); }
        
        let msg4 = b"L1[0] set\r\n";
        for &byte in msg4 {
            unsafe { core::ptr::write_volatile(uart_base, byte); }
        }
        
        // L1[1]: Normal memory block (0x40000000 ~ 0x80000000)
        // 关键修复: 1GB block 的地址位于 [47:30]，但这些位在 descriptor 中就是 [47:30]
        // 0x40000000 = 0b0100_0000_0000_0000_0000_0000_0000_0000
        // bits [47:30] = 0b00_0000_0000_0000_01 = 1
        // 所以应该是 (1 << 30)
        //let l1_1_desc: u64 = (1u64 << 30) | (1 << 2) | (0b00 << 6) | (0b11 << 8) | (1 << 10) | (1 << 1) | (1 << 0);
        let l1_1_desc: u64 = (0x705) | (0x4000 << 16);
        unsafe { write_volatile(&mut boot_pt_l1[1] as *mut _ as *mut u64, l1_1_desc); }
        
        let msg5 = b"L1[1] set, PT done!\r\n";
        for &byte in msg5 {
            unsafe { core::ptr::write_volatile(uart_base, byte); }
        }
    }
    
    #[cfg(not(feature = "el2"))]
    {
        // EL1 模式:使用 TTBR0_EL1 和 TTBR1_EL1,只需映射低地址
        // 0x0000_0000_0000 ~ 0x0080_0000_0000, table
        boot_pt_l0[0] = A64PTE::new_table(pa!(boot_pt_l1.as_ptr() as usize));
        // 0x0000_0000_0000..0x0000_4000_0000, 1G block, device memory
        boot_pt_l1[0] = A64PTE::new_page(
            pa!(0),
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
            true,
        );
        // 0x0000_4000_0000..0x0000_8000_0000, 1G block, normal memory
        boot_pt_l1[1] = A64PTE::new_page(
            pa!(0x4000_0000),
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
            true,
        );
    }
}
