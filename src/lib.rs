use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    DelegatorFactory,
    "contracts/out/DelegatorFactory.sol/DelegatorFactory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    NetworkRegistry,
    "contracts/out/NetworkRegistry.sol/NetworkRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    OperatorRegistry,
    "contracts/out/OperatorRegistry.sol/OperatorRegistry.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SlasherFactory,
    "contracts/out/SlasherFactory.sol/SlasherFactory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    VaultConfigurator,
    "contracts/out/VaultConfigurator.sol/VaultConfigurator.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    VaultFactory,
    "contracts/out/VaultFactory.sol/VaultFactory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    BaseSlasher,
    "contracts/out/BaseSlasher.sol/BaseSlasher.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    Slasher,
    "contracts/out/Slasher.sol/Slasher.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    VetoSlasher,
    "contracts/out/VetoSlasher.sol/VetoSlasher.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    MetadataService,
    "contracts/out/MetadataService.sol/MetadataService.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    NetworkMiddlewareService,
    "contracts/out/NetworkMiddlewareService.sol/NetworkMiddlewareService.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    OptInService,
    "contracts/out/OptInService.sol/OptInService.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    BaseDelegator,
    "contracts/out/BaseDelegator.sol/BaseDelegator.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    FullRestakeDelegator,
    "contracts/out/FullRestakeDelegator.sol/FullRestakeDelegator.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    NetworkRestakeDelegator,
    "contracts/out/NetworkRestakeDelegator.sol/NetworkRestakeDelegator.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    OperatorSpecificDelegator,
    "contracts/out/OperatorSpecificDelegator.sol/OperatorSpecificDelegator.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    Hints,
    "contracts/out/Hints.sol/Hints.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    DelegatorHints,
    "contracts/out/DelegatorHints.sol/BaseDelegatorHints.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    OptInServiceHints,
    "contracts/out/OptInServiceHints.sol/OptInServiceHints.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    SlasherHints,
    "contracts/out/SlasherHints.sol/SlasherHints.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    VaultHints,
    "contracts/out/VaultHints.sol/VaultHints.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    Vault,
    "contracts/out/Vault.sol/Vault.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    VaultStorage,
    "contracts/out/VaultStorage.sol/VaultStorage.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    VaultTokenized,
    "contracts/out/VaultTokenized.sol/VaultTokenized.json"
);
