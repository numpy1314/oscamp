#![allow(dead_code)]
pub trait AArch64SysRegTrait {
    fn read(&self) -> usize;
    fn write(&self, val: usize);
    fn read_value(&self) -> usize {
        self.read()
    }
    fn write_value(&self, val: usize) {
        self.write(val)
    }
}
pub struct SysRegs {
    pub hcr_el2: HcrEl2,
    pub vttbr_el2: VttbrEl2,
    pub vtcr_el2: VtcrEl2,
    pub elr_el2: ElrEl2,
    pub spsr_el2: SpsrEl2,
    pub esr_el2: EsrEl2,
    pub far_el2: FarEl2,
}
pub static SYSREG: SysRegs = SysRegs {
    hcr_el2: HcrEl2,
    vttbr_el2: VttbrEl2,
    vtcr_el2: VtcrEl2,
    elr_el2: ElrEl2,
    spsr_el2: SpsrEl2,
    esr_el2: EsrEl2,
    far_el2: FarEl2,
};
pub struct HcrEl2;
impl AArch64SysRegTrait for HcrEl2 {
    fn read(&self) -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mrs {}, hcr_el2", out(reg) val);
        }
        val
    }
    fn write(&self, val: usize) {
        unsafe {
            core::arch::asm!("msr hcr_el2, {}", in(reg) val);
        }
    }
}
pub struct VttbrEl2;
impl AArch64SysRegTrait for VttbrEl2 {
    fn read(&self) -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mrs {}, vttbr_el2", out(reg) val);
        }
        val
    }
    fn write(&self, val: usize) {
        unsafe {
            core::arch::asm!(
                "msr vttbr_el2, {}",
                "tlbi vmalle1",
                "tlbi alle2",
                "dsb sy",
                "isb",
                in(reg) val
            );
        }
    }
}
pub struct ElrEl2;
impl AArch64SysRegTrait for ElrEl2 {
    fn read(&self) -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mrs {}, elr_el2", out(reg) val);
        }
        val
    }
    fn write(&self, val: usize) {
        unsafe {
            core::arch::asm!("msr elr_el2, {}", in(reg) val);
        }
    }
}
pub struct SpsrEl2;
impl AArch64SysRegTrait for SpsrEl2 {
    fn read(&self) -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mrs {}, spsr_el2", out(reg) val);
        }
        val
    }
    fn write(&self, val: usize) {
        unsafe {
            core::arch::asm!("msr spsr_el2, {}", in(reg) val);
        }
    }
}
pub struct EsrEl2;
impl AArch64SysRegTrait for EsrEl2 {
    fn read(&self) -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mrs {}, esr_el2", out(reg) val);
        }
        val
    }
    fn write(&self, val: usize) {
        unsafe {
            core::arch::asm!("msr esr_el2, {}", in(reg) val);
        }
    }
}
pub struct FarEl2;
impl AArch64SysRegTrait for FarEl2 {
    fn read(&self) -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mrs {}, far_el2", out(reg) val);
        }
        val
    }
    fn write(&self, val: usize) {
        unsafe {
            core::arch::asm!("msr far_el2, {}", in(reg) val);
        }
    }
}
pub struct VtcrEl2;
impl AArch64SysRegTrait for VtcrEl2 {
    fn read(&self) -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mrs {}, vtcr_el2", out(reg) val);
        }
        val
    }
    fn write(&self, val: usize) {
        unsafe {
            core::arch::asm!(
                "msr vtcr_el2, {}",
                "isb",
                in(reg) val
            );
        }
    }
}
pub mod exception_class {
    pub const EC_UNKNOWN: u32 = 0x00;
    pub const EC_WFI_WFE: u32 = 0x01;
    pub const EC_HVC64: u32 = 0x16;
    pub const EC_SMC64: u32 = 0x17;
    pub const EC_IABT_LOWER: u32 = 0x20;
    pub const EC_IABT_CURRENT: u32 = 0x21;
    pub const EC_DABT_LOWER: u32 = 0x24;
    pub const EC_DABT_CURRENT: u32 = 0x25;
}
