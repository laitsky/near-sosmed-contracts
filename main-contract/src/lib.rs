mod users;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{near_bindgen, BorshStorageKey, require, AccountId, env, PanicOnDefault};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    UserList,
    UserFollowers,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Users struct
    user_list: LookupMap<AccountId, users::UserAccountDetail>,
    user_followers: Vector<users::UserFollowers>,
}

#[near_bindgen]
impl Contract {
    // Contract initialization
    #[init]
    pub fn new() -> Self {
        Self {
            user_list: LookupMap::new(StorageKeys::UserList),
            user_followers: Vector::new(StorageKeys::UserFollowers),
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
            &users::UserAccountDetail {
                address: address.to_string(),
                name: name.unwrap_or("".into()),
                profile_image_url: profile_image_url.unwrap_or("".into()),
                location: location.unwrap_or("".into()),
                url: url.unwrap_or("".into()),
                description: description.unwrap_or("".into()),
                created_at: env::block_timestamp(),
                followers_count: 0,
                following_count: 0,
            },
        );
    }

    // Find account details
    pub fn get_account_details(&self, address: AccountId) -> Option<users::UserAccountDetail> {
        self.user_list.get(&address)
    }

    // Check if destination account id is followed by a user
    pub fn is_user_followed(
        &self,
        user_account_id: &AccountId,
        destination_account_id: &AccountId,
    ) -> Option<usize> {
        self.user_followers.iter().position(|u| {
            u.user_account_id == *user_account_id
                && u.follower_account_id == *destination_account_id
        })
    }

    // Follow new user
    pub fn follow_user(&mut self, address: AccountId) {
        let user_account_id = env::signer_account_id();
        let destination_account_id = address;

        if self
            .is_user_followed(&user_account_id, &destination_account_id)
            .is_none()
        {
            self.user_followers.push(&users::UserFollowers {
                user_account_id,
                follower_account_id: destination_account_id,
            });
        } else {
            env::panic_str("You have already followed this account!");
        }
    }

    // Unfollow a user
    pub fn unfollow_user(&mut self, address: AccountId) {
        let user_account_id = env::signer_account_id();
        let destination_account_id = address;
        let is_user_followed = self.is_user_followed(&user_account_id, &destination_account_id);

        if is_user_followed.is_some() {
            self.user_followers
                .swap_remove(is_user_followed.unwrap() as u64);
        } else {
            env::panic_str("You have not followed this account!");
        }
    }

    // Get user following list
    pub fn get_user_following_list(&self, user_account_id: AccountId) -> Vec<String> {
        require!(self.is_user_exists(user_account_id.clone()), "User does not exist!");
        self.user_followers
            .iter()
            .filter(|u| u.user_account_id == user_account_id)
            .map(|u| u.follower_account_id.to_string())
            .collect::<Vec<String>>()
    }

    // Get user following count
    pub fn get_user_following_count(&self, user_account_id: AccountId) -> u64 {
        require!(self.is_user_exists(user_account_id.clone()), "User does not exist!");
        self.user_followers
            .iter()
            .filter(|u| u.user_account_id == user_account_id)
            .count() as u64
    }

    // Get user followers list
    pub fn get_user_followers_list(&self, user_account_id: AccountId) -> Vec<String> {
        require!(self.is_user_exists(user_account_id.clone()), "User does not exist!");
        self.user_followers
            .iter()
            .filter(|u| u.follower_account_id == user_account_id)
            .map(|u| u.user_account_id.to_string())
            .collect::<Vec<String>>()
    }

    // Get user followers count
    pub fn get_user_followers_count(&self, user_account_id: AccountId) -> u64 {
        require!(self.is_user_exists(user_account_id.clone()), "User does not exist!");
        self.user_followers
            .iter()
            .filter(|u| u.follower_account_id == user_account_id)
            .count() as u64
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

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
            output_data_receivers: vec![],
            epoch_height: 19,
            view_config: None
        }
    }

    #[test]
    #[should_panic]
    fn test_create_account() {
        let ctx = get_context(vec![]);
        testing_env!(ctx);

        let mut contract = Contract::new();
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
