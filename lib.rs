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
                        closed: false,
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn insert_track_should_be_ok() {
            // Arrange
            let mut contract = TrackingChain::new();
            
            // Act
            // Call the function `insert_track`
            let result = contract.insert_track([0; 32], vec![1, 2, 3], false);

            // Assert
            // Check if the result is successful
            assert!(result.is_ok(), "Inserting track should succeed");

            // Check that the track data has been updated correctly
            let track_data = contract.tracked_codes.get(&[0; 32]);
            assert!(track_data.is_some(), "Track data should exist");
            let track_data = track_data.unwrap();
            assert_eq!(track_data.code, [0; 32], "Track code should match");
            assert_eq!(track_data.histories.len(), 1, "There should be one history entry");
            assert_eq!(
                track_data.histories[0].data_value,
                vec![1, 2, 3],
                "Data value should match"
            );
            assert_eq!(
                track_data.histories[0].closed,
                false,
                "Closed status should be false"
            );
        }

        #[ink::test]
        fn get_track_should_be_get_data() {
            // Arrange
            let mut contract = TrackingChain::new(); 

            // Act
            // Call the function `insert_track` to add some data
            contract.insert_track([0; 32], vec![1, 2, 3], false).unwrap();

            // Call the function `get_track` to retrieve the added data
            let track_data = contract.get_track([0; 32]);

            // Assert
            // Check that the retrieved track data is not None
            assert!(track_data.is_some(), "Track data should exist");

            // Check that the retrieved track data matches the inserted data
            let track_data = track_data.unwrap();
            assert_eq!(track_data.code, [0; 32], "Track code should match");
            assert_eq!(track_data.histories.len(), 1, "There should be one history entry");
            assert_eq!(
                track_data.histories[0].data_value,
                vec![1, 2, 3],
                "Data value should match"
            );
            assert_eq!(
                track_data.histories[0].closed,
                false,
                "Closed status should be false"
            );
        }

        #[ink::test]
        fn get_track_should_be_get_multiple_data() {
            // Arrange
            let mut contract = TrackingChain::new(); 
    
            // Act
            // Call the function `insert_track` to add two data entries with the same code but different data_value
            contract.insert_track([0; 32], vec![1, 2, 3], false).unwrap();
            contract.insert_track([0; 32], vec![4, 5, 6], true).unwrap();
    
            // Call the function `get_track` to retrieve the data with the same code
            let track_data = contract.get_track([0; 32]);
    
            // Assert
            // Check that the retrieved track data is not None
            assert!(track_data.is_some(), "Track data should exist");
    
            // Check that the retrieved track data has both data entries
            let track_data = track_data.unwrap();
            assert_eq!(track_data.code, [0; 32], "Track code should match");
    
            // Check that there are two history entries
            assert_eq!(track_data.histories.len(), 2, "There should be two history entries");
    
            // Check the first history entry
            assert_eq!(
                track_data.histories[0].data_value,
                vec![1, 2, 3],
                "First data value should match"
            );
            assert_eq!(
                track_data.histories[0].closed,
                false,
                "First closed status should be false"
            );
    
            // Check the second history entry
            assert_eq!(
                track_data.histories[1].data_value,
                vec![4, 5, 6],
                "Second data value should match"
            );
            assert_eq!(
                track_data.histories[1].closed,
                true,
                "Second closed status should be true"
            );
        }

        #[ink::test]
        fn get_track_should_be_get_multiple_code_data() {
            // Arrange
            let mut contract = TrackingChain::new();

            // Act
            // Call the function `insert_track` to add two data entries with the same code but different data_value
            contract.insert_track([0; 32], vec![1, 2, 3], false).unwrap();
            contract.insert_track([0; 32], vec![4, 5, 6], true).unwrap();

            // Call the function `insert_track` again to add data with a different code
            contract.insert_track([1; 32], vec![7, 8, 9], false).unwrap();

            // Call the function `get_track` to retrieve the data with the same code
            let track_data_1 = contract.get_track([0; 32]);
            let track_data_2 = contract.get_track([1; 32]);

            // Assert
            // Check that the retrieved track data is not None
            assert!(track_data_1.is_some(), "Track data 1 should exist");
            assert!(track_data_2.is_some(), "Track data 2 should exist");

            // Check that the retrieved track data has both data entries for code [0; 32]
            let track_data_1 = track_data_1.unwrap();
            assert_eq!(track_data_1.code, [0; 32], "Track code 1 should match");
            assert_eq!(track_data_1.histories.len(), 2, "There should be two history entries for code 1");

            // Check the first history entry for code [0; 32]
            assert_eq!(
                track_data_1.histories[0].data_value,
                vec![1, 2, 3],
                "First data value for code 1 should match"
            );
            assert_eq!(
                track_data_1.histories[0].closed,
                false,
                "First closed status for code 1 should be false"
            );

            // Check that the retrieved track data has both data entries for code [1; 32]
            let track_data_2 = track_data_2.unwrap();
            assert_eq!(track_data_2.code, [1; 32], "Track code 2 should match");
            assert_eq!(track_data_2.histories.len(), 1, "There should be one history entry for code 2");

            // Check the history entry for code [1; 32]
            assert_eq!(
                track_data_2.histories[0].data_value,
                vec![7, 8, 9],
                "Data value for code 2 should match"
            );
            assert_eq!(
                track_data_2.histories[0].closed,
                false,
                "Closed status for code 2 should be false"
            );
        }

        #[ink::test]
        fn insert_track_should_be_error_with_closed_true() {
            // Arrange
            let mut contract = TrackingChain::new();

            // Act
            // Call the function `insert_track` with closed set to true
            let _ = contract.insert_track([0; 32], vec![1, 2, 3], true);
            let result = contract.insert_track([0; 32], vec![3, 4, 5], false);

            // Assert
            // Check that the result is an error
            assert!(result.is_err(), "Inserting track with closed=true should return an error");

            // Check that the error is `TrackingCodeAlreadyClosed`
            assert_eq!(result.err().unwrap(), Error::TrackingCodeAlreadyClosed);

            // Check that the second track data is not added
            let track_data = contract.tracked_codes.get(&[0; 32]);
            assert!(track_data.is_some(), "Track data should exist");
            let track_data = track_data.unwrap();
            assert_eq!(track_data.code, [0; 32], "Track code should match");
            assert_eq!(track_data.histories.len(), 1, "There should be one history entry");
            assert_eq!(
                track_data.histories[0].data_value,
                vec![1, 2, 3],
                "Data value should match"
            );
            assert_eq!(
                track_data.histories[0].closed,
                true,
                "Closed status should be true"
            );
        }
    }
}
