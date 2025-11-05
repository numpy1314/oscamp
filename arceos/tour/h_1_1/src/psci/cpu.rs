use axerrno::{AxError, AxResult};
use super::{PsciReturn, PSCI_ERR_NOT_SUPPORTED};
#[derive(Clone, Copy, Debug)]
pub enum CpuFunction {
    CpuSuspend {
        power_state: u32,
        entry_point: usize,
        context_id: usize,
    },
    CpuOff,
    CpuOn {
        target_cpu: usize,
        entry_point: usize,
        context_id: usize,
    },
}
impl CpuFunction {
    pub fn cpu_suspend(args: &[usize]) -> AxResult<Self> {
        Ok(Self::CpuSuspend {
            power_state: args[1] as u32,
            entry_point: args[2],
            context_id: args[3],
        })
    }
    pub fn cpu_on(args: &[usize]) -> AxResult<Self> {
        Ok(Self::CpuOn {
            target_cpu: args[1],
            entry_point: args[2],
            context_id: args[3],
        })
    }
    pub fn handle(&self) -> PsciReturn {
        match self {
            Self::CpuSuspend { .. } => {
                warn!("PSCI_CPU_SUSPEND not implemented");
                PsciReturn::error(PSCI_ERR_NOT_SUPPORTED)
            }
            Self::CpuOff => {
                warn!("PSCI_CPU_OFF called");
                PsciReturn::success(0)
            }
            Self::CpuOn { .. } => {
                warn!("PSCI_CPU_ON not implemented");
                PsciReturn::error(PSCI_ERR_NOT_SUPPORTED)
            }
        }
    }
}
