use crate::{rental_space::SplitId, user::UserId, BaseFields, PrefixedUuid};

#[derive(Debug)]
pub struct Contract {
    base: BaseFields<ContractId>,
    split_id: SplitId,
    host_id: UserId,
    guest_id: UserId,
    price: u32,
}

#[derive(Debug)]
pub struct ContractId {
    value: String,
}

impl PrefixedUuid for ContractId {
    const PREFIX: &'static str = "agr";
}

impl Contract {
    pub fn new(split_id: SplitId, host_id: UserId, guest_id: UserId, price: u32) -> Self {
        Self {
            base: BaseFields::new(ContractId {
                value: ContractId::generate(),
            }),
            split_id,
            host_id,
            guest_id,
            price,
        }
    }
}
