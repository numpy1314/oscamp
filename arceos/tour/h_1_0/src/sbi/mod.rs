#![allow(dead_code)]
mod base;
mod dbcn;
mod pmu;
mod rfnc;
mod srst;
use axerrno::{AxError, AxResult};
pub use base::BaseFunction;
use dbcn::DebugConsoleFunction;
pub use pmu::PmuFunction;
pub use rfnc::RemoteFenceFunction;
use sbi_spec;
pub use srst::ResetFunction;
pub const SBI_SUCCESS: usize = 0;
pub const SBI_ERR_FAILUER: isize = -1;
pub const SBI_ERR_NOT_SUPPORTED: isize = -2;
pub const SBI_ERR_INAVLID_PARAM: isize = -3;
pub const SBI_ERR_DENIED: isize = -4;
pub const SBI_ERR_INVALID_ADDRESS: isize = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: isize = -6;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SbiReturn {
    pub error_code: i64,
    pub return_value: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SbiReturnTyoe {
    Legacy(u64),
    Standard(SbiReturn),
}
#[derive(Clone, Copy, Debug)]
pub enum SbiMessage {
    Base(BaseFunction),
    GetChar,
    PutChar(usize),
    SetTimer(usize),
    DebugConsole(DebugConsoleFunction),
    Reset(ResetFunction),
    RemoteFence(RemoteFenceFunction),
    PMU(PmuFunction),
}
impl SbiMessage {
    pub fn from_regs(args: &[usize]) -> AxResult<Self> {
        match args[7] {
            sbi_spec::base::EID_BASE => BaseFunction::from_regs(args).map(SbiMessage::Base),
            sbi_spec::legacy::LEGACY_CONSOLE_PUTCHAR => Ok(SbiMessage::PutChar(args[0])),
            sbi_spec::legacy::LEGACY_CONSOLE_GETCHAR => Ok(SbiMessage::GetChar),
            sbi_spec::legacy::LEGACY_SET_TIMER => Ok(SbiMessage::SetTimer(args[0])),
            sbi_spec::legacy::LEGACY_SHUTDOWN => Ok(SbiMessage::Reset(ResetFunction::shutdown())),
            sbi_spec::time::EID_TIME => Ok(SbiMessage::SetTimer(args[0])),
            sbi_spec::srst::EID_SRST => ResetFunction::from_regs(args).map(SbiMessage::Reset),
            sbi_spec::rfnc::EID_RFNC => {
                RemoteFenceFunction::from_args(args).map(SbiMessage::RemoteFence)
            }
            sbi_spec::pmu::EID_PMU => PmuFunction::from_regs(args).map(SbiMessage::PMU),
            _ => {
                error!("args: {:?}", args);
                error!("args[7]: {:#x}", args[7]);
                error!("EID_RFENCE: {:#x}", sbi_spec::rfnc::EID_RFNC);
                Err(AxError::NotFound)
            }
        }
    }
}
