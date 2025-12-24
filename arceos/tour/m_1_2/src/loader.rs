use axstd::io;
use axhal::paging::MappingFlags;
use axmm::AddrSpace;
use crate::APP_ENTRY;

pub fn load_user_app(uspace: &mut AddrSpace) -> io::Result<()> {
    ax_println!("[m_1_2] Loading hardcoded user program...");
    
    let user_code: &[u8] = &[
        // mov x8, #93      // sys_exit syscall number  
        0xe8, 0x0b, 0x80, 0xd2,
        // mov x0, #42      // exit code
        0x40, 0x05, 0x80, 0xd2,
        // svc #0           // make syscall
        0x01, 0x00, 0x00, 0xd4,
    ];

    // Map at least one page (4KB) for user code
    let code_size = 0x1000;  // Use full 4KB for user code
    uspace.map_alloc(
        APP_ENTRY.into(), 
        code_size, 
        MappingFlags::READ|MappingFlags::WRITE|MappingFlags::EXECUTE|MappingFlags::USER, 
        true
    ).unwrap();

    // Use write method instead of translated_byte_buffer to avoid phys_to_virt issues in HV environment
    uspace.write(APP_ENTRY.into(), user_code)
        .expect("Failed to write user code to user space");

    ax_println!("[m_1_2] User program loaded successfully ({} bytes)", user_code.len());

    Ok(())
}
