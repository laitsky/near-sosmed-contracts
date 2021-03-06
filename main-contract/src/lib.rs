mod post;
mod user;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    UserList,
    UserFollowers,
    AllPosts,
    PostLikes,
    PostComments,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // User fields
    user_list: LookupMap<AccountId, user::UserAccountDetail>,
    user_followers: Vector<user::UserFollowers>,
    // Post fields
    all_posts: Vector<post::PostDetail>,
    post_likes: Vector<post::PostLikes>,
    post_comments: Vector<post::PostComment>,
    post_counter: u64,
    comment_counter: u64,
}

#[near_bindgen]
impl Contract {
    // Contract initialization
    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "The contract is already initialized");
        Self {
            user_list: LookupMap::new(StorageKeys::UserList),
            user_followers: Vector::new(StorageKeys::UserFollowers),
            all_posts: Vector::new(StorageKeys::AllPosts),
            post_likes: Vector::new(StorageKeys::PostLikes),
            post_comments: Vector::new(StorageKeys::PostComments),
            post_counter: 0,
            comment_counter: 0,
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
    ) -> bool {
        let address: AccountId = env::signer_account_id();
        require!(
            !self.user_list.contains_key(&address),
            "Account already exist!"
        );
        self.user_list.insert(
            &address,
            &user::UserAccountDetail {
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

        true
    }

    // Find account details
    pub fn get_account_details(&self, address: AccountId) -> Option<user::UserAccountDetail> {
        require!(self.user_list.contains_key(&address), "Account not found");
        self.user_list.get(&address)
    }

    // Edit account details
    pub fn edit_account_details(
        &mut self,
        name: String,
        location: String,
        url: String,
        description: String,
    ) {
        let signer: AccountId = env::signer_account_id();
        let account_details = self.user_list.remove(&signer);
        match account_details {
            None => env::panic_str("Account does not exist!"),
            Some(mut acc) => {
                acc.name = name;
                acc.location = location;
                acc.url = url;
                acc.description = description;
                self.user_list.insert(&signer, &acc);
            }
        }
    }

    // Edit profile image
    pub fn edit_profile_image(&mut self, profile_image_url: String) {
        let signer: AccountId = env::signer_account_id();
        let account_details = self.user_list.remove(&signer);
        match account_details {
            None => env::panic_str("Account does not exist!"),
            Some(mut acc) => {
                acc.profile_image_url = profile_image_url;
                self.user_list.insert(&signer, &acc);
            }
        }
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

    // Follow and unfollow user
    pub fn follow_user(&mut self, address: AccountId) {
        let user_account_id = env::signer_account_id();
        let destination_account_id = address;
        let is_followed = self.is_user_followed(&user_account_id, &destination_account_id);

        match is_followed {
            None => {
                self.user_followers.push(&user::UserFollowers {
                    user_account_id,
                    follower_account_id: destination_account_id,
                });
            }
            Some(u) => {
                self.user_followers.swap_remove(u as u64);
            }
        }
    }

    // Get user following list
    pub fn get_user_following_list(&self, user_account_id: AccountId) -> Vec<user::UserFollowList> {
        require!(
            self.is_user_exists(user_account_id.clone()),
            "User does not exist!"
        );
        let mut follow_list: Vec<user::UserFollowList> = vec![];
        // self.user_followers
        //     .iter()
        //     .filter(|u| u.user_account_id == user_account_id)
        //     .map(|u| u.follower_account_id.to_string())
        //     .collect::<Vec<String>>()
        for (_, user) in self
            .user_followers
            .iter()
            .filter(|u| u.user_account_id == user_account_id)
            .enumerate()
        {
            let profile = self.get_account_details(user.follower_account_id.clone()).unwrap();
            follow_list.push(user::UserFollowList {
                profile_image_url: profile.profile_image_url,
                user_account_id: user.follower_account_id,
                is_followed: true
            });
        }
        follow_list
    }

    // Get user following count
    pub fn get_user_following_count(&self, user_account_id: AccountId) -> u64 {
        require!(
            self.is_user_exists(user_account_id.clone()),
            "User does not exist!"
        );
        self.user_followers
            .iter()
            .filter(|u| u.user_account_id == user_account_id)
            .count() as u64
    }

    // Get user followers list
    pub fn get_user_followers_list(&self, user_account_id: AccountId) -> Vec<user::UserFollowList> {
        require!(
            self.is_user_exists(user_account_id.clone()),
            "User does not exist!"
        );
        let mut follow_list: Vec<user::UserFollowList> = vec![];
        // self.user_followers
        //     .iter()
        //     .filter(|u| u.follower_account_id == user_account_id)
        //     .map(|u| u.user_account_id.to_string())
        //     .collect::<Vec<String>>()

        for (_, user) in self
            .user_followers
            .iter()
            .filter(|u| u.follower_account_id == user_account_id)
            .enumerate()
        {
            let profile = self.get_account_details(user.user_account_id.clone()).unwrap();
            let is_followed = self.is_user_followed(&user_account_id, &user.user_account_id);
            follow_list.push(user::UserFollowList {
                profile_image_url: profile.profile_image_url,
                user_account_id: user.user_account_id,
                is_followed: match is_followed {
                    None => false,
                    Some(_) => true
                }
            });
        }
        follow_list

    }

    // Get user followers count
    pub fn get_user_followers_count(&self, user_account_id: AccountId) -> u64 {
        require!(
            self.is_user_exists(user_account_id.clone()),
            "User does not exist!"
        );
        self.user_followers
            .iter()
            .filter(|u| u.follower_account_id == user_account_id)
            .count() as u64
    }

    // Create new post
    pub fn create_post(&mut self, content: String) {
        let user_address: AccountId = env::signer_account_id();
        self.all_posts.push(&post::PostDetail {
            post_id: self.post_counter + 1,
            user_address,
            content,
            created_at: env::block_timestamp(),
        });
        self.post_counter = self.post_counter + 1;
    }

    // Like and unlike a post by its post ID
    pub fn like_post(&mut self, post_id: u64) {
        let address = env::signer_account_id();
        let is_liked = self
            .post_likes
            .iter()
            .filter(|pl| pl.post_id == post_id)
            .find(|pl| pl.user_address == address);

        if is_liked.is_none() {
            self.post_likes.push(&post::PostLikes {
                post_id,
                user_address: address,
                created_at: env::block_timestamp(),
            })
        } else {
            let index = self
                .post_likes
                .iter()
                .position(|pl| pl.post_id == post_id && pl.user_address == address)
                .unwrap() as u64;
            self.post_likes.swap_remove(index);
        }
    }

    // Comment on a post
    pub fn comment_on_post(&mut self, post_id: u64, comment: String) {
        require!(comment.chars().count() > 0, "Comment cannot be empty!");

        let address = env::signer_account_id();

        self.post_comments.push(&post::PostComment {
            comment_id: self.comment_counter + 1,
            post_id,
            user_address: address,
            comment,
            created_at: env::block_timestamp(),
        });

        self.comment_counter = self.comment_counter + 1;
    }

    // Retrieve post comments details
    pub fn get_post_comment_details(&self, post_id: u64) -> Vec<post::PostCommentDetailsOutput> {
        let mut post_comments: Vec<post::PostCommentDetailsOutput> = vec![];
        for (_, post_comment) in self
            .post_comments
            .iter()
            .filter(|pc| pc.post_id == post_id)
            .enumerate()
        {
            let profile_image_url = self
                .get_account_details(post_comment.user_address.clone())
                .unwrap()
                .profile_image_url;
            post_comments.push(post::PostCommentDetailsOutput {
                comment_id: post_comment.comment_id,
                user_address: post_comment.user_address,
                profile_image_url,
                comment: post_comment.comment,
                created_at: post_comment.created_at,
            })
        }
        post_comments
    }

    // Retrieve post likes details
    pub fn get_post_likes_details(&self, post_id: u64) -> Vec<post::PostLikeDetailsOutput> {
        let mut post_likes: Vec<post::PostLikeDetailsOutput> = vec![];
        for (_, post_like) in self
            .post_likes
            .iter()
            .filter(|pl| pl.post_id == post_id)
            .enumerate()
        {
            let profile_image_url = self
                .get_account_details(post_like.user_address.clone())
                .unwrap()
                .profile_image_url;
            post_likes.push(post::PostLikeDetailsOutput {
                    user_address: post_like.user_address,
                profile_image_url,
            })
        }
        post_likes
    }

    // Get poster address based on post ID
    pub fn get_poster_address(&self, post_id: u64) -> AccountId {
        self.all_posts
            .iter()
            .find(|p| p.post_id == post_id)
            .unwrap()
            .user_address
    }

    // Retrieve all available posts
    pub fn get_all_posts(&self, account_id: Option<AccountId>) -> Vec<post::PostOutputFormat> {
        let mut posts: Vec<post::PostOutputFormat> = vec![];
        for (_, post) in (self.all_posts.iter().enumerate()).rev() {
            let profile = self
                .get_account_details(self.get_poster_address(post.post_id))
                .unwrap();
            let like_count = self
                .post_likes
                .iter()
                .filter(|p| p.post_id == post.post_id)
                .count() as u64;
            let comment_count = self
                .post_comments
                .iter()
                .filter(|p| p.post_id == post.post_id)
                .count() as u64;
            let mut is_liked = false;

            if account_id != None {
                is_liked = self
                    .post_likes
                    .iter()
                    .filter(|p| p.post_id == post.post_id)
                    .any(|p| p.user_address == account_id.clone().unwrap());
            }

            posts.push(post::PostOutputFormat {
                name: profile.name,
                profile_image_url: profile.profile_image_url,
                post,
                like_count,
                comment_count,
                like_details: None,
                comment_details: None,
                is_liked: if account_id != None {
                    Some(is_liked)
                } else {
                    None
                },
            })
        }
        posts
    }

    // Retrieve single post detail
    pub fn get_single_post(
        &self,
        post_id: u64,
        account_id: Option<AccountId>,
    ) -> post::PostOutputFormat {
        require!(
            self.all_posts
                .iter()
                .find(|p| p.post_id == post_id)
                .is_some(),
            "Post does not exist!"
        );
        let profile = self
            .get_account_details(self.get_poster_address(post_id))
            .unwrap();
        let post = self
            .all_posts
            .iter()
            .find(|p| p.post_id == post_id)
            .unwrap();
        let like_details = self
            .post_likes
            .iter()
            .filter(|p| p.post_id == post_id)
            .collect::<Vec<post::PostLikes>>();
        let comment_details = self
            .post_comments
            .iter()
            .filter(|p| p.post_id == post_id)
            .collect::<Vec<post::PostComment>>();
        let mut is_liked = false;

        if account_id != None {
            is_liked = self
                .post_likes
                .iter()
                .filter(|p| p.post_id == post.post_id)
                .any(|p| p.user_address == account_id.clone().unwrap());
        }

        post::PostOutputFormat {
            name: profile.name,
            profile_image_url: profile.profile_image_url,
            post,
            like_count: like_details.iter().count() as u64,
            comment_count: comment_details.iter().count() as u64,
            like_details: Some(like_details),
            comment_details: Some(comment_details),
            is_liked: if account_id != None {
                Some(is_liked)
            } else {
                None
            },
        }
    }

    // Get specific user posts
    pub fn get_user_posts(
        &self,
        account_id: AccountId,
        perspective: Option<AccountId>,
    ) -> Vec<post::PostOutputFormat> {
        let mut posts: Vec<post::PostOutputFormat> = vec![];
        for (_, post) in self
            .all_posts
            .iter()
            .filter(|p| p.user_address == account_id.clone())
            .enumerate()
        {
            let profile = self
                .get_account_details(self.get_poster_address(post.post_id))
                .unwrap();
            let like_count = self
                .post_likes
                .iter()
                .filter(|p| p.post_id == post.post_id)
                .count() as u64;
            let comment_count = self
                .post_comments
                .iter()
                .filter(|p| p.post_id == post.post_id)
                .count() as u64;
            let is_liked;
            match perspective.clone() {
                None => {
                    is_liked = self
                        .post_likes
                        .iter()
                        .filter(|p| p.post_id == post.post_id)
                        .any(|p| p.user_address == account_id.clone());
                }
                Some(account) => {
                    is_liked = self
                        .post_likes
                        .iter()
                        .filter(|p| p.post_id == post.post_id)
                        .any(|p| p.user_address == account);
                }
            }

            posts.push(post::PostOutputFormat {
                name: profile.name,
                profile_image_url: profile.profile_image_url,
                post,
                like_count,
                comment_count,
                like_details: None,
                comment_details: None,
                is_liked: Some(is_liked),
            })
        }
        posts
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
            view_config: None,
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

    #[test]
    fn test_change_profile_image() {
        let ctx = get_context(vec![]);
        testing_env!(ctx);

        let mut contract = Contract::new();
        contract.create_account(
            Some("Mond".into()),
            Some("changethis".into()),
            Some("".into()),
            Some("".into()),
            Some("".into()),
        );

        println!(
            "prev image url: {:?}",
            contract
                .get_account_details("robert.testnet".to_string().parse().unwrap())
                .unwrap()
                .profile_image_url
        );

        contract.edit_profile_image("changed!".into());
        println!(
            "after image url: {:?}",
            contract.get_account_details("robert.testnet".to_string().parse().unwrap())
        );
    }
}
