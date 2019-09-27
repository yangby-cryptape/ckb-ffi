use ckb_types::prelude::*;

use crate::utils;

#[no_mangle]
pub unsafe extern "C" fn ckb_load_transaction(
    mut output: &mut utils::Buffer,
    json: *const libc::c_char,
) -> i32 {
    let mut retcode = 0;
    let json_str = utils::cstring_to_str(json);
    let result = serde_json::from_str::<ckb_jsonrpc_types::Transaction>(json_str);
    if let Ok(json_struct) = result {
        let packed_struct: ckb_types::packed::Transaction = json_struct.into();
        utils::vector_into_buffer(&mut output, packed_struct.as_slice().to_owned());
    } else {
        retcode = 1;
    }
    retcode
}

#[no_mangle]
pub unsafe extern "C" fn ckb_load_block(
    mut output: &mut utils::Buffer,
    json: *const libc::c_char,
) -> i32 {
    let mut retcode = 0;
    let json_str = utils::cstring_to_str(json);
    let result = serde_json::from_str::<ckb_jsonrpc_types::Block>(json_str);
    if let Ok(json_struct) = result {
        let packed_struct: ckb_types::packed::Block = json_struct.into();
        utils::vector_into_buffer(&mut output, packed_struct.as_slice().to_owned());
    } else {
        retcode = 1;
    }
    retcode
}

#[no_mangle]
pub unsafe extern "C" fn ckb_load_block_from_template(
    mut output: &mut utils::Buffer,
    json: *const libc::c_char,
) -> i32 {
    let mut retcode = 0;
    let json_str = utils::cstring_to_str(json);
    let result = serde_json::from_str::<ckb_jsonrpc_types::BlockTemplate>(json_str);
    if let Ok(json_struct) = result {
        let packed_struct: ckb_types::packed::Block = json_struct.into();
        utils::vector_into_buffer(&mut output, packed_struct.as_slice().to_owned());
    } else {
        retcode = 1;
    }
    retcode
}

#[no_mangle]
pub unsafe extern "C" fn ckb_transaction_calc_hash(
    mut output: &mut utils::Buffer,
    input: *const utils::Buffer,
) -> i32 {
    let mut retcode = 0;
    let slice = utils::buffer_to_slice(input);
    let result = ckb_types::packed::TransactionReader::from_slice(slice);
    if let Ok(packed_struct) = result {
        let hash = packed_struct.calc_tx_hash();
        utils::vector_into_buffer(&mut output, hash.as_slice().to_owned());
    } else {
        retcode = 1;
    }
    retcode
}

#[no_mangle]
pub unsafe extern "C" fn ckb_transaction_calc_witness_hash(
    mut output: &mut utils::Buffer,
    input: *const utils::Buffer,
) -> i32 {
    let mut retcode = 0;
    let slice = utils::buffer_to_slice(input);
    let result = ckb_types::packed::TransactionReader::from_slice(slice);
    if let Ok(packed_struct) = result {
        let hash = packed_struct.calc_witness_hash();
        utils::vector_into_buffer(&mut output, hash.as_slice().to_owned());
    } else {
        retcode = 1;
    }
    retcode
}

#[no_mangle]
pub unsafe extern "C" fn ckb_block_insert_transaction(
    mut output: &mut utils::Buffer,
    input_block: *const utils::Buffer,
    input_tx: *const utils::Buffer,
) -> i32 {
    let mut retcode = 0;
    let slice_block = utils::buffer_to_slice(input_block);
    let result_block = ckb_types::packed::Block::from_slice(slice_block);
    if let Ok(packed_block) = result_block {
        let slice_tx = utils::buffer_to_slice(input_tx);
        let result_tx = ckb_types::packed::Transaction::from_slice(slice_tx);
        if let Ok(packed_tx) = result_tx {
            let new_txs = packed_block
                .transactions()
                .as_builder()
                .push(packed_tx)
                .build();
            let new_block = packed_block
                .as_builder()
                .transactions(new_txs)
                .build()
                .reset_header();
            utils::vector_into_buffer(&mut output, new_block.as_slice().to_owned());
        } else {
            retcode = 1;
        }
    } else {
        retcode = 1;
    }
    retcode
}

#[no_mangle]
pub unsafe extern "C" fn ckb_block_set_nonce(
    mut output: &mut utils::Buffer,
    input: *const utils::Buffer,
    nonce: u64,
) -> i32 {
    let mut retcode = 0;
    let slice = utils::buffer_to_slice(input);
    let result = ckb_types::packed::Block::from_slice(slice);
    if let Ok(packed_block) = result {
        let new_header = packed_block
            .header()
            .as_builder()
            .nonce(nonce.pack())
            .build();
        let new_block = packed_block
            .as_builder()
            .header(new_header)
            .build()
            .reset_header();
        utils::vector_into_buffer(&mut output, new_block.as_slice().to_owned());
    } else {
        retcode = 1;
    }
    retcode
}

#[no_mangle]
pub unsafe extern "C" fn ckb_block_reset_header(
    mut output: &mut utils::Buffer,
    input: *const utils::Buffer,
) -> i32 {
    let mut retcode = 0;
    let slice = utils::buffer_to_slice(input);
    let result = ckb_types::packed::Block::from_slice(slice);
    if let Ok(packed_block) = result {
        let new_block = packed_block.reset_header();
        utils::vector_into_buffer(&mut output, new_block.as_slice().to_owned());
    } else {
        retcode = 1;
    }
    retcode
}

#[no_mangle]
pub unsafe extern "C" fn ckb_block_calc_pow_hash(
    mut output: &mut utils::Buffer,
    input: *const utils::Buffer,
) -> i32 {
    let mut retcode = 0;
    let slice = utils::buffer_to_slice(input);
    let result = ckb_types::packed::BlockReader::from_slice(slice);
    if let Ok(packed_struct) = result {
        let hash = packed_struct.header().calc_pow_hash();
        utils::vector_into_buffer(&mut output, hash.as_slice().to_owned());
    } else {
        retcode = 1;
    }
    retcode
}
