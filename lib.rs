#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod tracking_chain {
    use ink::prelude::vec::Vec;

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
        tracked_codes: ink::storage::Mapping<[u8; 32], TrackData>
    }

    impl TrackingChain {
        
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                tracked_codes: Default::default()
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn get_track(&self, code: [u8; 32]) -> TrackData {
            self.tracked_codes.get(code).unwrap()
        }

        #[ink(message)]
        pub fn insert_track(&mut self, code: [u8; 32], data_value: Vec<u8>, closed: bool) {
            let mut binding = self.tracked_codes.get(code);
            let track_data = binding.get_or_insert(TrackData {
                code,
                closed: closed,
                histories: Default::default()
            });

            track_data.histories.push(StatusData {
                data_value,
                timestamp: Self::env().block_timestamp(),
                block_number: Self::env().block_number(),
                closed,
            });
            track_data.closed = closed;
        }
    }
}
