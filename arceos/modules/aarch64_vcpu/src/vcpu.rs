use core::arch::global_asm;
use core::mem::size_of;
use memoffset::offset_of;
use super::regs::GeneralPurposeRegisters;
use super::sysregs::exception_class;
use super::psci::{self, PsciMessage};

use axerrno::AxResult;
use memory_addr::{VirtAddr, PhysAddr};
use axhal::paging::MappingFlags;

/// Guest physical address.
pub type GuestPhysAddr = VirtAddr;
/// Host physical address.
pub type HostPhysAddr = PhysAddr;

#[derive(Default)]
#[repr(C)]
struct HypervisorCpuState {
    gprs: GeneralPurposeRegisters,
    elr_el2: usize,
    spsr_el2: usize,
    sp_el2: usize,
}
#[derive(Default)]
#[repr(C)]
pub struct GuestCpuState {
    pub gprs: GeneralPurposeRegisters,
    pub elr_el2: usize,
    pub spsr_el2: usize,
    pub hcr_el2: usize,
    pub vtcr_el2: usize,
    pub vttbr_el2: usize,
}
#[derive(Default)]
#[repr(C)]
pub struct VmCpuRegisters {
    pub hyp_regs: HypervisorCpuState,
    pub guest_regs: GuestCpuState,
}
#[allow(dead_code)]
const fn hyp_gpr_offset(index: usize) -> usize {
    offset_of!(VmCpuRegisters, hyp_regs)
        + offset_of!(HypervisorCpuState, gprs)
        + offset_of!(GeneralPurposeRegisters, x)
        + index * size_of::<usize>()
}
#[allow(dead_code)]
const fn hyp_sp_offset() -> usize {
    offset_of!(VmCpuRegisters, hyp_regs)
        + offset_of!(HypervisorCpuState, gprs)
        + offset_of!(GeneralPurposeRegisters, sp)
}
#[allow(dead_code)]
const fn guest_gpr_offset(index: usize) -> usize {
    offset_of!(VmCpuRegisters, guest_regs)
        + offset_of!(GuestCpuState, gprs)
        + offset_of!(GeneralPurposeRegisters, x)
        + index * size_of::<usize>()
}
#[allow(dead_code)]
const fn guest_sp_offset() -> usize {
    offset_of!(VmCpuRegisters, guest_regs)
        + offset_of!(GuestCpuState, gprs)
        + offset_of!(GeneralPurposeRegisters, sp)
}
#[allow(unused_macros)]
macro_rules! hyp_csr_offset {
    ($reg:tt) => {
        offset_of!(VmCpuRegisters, hyp_regs) + offset_of!(HypervisorCpuState, $reg)
    };
}
#[allow(unused_macros)]
macro_rules! guest_csr_offset {
    ($reg:tt) => {
        offset_of!(VmCpuRegisters, guest_regs) + offset_of!(GuestCpuState, $reg)
    };
}
global_asm!(
    include_str!("guest.S"),
    hyp_x1 = const hyp_gpr_offset(1),
    hyp_x2 = const hyp_gpr_offset(2),
    hyp_x3 = const hyp_gpr_offset(3),
    hyp_x4 = const hyp_gpr_offset(4),
    hyp_x5 = const hyp_gpr_offset(5),
    hyp_x6 = const hyp_gpr_offset(6),
    hyp_x7 = const hyp_gpr_offset(7),
    hyp_x8 = const hyp_gpr_offset(8),
    hyp_x9 = const hyp_gpr_offset(9),
    hyp_x10 = const hyp_gpr_offset(10),
    hyp_x11 = const hyp_gpr_offset(11),
    hyp_x12 = const hyp_gpr_offset(12),
    hyp_x13 = const hyp_gpr_offset(13),
    hyp_x14 = const hyp_gpr_offset(14),
    hyp_x15 = const hyp_gpr_offset(15),
    hyp_x16 = const hyp_gpr_offset(16),
    hyp_x17 = const hyp_gpr_offset(17),
    hyp_x18 = const hyp_gpr_offset(18),
    hyp_x19 = const hyp_gpr_offset(19),
    hyp_x20 = const hyp_gpr_offset(20),
    hyp_x21 = const hyp_gpr_offset(21),
    hyp_x22 = const hyp_gpr_offset(22),
    hyp_x23 = const hyp_gpr_offset(23),
    hyp_x24 = const hyp_gpr_offset(24),
    hyp_x25 = const hyp_gpr_offset(25),
    hyp_x26 = const hyp_gpr_offset(26),
    hyp_x27 = const hyp_gpr_offset(27),
    hyp_x28 = const hyp_gpr_offset(28),
    hyp_x29 = const hyp_gpr_offset(29),
    hyp_x30 = const hyp_gpr_offset(30),
    hyp_sp = const hyp_sp_offset(),
    hyp_elr_el2 = const hyp_csr_offset!(elr_el2),
    hyp_spsr_el2 = const hyp_csr_offset!(spsr_el2),
    guest_x0 = const guest_gpr_offset(0),
    guest_x1 = const guest_gpr_offset(1),
    guest_x2 = const guest_gpr_offset(2),
    guest_x3 = const guest_gpr_offset(3),
    guest_x4 = const guest_gpr_offset(4),
    guest_x5 = const guest_gpr_offset(5),
    guest_x6 = const guest_gpr_offset(6),
    guest_x7 = const guest_gpr_offset(7),
    guest_x8 = const guest_gpr_offset(8),
    guest_x9 = const guest_gpr_offset(9),
    guest_x10 = const guest_gpr_offset(10),
    guest_x11 = const guest_gpr_offset(11),
    guest_x12 = const guest_gpr_offset(12),
    guest_x13 = const guest_gpr_offset(13),
    guest_x14 = const guest_gpr_offset(14),
    guest_x15 = const guest_gpr_offset(15),
    guest_x16 = const guest_gpr_offset(16),
    guest_x17 = const guest_gpr_offset(17),
    guest_x18 = const guest_gpr_offset(18),
    guest_x19 = const guest_gpr_offset(19),
    guest_x20 = const guest_gpr_offset(20),
    guest_x21 = const guest_gpr_offset(21),
    guest_x22 = const guest_gpr_offset(22),
    guest_x23 = const guest_gpr_offset(23),
    guest_x24 = const guest_gpr_offset(24),
    guest_x25 = const guest_gpr_offset(25),
    guest_x26 = const guest_gpr_offset(26),
    guest_x27 = const guest_gpr_offset(27),
    guest_x28 = const guest_gpr_offset(28),
    guest_x29 = const guest_gpr_offset(29),
    guest_x30 = const guest_gpr_offset(30),
    guest_sp = const guest_sp_offset(),
    guest_elr_el2 = const guest_csr_offset!(elr_el2),
    guest_spsr_el2 = const guest_csr_offset!(spsr_el2),
    guest_hcr_el2 = const guest_csr_offset!(hcr_el2),
);
extern "C" {
    pub fn _run_guest(regs: *mut VmCpuRegisters);
}

