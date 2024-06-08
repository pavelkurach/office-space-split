use std::collections::{HashMap, HashSet};

use crate::{contract::Contract, object_storage::ObjectStorage, rental_space::Split, user::User};

#[derive(Debug)]
pub struct Matchings {
    pub year_1_contracts: Vec<Contract>,
    pub year_2_contracts: Vec<Contract>,
    pub percentage_of_matched_users: i32,
}

pub struct MatchingEngine<'a> {
    storage: &'a ObjectStorage,
}

impl<'a> MatchingEngine<'a> {
    pub fn new(storage: &'a ObjectStorage) -> Self {
        Self { storage }
    }

    pub fn get_greedy_matchings(&self, with_subsplit: bool) -> Matchings {
        let mut year_1_contracts: Vec<Contract> = Vec::new();
        let mut year_2_contracts: Vec<Contract> = Vec::new();

        let users_with_workspace_request: Vec<&User> = self
            .storage
            .users()
            .into_iter()
            .filter(|user| user.workspace_request.is_some())
            .collect();

        let nb_users_with_workspace_request = users_with_workspace_request.len();

        let original_splits: HashMap<String, Split> = self
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

        match with_subsplit {
            false => {
                let mut available_splits: HashSet<&str> =
                    original_splits.keys().map(String::as_str).collect();

                for user in users_with_workspace_request {
                    Self::match_user_greedily_without_split(
                        user,
                        &original_splits,
                        &mut available_splits,
                        &mut unmatched_users,
                        &mut year_1_contracts,
                    );
                }

                let unmatched_users_after_year_1 = unmatched_users.clone();
                let mut available_splits: HashSet<&str> =
                    original_splits.keys().map(String::as_str).collect();

                for user_id in unmatched_users_after_year_1 {
                    let user = self.storage.get_user(user_id).unwrap();
                    Self::match_user_greedily_without_split(
                        user,
                        &original_splits,
                        &mut available_splits,
                        &mut unmatched_users,
                        &mut year_2_contracts,
                    );
                }
            }
            true => {
                let mut splits: HashMap<String, Split> = original_splits
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                let mut available_splits: HashSet<String> =
                    original_splits.keys().map(String::to_owned).collect();

                for user in users_with_workspace_request {
                    Self::match_user_greedily_with_split(
                        user,
                        &mut splits,
                        &mut available_splits,
                        &mut unmatched_users,
                        &mut year_1_contracts,
                    );
                }

                let unmatched_users_after_year_1 = unmatched_users.clone();

                let mut splits: HashMap<String, Split> = original_splits
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                let mut available_splits: HashSet<String> =
                    original_splits.keys().map(String::to_owned).collect();

                for user_id in unmatched_users_after_year_1 {
                    let user = self.storage.get_user(user_id).unwrap();
                    Self::match_user_greedily_with_split(
                        user,
                        &mut splits,
                        &mut available_splits,
                        &mut unmatched_users,
                        &mut year_2_contracts,
                    );
                }
            }
        }
        Matchings {
            year_1_contracts,
            year_2_contracts,
            percentage_of_matched_users: ((1.0
                - (unmatched_users.len() as f32 / nb_users_with_workspace_request as f32))
                * 100.0)
                .round() as i32,
        }
    }

    fn match_user_greedily_with_split(
        user: &User,
        splits: &mut HashMap<String, Split>,
        available_splits: &mut HashSet<String>,
        unmatched_users: &mut HashSet<&str>,
        contracts: &mut Vec<Contract>,
    ) {
        let matched_rental_space_without_split = available_splits
            .iter()
            .cloned()
            .filter(|split_id| {
                !splits
                    .get(split_id)
                    .unwrap()
                    .can_be_subsplit(user.workspace_request.as_ref().unwrap().nb_workstations)
            })
            .filter(|split_id| {
                let split = splits.get(split_id).unwrap();
                split.nb_workstations >= user.workspace_request.as_ref().unwrap().nb_workstations
                    && split.price() <= user.workspace_request.as_ref().unwrap().budget
            })
            .min_by_key(|split_id| splits.get(split_id).unwrap().nb_workstations);

        if let Some(split_id) = matched_rental_space_without_split {
            unmatched_users.remove(user.id_value());
            available_splits.remove(&split_id);
            let split = splits.get(&split_id).unwrap().clone();
            contracts.push(Contract::new(
                split.parent_office_id.clone(),
                split.owner_id.clone(),
                user.id().clone(),
                split.nb_workstations,
                split.price().clone(),
            ));
        } else {
            let matched_rental_space_with_split = available_splits
                .iter()
                .cloned()
                .filter(|split_id| {
                    splits
                        .get(split_id)
                        .unwrap()
                        .can_be_subsplit(user.workspace_request.as_ref().unwrap().nb_workstations)
                })
                .max_by_key(|split_id| splits.get(split_id).unwrap().nb_workstations);

            if let Some(split_id) = matched_rental_space_with_split {
                unmatched_users.remove(user.id_value());
                let original_split = splits.remove(&split_id).unwrap();
                let (split1, split2) = original_split
                    .subsplit(user.workspace_request.as_ref().unwrap().nb_workstations)
                    .unwrap();
                available_splits.remove(&split_id);
                available_splits.insert(split2.id_value().to_owned());
                splits.insert(split2.id_value().to_owned(), split2);
                contracts.push(Contract::new(
                    split1.parent_office_id.clone(),
                    split1.owner_id.clone(),
                    user.id().clone(),
                    split1.nb_workstations,
                    split1.price().clone(),
                ));
            }
        }
    }

    fn match_user_greedily_without_split(
        user: &User,
        splits: &HashMap<String, Split>,
        available_splits: &mut HashSet<&str>,
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
                split.parent_office_id.clone(),
                split.owner_id.clone(),
                user.id().clone(),
                split.nb_workstations,
                split.price().clone(),
            ));
        }
    }
}
