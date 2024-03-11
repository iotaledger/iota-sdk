// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

export class ErrorBase<T extends string> extends Error {
    name: T;
    message: string;
    cause?: any;

    constructor({
        name,
        message,
        cause,
    }: {
        name: T;
        message: string;
        cause?: any;
    }) {
        super();
        this.name = name;
        this.message = message;
        this.cause = cause;
    }
}

export type ClientErrorName =
    | 'bech32HrpMismatch'
    | 'blake2b256'
    | 'block'
    | 'crypto'
    | 'inputAddressNotFound'
    | 'invalidAmount'
    | 'invalidMnemonic'
    | 'json'
    | 'missingParameter'
    | 'node'
    | 'noOutput'
    | 'placeholderSecretManager'
    | 'poisonError'
    | 'prefixHex'
    | 'quorumPoolSizeError'
    | 'quorumThresholdError'
    | 'secretManagerMismatch'
    | 'healthyNodePoolEmpty'
    | 'taggedData'
    | 'transactionAcceptance'
    | 'taskJoin'
    | 'timeNotSynced'
    | 'transactionSemantic'
    | 'unpack'
    | 'urlAuth'
    | 'url'
    | 'urlValidation'
    | 'inputSelection'
    | 'missingBip32Chain'
    | 'unexpectedBlockBodyKind'
    | 'missingTransactionPayload'
    | 'expirationDeadzone'
    | 'participation'
    | 'ledger'
    | 'mqtt'
    | 'stronghold';

export class ClientError extends ErrorBase<ClientErrorName> { }

export type WalletErrorName =
    | 'backup'
    | 'block'
    | 'burningOrMeltingFailed'
    | 'client'
    | 'bipPathMismatch'
    | 'crypto'
    | 'customInput'
    | 'insufficientFunds'
    | 'invalidEventType'
    | 'invalidMnemonic'
    | 'invalidOutputKind'
    | 'invalidVotingPower'
    | 'io'
    | 'json'
    | 'migration'
    | 'mintingFailed'
    | 'missingBipPath'
    | 'missingParameter'
    | 'nftNotFoundInUnspentOutputs'
    | 'noOutputsToConsolidate'
    | 'other'
    | 'participation'
    | 'storage'
    | 'storageIsEncrypted'
    | 'taskJoin'
    | 'transactionNotFound'
    | 'voting'
    | 'walletAddressMismatch'
    | 'nonEd25519Address'
    | 'implicitAccountNotFound'
    | 'accountNotFound';

export class WalletError extends ErrorBase<WalletErrorName> { }
export class BlockError extends ErrorBase<'block'> {
    constructor(message: string) {
        super({ name: 'block', message });
    }
}
export class PrefixHexError extends ErrorBase<'prefixHex'> {
    constructor(message: string) {
        super({ name: 'prefixHex', message });
    }
}
export class SerdeJsonError extends ErrorBase<'serdeJson'> {
    constructor(message: string) {
        super({ name: 'serdeJson', message });
    }
}
export class UnpackError extends ErrorBase<'unpack'> {
    constructor(message: string) {
        super({ name: 'unpack', message });
    }
}

export type Errors =
    | BlockError
    | ClientError
    | WalletError
    | PrefixHexError
    | SerdeJsonError
    | UnpackError;
