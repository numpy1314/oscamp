.global _run_guest
_run_guest:
    // x0 = &VmCpuRegisters

    // -----------------------------
    // Save hypervisor GPRs (x1-x30)
    // -----------------------------
    str     x1, [x0, {hyp_x1}]
    str     x2, [x0, {hyp_x2}]
    str     x3, [x0, {hyp_x3}]
    str     x4, [x0, {hyp_x4}]
    str     x5, [x0, {hyp_x5}]
    str     x6, [x0, {hyp_x6}]
    str     x7, [x0, {hyp_x7}]
    str     x8, [x0, {hyp_x8}]
    str     x9, [x0, {hyp_x9}]
    str     x10,[x0, {hyp_x10}]
    str     x11,[x0, {hyp_x11}]
    str     x12,[x0, {hyp_x12}]
    str     x13,[x0, {hyp_x13}]
    str     x14,[x0, {hyp_x14}]
    str     x15,[x0, {hyp_x15}]
    str     x16,[x0, {hyp_x16}]
    str     x17,[x0, {hyp_x17}]
    str     x18,[x0, {hyp_x18}]
    str     x19,[x0, {hyp_x19}]
    str     x20,[x0, {hyp_x20}]
    str     x21,[x0, {hyp_x21}]
    str     x22,[x0, {hyp_x22}]
    str     x23,[x0, {hyp_x23}]
    str     x24,[x0, {hyp_x24}]
    str     x25,[x0, {hyp_x25}]
    str     x26,[x0, {hyp_x26}]
    str     x27,[x0, {hyp_x27}]
    str     x28,[x0, {hyp_x28}]
    str     x29,[x0, {hyp_x29}]
    str     x30,[x0, {hyp_x30}]

    // Save hypervisor SP/ELR/SPSR (EL2)
    mov     x9, sp
    str     x9, [x0, {hyp_sp}]
    mrs     x9, elr_el2
    str     x9, [x0, {hyp_elr_el2}]
    mrs     x9, spsr_el2
    str     x9, [x0, {hyp_spsr_el2}]

    // Put VmCpuRegisters* into TPIDR_EL2 for exit path
    msr     tpidr_el2, x0

    // Set EL2 vector base to _guest_exit
    adr     x9, _guest_exit
    msr     vbar_el2, x9
    isb

    // Restore virtualization control (HCR_EL2), force TGE=0
    ldr     x9, [x0, {guest_hcr_el2}]
    bic     x9, x9, #(1 << 27)
    msr     hcr_el2, x9
    isb

    // Restore guest ELR/SPSR for ERET
    ldr     x9, [x0, {guest_elr_el2}]
    msr     elr_el2, x9
    ldr     x9, [x0, {guest_spsr_el2}]
    msr     spsr_el2, x9

    // Guest runs EL1h => restore SP_EL1 (not SP_EL2)
    ldr     x9, [x0, {guest_sp}]
    msr     sp_el1, x9
    isb

    // Restore guest GPRs
    // Use x28 as base pointer; restore x28 last.
    mov     x28, x0

    ldr     x1,  [x28, {guest_x1}]
    ldr     x2,  [x28, {guest_x2}]
    ldr     x3,  [x28, {guest_x3}]
    ldr     x4,  [x28, {guest_x4}]
    ldr     x5,  [x28, {guest_x5}]
    ldr     x6,  [x28, {guest_x6}]
    ldr     x7,  [x28, {guest_x7}]
    ldr     x8,  [x28, {guest_x8}]
    ldr     x9,  [x28, {guest_x9}]
    ldr     x10, [x28, {guest_x10}]
    ldr     x11, [x28, {guest_x11}]
    ldr     x12, [x28, {guest_x12}]
    ldr     x13, [x28, {guest_x13}]
    ldr     x14, [x28, {guest_x14}]
    ldr     x15, [x28, {guest_x15}]
    ldr     x16, [x28, {guest_x16}]
    ldr     x17, [x28, {guest_x17}]
    ldr     x18, [x28, {guest_x18}]
    ldr     x19, [x28, {guest_x19}]
    ldr     x20, [x28, {guest_x20}]
    ldr     x21, [x28, {guest_x21}]
    ldr     x22, [x28, {guest_x22}]
    ldr     x23, [x28, {guest_x23}]
    ldr     x24, [x28, {guest_x24}]
    ldr     x25, [x28, {guest_x25}]
    ldr     x26, [x28, {guest_x26}]
    ldr     x27, [x28, {guest_x27}]
    ldr     x29, [x28, {guest_x29}]
    ldr     x30, [x28, {guest_x30}]
    ldr     x0,  [x28, {guest_x0}]
    ldr     x28, [x28, {guest_x28}]     // restore base last

    eret


