// 参考 h_1_0 的实现，使用编译时偏移量替换
// 通过 global_asm! 的 const 参数注入偏移量

.global _run_guest
_run_guest:
    // x0 = &VmCpuRegisters
    // 简单测试：直接返回，不进入 guest
    ret

// 异常向量表
.align 11
.global _guest_exit
_guest_exit:
    // current EL, with SP_EL0
.align 7
    b    _invalid_exception     // Synchronous
.align 7
    b    _invalid_exception     // IRQ
.align 7
    b    _invalid_exception     // FIQ
.align 7
    b    _invalid_exception     // SError

    // current EL, with SP_ELx
.align 7
    b    _invalid_exception     // Synchronous
.align 7
    b    _invalid_exception     // IRQ
.align 7
    b    _invalid_exception     // FIQ
.align 7
    b    _invalid_exception     // SError

    // lower EL, aarch64
.align 7
    b    .Lguest_exit_sync      // Synchronous
.align 7
    b    _invalid_exception     // IRQ
.align 7
    b    _invalid_exception     // FIQ
.align 7
    b    _invalid_exception     // SError

    // lower EL, aarch32
.align 7
    b    _invalid_exception     // Synchronous
.align 7
    b    _invalid_exception     // IRQ
.align 7
    b    _invalid_exception     // FIQ
.align 7
    b    _invalid_exception     // SError

// Guest VM exit 处理
.Lguest_exit_sync:
    // 当前所有寄存器都是 guest 的状态
    // 使用 SP_EL0 作为临时存储
    msr     sp_el0, x0              // 保存 guest x0
    
    // 现在 x0 可以自由使用
    mrs     x0, tpidr_el2           // x0 = &VmCpuRegisters
    
    // 保存 guest x0
    mrs     x1, sp_el0              // 恢复 guest x0
    str     x1, [x0, {guest_x0}]
    
    // 保存 guest sp
    mov     x9, sp
    str     x9, [x0, {guest_sp}]
    
    // 保存 guest elr_el2 和 spsr_el2
    mrs     x9, elr_el2
    str     x9, [x0, {guest_elr_el2}]
    mrs     x9, spsr_el2
    str     x9, [x0, {guest_spsr_el2}]
    
    // 恢复 hypervisor 寄存器
    ldr     x30, [x0, {hyp_x30}]
    ldr     x1, [x0, {hyp_sp}]
    
    // 恢复 hypervisor sp 并返回
    mov     sp, x1
    ret

_invalid_exception:
    b    _invalid_exception
