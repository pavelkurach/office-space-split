use office_space_split::{rental_space::*, split::*, user::*};

fn main() {
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

    match RentalSpace::new(add_rental_space_request, user.id().clone()) {
        Ok(rental_space) => println!("{:#?}", rental_space),
        Err(e) => println!("{:#?}", e),
    }
}