// -----------------------------
// Exception Vector Table @ EL2
// -----------------------------
.align 11
.global _guest_exit
_guest_exit:
    // current EL, with SP_EL0
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception

    // current EL, with SP_ELx
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception

    // lower EL, aarch64
.align 7
    b    .Lguest_exit_sync
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception

    // lower EL, aarch32
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception
.align 7
    b    _invalid_exception


// -----------------------------
// Guest VM-exit handler (sync)
// -----------------------------
.Lguest_exit_sync:
    // Save guest x0-x3 on EL2 stack first (safe scratch)
    sub     sp, sp, #32
    stp     x0, x1, [sp, #0]
    stp     x2, x3, [sp, #16]

    // Load regs base
    mrs     x0, tpidr_el2           // x0 = &VmCpuRegisters

    // Recover guest x0-x3 into temps and pop stack
    ldp     x4, x5, [sp, #0]        // x4=guest_x0, x5=guest_x1
    ldp     x6, x7, [sp, #16]       // x6=guest_x2, x7=guest_x3
    add     sp, sp, #32

    // Store guest x0-x3
    str     x4, [x0, {guest_x0}]
    str     x5, [x0, {guest_x1}]
    str     x6, [x0, {guest_x2}]
    str     x7, [x0, {guest_x3}]

    // Store remaining guest regs (x8-x30, and x4-x7 already stored as x0-x3)
    str     x8,  [x0, {guest_x8}]
    str     x9,  [x0, {guest_x9}]
    str     x10, [x0, {guest_x10}]
    str     x11, [x0, {guest_x11}]
    str     x12, [x0, {guest_x12}]
    str     x13, [x0, {guest_x13}]
    str     x14, [x0, {guest_x14}]
    str     x15, [x0, {guest_x15}]
    str     x16, [x0, {guest_x16}]
    str     x17, [x0, {guest_x17}]
    str     x18, [x0, {guest_x18}]
    str     x19, [x0, {guest_x19}]
    str     x20, [x0, {guest_x20}]
    str     x21, [x0, {guest_x21}]
    str     x22, [x0, {guest_x22}]
    str     x23, [x0, {guest_x23}]
    str     x24, [x0, {guest_x24}]
    str     x25, [x0, {guest_x25}]
    str     x26, [x0, {guest_x26}]
    str     x27, [x0, {guest_x27}]
    str     x28, [x0, {guest_x28}]
    str     x29, [x0, {guest_x29}]
    str     x30, [x0, {guest_x30}]

    // Save guest stack from SP_EL1
    mrs     x9, sp_el1
    str     x9, [x0, {guest_sp}]

    // Save guest ELR/SPSR
    mrs     x9, elr_el2
    str     x9, [x0, {guest_elr_el2}]
    mrs     x9, spsr_el2
    str     x9, [x0, {guest_spsr_el2}]

    // Save guest HCR_EL2
    mrs     x9, hcr_el2
    str     x9, [x0, {guest_hcr_el2}]

    // Restore hypervisor regs x2-x30
    ldr     x2,  [x0, {hyp_x2}]
    ldr     x3,  [x0, {hyp_x3}]
    ldr     x4,  [x0, {hyp_x4}]
    ldr     x5,  [x0, {hyp_x5}]
    ldr     x6,  [x0, {hyp_x6}]
    ldr     x7,  [x0, {hyp_x7}]
    ldr     x8,  [x0, {hyp_x8}]
    ldr     x9,  [x0, {hyp_x9}]
    ldr     x10, [x0, {hyp_x10}]
    ldr     x11, [x0, {hyp_x11}]
    ldr     x12, [x0, {hyp_x12}]
    ldr     x13, [x0, {hyp_x13}]
    ldr     x14, [x0, {hyp_x14}]
    ldr     x15, [x0, {hyp_x15}]
    ldr     x16, [x0, {hyp_x16}]
    ldr     x17, [x0, {hyp_x17}]
    ldr     x18, [x0, {hyp_x18}]
    ldr     x19, [x0, {hyp_x19}]
    ldr     x20, [x0, {hyp_x20}]
    ldr     x21, [x0, {hyp_x21}]
    ldr     x22, [x0, {hyp_x22}]
    ldr     x23, [x0, {hyp_x23}]
    ldr     x24, [x0, {hyp_x24}]
    ldr     x25, [x0, {hyp_x25}]
    ldr     x26, [x0, {hyp_x26}]
    ldr     x27, [x0, {hyp_x27}]
    ldr     x28, [x0, {hyp_x28}]
    ldr     x29, [x0, {hyp_x29}]
    ldr     x30, [x0, {hyp_x30}]

    // Restore hypervisor SP/ELR/SPSR
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
