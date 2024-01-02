#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

pub fn program_entry() -> i8 {
    use alloc::string::ToString;
    ckb_std::syscalls::debug("This is printed in any config".to_string());

    ckb_std::debug!("This is only printed when debug_assertions cfg is enabled in RUSTFLAGS!");

    0
}
