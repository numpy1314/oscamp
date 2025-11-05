use axerrno::{AxError, AxResult};
#[derive(Copy, Clone, Debug)]
pub enum ResetFunction {
    Reset {
        reset_type: ResetType,
        reason: ResetReason,
    },
}
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResetType {
    Shutdown = 0,
    ColdReset = 1,
    WarmReset = 2,
}
impl ResetType {
    fn from_reg(a0: usize) -> AxResult<Self> {
        use ResetType::*;
        Ok(match a0 {
            0 => Shutdown,
            1 => ColdReset,
            2 => WarmReset,
            _ => return Err(AxError::InvalidInput),
        })
    }
}
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResetReason {
    NoReason = 0,
    SystemFailure = 1,
}
impl ResetReason {
    fn from_reg(a1: usize) -> AxResult<Self> {
        use ResetReason::*;
        Ok(match a1 {
            0 => NoReason,
            1 => SystemFailure,
            _ => return Err(AxError::InvalidInput),
        })
    }
}
impl ResetFunction {
    pub(crate) fn from_regs(args: &[usize]) -> AxResult<Self> {
        use ResetFunction::*;
        Ok(match args[6] {
            0 => Reset {
                reset_type: ResetType::from_reg(args[0])?,
                reason: ResetReason::from_reg(args[1])?,
            },
            _ => return Err(AxError::InvalidInput),
        })
    }
    pub fn shutdown() -> Self {
        ResetFunction::Reset {
            reset_type: ResetType::Shutdown,
            reason: ResetReason::NoReason,
        }
    }
}
