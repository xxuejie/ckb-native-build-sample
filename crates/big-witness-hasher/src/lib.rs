#![cfg_attr(not(test), no_std)]

use core::ffi::c_int;

#[cfg(target_arch = "riscv64")]
#[link(name = "c-impl", kind = "static")]
extern "C" {
    fn hash_extra_witnesses_inner(hash: *mut u8) -> c_int;
}

// For C code included in a crate, it might not be easily mocked out.
// So for now we recommended the following path here, and manually avoided
// calling into the corresponding C code at native side.
//
// Full contract level test can be leveraged to ensure the correctness
// of C code included. Unit testing C code directly might not be a worthwhile
// path to pursue as of now.
#[cfg(not(target_arch = "riscv64"))]
unsafe fn hash_extra_witnesses_inner(_hash: *mut u8) -> c_int {
    // Simply to make native build pass
    -1
}

pub fn hash_extra_witnesses() -> Result<[u8; 32], c_int> {
    let mut result = [0u8; 32];
    let s = unsafe { hash_extra_witnesses_inner(result.as_mut_ptr()) };
    if s != 0 {
        return Err(s);
    }
    Ok(result)
}
