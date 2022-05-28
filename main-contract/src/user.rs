use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserAccountDetail {
    pub address: String,
    pub name: String,
    pub profile_image_url: String,
    pub location: String,
    pub url: String,
    pub description: String,
    pub created_at: u64,
    pub followers_count: u32,
    pub following_count: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserFollowers {
    pub user_account_id: AccountId,
    pub follower_account_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserFollowList {
    pub profile_image_url: String,
    pub user_account_id: AccountId,
    pub is_followed: bool
}