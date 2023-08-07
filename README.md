<h1 align="center">Ink Tracking Chain</h1>
<div align="center">
<a href="https://github.com/TrackingChains/InkTrackingChain/actions/workflows/rust.yml"><img src="https://github.com/TrackingChains/InkTrackingChain/actions/workflows/rust.yml/badge.svg" alt="Rust"/></a>
<a href="https://github.com/TrackingChains/InkTrackingChain/actions/workflows/publish-stable.yml"><img src="https://github.com/TrackingChains/TrackingChain/actions/workflows/publish-stable.yml/badge.svg" alt="Publish stable release"/></a>
<a href="https://github.com/TrackingChains/InkTrackingChain/stargazers"><img src="https://img.shields.io/github/stars/TrackingChains/InkTrackingChain" alt="Stars Badge"/></a>
<a href="https://github.com/TrackingChains/InkTrackingChain/network/members"><img src="https://img.shields.io/github/forks/TrackingChains/InkTrackingChain" alt="Forks Badge"/></a>
<a href="https://github.com/TrackingChains/InkTrackingChain/pulls"><img src="https://img.shields.io/github/issues-pr/TrackingChains/InkTrackingChain" alt="Pull Requests Badge"/></a>
<a href="https://github.com/TrackingChains/InkTrackingChain/issues"><img src="https://img.shields.io/github/issues/TrackingChains/InkTrackingChain" alt="Issues Badge"/></a>
<a href="https://github.com/TrackingChains/InkTrackingChain/graphs/contributors"><img alt="GitHub contributors" src="https://img.shields.io/github/contributors/InkTrackingChain/TrackingChain?color=2b9348"></a>
<a href="https://github.com/TrackingChains/InkTrackingChain/blob/main/LICENSE"><img src="https://img.shields.io/github/license/TrackingChains/InkTrackingChain?color=2b9348" alt="License Badge"/></a>
</div>
<br>  
  
## Overview  
This smart contract allows the insertion of a triplet of data called "Code", "DataValue", and "Closed". For each "Code" entered, a corresponding list of data will be created, enabling the "Get" function to retrieve the entire list. During the storage phase, the contract also records the block and timestamp to ensure a chronological order of incoming data. It is important to note that data can continue to be inserted until a value is provided for the "Code" with "Closed" set to True, which blocks further insertions.

## Build 
cargo contract build --release

## Test 
cargo contract build --release
  
## Contents
  - [Wiki](https://github.com/TrackingChains/TrackingChain/wiki) for all info about the project.
  - [Configuration Step By Step](https://github.com/TrackingChains/TrackingChain/wiki/Configuration-Step-By-Step) for all configuration project.
  - [Demo Step By Step](https://github.com/TrackingChains/TrackingChain/wiki/Demo-Step-By-Step) for see a little demo of how project work.

# Contribute

Contributions are always welcome! Please create a PR to add Github Profile.

## :pencil: License

This project is licensed under [MIT](https://opensource.org/licenses/MIT) license.

## :man_astronaut: Show your support

Give a ⭐️ if this project helped you!
