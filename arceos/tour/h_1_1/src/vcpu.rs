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
    // 只定义实际使用的偏移量
    hyp_x30 = const hyp_gpr_offset(30),
    hyp_sp = const hyp_sp_offset(),
    guest_x0 = const guest_gpr_offset(0),
    guest_x1 = const guest_gpr_offset(1),
    guest_x2 = const guest_gpr_offset(2),
    guest_x3 = const guest_gpr_offset(3),
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