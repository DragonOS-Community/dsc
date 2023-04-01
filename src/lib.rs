#![allow(deprecated)] // llvm_asm!
#![deny(warnings)]
#![no_std]
#![cfg_attr(any(
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "sparc64",
    target_arch = "x86"
), feature(llvm_asm))]

#[cfg(test)]
extern crate std;

pub use platform::*;

pub mod macros;


#[cfg(all(any(target_os = "dragonos"),
          target_arch = "x86_64"))]
#[path="platform/x86_64/mod.rs"]
pub mod platform;
