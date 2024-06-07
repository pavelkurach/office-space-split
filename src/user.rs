use super::{BaseFields, PrefixedUuid};

use {
    serde::{Deserialize, Serialize},
    validator::{Validate, ValidationError},
};

#[derive(Debug)]
pub(crate) struct User {
    base: BaseFields<UserId>,
    first_name: String,
    last_name: String,
    workspace_request: Option<WorkspaceRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub(crate) struct UserId {
    pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub(crate) struct WorkspaceRequest {
    pub(crate) nb_workstations: u32,
    pub(crate) budget: u32,
}

impl PrefixedUuid for UserId {
    const PREFIX: &'static str = "usr";
}
