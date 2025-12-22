use core::arch::asm;
use memory_addr::VirtAddr;
#[cfg(feature = "uspace")]
use memory_addr::PhysAddr;

/// Saved registers when a trap (exception) occurs.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TrapFrame {
    /// General-purpose registers (R0..R30).
    pub r: [u64; 31],
    /// User Stack Pointer (SP_EL0).
    pub usp: u64,
    /// Exception Link Register (ELR_EL1).
    pub elr: u64,
    /// Saved Process Status Register (SPSR_EL1).
    pub spsr: u64,
}

impl TrapFrame {
    /// Gets the 0th syscall argument (x0).
    pub const fn arg0(&self) -> usize {
        self.r[0] as usize
    }

    /// Gets the 1st syscall argument (x1).
    pub const fn arg1(&self) -> usize {
        self.r[1] as usize
    }

    /// Gets the 2nd syscall argument (x2).
    pub const fn arg2(&self) -> usize {
        self.r[2] as usize
    }

    /// Gets the 3rd syscall argument (x3).
    pub const fn arg3(&self) -> usize {
        self.r[3] as usize
    }

    /// Gets the 4th syscall argument (x4).
    pub const fn arg4(&self) -> usize {
        self.r[4] as usize
    }

    /// Gets the 5th syscall argument (x5).
    pub const fn arg5(&self) -> usize {
        self.r[5] as usize
    }
}

/// FP & SIMD registers.
#[repr(C, align(16))]
#[derive(Debug, Default)]
pub struct FpState {
    /// 128-bit SIMD & FP registers (V0..V31)
    pub regs: [u128; 32],
    /// Floating-point Control Register (FPCR)
    pub fpcr: u32,
    /// Floating-point Status Register (FPSR)
    pub fpsr: u32,
}

#[cfg(feature = "fp_simd")]
impl FpState {
    fn switch_to(&mut self, next_fpstate: &FpState) {
        unsafe { fpstate_switch(self, next_fpstate) }
    }
}

/// Saved hardware states of a task.
///
/// The context usually includes:
///
/// - Callee-saved registers
/// - Stack pointer register
/// - Thread pointer register (for thread-local storage, currently unsupported)
/// - FP/SIMD registers
///
/// On context switch, current task saves its context from CPU to memory,
/// and the next task restores its context from memory to CPU.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug)]
pub struct TaskContext {
    pub sp: u64,
    pub tpidr_el0: u64,
    pub r19: u64,
    pub r20: u64,
    pub r21: u64,
    pub r22: u64,
    pub r23: u64,
    pub r24: u64,
    pub r25: u64,
    pub r26: u64,
    pub r27: u64,
    pub r28: u64,
    pub r29: u64,
    pub lr: u64, // r30
    /// The page table root physical address (TTBR0_EL1).
    #[cfg(feature = "uspace")]
    pub ttbr0: PhysAddr,
    #[cfg(feature = "fp_simd")]
    pub fp_state: FpState,
}

impl TaskContext {
    /// Creates a new default context for a new task.
    pub const fn new() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }

    /// Initializes the context for a new task, with the given entry point and
    /// kernel stack.
    pub fn init(&mut self, entry: usize, kstack_top: VirtAddr, tls_area: VirtAddr) {
        self.sp = kstack_top.as_usize() as u64;
        self.lr = entry as u64;
        self.tpidr_el0 = tls_area.as_usize() as u64;
        #[cfg(feature = "uspace")]
        {
            self.ttbr0 = crate::paging::kernel_page_table_root();
        }
    }

    /// Changes the page table root (TTBR0_EL1 register for aarch64).
    ///
    /// If not set, the kernel page table root is used (obtained by
    /// [`axhal::paging::kernel_page_table_root`][1]).
    ///
    /// [1]: crate::paging::kernel_page_table_root
    #[cfg(feature = "uspace")]
    pub fn set_page_table_root(&mut self, ttbr0: PhysAddr) {
        self.ttbr0 = ttbr0;
    }

    /// Switches to another task.
    ///
    /// It first saves the current task's context from CPU to this place, and then
    /// restores the next task's context from `next_ctx` to CPU.
    pub fn switch_to(&mut self, next_ctx: &Self) {
        #[cfg(feature = "fp_simd")]
        self.fp_state.switch_to(&next_ctx.fp_state);
        #[cfg(feature = "uspace")]
        unsafe {
            if self.ttbr0 != next_ctx.ttbr0 {
                super::write_page_table_root(next_ctx.ttbr0);
            }
        }
        unsafe { context_switch(self, next_ctx) }
    }
}

