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
    log::error!("Just a log line");

    let data = code::load();
    let r = code::run(data.slice(0..16));
    if r == 0 {
        return 0;
    }
    let data2 = code::load();
    let mut buf = bytes::BytesMut::new();
    buf.extend(data.slice(8..));
    buf.extend(data2);
    let r = code::run(buf.freeze());
    if r != 0 {
        0
    } else {
        1
    }
}

// Just some silly code to play with Bytes so it won't be eaten by compilers.
mod code {
    use bytes::Bytes;

    #[inline(never)]
    pub fn load() -> Bytes {
        let mut tx_hash = [0u8; 32];
        let len = ckb_std::syscalls::load_tx_hash(&mut tx_hash, 0).unwrap();
        assert_eq!(len, tx_hash.len());

        Bytes::from(tx_hash.to_vec())
    }

    #[inline(never)]
    pub fn run(data: Bytes) -> i8 {
        let mut i: i8 = 0;
        for d in data {
            i = i.wrapping_add(d as i8);
        }
        i
    }
}
