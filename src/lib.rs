/*
 * This is a proof of timestamp rust smart contract with two functions:
 *
 * 1. stamp: accepts a file hash, gets the current block timestamp, concatenates both variables and records their hash into the blockchain
 * 2. get_stamp: accepts file hash and returns the timestamp saved for it, defaulting to
 *    TimestampedFile { timestamp: 0, time_stamped_file_hash: [] }
 *
 * Learn more about proof of timestamp:
 * https://en.wikipedia.org/wiki/Trusted_timestamping
 * https://www.jamieweb.net/blog/proof-of-timestamp/
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::{env, near_bindgen};
use std::collections::HashMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct ProofOfTimestamp {
    records: HashMap<String, TimestampedFile>,
}

#[derive(Default, Clone,Debug,PartialEq, BorshDeserialize, BorshSerialize)]
pub struct TimestampedFile {
    timestamp: u64,
    time_stamped_file_hash: Vec<u8>
}

#[near_bindgen]
impl ProofOfTimestamp {

    pub fn stamp(&mut self, file_hash: String) {
        let block_timestamp = env::block_timestamp();
        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("Stamping file '{}' at '{}'", file_hash, block_timestamp,).as_bytes());
        let timestamped_file_hash = env::keccak256(format!("{}{}",file_hash,block_timestamp.to_string()).as_bytes());
        self.records.insert(file_hash,TimestampedFile{timestamp:block_timestamp,time_stamped_file_hash:timestamped_file_hash});
    }

    pub fn get_stamp(&self, file_hash: String) -> TimestampedFile {
        match self.records.get(&file_hash) {
            Some(stamp) => stamp.clone(),
            None => TimestampedFile::default(),
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool, block_timestamp:u64) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn stamp_then_get() {
        let block_timestamp = 100;
        let file_hash = "sample file hash".to_string();
        let context = get_context(vec![], false,block_timestamp);
        testing_env!(context);
        let mut contract = ProofOfTimestamp::default();
        contract.stamp(file_hash.clone());
        let timestamped_file_hash = env::keccak256(format!("{}{}",file_hash,block_timestamp.to_string()).as_bytes());
        let expected_result = TimestampedFile{timestamp:block_timestamp,time_stamped_file_hash:timestamped_file_hash};
        assert_eq!(
            expected_result,
            contract.get_stamp(file_hash)
        );
    }

    #[test]
    fn get_default_stamp() {
        let block_timestamp = 100;
        let context = get_context(vec![], true,block_timestamp);
        testing_env!(context);
        let contract = ProofOfTimestamp::default();
        assert_eq!(
            TimestampedFile::default(),
            contract.get_stamp("howdy".to_string())
        );
    }
}
