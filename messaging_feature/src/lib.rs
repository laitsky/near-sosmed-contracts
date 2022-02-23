//use std::fmt::Formatter;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector};
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    Promise, PromiseResult,
};
//use serde::{Serialize, Deserialize};

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

const NO_DEPOSIT: Balance = 0;
const BASE_GAS: Gas = Gas(100_000_000_000_000);

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    AllMessages,
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

#[ext_contract(ext_user)]
trait User {
    fn is_user_exists(&self, address: AccountId) -> bool;
}

// Cross-contract methods callback
#[ext_contract(ext_self)]
pub trait MessageContract {
    fn send_messages_cb(&self);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MessagingDb {
    messages: Vector<String>,
}

#[near_bindgen]
impl MessagingDb {
    // Initialize the contract
    #[init]
    pub fn new() -> Self {
        Self {
            messages: Vector::new(StorageKeys::AllMessages),
        }
    }

    pub fn test_cross_contract_call(&self) -> Promise {
        ext_user::is_user_exists(
            env::signer_account_id(),
            "user.dao-sosmed.testnet".to_string().parse().unwrap(), // contract account id
            NO_DEPOSIT,                                             // yocto NEAR to attach
            BASE_GAS,                                               // gas to attach
        )
        .then(ext_self::send_messages_cb(
            env::current_account_id(), // this contract account id
            NO_DEPOSIT,                // yocto NEAR to attach to the callback
            BASE_GAS,                  // gas to attach to the callback
        ))
    }

    pub fn send_messages_cb(&self) -> String {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops".to_string(),
            PromiseResult::Successful(result) => {
                let is_exist = near_sdk::serde_json::from_slice::<bool>(&result).unwrap();
                if is_exist == true {
                    "Account exist".to_string()
                } else {
                    "Account does not exist".to_string()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string().parse().unwrap(),
            signer_account_id: "robert.testnet".to_string().parse().unwrap(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string().parse().unwrap(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            view_config: None,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    #[should_panic]
    fn check_user_exists() {
        todo!()
    }
}