#[naked]
unsafe extern "C" fn context_switch(_current_task: &mut TaskContext, _next_task: &TaskContext) {
    asm!(
        "
        // save old context (callee-saved registers)
        stp     x29, x30, [x0, 12 * 8]
        stp     x27, x28, [x0, 10 * 8]
        stp     x25, x26, [x0, 8 * 8]
        stp     x23, x24, [x0, 6 * 8]
        stp     x21, x22, [x0, 4 * 8]
        stp     x19, x20, [x0, 2 * 8]
        mov     x19, sp
        mrs     x20, tpidr_el0
        stp     x19, x20, [x0]

        // restore new context
        ldp     x19, x20, [x1]
        mov     sp, x19
        msr     tpidr_el0, x20
        ldp     x19, x20, [x1, 2 * 8]
        ldp     x21, x22, [x1, 4 * 8]
        ldp     x23, x24, [x1, 6 * 8]
        ldp     x25, x26, [x1, 8 * 8]
        ldp     x27, x28, [x1, 10 * 8]
        ldp     x29, x30, [x1, 12 * 8]

        ret",
        options(noreturn),
    )
}

#[naked]
#[cfg(feature = "fp_simd")]
unsafe extern "C" fn fpstate_switch(_current_fpstate: &mut FpState, _next_fpstate: &FpState) {
    asm!(
        "
        // save fp/neon context
        mrs     x9, fpcr
        mrs     x10, fpsr
        stp     q0, q1, [x0, 0 * 16]
        stp     q2, q3, [x0, 2 * 16]
        stp     q4, q5, [x0, 4 * 16]
        stp     q6, q7, [x0, 6 * 16]
        stp     q8, q9, [x0, 8 * 16]
        stp     q10, q11, [x0, 10 * 16]
        stp     q12, q13, [x0, 12 * 16]
        stp     q14, q15, [x0, 14 * 16]
        stp     q16, q17, [x0, 16 * 16]
        stp     q18, q19, [x0, 18 * 16]
        stp     q20, q21, [x0, 20 * 16]
        stp     q22, q23, [x0, 22 * 16]
        stp     q24, q25, [x0, 24 * 16]
        stp     q26, q27, [x0, 26 * 16]
        stp     q28, q29, [x0, 28 * 16]
        stp     q30, q31, [x0, 30 * 16]
        str     x9, [x0, 64 *  8]
        str     x10, [x0, 65 * 8]

        // restore fp/neon context
        ldp     q0, q1, [x1, 0 * 16]
        ldp     q2, q3, [x1, 2 * 16]
        ldp     q4, q5, [x1, 4 * 16]
        ldp     q6, q7, [x1, 6 * 16]
        ldp     q8, q9, [x1, 8 * 16]
        ldp     q10, q11, [x1, 10 * 16]
        ldp     q12, q13, [x1, 12 * 16]
        ldp     q14, q15, [x1, 14 * 16]
        ldp     q16, q17, [x1, 16 * 16]
        ldp     q18, q19, [x1, 18 * 16]
        ldp     q20, q21, [x1, 20 * 16]
        ldp     q22, q23, [x1, 22 * 16]
        ldp     q24, q25, [x1, 24 * 16]
        ldp     q26, q27, [x1, 26 * 16]
        ldp     q28, q29, [x1, 28 * 16]
        ldp     q30, q31, [x1, 30 * 16]
        ldr     x9, [x1, 64 * 8]
        ldr     x10, [x1, 65 * 8]
        msr     fpcr, x9
        msr     fpsr, x10

        isb
        ret",
        options(noreturn),
    )
}

