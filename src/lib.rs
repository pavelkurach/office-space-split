pub mod contract;
pub mod matching;
pub mod object_storage;
pub mod rental_space;
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

#[derive(Debug, Clone)]
pub struct BaseFields<Id: PrefixedUuid> {
    id: Id,
    created_at: DateTime<Utc>,
}

impl<Id: PrefixedUuid> BaseFields<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            created_at: Utc::now(),
        }
    }
}
