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
}
#[derive(Default)]
#[repr(C)]
pub struct VmCpuRegisters {
    hyp_regs: HypervisorCpuState,
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
global_asm!(include_str!("guest.S"));
const _: () = {
    const HYP_SIZE: usize = size_of::<HypervisorCpuState>();
    const GUEST_OFFSET: usize = offset_of!(VmCpuRegisters, guest_regs);
};
extern "C" {
    pub fn _run_guest(regs: *mut VmCpuRegisters);
}
