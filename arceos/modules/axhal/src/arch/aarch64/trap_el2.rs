// EL2 异常处理 Rust 代码
use core::arch::global_asm;

use aarch64_cpu::registers::{ESR_EL2, FAR_EL2};
use page_table_entry::MappingFlags;
use tock_registers::interfaces::Readable;

use super::TrapFrame;

global_asm!(include_str!("trap_el2.S"));

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum TrapKind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u8)]
#[derive(Debug)]
#[allow(dead_code)]
enum TrapSource {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

#[no_mangle]
fn invalid_exception_el2(tf: &TrapFrame, kind: TrapKind, source: TrapSource) {
    panic!(
        "Invalid exception {:?} from {:?} (EL2):\n{:#x?}",
        kind, source, tf
    );
}

#[no_mangle]
fn handle_irq_exception_el2(_tf: &TrapFrame) {
    handle_trap!(IRQ, 0);
}

fn handle_instruction_abort_el2(tf: &TrapFrame, iss: u64, is_lower: bool) {
    let mut access_flags = MappingFlags::EXECUTE;
    if is_lower {
        access_flags |= MappingFlags::USER;
    }
    let vaddr = va!(FAR_EL2.get() as usize);

    // Only handle Translation fault and Permission fault
    if !matches!(iss & 0b111100, 0b0100 | 0b1100)
        || !handle_trap!(PAGE_FAULT, vaddr, access_flags, is_lower)
    {
        panic!(
            "Unhandled {} Instruction Abort @ {:#x}, fault_vaddr={:#x}, ISS={:#x} ({:?}):\n{:#x?}",
            if is_lower { "Lower EL" } else { "EL2" },
            tf.elr,
            vaddr,
            iss,
            access_flags,
            tf,
        );
    }
}

fn handle_data_abort_el2(tf: &TrapFrame, iss: u64, is_lower: bool) {
    let wnr = (iss & (1 << 6)) != 0; // WnR: Write not Read
    let cm = (iss & (1 << 8)) != 0; // CM: Cache maintenance
    let mut access_flags = if wnr & !cm {
        MappingFlags::WRITE
    } else {
        MappingFlags::READ
    };
    if is_lower {
        access_flags |= MappingFlags::USER;
    }
    let vaddr = va!(FAR_EL2.get() as usize);

    // Only handle Translation fault and Permission fault
    if !matches!(iss & 0b111100, 0b0100 | 0b1100)
        || !handle_trap!(PAGE_FAULT, vaddr, access_flags, is_lower)
    {
        panic!(
            "Unhandled {} Data Abort @ {:#x}, fault_vaddr={:#x}, ISS=0b{:08b} ({:?}):\n{:#x?}",
            if is_lower { "Lower EL" } else { "EL2" },
            tf.elr,
            vaddr,
            iss,
            access_flags,
            tf,
        );
    }
}

#[no_mangle]
fn handle_sync_exception_el2(tf: &mut TrapFrame) {
    let esr = ESR_EL2.extract();
    let iss = esr.read(ESR_EL2::ISS);
    
    // 使用 EC 位域的值进行匹配
    let ec = esr.read(ESR_EL2::EC);
    
    match ec {
        0x15 => { // SVC64 from lower EL
            warn!("No syscall is supported currently!");
        }
        0x16 => { // HVC64
            debug!("HVC instruction trapped, ISS={:#x}", iss);
            // 虚拟化场景下的 HVC 处理
        }
        0x20 => { // Instruction Abort from lower EL
            handle_instruction_abort_el2(tf, iss, true)
        }
        0x21 => { // Instruction Abort from current EL
            handle_instruction_abort_el2(tf, iss, false)
        }
        0x24 => { // Data Abort from lower EL
            handle_data_abort_el2(tf, iss, true)
        }
        0x25 => { // Data Abort from current EL
            handle_data_abort_el2(tf, iss, false)
        }
        0x3c => { // BRK64
            debug!("BRK #{:#x} @ {:#x} ", iss, tf.elr);
            tf.elr += 4;
        }
        _ => {
            panic!(
                "Unhandled synchronous exception @ {:#x}: ESR={:#x} (EC {:#08b}, ISS {:#x})",
                tf.elr,
                esr.get(),
                ec,
                iss,
            );
        }
    }
}
