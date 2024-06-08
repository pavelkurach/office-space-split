use office_space_split::{object_storage::*, rental_space::*, user::*, user_interface::*};

fn example_storage() -> ObjectStorage {
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

    for surface in [300] {
        for nb_workstations in [50, 80, 100, 120, 160] {
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

    for nb_workstations in [10, 20, 40, 80, 100, 150, 200] {
        let add_user_request = AddUserRequest {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            workspace_request: Some(WorkspaceRequest {
                nb_workstations,
                budget: 1_000_000_000,
            }),
        };

        let user = User::new(add_user_request.clone()).unwrap();
        storage.add_user(user);

        let user = User::new(add_user_request).unwrap();
        storage.add_user(user);
    }

    storage
}

fn main() {
    let mut storage = example_storage();
    let mut interface = Interface::new(&mut storage);
    loop {
        interface.inquire_command();
    }
}
