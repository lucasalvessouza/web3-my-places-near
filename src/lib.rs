use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, Vector};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::env;
use near_sdk::env::log_str;
use near_sdk::{near_bindgen, AccountId};

pub mod internal;
pub mod utils;
pub use crate::utils::*;

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

// Default state to use if no initialize method is called.
// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            // b"v" é um prefixador que vai ser usado como chave no store do contrato
            owner: "wendersonpires.testnet".parse().unwrap(),
            admins: UnorderedSet::new(b"s"),
            places: Vector::new(b"v"),
            last_id: 0,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner: AccountId, admins: Option<Vec<AccountId>>) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        Self {
            owner,
            admins: account_vec_to_set(
                if admins.is_some() {
                    admins.unwrap()
                } else {
                    vec![]
                },
                b"s",
            ),
            places: Vector::new(b"v"),
            last_id: 0,
        }
    }

    pub fn add_place(&mut self, place: PlaceInput) {
        let place_name = place.name.clone();
        log_str(&format!("Adding new place: {place_name}"));

        let new_place = Place {
            id: self.last_id,
            name: place.name,
            address: place.address,
            description: place.description,
            place_type: place.place_type,
            avarage_votes: 0,
            votes_counter: 0,
            votes: vec![],
            pictures: place.pictures,
        };

        self.places.push(&new_place);
        self.last_id += 1;
    }

    pub fn get_places(&self) -> Vec<Place> {
        return self.places.to_vec();
    }

    pub fn get_place_by_id(&self, place_id: u64) -> Option<Place> {
        if let Some(index) = self.places.iter().position(|place | place.id == place_id) {
            self.places.get(index as u64)
        } else {
            None
        }
    }
}


// Tests in a separated file (see more here -> http://xion.io/post/code/rust-unit-test-placement.html)
#[cfg(test)]
#[path = "./tests.rs"]
mod tests;