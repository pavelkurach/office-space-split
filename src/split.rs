use super::{BaseFields, PrefixedUuid};

use crate::user::UserId;

#[derive(Debug)]
pub(crate) struct SplitId {
    pub(crate) value: String,
}

impl PrefixedUuid for SplitId {
    const PREFIX: &'static str = "ofc";
}

#[derive(Debug)]
pub(crate) struct Split {
    pub(crate) base: BaseFields<SplitId>,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) surface: u32,
    pub(crate) nb_workstations: u32,
    pub(crate) price_per_workstation: u32,
    pub(crate) parent_office_id: Option<SplitId>,
    pub(crate) owner_id: UserId,
}
