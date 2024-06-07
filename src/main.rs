use near_sdk::near_bindgen;
use near_sdk::borsh::{
    self, BorshDeserialize, BorshSerialize
};
use serde::{
    Deserialize, Serialize
};
use near_sdk::collections::{
    Vector, LookupMap
};

#[derive(BorshDeserialize, BorshSerialize)]
struct Person {
    favourite_number: u64,
    name : String,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NumberStorage {
    a_favourite_number: u64,
    list_of_people: Vector<Person>,
    name_to_favourite_number: LookupMap<String, u64>,
}

#[near_bindgen]
impl NumberStorage {
    #[init]
    pub fn new() -> Self {
        Self {
            a_favourite_number: 0,
            list_of_people: Vector::new(b"people".to_vec()),
            name_to_favourite_number: LookupMap::new(b"name_to_favourite_number".to_vec()),
        }
    }

    pub fn store_number(&mut self, favourite_number: u64) {
        self.a_favourite_number = favourite_number;
    }

    pub fn retrieve_number(&self) -> u64 {
        self.a_favourite_number
    }

    pub fn add_person(&mut self, name: String, favourite_number: u64) {
        let person = Person {
            name: name.clone(),
            favourite_number,
        };
        self.list_of_people.push(&person);
        self.name_to_favourite_number.insert(&name, &favourite_number);
    }
}
fn main() {
    println!("Hello, world!");
}
