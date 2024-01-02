#![cfg_attr(not(test), no_std)]

use ckb_std::{ckb_constants::Source, error::SysError, syscalls::load_cell_data};

pub trait Visitor {
    fn visit(&mut self, data: &[u8]) -> bool;
}

pub fn fetch_big_cell_data<V: Visitor>(
    v: &mut V,
    buffer: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<(), SysError> {
    let (mut offset, total) = match load_cell_data(buffer, offset, index, source) {
        Ok(len) => {
            v.visit(&buffer[0..len]);
            return Ok(());
        }
        Err(SysError::LengthNotEnough(len)) => {
            if !v.visit(buffer) {
                return Ok(());
            }
            (offset + buffer.len(), len)
        }
        Err(e) => return Err(e),
    };
    while offset < total {
        let read = match load_cell_data(buffer, offset, index, source) {
            Ok(len) => {
                assert!(offset + len == total);
                len
            }
            Err(SysError::LengthNotEnough(_)) => buffer.len(),
            Err(e) => return Err(e),
        };
        if !v.visit(&buffer[0..read]) {
            return Ok(());
        }
        offset += read;
    }
    Ok(())
}

// Here we provide a native runnable test sample. The test uses ckb-x64-simulator
// to mock CKB syscalls.
#[cfg(test)]
mod tests {
    use super::*;
    use ckb_testtool::ckb_types::{
        bytes::Bytes,
        core::TransactionBuilder,
        packed::{CellInput, CellOutput},
        prelude::*,
    };
    use ckb_testtool::context::Context;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use rusty_fork::rusty_fork_test;
    use std::io::Write;

    #[derive(Default)]
    struct DataVisitor(Vec<u8>);

    impl Visitor for DataVisitor {
        fn visit(&mut self, data: &[u8]) -> bool {
            self.0.extend(data);
            true
        }
    }

    // TODO: Right now ckb-x64-simulator has no way of resetting the
    // test transaction after initial setup. Hence we have to use this
    // circumvent way of testing. Later we would want to fix ckb-x64-simulator
    // so test data can be properly mutated, after that, we can switch
    // to proptest for testing here.
    rusty_fork_test! {
        #[test]
        fn test_any_data() {
            let seed: u64 = match std::env::var("SEED") {
                Ok(val) => str::parse(&val).expect("parsing number"),
                Err(_) => std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64,
            };
            println!("Seed: {}", seed);

            let mut rng = StdRng::seed_from_u64(seed);
            let length = rng.gen_range(0..614400usize);
            let data = {
                let mut data = vec![0u8; length];
                rng.fill(&mut data[..]);
                data
            };

            let file = {
                // Build a tx using data as a cell
                let mut context = Context::default();
                let input_out_point0 = context.create_cell(
                        CellOutput::new_builder()
                            .capacity(1000u64.pack())
                            .build(),
                        Bytes::new(),
                    );
                let input0 = CellInput::new_builder()
                    .previous_output(input_out_point0)
                    .build();

                let input_out_point1 = context.create_cell(
                        CellOutput::new_builder()
                            .capacity(10000000u64.pack())
                            .build(),
                        Bytes::from(data.clone()),
                    );
                let input1 = CellInput::new_builder()
                    .previous_output(input_out_point1)
                    .build();

                let tx = TransactionBuilder::default()
                    .input(input0)
                    .input(input1)
                    .build();

                let mock_tx = context.dump_tx(&tx).expect("dump tx");

                // Keep the tx in a temporary file, then set the environment
                // variable for ckb-x64-simulator
                let json = serde_json::to_string_pretty(&mock_tx).expect("json");
                let mut file = tempfile::NamedTempFile::new().expect("tempfile");
                file.write_all(json.as_ref()).expect("write");
                file.flush().expect("flush");
                std::env::set_var("CKB_TX_FILE", file.path());
                file
            };

            let mut visitor = DataVisitor::default();
            let mut buf = [0u8; 1024];
            let result = fetch_big_cell_data(&mut visitor, &mut buf, 0, 1, Source::Input);
            assert_eq!(result, Ok(()));

            assert_eq!(visitor.0, data);

            drop(file);
        }
    }
}
