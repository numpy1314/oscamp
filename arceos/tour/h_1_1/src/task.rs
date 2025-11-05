use alloc::sync::Arc;
use axmm::AddrSpace;
use axsync::Mutex;
use crate::vcpu::VmCpuRegisters;
pub struct TaskExt {
    pub vcpu: VmCpuRegisters,
    pub aspace: Arc<Mutex<AddrSpace>>,
}
impl TaskExt {
    pub const fn new(vcpu: VmCpuRegisters, aspace: Arc<Mutex<AddrSpace>>) -> Self {
        Self {
            vcpu,
            aspace,
        }
    }
}
axtask::def_task_ext!(TaskExt);
