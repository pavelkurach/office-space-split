use crate::ObjectStorage;

use inquire::{error::InquireError, Select};

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
                "add" => self.inquire_add(),
                "print" => self.print_objects(),
                "match" => self.match_objects(),
                "exit" => std::process::exit(0),
                _ => println!("Invalid command"),
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }

    pub fn inquire_add(&mut self) {
        let categories: Vec<&str> = vec!["user", "rental_space"];

        let ans: Result<&str, InquireError> = Select::new("Select category", categories).prompt();

        match ans {
            Ok("user") => {}
            Ok("rental_space") => {}
            Ok(_) => println!("Invalid category"),
            Err(_) => println!("There was an error, please try again"),
        }
    }

    pub fn print_objects(&self) {
        let categories: Vec<&str> = vec!["users", "rental_spaces", "all"];

        let ans: Result<&str, InquireError> = Select::new("Select category", categories).prompt();

        match ans {
            Ok("users") => {
                for user in &self.storage.users {
                    println!("{:#?}\n", user);
                }
            }
            Ok("rental_spaces") => {
                for rental_space in &self.storage.rental_spaces {
                    println!("{:#?}\n", rental_space);
                }
            }
            Ok("all") => {
                println!("Users:\n");
                for user in &self.storage.users {
                    println!("{:#?}\n", user);
                }
                println!("Rental Spaces:\n");
                for rental_space in &self.storage.rental_spaces {
                    println!("{:#?}\n", rental_space);
                }
            }
            Ok(_) => println!("Invalid category"),
            Err(_) => println!("There was an error, please try again"),
        }
    }

    pub fn match_objects(&mut self) {}
}
