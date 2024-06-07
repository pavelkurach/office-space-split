pub mod rental_space;
pub mod split;
pub mod user;

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

impl<Id: PrefixedUuid> BaseFields<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            created_at: Utc::now(),
        }
    }
}
