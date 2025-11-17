use core::arch::global_asm;
use core::mem::size_of;
use memoffset::offset_of;
use super::regs::GeneralPurposeRegisters;
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
    guest_vtcr_el2 = const guest_csr_offset!(vtcr_el2),
    guest_vttbr_el2 = const guest_csr_offset!(vttbr_el2),
);
extern "C" {
    pub fn _run_guest(regs: *mut VmCpuRegisters);
}