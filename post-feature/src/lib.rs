use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, require, AccountId, Balance, BorshStorageKey, Gas,
    PanicOnDefault, Promise, PromiseResult,
};

const NO_DEPOSIT: Balance = 0;
const BASE_GAS: Gas = Gas(5_000_000_000_000);

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostOutputFormat {
    name: String,
    profile_image_url: String,
    post: PostDetail,
    like_count: u64,
    comment_count: u64,
    like_details: Option<Vec<PostLikes>>,
    comment_details: Option<Vec<PostComment>>
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    AllPosts,
    PostLikes,
    PostComments,
}

#[ext_contract(ext_user)]
pub trait ExtUser {
    fn is_user_exists(&self, address: AccountId) -> bool;
    fn get_account_details(&self, address: AccountId) -> Option<UserAccountDetail>;
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn get_single_post_cb(&self, post_id: u64);
    fn create_post_cb(&self, content: String);
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostDetail {
    post_id: u64,
    user_address: AccountId,
    content: String,
    created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostLikes {
    post_id: u64,
    user_address: AccountId,
    created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostComment {
    comment_id: u64,
    post_id: u64,
    user_address: AccountId,
    comment: String,
    created_at: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Posts {
    all_posts: Vector<PostDetail>,
    post_likes: Vector<PostLikes>,
    post_comments: Vector<PostComment>,
    post_counter: u64,
    comment_counter: u64
}

#[near_bindgen]
impl Posts {
    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "The contract is already initialized");
        Self {
            all_posts: Vector::new(StorageKeys::AllPosts),
            post_likes: Vector::new(StorageKeys::PostLikes),
            post_comments: Vector::new(StorageKeys::PostComments),
            post_counter: 0,
            comment_counter: 0
        }
    }

    pub fn create_post(&mut self, content: String) {
        let user_address: AccountId = env::signer_account_id();
        self.all_posts.push(&PostDetail {
            post_id: self.post_counter + 1,
            user_address,
            content,
            created_at: env::block_timestamp(),
        });
        self.post_counter = self.post_counter + 1;
    }

    pub fn like_post(&mut self, post_id: u64) {
        let address = env::signer_account_id();
        let is_liked = self
            .post_likes
            .iter()
            .filter(|pl| pl.post_id == post_id)
            .find(|pl| pl.user_address == address);

        if is_liked.is_none() {
            self.post_likes.push(&PostLikes {
                post_id,
                user_address: address,
                created_at: env::block_timestamp(),
            })
        } else {
            let index = self.post_likes
                .iter()
                .position(|pl| pl.post_id == post_id && pl.user_address == address)
                .unwrap() as u64;
            self.post_likes.swap_remove(index);
        }
    }

    pub fn comment_on_post(&mut self, post_id: u64, comment: String) {
        require!(comment.chars().count() > 0, "Comment cannot be empty!");

        let address = env::signer_account_id();

        self.post_comments.push(&PostComment {
            comment_id: self.comment_counter + 1,
            post_id,
            user_address: address,
            comment,
            created_at: env::block_timestamp(),
        });

        self.comment_counter = self.comment_counter + 1;
    }

    pub fn get_post_likes_details(&self, post_id: u64) -> Vec<String> {
        self.post_likes
            .iter()
            .filter(|pl| pl.post_id == post_id)
            .map(|pl| pl.user_address.to_string())
            .collect::<Vec<String>>()
    }

    pub fn get_all_posts(&self) -> Vec<PostDetail> {
        self.all_posts.to_vec()
    }

    pub fn get_single_post(&self, post_id: u64) {
        ext_user::get_account_details(
            self.get_poster_address(post_id),
            "user.dao-sosmed.testnet".to_string().parse().unwrap(),
            NO_DEPOSIT,
            BASE_GAS,
        )
        .then(ext_self::get_single_post_cb(
            post_id,
            env::current_account_id(),
            NO_DEPOSIT,
            BASE_GAS,
        ));
    }

    pub fn get_single_post_cb(&self, post_id: u64) -> PostOutputFormat {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => unreachable!(),
            PromiseResult::Successful(result) => {
                let profile =
                    near_sdk::serde_json::from_slice::<UserAccountDetail>(&result).unwrap();
                let post = self.all_posts.iter().find(|p| p.post_id == post_id);
                let like_details = self
                    .post_likes
                    .iter()
                    .filter(|p| p.post_id == post_id)
                    .collect::<Vec<PostLikes>>();
                let comment_details = self
                    .post_comments
                    .iter()
                    .filter(|p| p.post_id == post_id)
                    .collect::<Vec<PostComment>>();

                PostOutputFormat {
                    name: profile.name,
                    profile_image_url: profile.profile_image_url,
                    post: post.unwrap(),
                    like_count: like_details.iter().count() as u64,
                    comment_count: comment_details.iter().count() as u64,
                    like_details: Some(like_details),
                    comment_details: Some(comment_details),
                }
            }
        }
    }

    pub fn get_poster_address(&self, post_id: u64) -> AccountId {
        self.all_posts
            .iter()
            .find(|p| p.post_id == post_id)
            .unwrap()
            .user_address
    }

    pub fn xcc_create_post(&self, content: String) -> Promise {
        ext_user::get_account_details(
            env::signer_account_id(),
            "user.dao-sosmed.testnet".to_string().parse().unwrap(),
            NO_DEPOSIT,
            BASE_GAS,
        )
        .then(ext_self::create_post_cb(
            content,
            env::current_account_id(),
            NO_DEPOSIT,
            BASE_GAS,
        ))
    }

    pub fn create_post_cb(&self, content: String) -> String {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "Unable to connect to server, please try again".to_string(),
            PromiseResult::Successful(result) => {
                let profile =
                    near_sdk::serde_json::from_slice::<UserAccountDetail>(&result).unwrap();
                format!("add = {}, content = {}", profile.name.to_string(), content)
            }
        }
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
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_create_post_and_get_addrid() {
        let ctx = get_context(vec![]);
        testing_env!(ctx);

        let mut contract = Posts::new();

        contract.create_post("Hello world".into());
        contract.create_post("THis is tiring".into());
        contract.create_post("Good morning everyone".into());

        println!("{:?}", contract.get_all_posts());
    }
}
