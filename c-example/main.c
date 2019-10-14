#include <stdio.h>
#include <stdlib.h>

#include "ckb_ffi.h"

void print_buffer(char *tag, buffer_t *buf) {
    printf("\tInfo : %s = [\n\t\t", tag);
    for (int i=0; i<buf->len; i++) {
        printf("%02x", buf->data[i]);
    }
    printf("\n\t]\n");
}

void panic(char *message) {
    printf("%s\n\n", message);
    exit(1);
}

int main (int argc, char *argv[])
{
    printf("Begin:\n");

    buffer_t tx_buf;

    {
        buffer_t hash_buf;

        char tx_json[] ="{\
            \"version\": \"0x0\",\
            \"cell_deps\": [],\
            \"header_deps\": [],\
            \"inputs\": [],\
            \"outputs\": [],\
            \"witnesses\": [],\
            \"outputs_data\": []\
        }";
        if (ckb_load_transaction(&tx_buf, tx_json) != 0) {
            panic("    Error: failed to load a transaction from json string.");
        } else {
            print_buffer("transaction", &tx_buf);
        }

        if (ckb_transaction_calc_hash(&hash_buf, &tx_buf) != 0) {
            panic("    Error: failed to calc transaction hash.");
        } else {
            print_buffer("transaction hash", &hash_buf);
        }
        buffer_free(hash_buf);

        if (ckb_transaction_calc_witness_hash(&hash_buf, &tx_buf) != 0) {
            panic("    Error: failed to calc transaction witness hash.");
        } else {
            print_buffer("transaction witness hash", &hash_buf);
        }
        buffer_free(hash_buf);
    }

    {
        buffer_t block_buf;
        buffer_t block_with_tx_buf;
        buffer_t new_block_buf;
        buffer_t hash_buf;

        char block_json[] = "{\
            \"header\": {\
                \"version\": \"0x0\",\
                \"parent_hash\": \"0x0000000000000000000000000000000000000000000000000000000000000000\",\
                \"timestamp\": \"0x0\",\
                \"number\": \"0x0\",\
                \"epoch\": \"0x0\",\
                \"transactions_root\": \"0x0000000000000000000000000000000000000000000000000000000000000000\",\
                \"proposals_hash\": \"0x0000000000000000000000000000000000000000000000000000000000000000\",\
                \"compact_target\": \"0x0\",\
                \"uncles_hash\": \"0x0000000000000000000000000000000000000000000000000000000000000000\",\
                \"dao\": \"0x0000000000000000000000000000000000000000000000000000000000000000\",\
                \"nonce\": \"0x0\"\
            },\
            \"uncles\": [],\
            \"transactions\": [],\
            \"proposals\": []\
        }";
        if (ckb_load_block(&block_buf, block_json) != 0) {
            panic("    Error: failed to load a block from json string.");
        } else {
            print_buffer("block", &block_buf);
        }

        if (ckb_block_insert_transaction(&block_with_tx_buf, &block_buf, &tx_buf) != 0) {
            panic("    Error: failed to insert a transaction into a block.");
        } else {
            print_buffer("block with tx", &block_with_tx_buf);
        }

        if (ckb_block_reset_header(&new_block_buf, &block_with_tx_buf) != 0) {
            panic("    Error: failed to reset header for a block.");
        } else {
            print_buffer("block with new header", &new_block_buf);
        }
        buffer_free(block_buf);
        buffer_free(block_with_tx_buf);
        buffer_free(new_block_buf);

        char template_json[] = "{\
            \"version\": \"0x0\",\
            \"compact_target\": \"0x0\",\
            \"current_time\": \"0x0\",\
            \"number\": \"0x0\",\
            \"epoch\": \"0x0\",\
            \"parent_hash\": \"0x0000000000000000000000000000000000000000000000000000000000000000\",\
            \"cycles_limit\": \"0x0\",\
            \"bytes_limit\": \"0x0\",\
            \"uncles_count_limit\": \"0x0\",\
            \"uncles\": [],\
            \"transactions\": [],\
            \"proposals\": [],\
            \"cellbase\": {\
                \"hash\": \"0x0000000000000000000000000000000000000000000000000000000000000000\",\
                \"cycles\": null,\
                \"data\": {\
                    \"version\": \"0x0\",\
                    \"cell_deps\": [],\
                    \"header_deps\": [],\
                    \"inputs\": [],\
                    \"outputs\": [],\
                    \"witnesses\": [],\
                    \"outputs_data\": []\
                }\
            },\
            \"work_id\": \"0x0\",\
            \"dao\": \"0x0000000000000000000000000000000000000000000000000000000000000000\"\
        }";
        if (ckb_load_block_from_template(&block_buf, template_json) != 0) {
            panic("    Error: failed to load a block from block template json string.");
        } else {
            print_buffer("block from template", &block_buf);
        }

        if (ckb_block_set_nonce(&new_block_buf, &block_buf, 0x123456) != 0) {
            panic("    Error: failed to set nonce to a block.");
        } else {
            print_buffer("block with new nonce", &new_block_buf);
        }

        if (ckb_block_calc_pow_hash(&hash_buf, &new_block_buf) != 0) {
            panic("    Error: failed to calc pow hash for a block.");
        } else {
            print_buffer("pow hash", &hash_buf);
        }
        buffer_free(block_buf);
        buffer_free(new_block_buf);
        buffer_free(hash_buf);
    }

    buffer_free(tx_buf);

    printf("End.\n");
    return 0;
}
