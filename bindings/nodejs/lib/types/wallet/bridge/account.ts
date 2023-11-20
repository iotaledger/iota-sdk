// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type {
    SendParams,
    SendNativeTokenParams,
    SendNftParams,
    GenerateAddressOptions,
} from '../address';
import type { Burn, INode, PreparedTransactionData } from '../../client';
import type { OutputParams } from '../output-params';
import type { OutputsToClaim } from '../output';
import type { SignedTransactionData } from '../signed-transaction-data';
import type {
    AccountOutputParams,
    CreateNativeTokenParams,
    TransactionOptions,
    MintNftParams,
} from '../transaction-options';
import type {
    ParticipationEventId,
    ParticipationEventRegistrationOptions,
    ParticipationEventType,
} from '../participation';
import type { ConsolidationParams } from '../consolidation-params';
import {
    FilterOptions,
    HexEncodedAmount,
    NumericString,
    Output,
    OutputId,
    SyncOptions,
    TokenId,
    TransactionId,
} from '../../';

export type __PrepareBurnMethod__ = {
    name: 'prepareBurn';
    data: {
        burn: Burn;
        options?: TransactionOptions;
    };
};

export type __ClaimOutputsMethod__ = {
    name: 'claimOutputs';
    data: {
        outputIdsToClaim: OutputId[];
    };
};

export type __PrepareConsolidateOutputsMethod__ = {
    name: 'prepareConsolidateOutputs';
    data: {
        params: ConsolidationParams;
    };
};

export type __PrepareCreateAccountOutputMethod__ = {
    name: 'prepareCreateAccountOutput';
    data: {
        params?: AccountOutputParams;
        options?: TransactionOptions;
    };
};

export type __PrepareMeltNativeTokenMethod__ = {
    name: 'prepareMeltNativeToken';
    data: {
        tokenId: TokenId;
        meltAmount: HexEncodedAmount;
        options?: TransactionOptions;
    };
};

export type __DeregisterParticipationEventMethod__ = {
    name: 'deregisterParticipationEvent';
    data: {
        eventId: ParticipationEventId;
    };
};

export type __GenerateEd25519AddressesMethod__ = {
    name: 'generateEd25519Addresses';
    data: {
        amount: number;
        options?: GenerateAddressOptions;
    };
};

export type __GetBalanceMethod__ = {
    name: 'getBalance';
};

export type __GetIncomingTransactionMethod__ = {
    name: 'getIncomingTransaction';
    data: {
        transactionId: TransactionId;
    };
};

export type __GetOutputMethod__ = {
    name: 'getOutput';
    data: {
        outputId: OutputId;
    };
};

export type __GetFoundryOutputMethod__ = {
    name: 'getFoundryOutput';
    data: {
        tokenId: TokenId;
    };
};

export type __ClaimableOutputsMethod__ = {
    name: 'claimableOutputs';
    data: {
        outputsToClaim: OutputsToClaim;
    };
};

export type __GetTransactionMethod__ = {
    name: 'getTransaction';
    data: {
        transactionId: TransactionId;
    };
};

export type __AddressesMethod__ = {
    name: 'addresses';
};

export type __AddressesWithUnspentOutputsMethod__ = {
    name: 'addressesWithUnspentOutputs';
};

export type __OutputsMethod__ = {
    name: 'outputs';
    data: {
        filterOptions?: FilterOptions;
    };
};

export type __PendingTransactionsMethod__ = {
    name: 'pendingTransactions';
};

export type __ImplicitAccountCreationAddressMethod__ = {
    name: 'implicitAccountCreationAddress';
};

export type __AccountsMethod__ = {
    name: 'accounts';
};

export type __ImplicitAccountsMethod__ = {
    name: 'implicitAccounts';
};

export type __IncomingTransactionsMethod__ = {
    name: 'incomingTransactions';
};

export type __TransactionsMethod__ = {
    name: 'transactions';
};

export type __UnspentOutputsMethod__ = {
    name: 'unspentOutputs';
    data: {
        filterOptions?: FilterOptions;
    };
};

export type __PrepareMintNativeTokenMethod__ = {
    name: 'prepareMintNativeToken';
    data: {
        tokenId: TokenId;
        mintAmount: HexEncodedAmount;
        options?: TransactionOptions;
    };
};

