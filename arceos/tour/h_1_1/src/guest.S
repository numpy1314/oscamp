.global _run_guest
_run_guest:
    // x0 = &VmCpuRegisters
    // 保存 hypervisor 通用寄存器
    str     x1, [x0, {hyp_x1}]
    str     x2, [x0, {hyp_x2}]
    str     x3, [x0, {hyp_x3}]
    str     x4, [x0, {hyp_x4}]
    str     x5, [x0, {hyp_x5}]
    str     x6, [x0, {hyp_x6}]
    str     x7, [x0, {hyp_x7}]
    str     x8, [x0, {hyp_x8}]
    str     x9, [x0, {hyp_x9}]
    str     x10, [x0, {hyp_x10}]
    str     x11, [x0, {hyp_x11}]
    str     x12, [x0, {hyp_x12}]
    str     x13, [x0, {hyp_x13}]
    str     x14, [x0, {hyp_x14}]
    str     x15, [x0, {hyp_x15}]
    str     x16, [x0, {hyp_x16}]
    str     x17, [x0, {hyp_x17}]
    str     x18, [x0, {hyp_x18}]
    str     x19, [x0, {hyp_x19}]
    str     x20, [x0, {hyp_x20}]
    str     x21, [x0, {hyp_x21}]
    str     x22, [x0, {hyp_x22}]
    str     x23, [x0, {hyp_x23}]
    str     x24, [x0, {hyp_x24}]
    str     x25, [x0, {hyp_x25}]
    str     x26, [x0, {hyp_x26}]
    str     x27, [x0, {hyp_x27}]
    str     x28, [x0, {hyp_x28}]
    str     x29, [x0, {hyp_x29}]
    str     x30, [x0, {hyp_x30}]

    // 保存 hypervisor SP、ELR_EL2、SPSR_EL2
    mov     x9, sp
    str     x9, [x0, {hyp_sp}]
    mrs     x9, elr_el2
    str     x9, [x0, {hyp_elr_el2}]
    mrs     x9, spsr_el2
    str     x9, [x0, {hyp_spsr_el2}]

    // 在 TPIDR_EL2 存放 VmCpuRegisters 指针，便于异常退出路径使用
    msr     tpidr_el2, x0

    // 设置 EL2 异常向量表为 _guest_exit，确保 VM-Exit 能返回到此处
    adr     x9, _guest_exit
    msr     vbar_el2, x9
    isb

    // 恢复虚拟化关键系统寄存器（参考 axvisor 顺序）
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

    // 恢复 Guest 通用寄存器
    ldr     x1,  [x0, {guest_x1}]
    ldr     x2,  [x0, {guest_x2}]
    ldr     x3,  [x0, {guest_x3}]
    ldr     x4,  [x0, {guest_x4}]
    ldr     x5,  [x0, {guest_x5}]
    ldr     x6,  [x0, {guest_x6}]
    ldr     x7,  [x0, {guest_x7}]
    ldr     x8,  [x0, {guest_x8}]
    ldr     x10,  [x0, {guest_x10}]
    ldr     x11,  [x0, {guest_x11}]
    ldr     x12,  [x0, {guest_x12}]
    ldr     x13,  [x0, {guest_x13}]
    ldr     x14,  [x0, {guest_x14}]
    ldr     x15,  [x0, {guest_x15}]
    ldr     x16,  [x0, {guest_x16}]
    ldr     x17,  [x0, {guest_x17}]
    ldr     x18,  [x0, {guest_x18}]
    ldr     x19,  [x0, {guest_x19}]
    ldr     x20,  [x0, {guest_x20}]
    ldr     x21,  [x0, {guest_x21}]
    ldr     x22,  [x0, {guest_x22}]
    ldr     x23,  [x0, {guest_x23}]
    ldr     x24,  [x0, {guest_x24}]
    ldr     x25,  [x0, {guest_x25}]
    ldr     x26,  [x0, {guest_x26}]
    ldr     x27,  [x0, {guest_x27}]
    ldr     x28,  [x0, {guest_x28}]
    ldr     x29,  [x0, {guest_x29}]
    ldr     x30,  [x0, {guest_x30}]

    // 恢复 Guest 的 SP
    ldr     x9, [x0, {guest_sp}]
    mov     sp, x9

    // 恢复 Guest x0 和 x9
    ldr     x9, [x0, {guest_x9}]
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

    mov     x10, x0
    mrs     x0, tpidr_el2           // x0 = &VmCpuRegisters
    str     x10, [x0, {guest_x0}]
    str     x1,  [x0, {guest_x1}]
    str     x2,  [x0, {guest_x2}]
    str     x3,  [x0, {guest_x3}]
    str     x4,  [x0, {guest_x4}]
    str     x5,  [x0, {guest_x5}]
    str     x6,  [x0, {guest_x6}]
    str     x7,  [x0, {guest_x7}]
    str     x8,  [x0, {guest_x8}]
    str     x9,  [x0, {guest_x9}]
    str     x11,  [x0, {guest_x11}]
    str     x12,  [x0, {guest_x12}]
    str     x13,  [x0, {guest_x13}]
    str     x14,  [x0, {guest_x14}]
    str     x15,  [x0, {guest_x15}]
    str     x16,  [x0, {guest_x16}]
    str     x17,  [x0, {guest_x17}]
    str     x18,  [x0, {guest_x18}]
    str     x19,  [x0, {guest_x19}]
    str     x20,  [x0, {guest_x20}]
    str     x21,  [x0, {guest_x21}]
    str     x22,  [x0, {guest_x22}]
    str     x23,  [x0, {guest_x23}]
    str     x24,  [x0, {guest_x24}]
    str     x25,  [x0, {guest_x25}]
    str     x26,  [x0, {guest_x26}]
    str     x27,  [x0, {guest_x27}]
    str     x28,  [x0, {guest_x28}]
    str     x29,  [x0, {guest_x29}]
    str     x30,  [x0, {guest_x30}]
    
    // 保存 guest sp
    mov     x9, sp
    str     x9, [x0, {guest_sp}]
    
    // 保存 guest elr_el2 和 spsr_el2
    mrs     x9, elr_el2
    str     x9, [x0, {guest_elr_el2}]
    mrs     x9, spsr_el2
    str     x9, [x0, {guest_spsr_el2}]

    // 保存 guest hcr_el2
    mrs     x9, hcr_el2
    str     x9, [x0, {guest_hcr_el2}]
    
    // 恢复 hypervisor 寄存器
    ldr     x2,  [x0, {hyp_x2}]
    ldr     x3,  [x0, {hyp_x3}]
    ldr     x4,  [x0, {hyp_x4}]
    ldr     x5,  [x0, {hyp_x5}]
    ldr     x6,  [x0, {hyp_x6}]
    ldr     x7,  [x0, {hyp_x7}]
    ldr     x8,  [x0, {hyp_x8}]
    ldr     x9,  [x0, {hyp_x9}]
    ldr     x10,  [x0, {hyp_x10}]
    ldr     x11,  [x0, {hyp_x11}]
    ldr     x12,  [x0, {hyp_x12}]
    ldr     x13,  [x0, {hyp_x13}]
    ldr     x14,  [x0, {hyp_x14}]
    ldr     x15,  [x0, {hyp_x15}]
    ldr     x16,  [x0, {hyp_x16}]
    ldr     x17,  [x0, {hyp_x17}]
    ldr     x18,  [x0, {hyp_x18}]
    ldr     x19,  [x0, {hyp_x19}]
    ldr     x20,  [x0, {hyp_x20}]
    ldr     x21,  [x0, {hyp_x21}]
    ldr     x22,  [x0, {hyp_x22}]
    ldr     x23,  [x0, {hyp_x23}]
    ldr     x24,  [x0, {hyp_x24}]
    ldr     x25,  [x0, {hyp_x25}]
    ldr     x26,  [x0, {hyp_x26}]
    ldr     x27,  [x0, {hyp_x27}]
    ldr     x28,  [x0, {hyp_x28}]
    ldr     x29,  [x0, {hyp_x29}]
    ldr     x30, [x0, {hyp_x30}]
    
    // 恢复 hypervisor sp, elr_el2, spsr_el2 并返回
    ldr     x1,  [x0, {hyp_sp}]
    mov     sp, x1
    ldr     x1,  [x0, {hyp_elr_el2}]
    msr     elr_el2, x1
    ldr     x1,  [x0, {hyp_spsr_el2}]
    msr     spsr_el2, x1
    ldr     x1,  [x0, {hyp_x1}]
    ret

_invalid_exception:
    b    _invalid_exception