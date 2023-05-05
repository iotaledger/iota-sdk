---
"iota-sdk-bindings-core": patch
"iota-sdk-nodejs": patch
"iota-sdk-python": patch
---

Remove `IncreaseNativeTokenSupplyOptions`. 
Rename `SendAmount::addressesWithAmount`, `SendNft::addressesAndNftIds`, `SendNativeTokens::addressesAndNativeTokens`, `CreateAliasOutput::aliasOutputOptions`, `MintNfts::nftsOptions`, `MintNativeToken::nativeTokenOptions`, `PrepareOutput::options` to `params`.
Rename `AddressesWithAmount` to `SendAmountParams`.
Rename `AddressesAndNftIds` to `SendNftParams`.
Rename `AddressesAndNativeTokens` to `SendNativeTokensParams`.
Rename `AliasOutputOptions` to `CreateAliasParams`.
Rename `NftOptions` to `MintNftParams`.
Rename `NativeTokenOptions` to `MintNativeTokenParams`.
Rename `OutputOptions` to `OutputParams`.
Rename various `BuilderOptions` to `BuilderParams`.
