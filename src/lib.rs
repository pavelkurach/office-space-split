pub mod rental_space;
pub mod split;
pub mod user;
pub mod user_interface;

use {
    chrono::{DateTime, Utc},
    uuid::Uuid,
};

pub trait PrefixedUuid {
    const PREFIX: &'static str;

    fn generate() -> String {
        format!("{}-{}", Self::PREFIX, Uuid::new_v4())
    }
}

#[derive(Debug)]
pub struct BaseFields<Id: PrefixedUuid> {
    id: Id,
    created_at: DateTime<Utc>,
}

pub struct ObjectStorage {
    users: Vec<user::User>,
    rental_spaces: Vec<rental_space::RentalSpace>,
    splits: Vec<split::Split>,
}

impl<Id: PrefixedUuid> BaseFields<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            created_at: Utc::now(),
        }
    }
}

impl ObjectStorage {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            rental_spaces: Vec::new(),
            splits: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: user::User) {
        self.users.push(user);
    }

    pub fn add_rental_space(&mut self, rental_space: rental_space::RentalSpace) {
        self.rental_spaces.push(rental_space);
    }
}
