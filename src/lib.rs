pub mod contract;
pub mod matching;
pub mod object_storage;
pub mod rental_space;
pub mod user;
pub mod user_interface;

use {
    chrono::{DateTime, Utc},
    std::fmt,
    uuid::Uuid,
};

pub trait PrefixedUuid {
    const PREFIX: &'static str;

    fn generate() -> String {
        format!("{}-{}", Self::PREFIX, Uuid::new_v4())
    }
}

#[derive(Clone)]
pub struct BaseFields<Id: PrefixedUuid> {
    id: Id,
    created_at: DateTime<Utc>,
}

impl<Id: PrefixedUuid + fmt::Debug> fmt::Debug for BaseFields<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id: {:?},\n    created_at: {:?}",
            self.id, self.created_at
        )
    }
}

impl<Id: PrefixedUuid> BaseFields<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            created_at: Utc::now(),
        }
    }
}
