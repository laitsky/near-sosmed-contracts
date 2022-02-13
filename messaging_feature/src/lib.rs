use std::fmt::Formatter;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, log, near_bindgen, BorshStorageKey};
use near_sdk::collections::{LookupMap, Vector};
//use serde::{Serialize, Deserialize};

near_sdk::setup_alloc!();

    /*
#[derive(Default, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Message {
    account_id: String,
    timestamp: u64,
    message: String
}

impl std::fmt::Display for Message {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        //write!(fmt, "({}){}: {}", self.timestamp, self.account_id, self.message)
        todo!()
    }
}
    */

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    UserList,
    AllMessages
}
#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    name: String,
    friend_list: Option<Vector<Friend>>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Friend {
    address: AccountId,
    name: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Message {
    sender: AccountId,
    timestamp: u64,
    message: String,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MessagingDb {
    // Collection of users registered on the application
    user_list: LookupMap<AccountId, User>,
    all_messages: LookupMap<String, Vector<Message>>
}

#[near_bindgen]
impl MessagingDb {
    // Initialize the contract
    #[init]
    pub fn new() -> Self {
        Self {
            user_list: LookupMap::new(StorageKeys::UserList),
            all_messages: LookupMap::new(StorageKeys::AllMessages)
        }
    }

    // Returns true if user exist
    pub fn is_user_exists(&self, address: AccountId) -> bool {
        self.user_list.contains_key(&address)
    }

    // Create an account
    pub fn create_account(&mut self, name: String) {
        let new_address: AccountId = env::signer_account_id();
        assert!(!self.user_list.contains_key(&new_address), "Account already exist!");

        // If not exist, insert a new address
        self.user_list.insert(
            &new_address,
            &User {
                    name,
                    friend_list: None
            }
        );

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
    #[should_panic]
    fn check_user_exists() {
        let ctx = get_context(vec![], false);
        testing_env!(ctx);

        let mut contract = MessagingDb::new();
        println!("[ACCOUNT NOT CREATED]: check_user_exist value is [{}]",
                 contract.is_user_exists("robert.testnet".to_string() as AccountId));

        contract.create_account("Robert".to_string());
        println!("[ACCOUNT CREATED]: check_user_exist value is [{}]",
                 contract.is_user_exists("robert.testnet".to_string() as AccountId));

        contract.create_account("Robert".to_string());
    }

}

