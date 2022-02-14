use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    UserList,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UserAccountDetail {
    address: String,
    name: String,
    profile_image_url: String,
    location: String,
    url: String,
    description: String,
    created_at: u64,
    followers_count: u32,
    following_count: u32,
    last_activity: u64,
}

impl UserAccountDetail {}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Users {
    user_list: LookupMap<AccountId, UserAccountDetail>,
}

#[near_bindgen]
impl Users {
    // Initialize the contract
    #[init]
    pub fn new() -> Self {
        Self {
            user_list: LookupMap::new(StorageKeys::UserList),
        }
    }

    // Check if user exist
    pub fn is_user_exists(&self, address: AccountId) -> bool {
        self.user_list.contains_key(&address)
    }

    // Create an account
    pub fn create_account(
        &mut self,
        name: Option<String>,
        profile_image_url: Option<String>,
        location: Option<String>,
        url: Option<String>,
        description: Option<String>,
    ) {
        let address: AccountId = env::signer_account_id();
        require!(
            !self.user_list.contains_key(&address),
            "Account already exist!"
        );

        self.user_list.insert(
            &address,
            &UserAccountDetail {
                address: address.to_string(),
                name: name.unwrap_or("".into()),
                profile_image_url: profile_image_url.unwrap_or("".into()),
                location: location.unwrap_or("".into()),
                url: url.unwrap_or("".into()),
                description: description.unwrap_or("".into()),
                created_at: env::block_timestamp(),
                followers_count: 0,
                following_count: 0,
                last_activity: env::block_timestamp(),
            },
        );
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use near_sdk::{testing_env, VMContext};

    use super::*;

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
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // TESTS HERE
    #[test]
    #[should_panic]
    fn test_create_account() {
        let ctx = get_context(vec![]);
        testing_env!(ctx);

        let mut contract = Users::new();
        println!(
            "[ACCOUNT NOT CREATED]: check_user_exist value is [{}]",
            contract.is_user_exists("robert.testnet".to_string().parse().unwrap())
        );

        contract.create_account(
            Some("Robert".into()),
            Some("".into()),
            Some("".into()),
            Some("".into()),
            Some("".into()),
        );
        println!(
            "[ACCOUNT CREATED]: check_user_exist value is [{}]",
            contract.is_user_exists("robert.testnet".to_string().parse().unwrap())
        );

        contract.create_account(
            Some("Robert2".into()),
            Some("".into()),
            Some("".into()),
            Some("".into()),
            Some("".into()),
        );
    }
}
