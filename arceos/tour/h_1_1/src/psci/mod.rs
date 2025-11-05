#![allow(dead_code)]
mod version;
mod cpu;
mod system;
use axerrno::{AxError, AxResult};
pub use version::VersionFunction;
pub use cpu::CpuFunction;
pub use system::SystemFunction;
pub const PSCI_SUCCESS: i32 = 0;
pub const PSCI_ERR_NOT_SUPPORTED: i32 = -1;
pub const PSCI_ERR_INVALID_PARAMS: i32 = -2;
pub const PSCI_ERR_DENIED: i32 = -3;
pub const PSCI_ERR_ALREADY_ON: i32 = -4;
pub const PSCI_ERR_ON_PENDING: i32 = -5;
pub const PSCI_ERR_INTERNAL_FAILURE: i32 = -6;
pub const PSCI_ERR_NOT_PRESENT: i32 = -7;
pub const PSCI_ERR_DISABLED: i32 = -8;
pub const PSCI_ERR_INVALID_ADDRESS: i32 = -9;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PsciReturn {
    pub value: i64,
}
impl PsciReturn {
    pub fn success(value: i64) -> Self {
        Self { value }
    }
    pub fn error(code: i32) -> Self {
        Self { value: code as i64 }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum PsciMessage {
    Version(VersionFunction),
    Cpu(CpuFunction),
    System(SystemFunction),
}
impl PsciMessage {
    pub fn from_regs(args: &[usize]) -> AxResult<Self> {
        let function_id = args[0];
        match function_id {
            0x84000000 => Ok(PsciMessage::Version(VersionFunction)),
            0x84000001 => CpuFunction::cpu_suspend(args).map(PsciMessage::Cpu),
            0x84000002 => Ok(PsciMessage::Cpu(CpuFunction::CpuOff)),
            0x84000003 | 0xc4000003 => CpuFunction::cpu_on(args).map(PsciMessage::Cpu),
            0x84000008 => Ok(PsciMessage::System(SystemFunction::SystemOff)),
            0x84000009 => SystemFunction::system_reset(args).map(PsciMessage::System),
            _ => {
                error!("Unknown PSCI function ID: {:#x}", function_id);
                error!("args: {:?}", args);
                Err(AxError::NotFound)
            }
        }
    }
}
