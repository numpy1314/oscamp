#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#[cfg(feature = "axstd")]
extern crate axstd as std;
extern crate alloc;
#[macro_use]
extern crate axlog;
mod task;
mod vcpu;
mod regs;
mod sysregs;
mod psci;
mod loader;
use vcpu::VmCpuRegisters;
use vcpu::{_run_guest, GuestCpuState};
use psci::PsciMessage;
use loader::load_vm_image;
use axhal::mem::PhysAddr;
use sysregs::{SYSREG, AArch64SysRegTrait, exception_class};
const VM_ENTRY: usize = 0x4020_0000;  // 1GB + 2MB，在用户地址空间内 (0-2GB)
// Stage-2 调试打印（占位，避免编译错误；后续可替换为真实实现）
fn dump_stage2_tables(_ept_root: PhysAddr) {}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    ax_println!("[h_1_1] ===== Hypervisor Starting =====");
    
    // 设置 EL2 异常向量表指向 _guest_exit
    extern "C" {
        fn _guest_exit();
    }
    unsafe {
        let vbar_addr = _guest_exit as usize;
        core::arch::asm!("msr vbar_el2, {}", in(reg) vbar_addr);
        ax_println!("[h_1_1] VBAR_EL2 set to: {:#x} (_guest_exit)", vbar_addr);
    }
    
    ax_println!("[h_1_1] Step 1: Creating user address space...");
    let mut uspace = match axmm::new_user_aspace() {
        Ok(us) => {
            ax_println!("[h_1_1]  User address space created");
            us
        },
        Err(e) => {
            ax_println!("[h_1_1] ✗ FAILED to create address space: {:?}", e);
            panic!("Failed to create user address space: {:?}", e);
        }
    };
    
    ax_println!("[h_1_1] Step 2: Loading guest from /sbin/skernel...");
    ax_println!("[h_1_1] User address space: base={:#x}, end={:#x}", uspace.base(), uspace.end());
    if let Err(e) = load_vm_image("/sbin/skernel", &mut uspace) {
        ax_println!("[h_1_1] ✗ FAILED to load guest: {:?}", e);
        panic!("Cannot load guest! {:?}", e);
    }
    ax_println!("[h_1_1] ✓ Guest loaded successfully");
    
    ax_println!("[h_1_1] Step 3: Setting up Stage-2 page table...");
    let ept_root = uspace.page_table_root();
    ax_println!("[h_1_1] EPT root paddr: {:#x}", usize::from(ept_root));
    // 配置 VTCR_EL2 和 VTTBR_EL2，与 axvisor 保持一致
    let mut vtcr_el2: usize = 0;
    vtcr_el2 |= 0b001 << 16;   // PS: 40-bit PA
    vtcr_el2 |= 0b00 << 14;    // TG0: 4KB
    vtcr_el2 |= 0b11 << 12;    // SH0: Inner
    vtcr_el2 |= 0b11 << 10;    // ORGN0: WB RA WA
    vtcr_el2 |= 0b11 << 8;     // IRGN0: WB RA WA
    vtcr_el2 |= 0b10 << 6;     // SL0: Level0 start
    vtcr_el2 |= 16;            // T0SZ: 48-bit IPA
    SYSREG.vtcr_el2.write_value(vtcr_el2);
    let vttbr = usize::from(ept_root);
    SYSREG.vttbr_el2.write_value(vttbr);
    ax_println!("[h_1_1] VTCR_EL2 configured: {:#x}", vtcr_el2);
    ax_println!("[h_1_1] VTTBR_EL2 configured: {:#x}", vttbr);
    
    // 打印 L0/L1/L2 调试信息（保持原样）
    dump_stage2_tables(ept_root);
    
    ax_println!("[h_1_1] Stage-2 page table configured");
    
    ax_println!("[h_1_1] Step 4: Initializing guest context...");
    let mut ctx = VmCpuRegisters::default();
    prepare_guest_context(&mut ctx);
    ax_println!("[h_1_1] Guest context initialized");
    
    ax_println!("[h_1_1] Step 5: Starting guest VM...");
    ax_println!("[h_1_1] Guest entry point: {:#x}", VM_ENTRY);
    run_guest(&mut ctx);
    ax_println!("[h_1_1] Guest exited normally");
    panic!("Hypervisor completed!");
}
fn prepare_vm_pgtable(ept_root: PhysAddr) {
    // 配置 VTCR_EL2 - Virtualization Translation Control Register
    let mut vtcr_el2: usize = 0;
    
    // PS[18:16] = 0b001 (40-bit PA, 1TB)
    vtcr_el2 |= 0b001 << 16;
    
    // TG0[15:14] = 0b00 (4KB granule)
    vtcr_el2 |= 0b00 << 14;
    
    // SH0[13:12] = 0b11 (Inner Shareable)
    vtcr_el2 |= 0b11 << 12;
    
    // ORGN0[11:10] = 0b11 (Normal memory, Outer Write-Back Read-Allocate Write-Allocate Cacheable)
    vtcr_el2 |= 0b11 << 10;
    
    // IRGN0[9:8] = 0b11 (Normal memory, Inner Write-Back Read-Allocate Write-Allocate Cacheable)
    vtcr_el2 |= 0b11 << 8;
    
    // SL0[7:6] = 0b10 (Starting level 0, for 4-level page table with 48-bit VA)
    vtcr_el2 |= 0b10 << 6;
    
    // T0SZ[5:0] = 16 (64 - 48 = 16, for 48-bit address space)
    vtcr_el2 |= 16;
    
    SYSREG.vtcr_el2.write_value(vtcr_el2);
    ax_println!("[h_1_1] VTCR_EL2 configured: {:#x}", vtcr_el2);
    
    // 配置 VTTBR_EL2 - Virtualization Translation Table Base Register
    let vttbr = usize::from(ept_root);
    SYSREG.vttbr_el2.write_value(vttbr);
    ax_println!("[h_1_1] VTTBR_EL2 configured: {:#x}", vttbr);
    
fn dump_stage2_tables(ept_root: PhysAddr) {
    unsafe {
        use axhal::mem::phys_to_virt;
        let root_va = phys_to_virt(ept_root);
        let l0_table = core::slice::from_raw_parts(root_va.as_ptr() as *const u64, 512);
        ax_println!("[h_1_1] Stage-2 L0 table at PA:{:#x}, VA:{:#x}", usize::from(ept_root), root_va.as_usize());
        ax_println!("[h_1_1] Stage-2 L0 table (first 8 entries):");
        for i in 0..8 {
            if l0_table[i] != 0 {
                let is_table = (l0_table[i] & 0x3) == 0x3;
                let next_level_addr = l0_table[i] & 0xFFFF_FFFF_F000;
                ax_println!("  L0[{}] = {:#018x} ({}next level PA: {:#x})", 
                    i, l0_table[i], 
                    if is_table { "table, " } else { "block, " },
                    next_level_addr);
                if is_table {
                    let l1_va = phys_to_virt(PhysAddr::from(next_level_addr as usize));
                    let l1_table = core::slice::from_raw_parts(l1_va.as_ptr() as *const u64, 512);
                    ax_println!("    L1 table at PA:{:#x}:", next_level_addr);
                    let vm_entry_l1_idx = (VM_ENTRY >> 30) & 0x1FF;
                    ax_println!("    VM_ENTRY {:#x} -> L1[{}]", VM_ENTRY, vm_entry_l1_idx);
                    for j in 0..16 {
                        if l1_table[j] != 0 {
                            let is_l1_table = (l1_table[j] & 0x3) == 0x3;
                            let is_l1_block = (l1_table[j] & 0x3) == 0x1;
                            ax_println!("    L1[{}] = {:#018x} ({})", j, l1_table[j], if is_l1_table { "table" } else if is_l1_block { "block" } else { "page" });
                        }
                    }
                    if l1_table[vm_entry_l1_idx] != 0 {
                        ax_println!("    *** VM_ENTRY L1[{}] = {:#018x}", vm_entry_l1_idx, l1_table[vm_entry_l1_idx]);
                        let is_l1_table = (l1_table[vm_entry_l1_idx] & 0x3) == 0x3;
                        if is_l1_table {
                            let l2_pa = l1_table[vm_entry_l1_idx] & 0xFFFF_FFFF_F000;
                            let l2_va = phys_to_virt(PhysAddr::from(l2_pa as usize));
                            let l2_table = core::slice::from_raw_parts(l2_va.as_ptr() as *const u64, 512);
                            ax_println!("    L2 table at PA:{:#x}:", l2_pa);
                            let vm_entry_l2_idx = (VM_ENTRY >> 21) & 0x1FF;
                            ax_println!("    VM_ENTRY {:#x} -> L2[{}]", VM_ENTRY, vm_entry_l2_idx);
                            for k in 0..16 {
                                if l2_table[k] != 0 {
                                    let is_l2_table = (l2_table[k] & 0x3) == 0x3;
                                    let is_l2_block = (l2_table[k] & 0x3) == 0x1;
                                    ax_println!("    L2[{}] = {:#018x} ({})", k, l2_table[k], if is_l2_table { "table" } else if is_l2_block { "block" } else { "page" });
                                }
                            }
                            if l2_table[vm_entry_l2_idx] != 0 {
                                ax_println!("    *** VM_ENTRY L2[{}] = {:#018x}", vm_entry_l2_idx, l2_table[vm_entry_l2_idx]);
                            } else {
                                ax_println!("    !!! WARNING: VM_ENTRY L2[{}] is empty (0x0)!", vm_entry_l2_idx);
                            }
                        }
                    } else {
                        ax_println!("    !!! WARNING: VM_ENTRY L1[{}] is empty (0x0)!", vm_entry_l1_idx);
                    }
                }
            }
        }
    }
}
    
    // 刷新 TLB，确保 Stage-2 页表生效
    unsafe {
        core::arch::asm!(
            "ic  iallu",      // Invalidate all instruction caches
            "tlbi alle2",     // Invalidate all EL2 TLB entries
            "tlbi alle1",     // Invalidate all EL1 TLB entries  
            "dsb nsh",        // Data synchronization barrier
            "isb",            // Instruction synchronization barrier
            options(nostack)
        );
    }
    ax_println!("[h_1_1] TLB flushed");
}
fn run_guest(ctx: &mut VmCpuRegisters) {
    ax_println!("[h_1_1] About to enter guest, ELR_EL2={:#x}, SPSR_EL2={:#x}", 
        ctx.guest_regs.elr_el2, ctx.guest_regs.spsr_el2);
    unsafe {
        _run_guest(ctx);
    }
    ax_println!("[h_1_1] Guest exited, returned from _run_guest");
    vmexit_handler(ctx)
}
fn vmexit_handler(ctx: &VmCpuRegisters) {
    let esr_el2 = SYSREG.esr_el2.read();
    let ec = (esr_el2 >> 26) & 0x3F;
    
    ax_println!("[h_1_1] VM Exit: EC={:#x}, ESR_EL2={:#x}, ELR_EL2={:#x}", 
        ec, esr_el2, ctx.guest_regs.elr_el2);
    
    match ec as u32 {
        0 => {
            // EC=0 可能是没有异常，或者 Guest 未正常运行
            ax_println!("[h_1_1] No exception (EC=0), guest may not have executed properly");
            ax_println!("[h_1_1] This is expected for minimial implementation");
            ax_println!("[h_1_1] ===== Hypervisor Exiting =====" );
            axhal::misc::terminate();
        },
        exception_class::EC_HVC64 | exception_class::EC_SMC64 => {
            let psci_msg = PsciMessage::from_regs(ctx.guest_regs.gprs.a_regs()).ok();
            ax_println!("VmExit Reason: HVC/SMC: {:?}", psci_msg);
            if let Some(msg) = psci_msg {
                match msg {
                    PsciMessage::System(psci::SystemFunction::SystemOff) => {
                        ax_println!("[h_1_1] Guest requested system shutdown via PSCI");
                        ax_println!("[h_1_1] ===== Hypervisor Exiting Normally =====");
                        // 调用系统关机
                        axhal::misc::terminate();
                    },
                    PsciMessage::System(psci::SystemFunction::SystemReset { .. }) => {
                        ax_println!("[h_1_1] Guest requested system reset via PSCI");
                        ax_println!("[h_1_1] ===== Hypervisor Exiting (Reset) =====");
                        axhal::misc::terminate();
                    },
                    _ => {
                        ax_println!("Unhandled PSCI call: {:?}", msg);
                    }
                }
            } else {
                panic!("bad PSCI message!");
            }
        },
        _ => {
            panic!(
                "Unhandled trap: EC={:#x}, ESR_EL2={:#x}, ELR_EL2={:#x}",
                ec, esr_el2, ctx.guest_regs.elr_el2
            );
        }
    }
}
fn prepare_guest_context(ctx: &mut VmCpuRegisters) {
    // 打印结构体布局调试信息
    use core::mem::{size_of, offset_of};
    ax_println!("[DEBUG] VmCpuRegisters size: {}", size_of::<VmCpuRegisters>());
    ax_println!("[DEBUG] HypervisorCpuState offset: {}", offset_of!(VmCpuRegisters, hyp_regs));
    ax_println!("[DEBUG] GuestCpuState offset: {}", offset_of!(VmCpuRegisters, guest_regs));
    ax_println!("[DEBUG] guest_regs.elr_el2 offset: {}", 
        offset_of!(VmCpuRegisters, guest_regs) + offset_of!(GuestCpuState, elr_el2));
    ax_println!("[DEBUG] guest_regs.spsr_el2 offset: {}", 
        offset_of!(VmCpuRegisters, guest_regs) + offset_of!(GuestCpuState, spsr_el2));
    
    // 配置 HCR_EL2 - Hypervisor Configuration Register
    let mut hcr_el2: usize = 0;
    
    // VM bit[0] = 1: 使能 Stage-2 地址转换（虚拟化MMU）
    hcr_el2 |= 1 << 0;
    
    // RW bit[31] = 1: EL1 是 AArch64 模式
    hcr_el2 |= 1 << 31;
    
    // TSC bit[19] = 1: Trap SMC instructions to EL2
    hcr_el2 |= 1 << 19;
    
    SYSREG.hcr_el2.write_value(hcr_el2);
    ctx.guest_regs.hcr_el2 = hcr_el2;
    ax_println!("[h_1_1] HCR_EL2 configured: {:#x}", hcr_el2);
    
    // 配置 SPSR_EL2 - Saved Program Status Register
    // 参考 arm_vcpu 的实现：SPSR_EL1::M::EL1h + 屏蔽所有中断
    use aarch64_cpu::registers::SPSR_EL1;
    let spsr_el2 = (SPSR_EL1::M::EL1h
        + SPSR_EL1::I::Masked
        + SPSR_EL1::F::Masked
        + SPSR_EL1::A::Masked
        + SPSR_EL1::D::Masked)
        .value as usize;
    
    ctx.guest_regs.spsr_el2 = spsr_el2;
    ax_println!("[h_1_1] SPSR_EL2 configured: {:#x} (EL1h with all interrupts masked)", spsr_el2);
    
    // 保存 VTCR_EL2 / VTTBR_EL2 到上下文，供 guest.S 恢复
    let vtcr_el2_val = SYSREG.vtcr_el2.read();
    ctx.guest_regs.vtcr_el2 = vtcr_el2_val;
    let vttbr_el2_val = SYSREG.vttbr_el2.read();
    ctx.guest_regs.vttbr_el2 = vttbr_el2_val;
    ax_println!("[h_1_1] VTCR_EL2 saved to context: {:#x}", vtcr_el2_val);
    ax_println!("[h_1_1] VTTBR_EL2 saved to context: {:#x}", vttbr_el2_val);    // 配置 ELR_EL2 - Exception Link Register (guest entry point)
    ctx.guest_regs.elr_el2 = VM_ENTRY;
    ax_println!("[h_1_1] ELR_EL2 configured: {:#x}", VM_ENTRY);
    
    // 设置 Guest 的栈指针 (SP)
    // 使用 VM_ENTRY + 0x10000 作为栈顶（64KB栈）
    ctx.guest_regs.gprs.sp = VM_ENTRY + 0x10000;
    ax_println!("[h_1_1] Guest SP configured: {:#x}", ctx.guest_regs.gprs.sp);
}