#[derive(Default)]
pub struct AARCH64Vcpu {
    regs: VmCpuRegisters,
}

impl AARCH64Vcpu {
    pub fn set_entry(&mut self, entry: GuestPhysAddr) -> AxResult {
        let regs = &mut self.regs;
        regs.guest_regs.elr_el2 = entry.as_usize();
        Ok(())
    }

    pub fn set_ept_root(&mut self, ept_root: HostPhysAddr) -> AxResult {
        self.regs.guest_regs.vttbr_el2 = usize::from(ept_root);
        let vtcr_el2 = 16 << 0 |  // TOSZ
                     (0b10 << 6) |   // SLo = Granule4KBLevel0
                     (0b11 << 8) |   // IRGN0 = NormalWBRAnWA
                     (0b11 << 10) |  // ORGN0 = NormalWBRAnWA
                     (0b11 << 12) |  // SH0 = Inner
                     (0b00 << 14) |  // TG0 = Granule4KB
                     (0b001 << 16);  // PS = 40 bits
        self.regs.guest_regs.vtcr_el2 = vtcr_el2;
        unsafe {
            core::arch::asm!(
                "msr vttbr_el2, {}",
                "msr vtcr_el2, {}",
                "tlbi vmalle1",
                "tlbi alle2",
                "dsb sy",
                "isb",
                in(reg) self.regs.guest_regs.vttbr_el2,
                in(reg) self.regs.guest_regs.vtcr_el2,
            );
        }
        Ok(())
    }

