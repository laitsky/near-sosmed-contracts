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
pub struct PostOutputFormat {
    pub name: String,
    pub profile_image_url: String,
    pub post: PostDetail,
    pub like_count: u64,
    pub comment_count: u64,
    pub like_details: Option<Vec<PostLikes>>,
    pub comment_details: Option<Vec<PostComment>>,
    pub is_liked: Option<bool>
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostDetail {
    pub post_id: u64,
    pub user_address: AccountId,
    pub content: String,
    pub created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostLikes {
    pub post_id: u64,
    pub user_address: AccountId,
    pub created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostLikeDetailsOutput {
    pub user_address: AccountId,
    pub profile_image_url: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostComment {
    pub comment_id: u64,
    pub post_id: u64,
    pub user_address: AccountId,
    pub comment: String,
    pub created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostCommentDetailsOutput {
    pub comment_id: u64,
    pub user_address: AccountId,
    pub profile_image_url: String,
    pub comment: String,
    pub created_at: u64
}