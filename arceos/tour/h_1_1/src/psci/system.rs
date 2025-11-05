use axerrno::{AxError, AxResult};
use super::{PsciReturn, PSCI_SUCCESS};
#[derive(Clone, Copy, Debug)]
pub enum SystemFunction {
    SystemOff,
    SystemReset {
        reset_type: u32,
    },
}
impl SystemFunction {
    pub fn system_reset(args: &[usize]) -> AxResult<Self> {
        Ok(Self::SystemReset {
            reset_type: args[1] as u32,
        })
    }
    pub fn handle(&self) -> PsciReturn {
        match self {
            Self::SystemOff => {
                ax_println!("PSCI_SYSTEM_OFF: Guest requested shutdown");
                PsciReturn::success(PSCI_SUCCESS as i64)
            }
            Self::SystemReset { reset_type } => {
                ax_println!("PSCI_SYSTEM_RESET: Guest requested reset (type: {})", reset_type);
                PsciReturn::success(PSCI_SUCCESS as i64)
            }
        }
    }
}