/// Context to enter user space.
#[cfg(feature = "uspace")]
pub struct UspaceContext(TrapFrame);

#[cfg(feature = "uspace")]
impl UspaceContext {
    /// Creates an empty context with all registers set to zero.
    pub const fn empty() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }

    /// Creates a new context with the given entry point, user stack pointer.
    pub fn new(entry: usize, ustack_top: VirtAddr) -> Self {
        // SPSR_EL1:
        // - bit 0-3: M[3:0] = 0b0000 (EL0t - EL0 with SP_EL0)
        // - bit 6: F = 0 (FIQ not masked)
        // - bit 7: I = 0 (IRQ not masked)
        // - bit 8: A = 0 (SError not masked)
        // - bit 9: D = 0 (Debug exceptions not masked)
        const SPSR_EL1_EL0: u64 = 0b0000;
        Self(TrapFrame {
            r: [0; 31],
            usp: ustack_top.as_usize() as u64,
            elr: entry as u64,
            spsr: SPSR_EL1_EL0,
        })
    }

    /// Creates a new context from the given [`TrapFrame`].
    pub const fn from(trap_frame: &TrapFrame) -> Self {
        Self(*trap_frame)
    }

    /// Gets the instruction pointer.
    pub const fn get_ip(&self) -> usize {
        self.0.elr as usize
    }

    /// Gets the stack pointer.
    pub const fn get_sp(&self) -> usize {
        self.0.usp as usize
    }

    /// Sets the instruction pointer.
    pub fn set_ip(&mut self, pc: usize) {
        self.0.elr = pc as u64;
    }

    /// Sets the stack pointer.
    pub fn set_sp(&mut self, sp: usize) {
        self.0.usp = sp as u64;
    }

    /// Sets the return value register.
    pub fn set_retval(&mut self, a0: usize) {
        self.0.r[0] = a0 as u64;
    }

    /// Enters user space.
    ///
    /// It restores the user registers and jumps to the user entry point
    /// (saved in `elr`).
    /// When an exception or syscall occurs, the kernel stack pointer is
    /// switched to `kstack_top`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it changes processor mode and the stack.
    #[inline(never)]
    #[no_mangle]
    pub unsafe fn enter_uspace(&self, kstack_top: VirtAddr) -> ! {
        use aarch64_cpu::registers::{SPSR_EL1, ELR_EL1, SP_EL0};
        use tock_registers::interfaces::Writeable;

        super::disable_irqs();
        
        // Set up exception return context
        SPSR_EL1.set(self.0.spsr);
        ELR_EL1.set(self.0.elr);
        SP_EL0.set(self.0.usp);

        asm!(
            // Save kernel stack pointer for exception handling
            "mov    x9, {kstack_top}",
            "msr    sp_el0, x9",
            
            // Restore general-purpose registers
            "ldp    x0, x1, [{tf}, #0]",
            "ldp    x2, x3, [{tf}, #16]",
            "ldp    x4, x5, [{tf}, #32]",
            "ldp    x6, x7, [{tf}, #48]",
            "ldp    x8, x9, [{tf}, #64]",
            "ldp    x10, x11, [{tf}, #80]",
            "ldp    x12, x13, [{tf}, #96]",
            "ldp    x14, x15, [{tf}, #112]",
            "ldp    x16, x17, [{tf}, #128]",
            "ldp    x18, x19, [{tf}, #144]",
            "ldp    x20, x21, [{tf}, #160]",
            "ldp    x22, x23, [{tf}, #176]",
            "ldp    x24, x25, [{tf}, #192]",
            "ldp    x26, x27, [{tf}, #208]",
            "ldp    x28, x29, [{tf}, #224]",
            "ldr    x30, [{tf}, #240]",
            
            // Exception return to EL0
            "eret",
            
            tf = in(reg) &(self.0),
            kstack_top = in(reg) kstack_top.as_usize(),
            options(noreturn),
        )
    }
}
