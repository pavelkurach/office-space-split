use super::{BaseFields, PrefixedUuid};

use crate::user::UserId;

#[derive(Debug)]
pub struct Split {
    pub base: BaseFields<SplitId>,
    pub name: String,
    pub address: String,
    pub surface: u32,
    pub nb_workstations: u32,
    pub price_per_workstation: u32,
    pub parent_office_id: Option<SplitId>,
    pub owner_id: UserId,
}

#[derive(Debug)]
pub struct SplitId {
    pub value: String,
}

impl PrefixedUuid for SplitId {
    const PREFIX: &'static str = "spl";
}
