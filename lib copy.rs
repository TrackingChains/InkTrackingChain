#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod InkTrackingChain {
    use super::*;
    use ink_lang::contract;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        traits::{PackedLayout, SpreadLayout},
        collections::HashMap as StorageHashMap,
    };

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct StatusData {
        data_value: Vec<u8>,
        timestamp: u64,
        block_number: u64,
        closed: bool,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct TrackData {
        code: [u8; 32],
        index: u64,
        closed: bool,
        histories: StorageHashMap<u64, StatusData>,
    }

    #[ink(storage)]
    pub struct Tracking {
        owner: AccountId,
        admins: StorageHashMap<AccountId, u256>,
        tracked_codes: StorageHashMap<[u8; 32], TrackData>,
    }

    impl InkTrackingChain {
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            let mut admins = StorageHashMap::new();
            admins.insert(owner, 1.into());

            Self {
                owner,
                admins,
                tracked_codes: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn add_admin(&mut self, admin_address: AccountId) {
            let caller = Self::env().caller();
            self.ensure_only_owner(caller);

            self.admins.insert(admin_address, 1.into());
        }

        #[ink(message)]
        pub fn revoke_admin(&mut self, admin_address: AccountId) {
            let caller = Self::env().caller();
            self.ensure_only_owner(caller);

            self.admins.take(&admin_address);
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            let caller = Self::env().caller();
            self.ensure_only_owner(caller);

            self.owner = new_owner;
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn is_admin(&self, admin_address: AccountId) -> bool {
            self.admins.get(&admin_address).is_some()
        }

        #[ink(message)]
        pub fn insert_track(&mut self, code: [u8; 32], data_value: Vec<u8>, closed: bool) {
            let caller = Self::env().caller();
            self.ensure_only_admin_or_owner(caller);

            self.insert_data(code, data_value, closed);
        }

        #[ink(message)]
        pub fn multi_insert_track(
            &mut self,
            codes: Vec<[u8; 32]>,
            data_values: Vec<Vec<u8>>,
            closed: Vec<bool>,
        ) {
            let caller = Self::env().caller();
            self.ensure_only_admin_or_owner(caller);

            for i in 0..codes.len() {
                self.insert_data(codes[i], data_values[i].clone(), closed[i]);
            }
        }

        fn insert_data(&mut self, code: [u8; 32], data_value: Vec<u8>, closed: bool) {
            let track_data = self.tracked_codes.entry(code).or_insert_with(|| {
                TrackData {
                    code,
                    index: 0,
                    closed: false,
                    histories: StorageHashMap::new(),
                }
            });

            if track_data.closed {
                ink_env::revert(&RevertError::TrackingCodeAlreadyClosed);
            }

            let status_data = StatusData {
                data_value,
                timestamp: Self::env().block_timestamp(),
                block_number: Self::env().block_number(),
                closed,
            };

            track_data.histories.insert(track_data.index, status_data);
            track_data.index += 1;
            track_data.closed = closed;
        }

        #[ink(message)]
        pub fn get_code_tracking(&self, code: [u8; 32]) -> Vec<StatusData> {
            let track_data = self.tracked_codes.get(&code).unwrap();
            let mut ret = Vec::with_capacity(track_data.index as usize);
            for i in 0..track_data.index {
                ret.push(track_data.histories.get(&i).unwrap().clone());
            }
            ret
        }

        fn ensure_only_owner(&self, caller: AccountId) {
            if caller != self.owner {
                ink_env::revert(&RevertError::TrackingOnlyOwner(caller));
            }
        }

        fn ensure_only_admin_or_owner(&self, caller: AccountId) {
            if caller != self.owner && !self.is_admin(caller) {
                ink_env::revert(&RevertError::TrackingOnlyOwnerOrAdmin(caller));
            }
        }
    }

    #[ink(storage)]
    pub struct RevertError;

    impl RevertError {
        #[ink(message)]
        pub fn tracking_only_owner(&self, _forbidden_address: AccountId) {}

        #[ink(message)]
        pub fn tracking_only_owner_or_admin(&self, _forbidden_address: AccountId) {}

        #[ink(message)]
        pub fn tracking_code_already_closed(&self, _code: [u8; 32]) {}
    }
}
