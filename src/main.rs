
pub struct NumberStorage {
    a_favourite_number: u64,
}

impl NumberStorage {
    pub fn new() -> Self {
        Self {
            a_favourite_number: 0,
        }
    }

    pub fn store_number(&mut self, favourite_number: u64) {
        self.a_favourite_number = favourite_number;
    }

    pub fn retrieve_number(&self) -> u64 {
        self.a_favourite_number
    }
}
fn main() {
    println!("Hello, world!");
}
