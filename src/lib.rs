pub(crate) mod office;
pub(crate) mod user;

use {
    chrono::{DateTime, Utc},
    uuid::Uuid,
};

pub(crate) trait PrefixedUuid {
    const PREFIX: &'static str;

    fn generate() -> String {
        format!("{}-{}", Self::PREFIX, Uuid::new_v4())
    }
}

pub(crate) struct BaseFields<Id: PrefixedUuid> {
    pub(crate) id: Id,
    pub(crate) created_at: DateTime<Utc>,
}

impl<Id: PrefixedUuid> BaseFields<Id> {
    pub(crate) fn new(id: Id) -> Self {
        Self {
            id,
            created_at: Utc::now(),
        }
    }
}
