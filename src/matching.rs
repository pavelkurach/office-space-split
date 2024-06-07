use std::collections::{HashMap, HashSet};

use crate::{contract::Contract, object_storage::ObjectStorage, rental_space::Split, user::User};

#[derive(Debug)]
pub struct Matchings {
    pub year_1_contracts: Vec<Contract>,
    pub year_2_contracts: Vec<Contract>,
    splits: Vec<Split>,
}

pub struct MatchingEngine<'a> {
    storage: &'a ObjectStorage,
}

impl<'a> MatchingEngine<'a> {
    pub fn new(storage: &'a ObjectStorage) -> Self {
        Self { storage }
    }

    pub fn get_greedy_matchings(&self) -> Matchings {
        let mut matched_splits: Vec<Split> = Vec::new();
        let mut year_1_contracts: Vec<Contract> = Vec::new();
        let mut year_2_contracts: Vec<Contract> = Vec::new();

        let users_with_workspace_request: Vec<&User> = self
            .storage
            .users()
            .into_iter()
            .filter(|user| user.workspace_request.is_some())
            .collect();

        let mut splits: HashMap<String, Split> = self
            .storage
            .rental_spaces()
            .into_iter()
            .map(|rental_space| {
                let split: Split = rental_space.into();
                return (split.id_value().to_owned(), split);
            })
            .collect();

        let mut unmatched_users: HashSet<&str> = users_with_workspace_request
            .iter()
            .map(|user| user.id_value())
            .collect();

        let mut available_splits: HashSet<&str> = splits.keys().map(String::as_str).collect();
        for user in users_with_workspace_request {
            Self::match_greedily_without_split(
                user,
                &splits,
                &mut available_splits,
                &mut matched_splits,
                &mut unmatched_users,
                &mut year_1_contracts,
            );
        }

        let unmatched_users_after_year_1 = unmatched_users.clone();
        let mut available_splits: HashSet<&str> = splits.keys().map(String::as_str).collect();

        for user_id in unmatched_users_after_year_1 {
            let user = self.storage.get_user(user_id).unwrap();
            Self::match_greedily_without_split(
                user,
                &splits,
                &mut available_splits,
                &mut matched_splits,
                &mut unmatched_users,
                &mut year_2_contracts,
            );
        }

        Matchings {
            year_1_contracts,
            year_2_contracts,
            splits: matched_splits,
        }
    }

    fn match_greedily_without_split(
        user: &User,
        splits: &HashMap<String, Split>,
        available_splits: &mut HashSet<&str>,
        matched_splits: &mut Vec<Split>,
        unmatched_users: &mut HashSet<&str>,
        contracts: &mut Vec<Contract>,
    ) {
        let matched_rental_space = available_splits
            .iter()
            .cloned()
            .filter(|&split_id| {
                let split = splits.get(split_id).unwrap();
                split.nb_workstations >= user.workspace_request.as_ref().unwrap().nb_workstations
                    && split.price() <= user.workspace_request.as_ref().unwrap().budget
            })
            .min_by_key(|&split_id| splits.get(split_id).unwrap().nb_workstations);

        if let Some(split_id) = matched_rental_space {
            unmatched_users.remove(user.id_value());
            available_splits.remove(split_id);
            let split = splits.get(split_id).unwrap().clone();
            contracts.push(Contract::new(
                split.id().clone(),
                split.owner_id().clone(),
                user.id().clone(),
                split.price().clone(),
            ));
            matched_splits.push(split);
        }
    }
}
