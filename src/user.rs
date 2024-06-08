use super::{BaseFields, PrefixedUuid};

use {serde::Deserialize, std::fmt};

#[derive(Clone)]
pub struct User {
    base: BaseFields<UserId>,
    first_name: String,
    last_name: String,
    pub workspace_request: Option<WorkspaceRequest>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub workspace_request: Option<WorkspaceRequest>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct UserId {
    value: String,
}

#[derive(Clone, Deserialize)]
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

    pub fn id_value(&self) -> &str {
        &self.id().value
    }
}

impl PrefixedUuid for UserId {
    const PREFIX: &'static str = "usr";
}

impl fmt::Debug for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base_description = format!(
            "{} {}:\n    {:?}",
            self.first_name, self.last_name, self.base
        );
        match self.workspace_request {
            Some(ref workspace_request) => {
                write!(f, "{},\n    {:?}", base_description, workspace_request)
            }
            None => write!(f, "{}", base_description),
        }
    }
}

impl fmt::Debug for WorkspaceRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "workspace_request: {{ nb_workstations: {}, budget: {} }}",
            self.nb_workstations, self.budget
        )
    }
}
