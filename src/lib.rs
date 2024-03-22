use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VoteMeta {
    account_id: AccountId,
    vote_value: i8,
    feedback: Option<String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Address {
    address: String,
    country: String,
    state_or_province: String,
    city: String,
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PlaceInput {
    name: String,
    address: Address,
    description: String,
    pictures: Vec<String>,
    place_type: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Place {
    id: u64,
    name: String,
    address: Address,
    description: String,
    place_type: String,
    avarage_votes: i8,
    votes_counter: i32,
    votes: Vec<VoteMeta>,
    pictures: Vec<String>,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    admins: UnorderedSet<AccountId>,
    places: Vector<Place>,
    last_id: u64,
}