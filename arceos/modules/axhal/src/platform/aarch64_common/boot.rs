use aarch64_cpu::{asm, asm::barrier, registers::*};
use core::ptr::addr_of_mut;
use page_table_entry::aarch64::{MemAttr, A64PTE};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

use axconfig::TASK_STACK_SIZE;

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; TASK_STACK_SIZE] = [0; TASK_STACK_SIZE];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_L0: [A64PTE; 512] = [A64PTE::empty(); 512];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_L1: [A64PTE; 512] = [A64PTE::empty(); 512];

unsafe fn switch_to_el1() {
    SPSel.write(SPSel::SP::ELx);
    SP_EL0.set(0);
    let current_el = CurrentEL.read(CurrentEL::EL);
    if current_el >= 2 {
        if current_el == 3 {
            // Set EL2 to 64bit and enable the HVC instruction.
            SCR_EL3.write(
                SCR_EL3::NS::NonSecure + SCR_EL3::HCE::HvcEnabled + SCR_EL3::RW::NextELIsAarch64,
            );
            // Set the return address and exception level.
            SPSR_EL3.write(
                SPSR_EL3::M::EL1h
                    + SPSR_EL3::D::Masked
                    + SPSR_EL3::A::Masked
                    + SPSR_EL3::I::Masked
                    + SPSR_EL3::F::Masked,
            );
            ELR_EL3.set(LR.get());
        }
        // Disable EL1 timer traps and the timer offset.
        CNTHCTL_EL2.modify(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
        CNTVOFF_EL2.set(0);
        // Set EL1 to 64bit.
        HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
        // Set the return address and exception level.
        SPSR_EL2.write(
            SPSR_EL2::M::EL1h
                + SPSR_EL2::D::Masked
                + SPSR_EL2::A::Masked
                + SPSR_EL2::I::Masked
                + SPSR_EL2::F::Masked,
        );
        core::arch::asm!(
            "
            mov     x8, sp
            msr     sp_el1, x8"
        );
        ELR_EL2.set(LR.get());
        asm::eret();
    }
}

// 为虚拟化保持在 EL2
unsafe fn stay_in_el2() {
    // 早期调试输出
    let uart_base = 0x0900_0000 as *mut u8;
    let msg = b"stay_in_el2 called\r\n";
    for &byte in msg {
        core::ptr::write_volatile(uart_base, byte);
    }
    
    SPSel.write(SPSel::SP::ELx);
    SP_EL0.set(0);
    let current_el = CurrentEL.read(CurrentEL::EL);
    if current_el == 3 {
        // Set EL2 to 64bit and enable the HVC instruction.
        SCR_EL3.write(
            SCR_EL3::NS::NonSecure + SCR_EL3::HCE::HvcEnabled + SCR_EL3::RW::NextELIsAarch64,
        );
        // Set the return address and exception level to EL2.
        SPSR_EL3.write(
            SPSR_EL3::M::EL2h
                + SPSR_EL3::D::Masked
                + SPSR_EL3::A::Masked
                + SPSR_EL3::I::Masked
                + SPSR_EL3::F::Masked,
        );
        ELR_EL3.set(LR.get());
        asm::eret();
    }
    // 如果已经在 EL2,不做任何事
    // 配置 EL2 的基本设置
    if current_el >= 2 {
        // Disable EL1 timer traps
        CNTHCTL_EL2.modify(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
        CNTVOFF_EL2.set(0);
        // 关键: 显式禁用 Stage-2 转换 (VM=0)
        // HCR_EL2.RW = 1 (EL1 is AArch64)
        // HCR_EL2.VM = 0 (禁用 Stage-2 MMU)
        let hcr_val: u64 = (1 << 31);  // 只设置 RW 位
        unsafe {
            core::arch::asm!("msr hcr_el2, {}", in(reg) hcr_val);
            core::arch::asm!("isb");
        }
        
        // 关键: 在 MMU 开启前设置 VBAR_EL2 为物理地址
        extern "C" {
            fn exception_vector_base_el2();
        }
        let vbar_virt = exception_vector_base_el2 as usize;
        // 转换为物理地址: 减去 PHYS_VIRT_OFFSET
        let vbar_phys = vbar_virt - axconfig::PHYS_VIRT_OFFSET;
        core::arch::asm!("msr vbar_el2, {}", in(reg) vbar_phys);
        
        let msg2 = b"VBAR_EL2 set to phys addr (early)\r\n";
        for &byte in msg2 {
            core::ptr::write_volatile(uart_base, byte);
        }
    }
}

unsafe fn init_mmu() {
    // 立即输出
    let uart_base = 0x0900_0000 as *mut u8;
    let msg = b"Entered init_mmu\r\n";
    for &byte in msg {
        core::ptr::write_volatile(uart_base, byte);
    }
    
    let current_el = CurrentEL.read(CurrentEL::EL);
    
    let msg2 = b"Read CurrentEL\r\n";
    for &byte in msg2 {
        core::ptr::write_volatile(uart_base, byte);
    }
    
    if current_el == 2 {
        let msg3 = b"In EL2 branch\r\n";
        for &byte in msg3 {
            core::ptr::write_volatile(uart_base, byte);
        }
        
        // EL2 MMU 初始化
        MAIR_EL2.set(MemAttr::MAIR_VALUE);
        
        let msg4 = b"MAIR_EL2 set\r\n";
        for &byte in msg4 {
            core::ptr::write_volatile(uart_base, byte);
        }
        
        // TCR_EL2: T0SZ=16 (48-bit VA), 与 EL1 一致
        let tcr_val: u64 = (16 << 0) | (0b01 << 8) | (0b01 << 10) | (0b11 << 12) | (0b00 << 14) | (0b101 << 16);
        core::arch::asm!("msr tcr_el2, {}", in(reg) tcr_val);
        
        let msg5 = b"TCR_EL2 set\r\n";
        for &byte in msg5 {
            core::ptr::write_volatile(uart_base, byte);
        }
        
        barrier::isb(barrier::SY);
        
        let msg6 = b"After ISB\r\n";
        for &byte in msg6 {
            core::ptr::write_volatile(uart_base, byte);
        }
        
        // TTBR0_EL2
        let root_paddr = pa!(BOOT_PT_L0.as_ptr() as usize).as_usize() as u64;
        
        let msg7 = b"Got root_paddr\r\n";
        for &byte in msg7 {
            core::ptr::write_volatile(uart_base, byte);
        }
        
        TTBR0_EL2.set(root_paddr);
        
        let msg8 = b"TTBR0_EL2 set\r\n";
        for &byte in msg8 {
            core::ptr::write_volatile(uart_base, byte);
        }
        
        // TLB Flush
        core::arch::asm!("tlbi alle2", "dsb sy", "isb");
        
        let msg9 = b"TLB flushed\r\n";
        for &byte in msg9 {
            core::ptr::write_volatile(uart_base, byte);
        }
        
        // 逐条调试 MMU 开启
        let msg10 = b"Preparing to enable MMU...\r\n";
        for &byte in msg10 {
            core::ptr::write_volatile(uart_base, byte);
        }
        
        let msg12 = b"Enabling MMU in ASM block...\r\n";
        for &byte in msg12 {
            core::ptr::write_volatile(uart_base, byte);
        }
        
        // 关键: 在一个汇编块中完成所有操作，并确保 MMU 开启后的指令可以被访问
        core::arch::asm!(
            "mrs x0, sctlr_el2",
            "bic x0, x0, #(1 << 2)",     // 禁用 D-cache
            "bic x0, x0, #(1 << 12)",    // 禁用 I-cache  
            "orr x0, x0, #1",            // 开启 MMU
            "msr sctlr_el2, x0",         // 关键指令
            "isb",                        // 同步
            "nop",                        // 确保后续指令可获取
            "nop",
            "nop",
            "nop",
            out("x0") _,
        );
        
        let msg13 = b"MMU ENABLED!!!\r\n";
        for &byte in msg13 {
            core::ptr::write_volatile(uart_base, byte);
        }
    } else {
        // EL1 MMU 初始化 (原有代码)
        MAIR_EL1.set(MemAttr::MAIR_VALUE);

        // Enable TTBR0 and TTBR1 walks, page size = 4K, vaddr size = 48 bits, paddr size = 40 bits.
        let tcr_flags0 = TCR_EL1::EPD0::EnableTTBR0Walks
            + TCR_EL1::TG0::KiB_4
            + TCR_EL1::SH0::Inner
            + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::T0SZ.val(16);
        let tcr_flags1 = TCR_EL1::EPD1::EnableTTBR1Walks
            + TCR_EL1::TG1::KiB_4
            + TCR_EL1::SH1::Inner
            + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::T1SZ.val(16);
        TCR_EL1.write(TCR_EL1::IPS::Bits_48 + tcr_flags0 + tcr_flags1);
        barrier::isb(barrier::SY);

        // Set both TTBR0 and TTBR1
        let root_paddr = pa!(BOOT_PT_L0.as_ptr() as usize).as_usize() as _;
        TTBR0_EL1.set(root_paddr);
        TTBR1_EL1.set(root_paddr);

        // Flush the entire TLB
        crate::arch::flush_tlb(None);

        // Enable the MMU and turn on I-cache and D-cache
        SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);
        barrier::isb(barrier::SY);
    }
}

unsafe fn enable_fp() {
    if cfg!(feature = "fp_simd") {
        let current_el = CurrentEL.read(CurrentEL::EL);
        if current_el == 2 {
            // EL2: 配置 CPTR_EL2 以允许 FP/SIMD
            // CPTR_EL2.TFP (bit 10) = 0: 不 trap FP/SIMD
            core::arch::asm!(
                "mrs x0, cptr_el2",
                "bic x0, x0, #(1 << 10)",  // Clear TFP bit
                "msr cptr_el2, x0",
                out("x0") _,
            );
        } else {
            // EL1: 配置 CPACR_EL1
            CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
        }
        barrier::isb(barrier::SY);
    }
}

unsafe fn init_boot_page_table() {
    crate::platform::mem::init_boot_page_table(addr_of_mut!(BOOT_PT_L0), addr_of_mut!(BOOT_PT_L1));
    
    // 调试: 页表初始化后的输出
    let uart_base = 0x0900_0000 as *mut u8;
    let msg = b"Back from init_boot_page_table\r\n";
    for &byte in msg {
        core::ptr::write_volatile(uart_base, byte);
    }
}

/// The earliest entry point for the primary CPU.
#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn _start() -> ! {
    // PC = 0x8_0000
    // X0 = dtb
    #[cfg(feature = "el2")]
    core::arch::asm!("
        mrs     x19, mpidr_el1
        and     x19, x19, #0xffffff     // get current CPU id
        mov     x20, x0                 // save DTB pointer

        adrp    x8, {boot_stack}        // setup boot stack
        add     x8, x8, {boot_stack_size}
        mov     sp, x8

        bl      {stay_in_el2}           // stay in EL2 for virtualization
        bl      {enable_fp}             // enable fp/neon  
        bl      {init_boot_page_table}
        bl      {init_mmu}              // setup MMU

        mov     x8, {phys_virt_offset}  // set SP to the high address
        add     sp, sp, x8

        mov     x0, x19                 // call rust_entry(cpu_id, dtb)
        mov     x1, x20
        ldr     x8, ={entry}
        blr     x8
        b      .",
        stay_in_el2 = sym stay_in_el2,
        init_boot_page_table = sym init_boot_page_table,
        init_mmu = sym init_mmu,
        enable_fp = sym enable_fp,
        boot_stack = sym BOOT_STACK,
        boot_stack_size = const TASK_STACK_SIZE,
        phys_virt_offset = const axconfig::PHYS_VIRT_OFFSET,
        entry = sym crate::platform::rust_entry,
        options(noreturn),
    );
    
    #[cfg(not(feature = "el2"))]
    core::arch::asm!("
        mrs     x19, mpidr_el1
        and     x19, x19, #0xffffff     // get current CPU id
        mov     x20, x0                 // save DTB pointer

        adrp    x8, {boot_stack}        // setup boot stack
        add     x8, x8, {boot_stack_size}
        mov     sp, x8

        bl      {switch_to_el1}         // switch to EL1
        bl      {enable_fp}             // enable fp/neon
        bl      {init_boot_page_table}
        bl      {init_mmu}              // setup MMU

        mov     x8, {phys_virt_offset}  // set SP to the high address
        add     sp, sp, x8

        mov     x0, x19                 // call rust_entry(cpu_id, dtb)
        mov     x1, x20
        ldr     x8, ={entry}
        blr     x8
        b      .",
        switch_to_el1 = sym switch_to_el1,
        init_boot_page_table = sym init_boot_page_table,
        init_mmu = sym init_mmu,
        enable_fp = sym enable_fp,
        boot_stack = sym BOOT_STACK,
        boot_stack_size = const TASK_STACK_SIZE,
        phys_virt_offset = const axconfig::PHYS_VIRT_OFFSET,
        entry = sym crate::platform::rust_entry,
        options(noreturn),
    )
}

/// The earliest entry point for the secondary CPUs.
#[cfg(feature = "smp")]
#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn _start_secondary() -> ! {
    core::arch::asm!("
        mrs     x19, mpidr_el1
        and     x19, x19, #0xffffff     // get current CPU id

        mov     sp, x0
        bl      {switch_to_el1}
        bl      {init_mmu}
        bl      {enable_fp}

        mov     x8, {phys_virt_offset}  // set SP to the high address
        add     sp, sp, x8

        mov     x0, x19                 // call rust_entry_secondary(cpu_id)
        ldr     x8, ={entry}
        blr     x8
        b      .",
        switch_to_el1 = sym switch_to_el1,
        init_mmu = sym init_mmu,
        enable_fp = sym enable_fp,
        phys_virt_offset = const axconfig::PHYS_VIRT_OFFSET,
        entry = sym crate::platform::rust_entry_secondary,
        options(noreturn),
    )
}
