use std::io::{self, Read};
use std::fs::File;
use axhal::paging::MappingFlags;
use axhal::mem::{PAGE_SIZE_4K, phys_to_virt};
use axmm::AddrSpace;
use crate::VM_ENTRY;
pub fn load_vm_image(fname: &str, uspace: &mut AddrSpace) -> io::Result<()> {
    let mut buf = [0u8; 64];
    load_file(fname, &mut buf)?;
    ax_println!("[DEBUG] User address space range: {:#x} - {:#x}", uspace.base(), uspace.end());
    ax_println!("[DEBUG] Attempting to map VM_ENTRY: {:#x}, size: {:#x}", VM_ENTRY, PAGE_SIZE_4K);
    ax_println!("[DEBUG] contains_range check: {}", uspace.contains_range(VM_ENTRY.into(), PAGE_SIZE_4K));
    match uspace.map_alloc(
        VM_ENTRY.into(),
        PAGE_SIZE_4K,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER,
        true
    ) {
        Ok(_) => ax_println!("[DEBUG] map_alloc succeeded"),
        Err(e) => {
            ax_println!("[DEBUG] map_alloc failed: {:?}", e);
            panic!("map_alloc failed for VM_ENTRY {:#x}: {:?}", VM_ENTRY, e);
        }
    };
    let (paddr, _, _) = uspace
        .page_table()
        .query(VM_ENTRY.into())
        .unwrap_or_else(|_| panic!("Mapping failed for segment: {:#x}", VM_ENTRY));
    ax_println!("paddr: {:#x}", paddr);
    unsafe {
        core::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            phys_to_virt(paddr).as_mut_ptr(),
            PAGE_SIZE_4K,
        );
    }
    Ok(())
}
fn load_file(fname: &str, buf: &mut [u8]) -> io::Result<usize> {
    ax_println!("app: {}", fname);
    let mut file = File::open(fname)?;
    let n = file.read(buf)?;
    Ok(n)
}