    pub fn run(&mut self) -> AxResult<AxVCpuExitReason> {
        let regs = &mut self.regs;
        unsafe {
            _run_guest(regs);
        }
        self.vmexit_handler()
    }
}

impl AARCH64Vcpu {
    pub fn init() -> Self {
        let mut regs = VmCpuRegisters::default();

        let mut hcr_el2: usize = 0;
        hcr_el2 |= 1 << 31; // RW
        hcr_el2 |= 1 << 0;  // VM
        hcr_el2 |= 1 << 19; // Trap SMC instructions to EL2
        regs.guest_regs.hcr_el2 = hcr_el2;

        let mut spsr_el2: usize = 0;
        spsr_el2 |= 0b0101; // EL1h
        spsr_el2 |= 1 << 6; // F
        spsr_el2 |= 1 << 7; // I
        spsr_el2 |= 1 << 8; // A
        spsr_el2 |= 1 << 9; // D
        regs.guest_regs.spsr_el2 = spsr_el2;

        Self { regs }
    }

    pub fn advance_pc(&mut self, instr_len: usize) {
        self.regs.guest_regs.elr_el2 += instr_len;
    }

    pub fn regs(&mut self) -> &mut VmCpuRegisters {
        &mut self.regs
    }
}

impl AARCH64Vcpu {
    fn vmexit_handler(&mut self) -> AxResult<AxVCpuExitReason> {
        let esr_el2: usize;
        unsafe {
            core::arch::asm!("mrs {}, esr_el2", out(reg) esr_el2);
        }
        let ec = (esr_el2 >> 26) & 0x3F;
        warn!(
            "VmExit: EC={:#x}, ESR_EL2={:#x}, ELR_EL2={:#x}",
            ec, esr_el2, self.regs.guest_regs.elr_el2
        );
        match ec as u32 {
            exception_class::EC_HVC64 | exception_class::EC_SMC64 => {
                let psci_msg = PsciMessage::from_regs(self.regs.guest_regs.gprs.a_regs()).ok();
                warn!("VmExit Reason: HVC/SMC: {:?}", psci_msg);
                if let Some(msg) = psci_msg {
                    match msg {
                        PsciMessage::System(psci::SystemFunction::SystemOff) => {
                            warn!("Guest requested system shutdown via PSCI");
                            warn!("===== Hypervisor Exiting Normally =====");
                            // 调用系统关机
                            axhal::misc::terminate();
                        },
                        PsciMessage::System(psci::SystemFunction::SystemReset { .. }) => {
                            warn!("Guest requested system reset via PSCI");
                            warn!("===== Hypervisor Exiting (Reset) =====");
                            axhal::misc::terminate();
                        },
                        _ => {
                            warn!("Unhandled PSCI call: {:?}", msg);
                        }
                    }
                } else {
                    panic!("bad PSCI message!");
                }
                Ok(AxVCpuExitReason::Nothing)
            },
            exception_class::EC_DABT_LOWER => {
                let iss = esr_el2 & 0x1FFFFFF;
                let wnr = (iss & (1 << 6)) != 0; // WnR: Write not Read
                let cm = (iss & (1 << 8)) != 0; // CM: Cache maintenance
                let access_flags = if wnr & !cm {
                    MappingFlags::WRITE | MappingFlags::USER
                } else {
                    MappingFlags::READ | MappingFlags::USER
                };

                let far_el2: usize;
                unsafe {
                    core::arch::asm!("mrs {}, far_el2", out(reg) far_el2);
                }
                let vaddr = GuestPhysAddr::from(far_el2);

                Ok(AxVCpuExitReason::PageFault {
                    addr: vaddr,
                    access_flags,
                })
            }
            _ => {
                panic!(
                    "Unhandled trap: EC={:#x}, ESR_EL2={:#x}, ELR_EL2={:#x}",
                    ec, esr_el2, self.regs.guest_regs.elr_el2
                );
            }
        }
    }
}

#[derive(Debug)]
pub enum AxVCpuExitReason {
    Nothing,
    PageFault { addr: GuestPhysAddr, access_flags: MappingFlags },
}