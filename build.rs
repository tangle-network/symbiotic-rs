use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/cli");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/main.rs");

    let contract_dirs: Vec<&str> = vec!["./contracts"];

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

    for dir in contract_dirs {
        let full_path = root.join(dir).canonicalize().unwrap_or_else(|_| {
            println!(
                "Directory not found or inaccessible: {}",
                root.join(dir).display()
            );
            root.join(dir)
        });

        if full_path.exists() {
            println!("cargo:rerun-if-changed={}", full_path.display());

            let status = Command::new(&forge_executable)
                .current_dir(&full_path)
                .arg("build")
                .status()
                .expect("Failed to execute Forge build");

            if !status.success() {
                panic!("Forge build failed for directory: {}", full_path.display());
            }
        } else {
            println!(
                "Directory not found or does not exist: {}",
                full_path.display()
            );
        }
    }

    // Create a new directory for JSON files
    let json_dir = root.join("json");
    fs::create_dir_all(&json_dir).expect("Failed to create JSON directory");

    // List of JSON files to move
    let json_files = vec![
        "contracts/out/NetworkRegistry.sol/NetworkRegistry.json",
        "contracts/out/OperatorRegistry.sol/OperatorRegistry.json",
        "contracts/out/SlasherFactory.sol/SlasherFactory.json",
        "contracts/out/VaultConfigurator.sol/VaultConfigurator.json",
        "contracts/out/VaultFactory.sol/VaultFactory.json",
        "contracts/out/BaseSlasher.sol/BaseSlasher.json",
        "contracts/out/Slasher.sol/Slasher.json",
        "contracts/out/VetoSlasher.sol/VetoSlasher.json",
        "contracts/out/MetadataService.sol/MetadataService.json",
        "contracts/out/NetworkMiddlewareService.sol/NetworkMiddlewareService.json",
        "contracts/out/OptInService.sol/OptInService.json",
        "contracts/out/BaseDelegator.sol/BaseDelegator.json",
        "contracts/out/FullRestakeDelegator.sol/FullRestakeDelegator.json",
        "contracts/out/NetworkRestakeDelegator.sol/NetworkRestakeDelegator.json",
        "contracts/out/OperatorSpecificDelegator.sol/OperatorSpecificDelegator.json",
        "contracts/out/Hints.sol/Hints.json",
        "contracts/out/DelegatorHints.sol/BaseDelegatorHints.json",
        "contracts/out/OptInServiceHints.sol/OptInServiceHints.json",
        "contracts/out/SlasherHints.sol/SlasherHints.json",
        "contracts/out/VaultHints.sol/VaultHints.json",
        "contracts/out/Vault.sol/Vault.json",
        "contracts/out/VaultStorage.sol/VaultStorage.json",
        "contracts/out/VaultTokenized.sol/VaultTokenized.json",
    ];

    for json_file in json_files {
        let source = root.join(json_file);
        let destination = root.join("json").join(json_file.split('/').last().unwrap());
        
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
            println!("No 'abi' field found in {}", source.display());
        }
    }

    // Update cargo to rerun if any JSON file changes
    println!("cargo:rerun-if-changed={}", json_dir.display());
}
