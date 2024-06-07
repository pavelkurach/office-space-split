use super::{BaseFields, PrefixedUuid};

use serde::Deserialize;

#[derive(Debug)]
pub struct User {
    base: BaseFields<UserId>,
    first_name: String,
    last_name: String,
    workspace_request: Option<WorkspaceRequest>,
}

#[derive(Debug, Deserialize)]
pub struct AddUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub workspace_request: Option<WorkspaceRequest>,
}

#[derive(Debug, Clone)]
pub struct UserId {
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceRequest {
    pub nb_workstations: u32,
    pub budget: u32,
}

impl User {
    pub fn new(request: AddUserRequest) -> anyhow::Result<Self> {
        Ok(Self {
            base: BaseFields::new(UserId {
                value: UserId::generate(),
            }),
            first_name: request.first_name,
            last_name: request.last_name,
            workspace_request: request.workspace_request,
        })
    }

    pub fn id(&self) -> &UserId {
        &self.base.id
    }
}

impl PrefixedUuid for UserId {
    const PREFIX: &'static str = "usr";
}
