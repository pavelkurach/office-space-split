use crate::{
    rental_space::{AddRentalSpaceRequest, RentalSpace},
    user::{AddUserRequest, User, WorkspaceRequest},
};

use std::collections::HashMap;

#[derive(Default)]
pub struct ObjectStorage {
    users: HashMap<String, User>,
    rental_spaces: HashMap<String, RentalSpace>,
}

impl ObjectStorage {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id_value().to_owned(), user);
    }

    pub fn add_rental_space(&mut self, rental_space: RentalSpace) {
        self.rental_spaces
            .insert(rental_space.id_value().to_owned(), rental_space);
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

    pub fn merge(&mut self, other: ObjectStorage) {
        for (key, value) in other.users {
            self.users.insert(key, value);
        }

        for (key, value) in other.rental_spaces {
            self.rental_spaces.insert(key, value);
        }
    }
}

pub fn example_storage() -> ObjectStorage {
    let mut storage = ObjectStorage::new();

    let mut hosts = vec![];

    for _ in 0..10 {
        let add_user_request = AddUserRequest {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            workspace_request: None,
        };

        let user = User::new(add_user_request).unwrap();
        hosts.push(user.clone());
        storage.add_user(user);
    }

    for surface in [30000] {
        for nb_workstations in [160, 160, 160] {
            let add_rental_space_request = AddRentalSpaceRequest {
                name: "Rental Space".to_string(),
                address: "123 Main St".to_string(),
                surface,
                nb_workstations,
                price_per_workstation: 400,
                owner_id: "usr-123".to_string(),
            };

            use rand::seq::SliceRandom;

            let rental_space = RentalSpace::new(
                add_rental_space_request,
                hosts.choose(&mut rand::thread_rng()).unwrap().id().clone(),
            );
            if let Ok(rental_space) = rental_space {
                storage.add_rental_space(rental_space);
            }
        }
    }

    for nb_workstations in [10, 15, 20, 25, 30, 35, 40, 45, 50, 55] {
        let add_user_request = AddUserRequest {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            workspace_request: Some(WorkspaceRequest {
                nb_workstations,
                budget: 1_000_000_000,
            }),
        };

        let user = User::new(add_user_request).unwrap();
        storage.add_user(user);
    }

    storage
}
