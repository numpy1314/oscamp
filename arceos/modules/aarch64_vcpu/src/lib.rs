#![no_std]
#![feature(naked_functions)]

#[macro_use]
extern crate axlog;

pub mod sysregs;
mod regs;
pub mod psci;
mod vcpu;

pub use self::vcpu::AARCH64Vcpu;
use sysregs::{AArch64SysRegTrait, SYSREG};
pub use vcpu::AxVCpuExitReason;