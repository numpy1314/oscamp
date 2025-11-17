// 参考 h_1_0 的实现，使用编译时偏移量替换
// 通过 global_asm! 的 const 参数注入偏移量

.global _run_guest
_run_guest:
    // x0 = &VmCpuRegisters
    // 保存 hypervisor 返回上下文（x30, sp）
    str     x30, [x0, {hyp_x30}]
    mov     x9, sp
    str     x9, [x0, {hyp_sp}]

    // 在 TPIDR_EL2 存放 VmCpuRegisters 指针，便于异常退出路径使用
    msr     tpidr_el2, x0

    // 设置 EL2 异常向量表为 _guest_exit，确保 VM-Exit 能返回到此处
    adr     x9, _guest_exit
    msr     vbar_el2, x9
    isb

    // 恢复虚拟化关键系统寄存器（参考 axvisor 顺序）
    ldr     x9, [x0, {guest_vtcr_el2}]
    msr     vtcr_el2, x9

    ldr     x9, [x0, {guest_vttbr_el2}]
    msr     vttbr_el2, x9

    ldr     x9, [x0, {guest_hcr_el2}]
    msr     hcr_el2, x9

    // 刷新 TLB，确保新的 Stage-2 生效
    ic      iallu
    tlbi    alle2
    tlbi    vmalle1
    dsb     nsh
    isb

    // 恢复 ELR_EL2 / SPSR_EL2 指向 Guest 入口与 PSTATE
    ldr     x9, [x0, {guest_elr_el2}]
    msr     elr_el2, x9
    ldr     x9, [x0, {guest_spsr_el2}]
    msr     spsr_el2, x9

    // 恢复 Guest 的 SP
    ldr     x9, [x0, {guest_sp}]
    mov     sp, x9

    // 恢复 Guest x0（最后恢复，以免覆盖 x0 指针）
    ldr     x0, [x0, {guest_x0}]

    // 进入 Guest（从 EL2 退到 EL1）
    eret

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
    // 保存 guest x0..x3，不再使用 SP_EL0 作为临时寄存器
    mov     x10, x0
    mrs     x0, tpidr_el2           // x0 = &VmCpuRegisters
    str     x10, [x0, {guest_x0}]
    str     x1,  [x0, {guest_x1}]
    str     x2,  [x0, {guest_x2}]
    str     x3,  [x0, {guest_x3}]
    
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
    ldr     x1,  [x0, {hyp_sp}]
    
    // 恢复 hypervisor sp 并返回
    mov     sp, x1
    ret

_invalid_exception:
    b    _invalid_exception