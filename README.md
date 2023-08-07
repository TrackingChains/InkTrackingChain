# InkTrackingChain

## Overview
This smart contract allows the insertion of a triplet of data called "Code", "DataValue", and "Closed". For each "Code" entered, a corresponding list of data will be created, enabling the "Get" function to retrieve the entire list. During the storage phase, the contract also records the block and timestamp to ensure a chronological order of incoming data. It is important to note that data can continue to be inserted until a value is provided for the "Code" with "Closed" set to True, which blocks further insertions.

## Build 
cargo contract build --release

## Test 
cargo contract build --release
