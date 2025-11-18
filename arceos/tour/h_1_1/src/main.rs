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
use vcpu::_run_guest;
use psci::PsciMessage;
use loader::load_vm_image;
use axhal::mem::PhysAddr;
use sysregs::{SYSREG, AArch64SysRegTrait, exception_class};

const VM_ENTRY: usize = 0x4020_0000;  // 1GB + 2MB，在用户地址空间内 (0-2GB)
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    ax_println!("Hypervisor ...");

    // A new address space for vm.
    let mut uspace = axmm::new_user_aspace().unwrap();

    // Load vm binary file into address space.
    if let Err(e) = load_vm_image("/sbin/skernel_aarch64", &mut uspace) {
        panic!("Cannot load app! {:?}", e);
    }

    // Setup context to prepare to enter guest mode.
    let mut ctx = VmCpuRegisters::default();
    prepare_guest_context(&mut ctx);

    // Setup pagetable for 2nd address mapping.
    let ept_root = uspace.page_table_root();
    prepare_vm_pgtable(ept_root);

    // Kick off vm and wait for it to exit.
    run_guest(&mut ctx);

    panic!("Hypervisor ok!");
}

fn prepare_vm_pgtable(ept_root: PhysAddr) {
    let vttbr = usize::from(ept_root);
    SYSREG.vttbr_el2.write_value(vttbr);

    let vtcr_el2 = 16 << 0 |  // TOSZ
                     (0b10 << 6) |   // SLo = Granule4KBLevel0
                     (0b11 << 8) |   // IRGN0 = NormalWBRAnWA
                     (0b11 << 10) |  // ORGN0 = NormalWBRAnWA
                     (0b11 << 12) |  // SH0 = Inner
                     (0b00 << 14) |  // TG0 = Granule4KB
                     (0b001 << 16);  // PS = 40 bits
    SYSREG.vtcr_el2.write_value(vtcr_el2);
}

fn run_guest(ctx: &mut VmCpuRegisters) {
    unsafe {
        _run_guest(ctx);
    }

    vmexit_handler(ctx)
}

fn vmexit_handler(ctx: &VmCpuRegisters) {
    let esr_el2 = SYSREG.esr_el2.read();
    let ec = (esr_el2 >> 26) & 0x3F;
    match ec as u32 {
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
    let mut hcr_el2: usize = 0;
    hcr_el2 |= 1 << 31; // RW
    hcr_el2 |= 1 << 0;  // VM
    hcr_el2 |= 1 << 19; // Trap SMC instructions to EL2
    ctx.guest_regs.hcr_el2 = hcr_el2;

    let mut spsr_el2: usize = 0;
    spsr_el2 |= 0b0101; // EL1h
    spsr_el2 |= 1 << 6; // F
    spsr_el2 |= 1 << 7; // I
    spsr_el2 |= 1 << 8; // A
    spsr_el2 |= 1 << 9; // D
    ctx.guest_regs.spsr_el2 = spsr_el2;

    ctx.guest_regs.elr_el2 = VM_ENTRY;
}
