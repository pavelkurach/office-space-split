use office_space_split::{rental_space::*, user::*, user_interface::*, *};

fn example_storage() -> ObjectStorage {
    let add_user_request = AddUserRequest {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        workspace_request: Some(WorkspaceRequest {
            nb_workstations: 10,
            budget: 1000,
        }),
    };

    let user = User::new(add_user_request).unwrap();

    let add_rental_space_request = AddRentalSpaceRequest {
        name: "Rental Space".to_string(),
        address: "123 Main St".to_string(),
        surface: 100,
        nb_workstations: 50,
        price_per_workstation: 400,
        parent_office_id: None,
        owner_id: "usr-123".to_string(),
    };

    let rental_space = RentalSpace::new(add_rental_space_request, user.id().clone()).unwrap();

    let mut storage = ObjectStorage::new();
    storage.add_user(user);
    storage.add_rental_space(rental_space);
    storage
}

fn main() {
    let mut storage = example_storage();
    let mut interface = Interface::new(&mut storage);
    loop {
        interface.inquire_command();
    }
}
