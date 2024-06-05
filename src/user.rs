use super::{BaseFields, PrefixedUuid};

pub(crate) struct UserId {
    pub(crate) value: String,
}

impl PrefixedUuid for UserId {
    const PREFIX: &'static str = "usr";
}

pub(crate) struct User {
    pub(crate) base: BaseFields<UserId>,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}
