# Symbiotic Protocol Rust Bindings

This repository contains Rust bindings for the Symbiotic Protocol smart contracts. These bindings allow for easy interaction with the Symbiotic Protocol from Rust applications.

## Features

- Type-safe Rust interfaces for Symbiotic Protocol contracts
- Generated from Solidity ABIs using `alloy-sol-types`
- Serialization support via `serde`

## Installation

Add this to your `Cargo.toml`:
```
symbiotic-rs = { version = "0.1.0" }
```

## Adding new contracts

To add new contracts to the Rust bindings, follow these steps:

1. Add the contract import to `contracts/src/Utility.sol`:
   ```solidity
   import "path/to/NewContract.sol";
   ```

2. Add the JSON file for the new contract to the list in the `test_forge_build_and_json_extraction` test in `src/lib.rs`:
   ```rust
   let json_files = vec![
       // ... existing files ...
       "NewContract.sol/NewContract.json",
   ];
   ```

3. Run the test to generate the JSON files:
   ```
   cargo test test_forge_build_and_json_extraction
   ```

4. Add the new contract to the `generate_sol_types!` macro in `src/lib.rs`:
   ```rust
   generate_sol_types!(
       // ... existing contracts ...
       NewContract, "json/NewContract.json",
   );
   ```

These steps will ensure that the new contract is properly imported, its JSON artifact is copied to the correct location, and it's exposed in the Rust bindings for use in your project.



