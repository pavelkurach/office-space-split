use super::{BaseFields, PrefixedUuid};

use {
    serde::Deserialize,
    validator::{Validate, ValidationError},
};

use crate::user::UserId;

#[derive(Debug)]
pub struct RentalSpace {
    base: BaseFields<RentalSpaceId>,
    name: String,
    address: String,
    surface: u32,
    nb_workstations: u32,
    price_per_workstation: u32,
    owner_id: UserId,
}

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_workstation_density"))]
pub struct AddRentalSpaceRequest {
    pub name: String,
    pub address: String,
    pub surface: u32,
    #[validate(range(exclusive_min = 40, exclusive_max = 180))]
    pub nb_workstations: u32,
    #[validate(range(exclusive_min = 300, max = 800))]
    pub price_per_workstation: u32,
    pub owner_id: String,
}

#[derive(Debug)]
pub struct Split {
    base: BaseFields<SplitId>,
    name: String,
    address: String,
    surface: u32,
    nb_workstations: u32,
    price_per_workstation: u32,
    parent_office_id: RentalSpaceId,
    owner_id: UserId,
}

#[derive(Debug)]
struct SplitId {
    value: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RentalSpaceId {
    value: String,
}

impl RentalSpaceId {
    pub fn mathces(&self, id: &str) -> bool {
        self.value == id
    }
}

impl SplitId {
    pub fn mathces(&self, id: &str) -> bool {
        self.value == id
    }
}

impl PrefixedUuid for RentalSpaceId {
    const PREFIX: &'static str = "ofc";
}

impl PrefixedUuid for SplitId {
    const PREFIX: &'static str = "spl";
}

impl RentalSpace {
    pub fn new(request: AddRentalSpaceRequest, owner_id: UserId) -> anyhow::Result<Self> {
        request.validate()?;
        Ok(Self {
            base: BaseFields::new(RentalSpaceId {
                value: RentalSpaceId::generate(),
            }),
            name: request.name,
            address: request.address,
            surface: request.surface,
            nb_workstations: request.nb_workstations,
            price_per_workstation: request.price_per_workstation,
            owner_id,
        })
    }

    pub fn id(&self) -> &RentalSpaceId {
        &self.base.id
    }

    pub fn id_value(&self) -> &str {
        &self.id().value
    }
}

impl Into<Split> for &RentalSpace {
    fn into(self) -> Split {
        Split {
            base: BaseFields::new(SplitId {
                value: SplitId::generate(),
            }),
            name: self.name.to_owned(),
            address: self.address.to_owned(),
            surface: self.surface,
            nb_workstations: self.nb_workstations,
            price_per_workstation: self.price_per_workstation,
            parent_office_id: self.base.id.clone(),
            owner_id: self.owner_id.clone(),
        }
    }
}

fn validate_workstation_density(
    add_rental_space_request: &AddRentalSpaceRequest,
) -> Result<(), ValidationError> {
    let error = ValidationError::new("workstation_density");
    if add_rental_space_request.nb_workstations < 60
        && add_rental_space_request.nb_workstations * 8 > add_rental_space_request.surface * 5
    {
        return Err(error.with_message("Is is not allowed to have more than 5 workstations per 8 square meters if there are less than 60 workstations".into()));
    } else if add_rental_space_request.nb_workstations >= 60
        && add_rental_space_request.nb_workstations * 7 > add_rental_space_request.surface * 5
    {
        return Err(error.with_message("Is is not allowed to have more than 5 workstations per 7 square meters if there are more than 60 workstations".into()));
    }
    Ok(())
}
