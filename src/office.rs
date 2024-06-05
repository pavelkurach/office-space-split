use super::{BaseFields, PrefixedUuid};

use crate::user::UserId;

pub(crate) struct OfficeId {
    pub(crate) value: String,
}

impl PrefixedUuid for OfficeId {
    const PREFIX: &'static str = "ofc";
}

pub(crate) struct Office {
    pub(crate) base: BaseFields<OfficeId>,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) surface: u32,
    pub(crate) nb_workstations: u32,
    pub(crate) price_per_workstation: u32,
    pub(crate) parent_office_id: Option<OfficeId>,
    pub(crate) owner_id: UserId,
}
