use super::{BaseFields, PrefixedUuid};

use {
    serde::Deserialize,
    std::fmt,
    validator::{Validate, ValidationError},
};

use crate::user::UserId;

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
    #[validate(range(min = 40, max = 180))]
    pub nb_workstations: u32,
    #[validate(range(min = 300, max = 800))]
    pub price_per_workstation: u32,
    pub owner_id: String,
}

#[derive(Debug, Clone)]
pub struct Split {
    base: BaseFields<SplitId>,
    name: String,
    address: String,
    surface: u32,
    pub nb_workstations: u32,
    pub price_per_workstation: u32,
    pub parent_office_id: RentalSpaceId,
    pub owner_id: UserId,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SplitId {
    value: String,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RentalSpaceId {
    value: String,
}

impl PrefixedUuid for RentalSpaceId {
    const PREFIX: &'static str = "ofc";
}

impl PrefixedUuid for SplitId {
    const PREFIX: &'static str = "spl";
}

impl fmt::Debug for RentalSpaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for SplitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
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

impl Split {
    pub fn id(&self) -> &SplitId {
        &self.base.id
    }

    pub fn id_value(&self) -> &str {
        &self.id().value
    }

    pub fn price(&self) -> u32 {
        self.nb_workstations * self.price_per_workstation
    }

    pub fn subsplit_min_nb_workstations(&self) -> Option<u32> {
        let min_nb_workstations_to_respect_density_constraint =
            if self.nb_workstations * 8 < self.surface * 5 {
                40
            } else {
                60
            };

        if self.nb_workstations > 2 * min_nb_workstations_to_respect_density_constraint {
            Some(min_nb_workstations_to_respect_density_constraint)
        } else {
            None
        }
    }

    pub fn subsplit_max_nb_workstations(&self) -> Option<u32> {
        self.subsplit_min_nb_workstations()
            .map(|min| self.nb_workstations - min)
    }

    pub fn can_be_subsplit(&self, nb_workstations: u32) -> bool {
        self.subsplit_min_nb_workstations().is_some_and(|min| {
            nb_workstations >= min && nb_workstations <= self.nb_workstations - min
        })
    }

    pub fn subsplit(&self, nb_workstations: u32) -> Option<(Split, Split)> {
        self.can_be_subsplit(nb_workstations).then(|| {
            (
                Split {
                    base: BaseFields::new(SplitId {
                        value: SplitId::generate(),
                    }),
                    name: self.name.to_owned(),
                    address: self.address.to_owned(),
                    surface: self.surface * nb_workstations / self.nb_workstations,
                    nb_workstations,
                    price_per_workstation: self.price_per_workstation,
                    parent_office_id: self.parent_office_id.clone(),
                    owner_id: self.owner_id.clone(),
                },
                Split {
                    base: BaseFields::new(SplitId {
                        value: SplitId::generate(),
                    }),
                    name: self.name.to_owned(),
                    address: self.address.to_owned(),
                    surface: self.surface * (self.nb_workstations - nb_workstations)
                        / self.nb_workstations,
                    nb_workstations: self.nb_workstations - nb_workstations,
                    price_per_workstation: self.price_per_workstation,
                    parent_office_id: self.parent_office_id.clone(),
                    owner_id: self.owner_id.clone(),
                },
            )
        })
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

impl fmt::Debug for RentalSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:
    {:?},
    address: {},
    surface: {},
    nb_workstations: {},
    price_per_workstation: {},
    owner_id: {:?}",
            self.name,
            self.base,
            self.address,
            self.surface,
            self.nb_workstations,
            self.price_per_workstation,
            self.owner_id
        )
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