export type __PrepareCreateNativeTokenMethod__ = {
    name: 'prepareCreateNativeToken';
    data: {
        params: CreateNativeTokenParams;
        options?: TransactionOptions;
    };
};

export type __PrepareMintNftsMethod__ = {
    name: 'prepareMintNfts';
    data: {
        params: MintNftParams[];
        options?: TransactionOptions;
    };
};

export type __PrepareOutputMethod__ = {
    name: 'prepareOutput';
    data: {
        params: OutputParams;
        transactionOptions?: TransactionOptions;
    };
};

export type __PrepareSendMethod__ = {
    name: 'prepareSend';
    data: {
        params: SendParams[];
        options?: TransactionOptions;
    };
};

export type __PrepareTransactionMethod__ = {
    name: 'prepareTransaction';
    data: {
        outputs: Output[];
        options?: TransactionOptions;
    };
};

export type __RegisterParticipationEventsMethod__ = {
    name: 'registerParticipationEvents';
    data: {
        options: ParticipationEventRegistrationOptions;
    };
};

export type __ReissueTransactionUntilIncludedMethod__ = {
    name: 'reissueTransactionUntilIncluded';
    data: {
        transactionId: TransactionId;
        interval?: number;
        maxAttempts?: number;
    };
};

export type __SendMethod__ = {
    name: 'send';
    data: {
        amount: NumericString;
        address: string;
        options?: TransactionOptions;
    };
};

export type __SendWithParamsMethod__ = {
    name: 'sendWithParams';
    data: {
        params: SendParams[];
        options?: TransactionOptions;
    };
};

export type __PrepareSendNativeTokensMethod__ = {
    name: 'prepareSendNativeTokens';
    data: {
        params: SendNativeTokenParams[];
        options?: TransactionOptions;
    };
};

export type __PrepareSendNftMethod__ = {
    name: 'prepareSendNft';
    data: {
        params: SendNftParams[];
        options?: TransactionOptions;
    };
};

export type __SendOutputsMethod__ = {
    name: 'sendOutputs';
    data: {
        outputs: Output[];
        options?: TransactionOptions;
    };
};

export type __SetAliasMethod__ = {
    name: 'setAlias';
    data: {
        alias: string;
    };
};

export type __SetDefaultSyncOptionsMethod__ = {
    name: 'setDefaultSyncOptions';
    data: {
        options: SyncOptions;
    };
};

export type __SignTransactionMethod__ = {
    name: 'signTransaction';
    data: {
        preparedTransactionData: PreparedTransactionData;
    };
};

export type __SignAndSubmitTransactionMethod__ = {
    name: 'signAndSubmitTransaction';
    data: {
        preparedTransactionData: PreparedTransactionData;
    };
};

export type __SubmitAndStoreTransactionMethod__ = {
    name: 'submitAndStoreTransaction';
    data: {
        signedTransactionData: SignedTransactionData;
    };
};

export type __SyncAccountMethod__ = {
    name: 'sync';
    data: {
        options?: SyncOptions;
    };
};

export type __PrepareVoteMethod__ = {
    name: 'prepareVote';
    data: {
        eventId?: ParticipationEventId;
        answers?: number[];
    };
};

export type __PrepareStopParticipatingMethod__ = {
    name: 'prepareStopParticipating';
    data: {
        eventId: ParticipationEventId;
    };
};

export type __GetParticipationOverviewMethod__ = {
    name: 'getParticipationOverview';
    data: {
        eventIds?: ParticipationEventId[];
    };
};

export type __PrepareIncreaseVotingPowerMethod__ = {
    name: 'prepareIncreaseVotingPower';
    data: {
        amount: NumericString;
    };
};

export type __GetParticipationEventMethod__ = {
    name: 'getParticipationEvent';
    data: {
        eventId: ParticipationEventId;
    };
};

export type __GetParticipationEventIdsMethod__ = {
    name: 'getParticipationEventIds';
    data: {
        node: INode;
        eventType?: ParticipationEventType;
    };
};

export type __GetParticipationEventsMethod__ = {
    name: 'getParticipationEvents';
};

export type __GetParticipationEventStatusMethod__ = {
    name: 'getParticipationEventStatus';
    data: {
        eventId: ParticipationEventId;
    };
};

export type __PrepareDecreaseVotingPowerMethod__ = {
    name: 'prepareDecreaseVotingPower';
    data: {
        amount: NumericString;
    };
};
