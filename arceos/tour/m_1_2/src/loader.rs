use axstd::io;
use axhal::paging::MappingFlags;
use axmm::AddrSpace;
use crate::APP_ENTRY;
use alloc::vec;
use alloc::vec::Vec;
use axstd::fs::File;
use axstd::io::Read;

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

    uspace.map_alloc(APP_ENTRY.into(), user_code.len(), MappingFlags::READ|MappingFlags::WRITE|MappingFlags::EXECUTE|MappingFlags::USER, true).unwrap();

    let image_load_regions = uspace
        .translated_byte_buffer(APP_ENTRY.into(), user_code.len())
        .expect("Failed to translate user app load address");

    ax_println!("Loading user program into user space at vaddr: {:#x}", APP_ENTRY);

    let mut offset = 0;
    for buffer in image_load_regions {
        let len = core::cmp::min(buffer.len(), user_code.len() - offset);
        buffer[..len].copy_from_slice(&user_code[offset..offset + len]);
        offset += len;
        if offset >= user_code.len() {
            break;
        }
    }

    ax_println!("[m_1_2] User program loaded successfully ({} bytes)", user_code.len());

    Ok(())
}

fn load_from_disk(path: &str) -> io::Result<Vec<u8>> {
    ax_println!("Loading user program from disk: {}", path);
    
    let mut file = File::open(path)
        .map_err(|e| {
            ax_println!("Failed to open {}: {:?}", path, e);
            e
        })?;
    
    let file_size = file.metadata()?.size() as usize;
    ax_println!("User program size: {} bytes", file_size);
    
    let mut buf = vec![0u8; file_size];
    file.read_exact(&mut buf)
        .map_err(|e| {
            ax_println!("Failed to read {}: {:?}", path, e);
            e
        })?;
    
    ax_println!("Successfully loaded user program from disk");
    Ok(buf)
}
