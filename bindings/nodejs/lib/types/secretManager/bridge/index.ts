import type {
    __GenerateAddressesMethod__,
    __GetLedgerNanoStatusMethod__,
    __SignTransactionMethod__,
    __StoreMnemonicMethod__,
    __SignatureUnlockMethod__,
    __SignEd25519Method__,
} from './secretManager';

export type __SecretManagerMethods__ =
    | __GenerateAddressesMethod__
    | __GetLedgerNanoStatusMethod__
    | __SignTransactionMethod__
    | __SignatureUnlockMethod__
    | __StoreMnemonicMethod__
    | __SignEd25519Method__;
