use crate::{verify_and_dump_failed_tx, Loader};
use ckb_testtool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};
use ckb_testtool::context::Context;

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn test_stack_reorder_should_run() {
    let loader = Loader::default();
    let mut context = Context::default();

    let stack_reorder_bin = loader.load_binary("stack-reorder");

    let out_point = context.deploy_cell(stack_reorder_bin);

    let lock_script = context
        .build_script(&out_point, Default::default())
        .expect("script");

    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();

    let tx = context.complete_tx(tx);

    verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
}

// Uncomment this to try out a failed test
// #[test]
// fn test_loads_of_hashes_runs() {
//     let loader = Loader::default();
//     let mut context = Context::default();
//
//     let stack_reorder_bin = loader.load_binary("loads-of-hashes");
//
//     let out_point = context.deploy_cell(stack_reorder_bin);
//
//     let lock_script = context
//         .build_script(&out_point, vec![0; 64].into())
//         .expect("script");
//
//     let input_out_point = context.create_cell(
//         CellOutput::new_builder()
//             .capacity(1000u64.pack())
//             .lock(lock_script.clone())
//             .build(),
//         Bytes::new(),
//     );
//     let input = CellInput::new_builder()
//         .previous_output(input_out_point)
//         .build();
//
//     let input_out_point2 = context.create_cell(
//         CellOutput::new_builder()
//             .capacity(1000u64.pack())
//             .lock(lock_script.clone())
//             .build(),
//         vec![1, 2, 3, 4, 5].into(),
//     );
//     let input2 = CellInput::new_builder()
//         .previous_output(input_out_point2)
//         .build();
//
//     let outputs = vec![CellOutput::new_builder()
//         .capacity(500u64.pack())
//         .lock(lock_script.clone())
//         .build()];
//
//     let outputs_data: Vec<Bytes> = vec![vec![6, 7, 8].into()];
//
//     let tx = TransactionBuilder::default()
//         .input(input)
//         .input(input2)
//         .outputs(outputs)
//         .outputs_data(outputs_data.pack())
//         .build();
//
//     let tx = context.complete_tx(tx);
//
//     verify_and_dump_failed_tx(&context, &tx, MAX_CYCLES).expect("pass");
// }
