use chrono::prelude::*;
use exonum_crypto::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChainInfo {
    server_version: String,
    chain_id: Hash,
    head_block_num: i64,
    last_irreversible_block_num: i64,
    last_irreversible_block_id: Hash,
    head_block_id: Hash,
    head_block_time: NaiveDateTime,
    head_block_producer: String,
    virtual_block_cpu_limit: i64,
    virtual_block_net_limit: i64,
    block_cpu_limit: i64,
    block_net_limit: i64,
    server_version_string: String,
}

