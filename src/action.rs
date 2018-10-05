use chrono::prelude::*;
use exonum_crypto::*;

use std::future::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Action {
    kind: String,
    payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockInfo {
    block_hash: String,
    block_number: i64,
    previous_block_hash: String,
    timestamp: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    actions: Vec<Action>,
    block_info: BlockInfo,
}

pub struct ActionReader {
    endpoint: String,
    head_block_number: i64,
    current_block_number: i64,
    is_first_block: bool,
    current_block_data: Option<Block>,
    block_history: Vec<Block>,
    start_at_block: i64,
    only_irreversible: bool,
    max_history_length: usize,
    // requestInstance: any = request,
}

impl Default for ActionReader {
    fn default() -> ActionReader {
        ActionReader {
            endpoint: "http://127.0.0.1:8888".to_owned(),
            head_block_number: 0,
            current_block_number: 0,
            is_first_block: true,
            current_block_data: None,
            block_history: vec![],
            start_at_block: 1,
            only_irreversible: false,
            max_history_length: 600,
            // protected requestInstance: any = request,
        }
    }
}

impl ActionReader {
    /**
     * Loads the head block number, returning an int.
     * If onlyIrreversible is true, return the most recent irreversible block number
     * @return {Promise<number>}
     */
    async fn get_head_block_number(&self) -> i64 {
        0
    }

    /**
     * Loads a block with the given block number
     * @param {number} blockNumber - Number of the block to retrieve
     * @returns {Block}
     */
    async fn get_block(&self, block_number: i64) -> Option<Block> {
        let block_info = BlockInfo {
            block_hash: "000".to_owned(),
            block_number: block_number,
            previous_block_hash: "000".to_owned(),
            timestamp: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        };
        Some(Block {
            actions: vec![],
            block_info: block_info,
        })
    }

    /**
     * Loads the next block with chainInterface after validating, updating all relevant state.
     * If block fails validation, resolveFork will be called, and will update state to last block unseen.
     */
    async fn next_block(&mut self) -> (Block, bool, bool) {
        let mut block_data: Option<Block> = None;
        let mut is_rollback = false;
        let mut is_new_block = false;

        // If we're on the head block, refresh current head block
        if self.current_block_number == self.head_block_number || self.head_block_number == 0 {
            self.head_block_number = await!(self.get_head_block_number())
        }

        // If currentBlockNumber is negative, it means we wrap to the end of the chain (most recent blocks)
        // This should only ever happen when we first start, so we check that there's no block history
        if self.current_block_number < 0 && self.block_history.len() == 0 {
            self.current_block_number = self.head_block_number + self.current_block_number;
            self.start_at_block = self.current_block_number + 1;
        }

        // If we're now behind one or more new blocks, process them
        if self.current_block_number < self.head_block_number {
            let unvalidated_block_data = await!(self.get_block(self.current_block_number + 1));

            let expected_hash = if let Some(data) = &self.current_block_data { data.block_info.block_hash.clone() } else { "Invalid 0".to_owned() };
            let actual_hash = if let Some(data) = &unvalidated_block_data {data.block_info.previous_block_hash.clone() } else { "Invalid 1".to_owned() };

            // Continue if the new block is on the same chain as our history, or if we've just started
            if expected_hash == actual_hash || self.block_history.len() == 0 {
                block_data = unvalidated_block_data; // Block is now validated
                if let Some(block) = &self.current_block_data {
                    self.block_history.push(block.clone()); // No longer current, belongs on history
                }
                // self.block_history.splice(0, self.block_history.len() - slef.max_history_length); // Trim history
                self.block_history = self.block_history[0..(self.block_history.len() - self.max_history_length)].into(); // Trim history
                self.current_block_data = block_data; // Replaced with the real current block
                is_new_block = true;
                if let Some(block) = &self.current_block_data {
                    self.current_block_number = block.block_info.block_number;
                }
            } else {
                // Since the new block did not match our history, we can assume our history is wrong
                // and need to roll back
                // console.info("!! FORK DETECTED !!")
                // console.info(`  MISMATCH:`)
                // console.info(`    ✓ NEW Block ${unvalidatedBlockData.blockInfo.blockNumber} previous: ${actualHash}`)
                // console.info(`    ✕ OLD Block ${self.current_block_number} id:       ${expectedHash}`)
                await!(self.resolve_fork());
                is_new_block = true;
                is_rollback = true; // Signal action handler that we must roll back
                // Reset for safety, as new fork could have less blocks than the previous fork
                self.head_block_number = await!(self.get_head_block_number());
            }
        }

        // Let handler know if this is the earliest block we'll send
        self.is_first_block = self.current_block_number == self.start_at_block;

        (self.current_block_data.clone().unwrap(), is_rollback, is_new_block)
    }

    /**
     * Move to the specified block.
     */
    async fn seek_to_block(&mut self, block_number: i64) -> () {
        // Clear current block data
        self.current_block_data = None;
        self.head_block_number = 0;

        if block_number < self.start_at_block {
            // throw Error("Cannot seek to block before configured startAtBlock.")
        }

        // If we're going back to the first block, we don't want to get the preceding block
        if block_number == 1 {
            self.block_history = vec![];
            self.current_block_number = 0;
            return ()
        }

        // Check if block exists in history
        let mut to_delete : i64 = -1;
        for i in (0..=(self.block_history.len() - 1)).rev() {
            if self.block_history[i].block_info.block_number == block_number {
                break
            } else {
                to_delete += 1;
            }
        }
        if to_delete >= 0 {
            self.block_history = self.block_history[0..to_delete as usize].into();
            self.current_block_data = self.block_history.pop();
        }

        // Load current block
        self.current_block_number = block_number - 1;
        if let None = self.current_block_data {
            self.current_block_data = await!(self.get_block(self.current_block_number));
        }
    }

    /**
     * Incrementally rolls back reader state one block at a time, comparing the blockHistory with
     * newly fetched blocks. Fork resolution is finished when either the current block's previous hash
     * matches the previous block's hash, or when history is exhausted.
     *
     * @return {Promise<void>}
     */
    async fn resolve_fork(&self) -> () {
    }

    /**
     * When history is exhausted in resolveFork(), this is run to handle the situation. If left unimplemented,
     * then only instantiate with `onlyIrreversible` set to true.
     */
    fn history_exhausted() -> () {
        // console.info("Fork resolution history has been exhausted!")
        // throw Error("Fork resolution history has been exhausted, and no history exhaustion handling has been implemented.")
    }
}
