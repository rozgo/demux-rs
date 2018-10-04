use chrono::prelude::*;
use exonum_crypto::*;
// use hyper::rt::{Future};

// use hyper;
// use hyper::rt::{self, Future, Stream};
// use hyper::Client;

use futures::{future, Future, Stream};

pub struct Action {
    kind: String,
    payload: serde_json::Value,
}

pub struct BlockInfo {
    block_hash: String,
    block_number: i64,
    previous_block_hash: String,
    timestamp: NaiveDateTime,
}

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
    max_history_length: i64,
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
    fn get_head_block_number(&self) -> impl Future<Item = i64, Error = FetchError> {
        future::ok(0)
    }

    /**
     * Loads a block with the given block number
     * @param {number} blockNumber - Number of the block to retrieve
     * @returns {Block}
     */
    fn get_block(&self, block_number: i64) -> impl Future<Item = Block, Error = FetchError> {
        let block_info = BlockInfo {
            block_hash: "000".to_owned(),
            block_number: block_number,
            previous_block_hash: "000".to_owned(),
            timestamp: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        };
        futures::future::ok(Block {
            actions: vec![],
            block_info: block_info,
        })
    }

    /**
     * Loads the next block with chainInterface after validating, updating all relevant state.
     * If block fails validation, resolveFork will be called, and will update state to last block unseen.
     */
    fn next_block(&mut self) -> impl Future<Item = (Block, bool, bool), Error = FetchError> {
        let block_data: Option<Block> = None;
        let is_rollback = false;
        let is_new_block = false;

        let p = if self.current_block_number == self.head_block_number || self.head_block_number == 0 {
            self.get_head_block_number().boxed()
        }
        else {
            Box::new(future::ok::<i64, FetchError>(0))
        };


        let block_info = BlockInfo {
            block_hash: "000".to_owned(),
            block_number: 0,
            previous_block_hash: "000".to_owned(),
            timestamp: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        };
        futures::future::ok((
            Block {
                actions: vec![],
                block_info: block_info,
            },
            true,
            true,
        ))
    }
}

enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}
