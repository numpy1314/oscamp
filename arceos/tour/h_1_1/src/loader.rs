use std::io::{self, Read};
use std::fs::File;
use axhal::paging::MappingFlags;
use axhal::mem::{PAGE_SIZE_4K, phys_to_virt};
use axmm::AddrSpace;
use crate::VM_ENTRY;
pub fn load_vm_image(fname: &str, uspace: &mut AddrSpace) -> io::Result<()> {
    let mut buf = [0u8; 64];
    load_file(fname, &mut buf)?;
    ax_println!("[loader] User address space range: {:#x} - {:#x}", uspace.base(), uspace.end());
    
    // 映射 VM_ENTRY 开始的 128KB，包括代码、数据和栈
    let map_size = 128 * 1024; // 128KB
    ax_println!("[loader] Attempting to map VM_ENTRY: {:#x}, size: {:#x}", VM_ENTRY, map_size);
    ax_println!("[loader] Buffer size: {} bytes", buf.len());
    match uspace.map_alloc(
        VM_ENTRY.into(),
        map_size,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER,
        true
    ) {
        Ok(_) => ax_println!("[loader] map_alloc succeeded"),
        Err(e) => {
            ax_println!("[loader] map_alloc failed: {:?}", e);
            panic!("map_alloc failed for VM_ENTRY {:#x}: {:?}", VM_ENTRY, e);
        }
    };
    let (paddr, _, _) = uspace
        .page_table()
        .query(VM_ENTRY.into())
        .unwrap_or_else(|_| panic!("Mapping failed for segment: {:#x}", VM_ENTRY));
    ax_println!("[loader] Guest image paddr: {:#x}, size: {} bytes", paddr, buf.len());
    
    // 读取 Guest 代码的前几个字节
    ax_println!("[loader] Guest code: {:02x?}", &buf[..20.min(buf.len())]);
    
    unsafe {
        core::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            phys_to_virt(paddr).as_mut_ptr(),
            buf.len(),
        );
        
        // 验证复制是否成功
        let copied = core::slice::from_raw_parts(phys_to_virt(paddr).as_ptr(), 20.min(buf.len()));
        ax_println!("[loader] Verified copied data: {:02x?}", copied);
    }
    ax_println!("[loader] Guest image copied to memory successfully");
    Ok(())
}
fn load_file(fname: &str, buf: &mut [u8]) -> io::Result<usize> {
    ax_println!("app: {}", fname);
    let mut file = File::open(fname)?;
    let n = file.read(buf)?;
    Ok(n)
}
