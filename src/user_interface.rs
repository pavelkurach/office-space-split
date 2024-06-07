use crate::{
    object_storage::ObjectStorage,
    rental_space::{AddRentalSpaceRequest, RentalSpace},
    user::{AddUserRequest, User},
};

use inquire::{
    error::InquireError,
    ui::{Color, RenderConfig, Styled},
    Editor, Select,
};

pub struct Interface<'a> {
    storage: &'a mut ObjectStorage,
}

impl<'a> Interface<'a> {
    pub fn new(storage: &'a mut ObjectStorage) -> Self {
        Self { storage }
    }

    pub fn inquire_command(&mut self) {
        let commands: Vec<&str> = vec!["add", "print", "match", "exit"];

        let ans: Result<&str, InquireError> = Select::new("Select command", commands).prompt();

        match ans {
            Ok(command) => match command {
                "add" => self.add_object(),
                "print" => self.print_objects(),
                "match" => self.match_objects(),
                "exit" => std::process::exit(0),
                _ => println!("Invalid command"),
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }

    pub fn add_object(&mut self) {
        let categories: Vec<&str> = vec!["user", "rental_space"];

        let ans: Result<&str, InquireError> = Select::new("Select category", categories).prompt();

        match ans {
            Ok("user") => {
                let user_json = Self::get_object_json("user");
                match user_json.and_then(|user| self.add_user(&user)) {
                    Ok(_) => println!("User added successfully!"),
                    Err(e) => println!("Error adding user: {}", e),
                }
            }
            Ok("rental_space") => {
                let rental_space_json = Self::get_object_json("rental_space");
                match rental_space_json
                    .and_then(|rental_space| self.add_rental_space(&rental_space))
                {
                    Ok(_) => println!("Rental space added successfully!"),
                    Err(e) => println!("Error adding rental space: {}", e),
                }
            }
            Ok(_) => println!("Invalid category"),
            Err(_) => println!("There was an error, please try again"),
        }
    }

    pub fn print_objects(&self) {
        let categories: Vec<&str> = vec!["users", "rental_spaces", "splits", "all"];

        let ans: Result<&str, InquireError> = Select::new("Select category", categories).prompt();

        match ans {
            Ok("users") => {
                self.print_users();
            }
            Ok("rental_spaces") => {
                self.print_rental_spaces();
            }
            Ok("splits") => {
                self.print_splits();
            }
            Ok("all") => {
                println!("Users:\n");
                self.print_users();
                println!("Rental Spaces:\n");
                self.print_rental_spaces();
                println!("Splits:\n");
                self.print_splits();
            }
            Ok(_) => println!("Invalid category"),
            Err(_) => println!("There was an error, please try again"),
        }
    }

    pub fn match_objects(&mut self) {}

    fn add_user(&mut self, user_json: &str) -> anyhow::Result<()> {
        let request: AddUserRequest = serde_json::from_str(user_json)?;
        let user = User::new(request)?;
        self.storage.add_user(user);
        Ok(())
    }

    fn add_rental_space(&mut self, rental_space_json: &str) -> anyhow::Result<()> {
        let request: AddRentalSpaceRequest = serde_json::from_str(rental_space_json)?;
        let user = self
            .storage
            .get_user(&request.owner_id)
            .ok_or(anyhow::anyhow!(
                "User with id {} not found",
                request.owner_id
            ))?;
        let rental_space = RentalSpace::new(request, user.id().clone())?;
        self.storage.add_rental_space(rental_space);
        Ok(())
    }

    fn print_users(&self) {
        for user in &self.storage.users() {
            println!("{:#?}\n", user);
        }
    }

    fn print_rental_spaces(&self) {
        for rental_space in &self.storage.rental_spaces() {
            println!("{:#?}\n", rental_space);
        }
    }

    fn print_splits(&self) {
        for split in &self.storage.splits() {
            println!("{:#?}\n", split);
        }
    }

    fn get_object_json(object_name: &str) -> anyhow::Result<String> {
        Ok(Editor::new(format!("Enter {}! in JSON format: ", object_name).as_str()).prompt()?)
    }
}
