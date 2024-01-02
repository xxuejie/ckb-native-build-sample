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

use big_cell_fetcher::{fetch_big_cell_data, Visitor};
use big_witness_hasher::hash_extra_witnesses;
use blake2b_ref::{Blake2b, Blake2bBuilder};
use ckb_std::{ckb_constants::Source, debug, high_level::load_script};

struct Hasher(Blake2b);

impl Hasher {
    fn new() -> Self {
        Hasher(Blake2bBuilder::new(32).personal(b"cell").build())
    }

    fn finalize(self) -> [u8; 32] {
        let mut r = [0u8; 32];
        self.0.finalize(&mut r);
        r
    }
}

impl Visitor for Hasher {
    fn visit(&mut self, data: &[u8]) -> bool {
        self.0.update(data);
        true
    }
}

pub fn program_entry() -> i8 {
    let witness_hash = match hash_extra_witnesses() {
        Ok(hash) => hash,
        Err(value) => {
            debug!("Error occured when hashing extra witnesses: {}", value);
            return -1;
        }
    };

    let mut cell_hasher = Hasher::new();
    let mut buffer = [0u8; 1024];

    fetch_big_cell_data(&mut cell_hasher, &mut buffer, 0, 1, Source::Input)
        .expect("fetch input cell #1");
    fetch_big_cell_data(&mut cell_hasher, &mut buffer, 0, 0, Source::Output)
        .expect("fetch output cell #0");

    let cell_hash = cell_hasher.finalize();

    let script = load_script().expect("load script");

    if script.args().len() != 64 {
        debug!("Invalid script args length: {}", script.args().len());
        return -1;
    }
    assert_eq!(
        &script.args().raw_data()[0..32],
        &cell_hash[..],
        "cell hash"
    );
    assert_eq!(
        &script.args().raw_data()[32..64],
        &witness_hash[..],
        "witness hash"
    );

    0
}
