---
"wallet-nodejs-binding": patch
---

Rename `Account::mintNativeToken` to `createNativeToken`, `Account::increaseNativeTokenSupply` to `mintNativeToken`, `Account::decreaseNativeTokenSupply` to `meltNativeToken`;
Rename `MintNativeTokenParams` to `CreateNativeTokenParams`;
Rename `MintTokenTransaction` to `CreateNativeTokenTransaction` and `PreparedMintTokenTransaction` to `PreparedCreateNativeTokenTransaction` (including their corresponding `Data` types);
