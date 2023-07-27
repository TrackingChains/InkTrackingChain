#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod tracking_chain {
    use ink::prelude::vec::Vec;
    use ink::prelude::borrow::ToOwned;

    // Error.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Provide a detailed comment on the error
        TrackingOnlyOwner,
        TrackingOnlyOwnerOrAdmin,
        TrackingCodeAlreadyClosed
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct StatusData {
        block_number: u32,
        closed: bool,
        data_value: Vec<u8>,
        timestamp: u64,
    }
    
    #[derive(Default, Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct TrackData {
        code: [u8; 32],
        closed: bool,
        histories: Vec<StatusData>,
    }

    #[ink(storage)]
    pub struct TrackingChain {
        admins: ink::storage::Mapping<AccountId, bool>,
        owner: AccountId,
        tracked_codes: ink::storage::Mapping<[u8; 32], TrackData>
    }

    impl TrackingChain {
        
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                admins: Default::default(),
                owner: Self::env().caller(),
                tracked_codes: Default::default()
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        // Get.
        #[ink(message)]
        pub fn get_track(&self, code: [u8; 32]) -> Option<TrackData> {
            let track_data: Option<TrackData> = self.tracked_codes.get(&code.to_owned());
            track_data
        }

        // Insert.
        #[ink(message)]
        pub fn insert_track(&mut self, code: [u8; 32], data_value: Vec<u8>, closed: bool) -> Result<()> {
            let caller = Self::env().caller();
            let call_result = self.ensure_only_admin_or_owner(caller);
            match call_result {
                Ok(()) => {
                    let mut binding = self.tracked_codes.get(&code.to_owned());
                    let track_data = binding.get_or_insert(TrackData {
                        code,
                        closed: closed,
                        histories: Default::default()
                    });
                    
                    if track_data.closed {
                        return Err(Error::TrackingCodeAlreadyClosed);
                    }

                    track_data.histories.push(StatusData {
                        data_value,
                        timestamp: Self::env().block_timestamp(),
                        block_number: Self::env().block_number(),
                        closed,
                    });
                    track_data.closed = closed;

                    self.tracked_codes.insert(code, &binding.unwrap());

                    Ok(())
                }
                Err(actual_error) => Err(actual_error.into())
            }
        }

        // Administrator section
        #[ink(message)]
        pub fn add_admin(&mut self, admin_address: AccountId) -> Result<()> {
            let caller = self.env().caller();
            let call_result = self.ensure_only_owner(caller);

            match call_result {
                Ok(()) => {
                    self.admins.insert(admin_address, &true);
                    Ok(())
                }
                Err(actual_error) => Err(actual_error.into())
            }
        }

        #[ink(message)]
        pub fn revoke_admin(&mut self, admin_address: AccountId) -> Result<()> {
            let caller = self.env().caller();
            let call_result = self.ensure_only_owner(caller);

            match call_result {
                Ok(()) => {
                    self.admins.remove(admin_address);
                    Ok(())
                }
                Err(actual_error) => Err(actual_error.into())
            }
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            let call_result = self.ensure_only_owner(caller);

            match call_result {
                Ok(()) => {
                    self.owner = new_owner;
                    Ok(())
                }
                Err(actual_error) => Err(actual_error.into())
            }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn is_admin(&self, admin_address: AccountId) -> bool {
            let is_admin = self.admins.get(admin_address).unwrap_or(false);
            is_admin
        }

        fn ensure_only_owner(&self, caller: AccountId) -> Result<()> {
            if caller != self.owner {
                return Err(Error::TrackingOnlyOwner);
            }
            Ok(())
        }

        fn ensure_only_admin_or_owner(&self, caller: AccountId) -> Result<()> {
            if caller != self.owner && !self.is_admin(caller) {
                return Err(Error::TrackingOnlyOwnerOrAdmin);
            }
            Ok(())
        }
    }
}
