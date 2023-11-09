//! # dsc - DragonOS Raw Syscall Binding
//!
//! This is a raw syscall binding for DragonOS. It is not meant to be used directly, but rather as a dependency for other crates.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! dsc = { git = "https://github.com/DragonOS-Community/dsc.git" , rev = "The Git Commit You Want" }
//! ```
//!
//! ## 其他
//!
//! 如果您正在开发dsc，请您在引入dsc的库的`Cargo.toml`中添加如下内容，而不是使用上述的代码：
//!
//! ```toml
//! [dependencies]
//! dsc = { path = "您本地存放dsc的源代码的路径" }
//! ```
//!
//! ## How to build
//!
//! ```bash
//! ARCH=x86_64 && cargo build -Zbuild-std --release --target src/platform/$ARCH/target.json
//! ```
//!
//! ## How to build docs
//!
//! ```bash
//! ARCH=x86_64 && cargo doc -Zbuild-std --release --target src/platform/$ARCH/target.json
//! ```
//!
//! ## What is DragonOS?
//!
//! DragonOS is an opensource operating system developed for the server field.
//! Its kernel and user mode environment are developed from scratch, and provides Linux compatibility.
//!
//! - [DragonOS Website](https://dragonos.org)
//! - [DragonOS Github](https://github.com/DragonOS-Community)
//!
//! ## License
//!
//! Licensed under
//!   * MIT license (<http://opensource.org/licenses/MIT>)

#![allow(deprecated)] // llvm_asm!
#![deny(warnings)]
#![no_std]
#![cfg_attr(
    any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "powerpc",
        target_arch = "powerpc64",
        target_arch = "sparc64",
        target_arch = "x86"
    ),
    feature(llvm_asm)
)]

#[cfg(test)]
extern crate std;

pub use platform::*;

pub(crate) mod macros;

#[cfg(all(any(target_os = "dragonos"), target_arch = "x86_64"))]
#[path = "platform/x86_64/mod.rs"]
pub mod platform;
