use std::fmt::Formatter;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::{Vector};
use serde::{Serialize, Deserialize};

near_sdk::setup_alloc!();

#[derive(Default, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Message {
    account_id: String,
    timestamp: u64,
    message: String
}

impl std::fmt::Display for Message {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "({}){}: {}", self.timestamp, self.account_id, self.message)
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VecMessages {
    records: Vector<Message>
}

impl Default for VecMessages {
    fn default() -> Self {
        Self {
            records: Vector::new(b"r")
        }
    }
}

#[near_bindgen]
impl VecMessages {
    pub fn insert_message(&mut self, message: String) {
        let account_id = env::signer_account_id();
        let message = Message {
            account_id: String::from(account_id),
            timestamp: env::block_timestamp(),
            message: String::from(message)
        };
        self.records.push(&message)
    }

    pub fn get_messages_count(&self) -> u64 {
        self.records.len()
    }

    pub fn get_all_messages(&self) -> Vec<Message> {
        /*
        for message in 0..self.get_messages_count() {
            println!(
                "{}: {}",
                self.records.get(message).unwrap().account_id,
                self.records.get(message).unwrap().message
            )
        }
         */
        let mut messages: Vec<Message> = Vec::new();
        for message in 0..self.get_messages_count() {
            messages.push(self.get_single_message(message));
        }
        messages
    }

    pub fn get_single_message(&self, index: u64) -> Message {
        self.records.get(index).unwrap_or(Message{
            account_id: "".to_string(),
            timestamp: 0,
            message: "".to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
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
    fn insert_message() {
        let ctx = get_context(vec![], false);
        testing_env!(ctx);

        let mut contract = VecMessages::default();
        contract.insert_message("Hello world".to_string());
        contract.insert_message("This is message 2".to_string());
        contract.insert_message("This is message 3".to_string());
        println!("{} total messages inserted in this test!", contract.get_messages_count())
    }


    /*
    #[test]
    fn get_all_messages() {
        let ctx = get_context(vec![], false);
        testing_env!(ctx);

        let mut contract = VecMessages::default();
        contract.insert_message("Hello world".to_string());
        contract.insert_message("This is message 2".to_string());
        contract.insert_message("This is message 3".to_string());

        println!("{}", contract.get_all_messages());
    }
    */
    #[test]
    fn get_single_message() {
        let ctx = get_context(vec![], false);
        testing_env!(ctx);

        let mut contract = VecMessages::default();
        contract.insert_message("Hello world".to_string());
        contract.insert_message("This is message 2".to_string());
        contract.insert_message("This is message 3".to_string());

        println!("{}", contract.get_single_message(2));
    }
}

