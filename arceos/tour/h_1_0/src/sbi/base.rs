use axerrno::{AxError, AxResult};
#[derive(Clone, Copy, Debug)]
pub enum BaseFunction {
    GetSepcificationVersion,
    GetImplementationID,
    GetImplementationVersion,
    ProbeSbiExtension(u64),
    GetMachineVendorID,
    GetMachineArchitectureID,
    GetMachineImplementationID,
}
impl BaseFunction {
    pub(crate) fn from_regs(args: &[usize]) -> AxResult<Self> {
        match args[6] {
            0 => Ok(BaseFunction::GetSepcificationVersion),
            1 => Ok(BaseFunction::GetImplementationID),
            2 => Ok(BaseFunction::GetImplementationVersion),
            3 => Ok(BaseFunction::ProbeSbiExtension(args[0] as u64)),
            4 => Ok(BaseFunction::GetMachineVendorID),
            5 => Ok(BaseFunction::GetMachineArchitectureID),
            6 => Ok(BaseFunction::GetMachineImplementationID),
            _ => Err(AxError::NotFound),
        }
    }
}
