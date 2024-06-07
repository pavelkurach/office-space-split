use crate::{
    rental_space::{RentalSpace, Split},
    user::User,
};

use std::collections::HashMap;

pub struct ObjectStorage {
    users: HashMap<String, User>,
    rental_spaces: HashMap<String, RentalSpace>,
}

impl ObjectStorage {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            rental_spaces: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert((&user).id_value().to_owned(), user);
    }

    pub fn add_rental_space(&mut self, rental_space: RentalSpace) {
        self.rental_spaces
            .insert((&rental_space).id_value().to_owned(), rental_space);
    }

    pub fn get_user(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    pub fn get_rental_space(&self, id: &str) -> Option<&RentalSpace> {
        self.rental_spaces.get(id)
    }

    pub fn users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    pub fn rental_spaces(&self) -> Vec<&RentalSpace> {
        self.rental_spaces.values().collect()
    }
}
