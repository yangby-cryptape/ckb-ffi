#ifndef CKB_FFI_H
#define CKB_FFI_H

#ifdef __cplusplus
#define _CPP_BEGIN extern "C" {
#define _CPP_END }
_CPP_BEGIN
#endif /* __cplusplus */

#include <stdint.h>

typedef struct {
    uint64_t len;
    uint8_t *data;
} buffer_t;

extern void buffer_free(buffer_t);

extern int32_t ckb_load_transaction(buffer_t* output_tx, char* json_tx);
extern int32_t ckb_load_block(buffer_t* output_block, char* json_block);
extern int32_t ckb_load_block_from_template(buffer_t* output_block, char* json_block_template);

extern int32_t ckb_transaction_calc_hash(buffer_t* output_hash, buffer_t* tx);
extern int32_t ckb_transaction_calc_witness_hash(buffer_t* output_hash, buffer_t* tx);
extern int32_t ckb_block_insert_transaction(buffer_t* output_block, buffer_t* block, buffer_t* tx);
extern int32_t ckb_block_set_nonce(buffer_t* output_block, buffer_t* block, uint64_t nonce);
extern int32_t ckb_block_reset_header(buffer_t* output_block, buffer_t* block);
extern int32_t ckb_block_calc_pow_hash(buffer_t* output_hash, buffer_t* block);

#ifdef __cplusplus
_CPP_END
#undef _CPP_BEGIN
#undef _CPP_END
#endif /* __cplusplus */

#endif /* CKB_FFI_H */
