import type {
    __GenerateEd25519AddressesMethod__,
    __GenerateEvmAddressesMethod__,
    __GetLedgerNanoStatusMethod__,
    __SignTransactionMethod__,
    __SignBlockMethod__,
    __StoreMnemonicMethod__,
    __SignatureUnlockMethod__,
    __SignEd25519Method__,
    __SignSecp256k1EcdsaMethod__,
    __SetStrongholdPasswordMethod__,
    __ChangeStrongholdPasswordMethod__,
    __ClearStrongholdPasswordMethod__,
} from './secret-manager';

export type __SecretManagerMethods__ =
    | __GenerateEd25519AddressesMethod__
    | __GenerateEvmAddressesMethod__
    | __GetLedgerNanoStatusMethod__
    | __SignTransactionMethod__
    | __SignBlockMethod__
    | __SignatureUnlockMethod__
    | __StoreMnemonicMethod__
    | __SignEd25519Method__
    | __SignSecp256k1EcdsaMethod__
    | __SetStrongholdPasswordMethod__
    | __ChangeStrongholdPasswordMethod__
    | __ClearStrongholdPasswordMethod__;
