use alloc::sync::Arc;

use axhal::arch::UspaceContext;
use axmm::AddrSpace;
use axsync::Mutex;
use axtask::{AxTaskRef, TaskExtRef, TaskInner};

/// Task extended data for the monolithic kernel.
pub struct TaskExt {
    /// The process ID.
    pub proc_id: usize,
    /// The user space context.
    pub uctx: UspaceContext,
    /// The virtual memory address space.
    pub aspace: Arc<Mutex<AddrSpace>>,
}

impl TaskExt {
    pub const fn new(uctx: UspaceContext, aspace: Arc<Mutex<AddrSpace>>) -> Self {
        Self {
            proc_id: 1,
            uctx,
            aspace,
        }
    }
}

axtask::def_task_ext!(TaskExt);

pub fn spawn_user_task(aspace: Arc<Mutex<AddrSpace>>, uctx: UspaceContext) -> AxTaskRef {
    axlog::warn!("[spawn_user_task] begin");

    let mut task = TaskInner::new(
        || {
            axlog::warn!("[userboot closure] running (before any task_ext access)");

            let curr = axtask::current();

            let kstack_top = curr.kernel_stack_top().unwrap();
            axlog::warn!("[userboot closure] got kstack_top={:#x}", kstack_top);

            axlog::warn!("[userboot closure] about to read task_ext()");
            axlog::warn!("[userboot closure] about to read task_ext()");

            let raw = unsafe { curr.task_ext_ptr() };
            axlog::warn!("[userboot closure] raw task_ext_ptr = {:#x}", raw as usize);

            let ext = curr.task_ext();

            let ext_addr = (ext as *const _) as usize;
            let uctx_addr = (&ext.uctx as *const _) as usize;
            axlog::warn!("[userboot closure] &TaskExt = {:#x}, &uctx = {:#x}", ext_addr, uctx_addr);

            ax_println!(
                "Enter user space: entry={:#x}, ustack={:#x}, kstack={:#x}",
                ext.uctx.get_ip(),
                ext.uctx.get_sp(),
                kstack_top,
            );

            unsafe { ext.uctx.enter_uspace(kstack_top) };
        },
        "userboot".into(),
        crate::KERNEL_STACK_SIZE,
    );

    axlog::warn!("[spawn_user_task] created TaskInner, BEFORE init_task_ext");

    task.ctx_mut()
        .set_page_table_root(aspace.lock().page_table_root());

    axlog::warn!("[spawn_user_task] about to call init_task_ext");
    task.init_task_ext(TaskExt::new(uctx, aspace));
    axlog::warn!("[spawn_user_task] init_task_ext done, about to spawn_task");

    let ret = axtask::spawn_task(task);
    axlog::warn!("[spawn_user_task] spawn_task returned");
    ret
}
