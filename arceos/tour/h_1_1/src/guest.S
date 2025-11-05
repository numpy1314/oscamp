.global _run_guest
_run_guest:
    msr  tpidr_el2, x0
    mov  x9, x0
    stp  x1, x2, [x9, #16]
    stp  x3, x4, [x9, #32]
    stp  x5, x6, [x9, #48]
    stp  x7, x8, [x9, #64]
    stp  x10, x11, [x9, #96]
    stp  x12, x13, [x9, #112]
    stp  x14, x15, [x9, #128]
    stp  x16, x17, [x9, #144]
    stp  x18, x19, [x9, #160]
    stp  x20, x21, [x9, #176]
    stp  x22, x23, [x9, #192]
    stp  x24, x25, [x9, #208]
    stp  x26, x27, [x9, #224]
    stp  x28, x29, [x9, #240]
    str  x30, [x9, #256]
    mov  x1, sp
    str  x1, [x9, #264]
    mrs  x1, elr_el2
    str  x1, [x9, #272]
    mrs  x1, spsr_el2
    str  x1, [x9, #280]
    adr  x1, _guest_exit
    msr  vbar_el2, x1
    add  x9, x0, #288
    add  x10, x9, #256
    ldr  x1, [x10, #16]
    msr  hcr_el2, x1
    ldr  x1, [x10, #8]
    msr  spsr_el2, x1
    ldr  x1, [x10, #0]
    msr  elr_el2, x1
    ldp  x1, x2, [x9, #16]
    ldp  x3, x4, [x9, #32]
    ldp  x5, x6, [x9, #48]
    ldp  x7, x8, [x9, #64]
    ldp  x10, x11, [x9, #96]
    ldp  x12, x13, [x9, #112]
    ldp  x14, x15, [x9, #128]
    ldp  x16, x17, [x9, #144]
    ldp  x18, x19, [x9, #160]
    ldp  x20, x21, [x9, #176]
    ldp  x22, x23, [x9, #192]
    ldp  x24, x25, [x9, #208]
    ldp  x26, x27, [x9, #224]
    ldp  x28, x29, [x9, #240]
    ldr  x30, [x9, #256]
    ldr  x1, [x9, #264]
    mov  sp, x1
    ldr  x0, [x9, #0]
    eret
.align 11
.global _guest_exit
_guest_exit:
    b    _exit_sync_curr_el_sp0
.align 7
    b    _exit_irq
.align 7
    b    _exit_fiq
.align 7
    b    _exit_serror
.align 7
    b    _exit_sync_curr_el_spx
.align 7
    b    _exit_irq
.align 7
    b    _exit_fiq
.align 7
    b    _exit_serror
.align 7
    b    _exit_sync_lower_aarch64
.align 7
    b    _exit_irq
.align 7
    b    _exit_fiq
.align 7
    b    _exit_serror
.align 7
    b    _exit_sync_lower_aarch32
.align 7
    b    _exit_irq
.align 7
    b    _exit_fiq
.align 7
    b    _exit_serror
_exit_sync_lower_aarch64:
    mrs  x0, tpidr_el2
    add  x9, x0, #288
    stp  x1, x2, [x9, #16]
    stp  x3, x4, [x9, #32]
    stp  x5, x6, [x9, #48]
    stp  x7, x8, [x9, #64]
    stp  x10, x11, [x9, #96]
    stp  x12, x13, [x9, #112]
    stp  x14, x15, [x9, #128]
    stp  x16, x17, [x9, #144]
    stp  x18, x19, [x9, #160]
    stp  x20, x21, [x9, #176]
    stp  x22, x23, [x9, #192]
    stp  x24, x25, [x9, #208]
    stp  x26, x27, [x9, #224]
    stp  x28, x29, [x9, #240]
    str  x30, [x9, #256]
    mov  x1, sp
    str  x1, [x9, #264]
    str  x0, [x9, #0]
    add  x10, x9, #256
    mrs  x1, elr_el2
    str  x1, [x10, #0]
    mrs  x1, spsr_el2
    str  x1, [x10, #8]
    mrs  x1, hcr_el2
    str  x1, [x10, #16]
    mov  x9, x0
    ldp  x1, x2, [x9, #16]
    ldp  x3, x4, [x9, #32]
    ldp  x5, x6, [x9, #48]
    ldp  x7, x8, [x9, #64]
    ldp  x10, x11, [x9, #96]
    ldp  x12, x13, [x9, #112]
    ldp  x14, x15, [x9, #128]
    ldp  x16, x17, [x9, #144]
    ldp  x18, x19, [x9, #160]
    ldp  x20, x21, [x9, #176]
    ldp  x22, x23, [x9, #192]
    ldp  x24, x25, [x9, #208]
    ldp  x26, x27, [x9, #224]
    ldp  x28, x29, [x9, #240]
    ldr  x30, [x9, #256]
    ldr  x1, [x9, #264]
    mov  sp, x1
    ldr  x1, [x9, #272]
    msr  elr_el2, x1
    ldr  x1, [x9, #280]
    msr  spsr_el2, x1
    ret
_exit_sync_curr_el_sp0:
_exit_sync_curr_el_spx:
_exit_sync_lower_aarch32:
_exit_irq:
_exit_fiq:
_exit_serror:
    ret
