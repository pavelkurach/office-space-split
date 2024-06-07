use super::{BaseFields, PrefixedUuid};

use {
    serde::{Deserialize, Serialize},
    validator::{Validate, ValidationError},
};

use crate::user::UserId;

#[derive(Debug)]
pub(crate) struct RentalSpace {
    base: BaseFields<RentalSpaceId>,
    name: String,
    address: String,
    surface: u32,
    nb_workstations: u32,
    price_per_workstation: u32,
    parent_office_id: Option<RentalSpaceId>,
    owner_id: UserId,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[validate(schema(function = "validate_workstation_density"))]
pub(crate) struct AddRentalSpaceRequest {
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) surface: u32,
    #[validate(range(exclusive_min = 40, exclusive_max = 180))]
    pub(crate) nb_workstations: u32,
    #[validate(range(exclusive_min = 300, max = 800))]
    pub(crate) price_per_workstation: u32,
    pub(crate) parent_office_id: Option<RentalSpaceId>,
    pub(crate) owner_id: UserId,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub(crate) struct RentalSpaceId {
    value: String,
}

impl PrefixedUuid for RentalSpaceId {
    const PREFIX: &'static str = "ofc";
}

impl RentalSpace {
    pub(crate) fn new(request: AddRentalSpaceRequest) -> anyhow::Result<Self> {
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
            parent_office_id: request.parent_office_id,
            owner_id: request.owner_id,
        })
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
