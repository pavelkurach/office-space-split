use std::fmt;

use crate::{rental_space::RentalSpaceId, user::UserId, BaseFields, PrefixedUuid};

pub struct Contract {
    base: BaseFields<ContractId>,
    rental_space_id: RentalSpaceId,
    host_id: UserId,
    guest_id: UserId,
    nb_workstations: u32,
    price: u32,
}

pub struct ContractId {
    value: String,
}

impl PrefixedUuid for ContractId {
    const PREFIX: &'static str = "agr";
}

impl Contract {
    pub fn new(
        rental_space_id: RentalSpaceId,
        host_id: UserId,
        guest_id: UserId,
        nb_workstations: u32,
        price: u32,
    ) -> Self {
        Self {
            base: BaseFields::new(ContractId {
                value: ContractId::generate(),
            }),
            rental_space_id,
            host_id,
            guest_id,
            nb_workstations,
            price,
        }
    }
}

impl fmt::Debug for ContractId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Contract {{
    {:?},
    rental_space_id: {:?},
    host_id: {:?},
    guest_id: {:?},
    nb_workstations: {:?},
    price: {:?}
}}",
            self.base,
            self.rental_space_id,
            self.host_id,
            self.guest_id,
            self.nb_workstations,
            self.price
        )
    }
}
