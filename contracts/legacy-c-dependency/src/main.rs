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

use blake2b_rs::Blake2bBuilder;
use ckb_std::{
    debug,
    high_level::{load_script, load_tx_hash},
};

pub fn program_entry() -> i8 {
    let tx_hash = load_tx_hash().expect("load tx hash");

    let mut blake2b = Blake2bBuilder::new(32).personal(b"tx").build();
    blake2b.update(&tx_hash);
    let mut result = [0u8; 32];
    blake2b.finalize(&mut result);

    let script = load_script().expect("load script");

    if script.args().len() != 32 {
        debug!("Invalid script args length: {}", script.args().len());
        return -1;
    }
    assert_eq!(
        &script.args().raw_data()[0..32],
        &tx_hash[..],
        "tx hash hash"
    );

    0
}
