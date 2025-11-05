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
const VM_ENTRY: usize = 0x8020_0000;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    unsafe {
        let uart_base = 0x0900_0000 as *mut u8;
        let msg = b"Hypervisor (AArch64) starting in EL2...\r\n";
        for &byte in msg {
            core::ptr::write_volatile(uart_base, byte);
        }
    }
    ax_println!("Hypervisor (AArch64) ...");
    ax_println!("Creating user address space...");
    let mut uspace = match axmm::new_user_aspace() {
        Ok(us) => {
            ax_println!("User address space created successfully");
            us
        },
        Err(e) => {
            panic!("Failed to create user address space: {:?}", e);
        }
    };
    if let Err(e) = load_vm_image("/sbin/skernel", &mut uspace) {
        panic!("Cannot load app! {:?}", e);
    }
    ax_println!("Initializing guest context...");
    let mut ctx = VmCpuRegisters::default();
    prepare_guest_context(&mut ctx);
    ax_println!("Setting up Stage-2 page table...");
    let ept_root = uspace.page_table_root();
    prepare_vm_pgtable(ept_root);
    ax_println!("Starting guest VM...");
    run_guest(&mut ctx);
    panic!("Hypervisor ok!");
}
fn prepare_vm_pgtable(ept_root: PhysAddr) {
    let vttbr = usize::from(ept_root);
    SYSREG.vttbr_el2.write_value(vttbr);
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
                        ax_println!("Shutdown vm normally!");
                    },
                    PsciMessage::System(psci::SystemFunction::SystemReset { .. }) => {
                        ax_println!("Reset vm normally!");
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
    hcr_el2 |= 1 << 31;
    hcr_el2 |= 1 << 0;
    SYSREG.hcr_el2.write_value(hcr_el2);
    ctx.guest_regs.hcr_el2 = hcr_el2;
    let mut spsr_el2: usize = 0;
    spsr_el2 |= 0b0101;
    spsr_el2 |= 1 << 6;
    spsr_el2 |= 1 << 7;
    spsr_el2 |= 1 << 8;
    spsr_el2 |= 1 << 9;
    ctx.guest_regs.spsr_el2 = spsr_el2;
    ctx.guest_regs.elr_el2 = VM_ENTRY;
}
