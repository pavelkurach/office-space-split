use office_space_split::{
    object_storage::ObjectStorage,
    user_interface::Interface,
};

fn main() {
    let mut storage = ObjectStorage::new();
    let mut interface = Interface::new(&mut storage);
    loop {
        interface.inquire_command();
    }
}
