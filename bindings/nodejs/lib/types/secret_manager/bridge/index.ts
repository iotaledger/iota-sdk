import type {
    __GenerateEd25519AddressesMethod__,
    __GenerateEvmAddressesMethod__,
    __GetLedgerNanoStatusMethod__,
    __SignTransactionMethod__,
    __StoreMnemonicMethod__,
    __SignatureUnlockMethod__,
    __SignEd25519Method__,
    __SignEvmMethod__,
    __VerifyEd25519Method__,
    __VerifyEvmMethod__,
} from './secret-manager';

export type __SecretManagerMethods__ =
    | __GenerateEd25519AddressesMethod__
    | __GenerateEvmAddressesMethod__
    | __GetLedgerNanoStatusMethod__
    | __SignTransactionMethod__
    | __SignatureUnlockMethod__
    | __StoreMnemonicMethod__
    | __SignEd25519Method__
    | __SignEvmMethod__
    | __VerifyEd25519Method__
    | __VerifyEvmMethod__;
