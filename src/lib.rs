use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};

macro_rules! generate_sol_types {
    ($($name:ident, $path:expr),* $(,)?) => {
        $(
            sol!(
                #[allow(missing_docs)]
                #[sol(rpc)]
                #[derive(Debug, Serialize, Deserialize)]
                $name,
                $path
            );
        )*
    };
}

generate_sol_types!(
    NetworkRegistry, "json/NetworkRegistry.json",
    OperatorRegistry, "json/OperatorRegistry.json",
    SlasherFactory, "json/SlasherFactory.json",
    VaultConfigurator, "json/VaultConfigurator.json",
    VaultFactory, "json/VaultFactory.json",
    BaseSlasher, "json/BaseSlasher.json",
    Slasher, "json/Slasher.json",
    VetoSlasher, "json/VetoSlasher.json",
    MetadataService, "json/MetadataService.json",
    NetworkMiddlewareService, "json/NetworkMiddlewareService.json",
    OptInService, "json/OptInService.json",
    BaseDelegator, "json/BaseDelegator.json",
    FullRestakeDelegator, "json/FullRestakeDelegator.json",
    NetworkRestakeDelegator, "json/NetworkRestakeDelegator.json",
    OperatorSpecificDelegator, "json/OperatorSpecificDelegator.json",
    Hints, "json/Hints.json",
    DelegatorHints, "json/BaseDelegatorHints.json",
    OptInServiceHints, "json/OptInServiceHints.json",
    SlasherHints, "json/SlasherHints.json",
    VaultHints, "json/VaultHints.json",
    Vault, "json/Vault.json",
    VaultStorage, "json/VaultStorage.json",
    VaultTokenized, "json/VaultTokenized.json",
);

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    #[test]
    fn test_forge_build_and_json_extraction() {
        // Get the project root directory
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        // Try to find the `forge` executable dynamically
        let forge_executable = match Command::new("which").arg("forge").output() {
            Ok(output) => {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if path.is_empty() {
                    panic!("Forge executable not found. Make sure Foundry is installed.");
                }
                path
            }
            Err(_) => panic!("Failed to locate `forge` executable. Make sure Foundry is installed."),
        };

        // Run forge build in the contracts directory
        let contracts_dir = root.join("contracts");
        let status = Command::new(&forge_executable)
            .current_dir(&contracts_dir)
            .arg("build")
            .status()
            .expect("Failed to execute Forge build");

        assert!(status.success(), "Forge build failed");

        // Create a new directory for JSON files
        let json_dir = root.join("json");
        fs::create_dir_all(&json_dir).expect("Failed to create JSON directory");

        // List of JSON files to process
        let json_files = vec![
            "NetworkRegistry.sol/NetworkRegistry.json",
            "OperatorRegistry.sol/OperatorRegistry.json",
            "SlasherFactory.sol/SlasherFactory.json",
            "VaultConfigurator.sol/VaultConfigurator.json",
            "VaultFactory.sol/VaultFactory.json",
            "BaseSlasher.sol/BaseSlasher.json",
            "Slasher.sol/Slasher.json",
            "VetoSlasher.sol/VetoSlasher.json",
            "MetadataService.sol/MetadataService.json",
            "NetworkMiddlewareService.sol/NetworkMiddlewareService.json",
            "OptInService.sol/OptInService.json",
            "BaseDelegator.sol/BaseDelegator.json",
            "FullRestakeDelegator.sol/FullRestakeDelegator.json",
            "NetworkRestakeDelegator.sol/NetworkRestakeDelegator.json",
            "OperatorSpecificDelegator.sol/OperatorSpecificDelegator.json",
            "Hints.sol/Hints.json",
            "DelegatorHints.sol/BaseDelegatorHints.json",
            "OptInServiceHints.sol/OptInServiceHints.json",
            "SlasherHints.sol/SlasherHints.json",
            "VaultHints.sol/VaultHints.json",
            "Vault.sol/Vault.json",
            "VaultStorage.sol/VaultStorage.json",
            "VaultTokenized.sol/VaultTokenized.json",
        ];

        for json_file in json_files.clone() {
            let source = contracts_dir.join("out").join(json_file);
            let destination = json_dir.join(json_file.split('/').last().unwrap());
            
            // Read the source JSON file
            let json_content = fs::read_to_string(&source)
                .expect(&format!("Failed to read {}", source.display()));
            
            // Parse the JSON content
            let json: serde_json::Value = serde_json::from_str(&json_content)
                .expect(&format!("Failed to parse JSON from {}", source.display()));
            
            // Extract only the "abi" field
            if let Some(abi) = json.get("abi") {
                let abi_only = serde_json::json!({ "abi": abi });
                
                // Write the "abi" only JSON to the destination
                fs::write(&destination, serde_json::to_string_pretty(&abi_only).unwrap())
                    .expect(&format!("Failed to write to {}", destination.display()));
                
                println!("Extracted ABI from {} to {}", source.display(), destination.display());
            } else {
                println!("Warning: No 'abi' field found in {}", source.display());
            }
        }

        // Assert that all expected JSON files were created
        for json_file in json_files {
            let file_path = json_dir.join(json_file.split('/').last().unwrap());
            assert!(file_path.exists(), "Expected JSON file not found: {}", file_path.display());
        }
    }
}
