use crate::mem::MemRegion;
use page_table_entry::{GenericPTE, MappingFlags, aarch64::A64PTE};

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
    
    // L0[0]: 0x0000_0000_0000 ~ 0x0080_0000_0000 (512GB), 指向 L1 表
    boot_pt_l0[0] = A64PTE::new_table(pa!(boot_pt_l1.as_ptr() as usize));
    
    // 映射 0-512GB，使用 1GB block
    // L1[0]: 0-1GB (Device memory, UART/GIC/VirtIO 等外设)
    boot_pt_l1[0] = A64PTE::new_page(
        pa!(0),
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::DEVICE,
        true,  // 1GB block
    );
    
    // L1[1..511]: 1GB-512GB (Normal memory + PCI memory)
    // 包括 RAM (1GB-2GB) 和 PCI ECAM (256GB+)
    for i in 1..512 {
        boot_pt_l1[i] = A64PTE::new_page(
            pa!(i * 0x4000_0000),  // i * 1GB
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
            true,  // 1GB block
        );
    }
}
