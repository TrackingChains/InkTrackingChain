#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod InkTrackingChain {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink::storage_item]
    pub struct StatusData {
        data_value: Vec<u8>,
        timestamp: u64,
        block_number: u64,
        closed: bool,
    }

    #[ink::storage_item]
    pub struct TrackData {
        code: [u8; 32],
        index: u64,
        closed: bool,
        histories: Mapping<u64, StatusData>,
    }

    #[ink(storage)]
    pub struct Tracking {
        tracked_codes: Mapping<[u8; 32], TrackData>,
    }

    impl InkTrackingChain {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                tracked_codes: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn insert_track(&mut self, code: [u8; 32], data_value: Vec<u8>, closed: bool) {
            self.insert_data(code, data_value, closed);
        }

        fn insert_data(&mut self, code: [u8; 32], data_value: Vec<u8>, closed: bool) {
            let track_data = self.tracked_codes.entry(code).or_insert_with(|| {
                TrackData {
                    code,
                    index: 0,
                    closed: false,
                    histories: Mapping::new(),
                }
            });

            if track_data.closed {
                //ink_env::revert(&RevertError::TrackingOnlyOwner(caller));
                //return Err(Error::TrackingCodeAlreadyClosed)
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

    }
}
