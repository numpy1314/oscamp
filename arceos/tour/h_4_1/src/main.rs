#![no_std]
#![no_main]

#[macro_use]
extern crate axlog;
#[macro_use]
extern crate alloc;
extern crate axstd as std;

use alloc::string::{String, ToString};
use aarch64_vcpu::{AARCH64Vcpu, AxVCpuExitReason};
use axerrno::{ax_err_type, AxResult};
use memory_addr::{MemoryAddr, VirtAddr};
use std::fs::File;

use axmm::AddrSpaceHV as AddrSpace;
use axhal::paging::MappingFlags;

const VM_ASPACE_BASE: usize = 0x0;
const VM_ASPACE_SIZE: usize = 0x7fff_ffff_f000;

const PHY_MEM_START: usize = 0x4000_0000;
const PHY_MEM_SIZE: usize = 0x800_0000;

const KERNEL_BASE: usize = 0x4008_0000;

const BOOTINFO_GPA: usize = 0x4000_0000;

#[inline(always)]
fn flush_stage2_tlb() {
    unsafe {
        core::arch::asm!(
            "dsb ishst",
            "tlbi vmalls12e1is",
            "dsb ish",
            "isb",
            options(nostack, preserves_flags),
        );
    }
}

#[no_mangle]
fn main() {
    ax_println!("[h_4_1] Starting virtualization with device passthrough...");

    let mut aspace = AddrSpace::new_empty(VirtAddr::from(VM_ASPACE_BASE), VM_ASPACE_SIZE).unwrap();

    ax_println!("[h_4_1] Setting up memory regions...");
    for r in axhal::mem::memory_regions() {
        let flags: MappingFlags = r.flags.into();
        if flags.contains(MappingFlags::DEVICE) {
            aspace.map_linear(r.paddr.as_usize().into(), r.paddr, r.size, flags).unwrap();
            ax_println!("[h_4_1] Pre-mapped device at {:#x}, size {:#x}", r.paddr, r.size);
        }
    }

    let ram_flags = MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER;
    aspace.map_alloc(PHY_MEM_START.into(), PHY_MEM_SIZE, ram_flags, true).unwrap();

    {
        let boot_page = aspace
            .translated_byte_buffer(VirtAddr::from(BOOTINFO_GPA), 4096)
            .expect("translate BOOTINFO_GPA failed");
        for seg in boot_page {
            for b in seg.iter_mut() {
                *b = 0;
            }
        }
        ax_println!("[h_4_1] Bootinfo/DTB placeholder page is zeroed at {:#x}", BOOTINFO_GPA);
    }
    flush_stage2_tlb();

    ax_println!("[h_4_1] loading guest image...");
    let image_fname = "/sbin/m_1_2_aarch64-qemu-virt-hv.bin";
    load_vm_image(image_fname.to_string(), KERNEL_BASE.into(), &aspace)
        .expect("Failed to load VM images");

    let mut arch_vcpu = AARCH64Vcpu::init();

    ax_println!(
        "[h_4_1] guest entry: {:#x}; stage2 root: {:#x}",
        KERNEL_BASE,
        aspace.page_table_root()
    );

    arch_vcpu.set_entry(KERNEL_BASE.into()).unwrap();
    arch_vcpu.set_ept_root(aspace.page_table_root()).unwrap();


    loop {
        match arch_vcpu.run() {
            Ok(exit_reason) => match exit_reason {
                AxVCpuExitReason::Nothing => {}
                AxVCpuExitReason::PageFault { addr, access_flags } => {
                    ax_println!(
                        "[h_4_1] Stage-2 PageFault IPA={:#x}, flags={:?}",
                        addr, access_flags
                    );

                    let ipa_page = addr.align_down_4k();
                    match aspace.map_alloc(
                        ipa_page,
                        4096,
                        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER,
                        true,
                    ) {
                        Ok(_) => {
                            flush_stage2_tlb();
                            ax_println!("[h_4_1] Stage-2 mapped IPA {:#x}", ipa_page);
                        }
                        Err(e) => {
                            ax_println!("[h_4_1] Stage-2 map IPA {:#x} failed: {:?}", ipa_page, e);
                            flush_stage2_tlb();
                        }
                    }
                }
            },
            Err(err) => panic!("run VCpu get error {:?}", err),
        }
    }
}

fn load_vm_image(image_path: String, image_load_gpa: VirtAddr, aspace: &AddrSpace) -> AxResult {
    use std::io::{BufReader, Read};
    let (image_file, image_size) = open_image_file(image_path.as_str())?;

    let image_load_regions = aspace
        .translated_byte_buffer(image_load_gpa, image_size)
        .expect("Failed to translate kernel image load address");
    let mut file = BufReader::new(image_file);

    for buffer in image_load_regions {
        file.read_exact(buffer).map_err(|err| {
            ax_err_type!(Io, format!("Failed in reading from file {}, err {:?}", image_path, err))
        })?
    }

    Ok(())
}

fn open_image_file(file_name: &str) -> AxResult<(File, usize)> {
    let file = File::open(file_name).map_err(|err| {
        ax_err_type!(
            NotFound,
            format!("Failed to open {}, err {:?}, please check your disk.img", file_name, err)
        )
    })?;
    let file_size = file
        .metadata()
        .map_err(|err| {
            ax_err_type!(Io, format!("Failed to get metadate of file {}, err {:?}", file_name, err))
        })?
        .size() as usize;
    Ok((file, file_size))
}
