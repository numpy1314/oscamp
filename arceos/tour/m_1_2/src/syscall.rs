#![allow(dead_code)]

use axhal::arch::TrapFrame;
use axhal::trap::{register_trap_handler, SYSCALL, PAGE_FAULT};
use axhal::mem::VirtAddr;
use axhal::paging::MappingFlags;
use axerrno::LinuxError;

const SYS_EXIT: usize = 93;

#[register_trap_handler(SYSCALL)]
fn handle_syscall(tf: &TrapFrame, syscall_num: usize) -> isize {
    ax_println!("handle_syscall ...");
    let ret = match syscall_num {
        SYS_EXIT => {
            ax_println!("[SYS_EXIT]: process is exiting ..");
            // AArch64: x0 is r[0]
            axtask::exit(tf.r[0] as _)
        },
        _ => {
            ax_println!("Unimplemented syscall: {}", syscall_num);
            -LinuxError::ENOSYS.code() as _
        }
    };
    ret
}

#[register_trap_handler(PAGE_FAULT)]
fn handle_page_fault(vaddr: VirtAddr, access_flags: MappingFlags, is_user: bool) -> bool {
    ax_println!(
        "[m_1_2] PAGE_FAULT at {:#x}, access={:#x}, is_user={} -> power off via PSCI",
        vaddr,
        access_flags.bits(),
        is_user,
    );

    #[cfg(target_arch = "aarch64")]
    crate::psci_system_off();

    #[allow(unreachable_code)]
    false
}
