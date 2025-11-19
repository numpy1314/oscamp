#[derive(Default)]
#[repr(C)]
pub struct GeneralPurposeRegisters {
    pub x: [usize; 31],
    pub sp: usize,
}
impl GeneralPurposeRegisters {
    pub fn a_regs(&self) -> &[usize] {
        &self.x[0..8]
    }
    pub fn a_regs_mut(&mut self) -> &mut [usize] {
        &mut self.x[0..8]
    }
}
