// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { SyncOptions, FilterOptions } from '../account';
import type {
    SendAmountParams,
    SendNativeTokensParams,
    SendNftParams,
    GenerateAddressOptions,
} from '../address';
import type {
    BuildAliasOutputData,
    BuildBasicOutputData,
    BuildFoundryOutputData,
    BuildNftOutputData,
} from '../buildOutputData';
import type { INode, IPreparedTransactionData } from '../../client';
import type { OutputParams } from '../outputParams';
import type { OutputsToClaim } from '../output';
import type { SignedTransactionEssence } from '../signedTransactionEssence';
import type {
    AliasOutputParams,
    MintNativeTokenParams,
    TransactionOptions,
    MintNftParams,
} from '../transactionOptions';
import type {
    ParticipationEventId,
    ParticipationEventRegistrationOptions,
    ParticipationEventType,
} from '../participation';
import { HexEncodedAmount, Output } from '../../';
import { Burn } from '../burn';

export type __BuildAliasOutputMethod__ = {
    name: 'buildAliasOutput';
    data: BuildAliasOutputData;
};

export type __BuildBasicOutputMethod__ = {
    name: 'buildBasicOutput';
    data: BuildBasicOutputData;
};

export type __BuildFoundryOutputMethod__ = {
    name: 'buildFoundryOutput';
    data: BuildFoundryOutputData;
};

export type __BuildNftOutputMethod__ = {
    name: 'buildNftOutput';
    data: BuildNftOutputData;
};

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
        outputIdsToClaim: string[];
    };
};

export type __PrepareConsolidateOutputsMethod__ = {
    name: 'prepareConsolidateOutputs';
    data: {
        force: boolean;
        outputConsolidationThreshold?: number;
    };
};

export type __PrepareCreateAliasOutputMethod__ = {
    name: 'prepareCreateAliasOutput';
    data: {
        params?: AliasOutputParams;
        options?: TransactionOptions;
    };
};

export type __PrepareDecreaseNativeTokenSupplyMethod__ = {
    name: 'prepareDecreaseNativeTokenSupply';
    data: {
        tokenId: string;
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

export type __GenerateAddressesMethod__ = {
    name: 'generateAddresses';
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
        transactionId: string;
    };
};

export type __GetOutputMethod__ = {
    name: 'getOutput';
    data: {
        outputId: string;
    };
};

export type __GetFoundryOutputMethod__ = {
    name: 'getFoundryOutput';
    data: {
        tokenId: string;
    };
};

export type __GetOutputsWithAdditionalUnlockConditionsMethod__ = {
    name: 'getOutputsWithAdditionalUnlockConditions';
    data: {
        outputsToClaim: OutputsToClaim;
    };
};

export type __GetTransactionMethod__ = {
    name: 'getTransaction';
    data: {
        transactionId: string;
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

export type __MinimumRequiredStorageDepositMethod__ = {
    name: 'minimumRequiredStorageDeposit';
    data: {
        output: Output;
    };
};

export type __PrepareIncreaseNativeTokenSupplyMethod__ = {
    name: 'prepareIncreaseNativeTokenSupply';
    data: {
        tokenId: string;
        mintAmount: HexEncodedAmount;
        options?: TransactionOptions;
    };
};

export type __PrepareMintNativeTokenMethod__ = {
    name: 'prepareMintNativeToken';
    data: {
        params: MintNativeTokenParams;
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

export type __PrepareSendAmountMethod__ = {
    name: 'prepareSendAmount';
    data: {
        params: SendAmountParams[];
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

export type __RetryTransactionUntilIncludedMethod__ = {
    name: 'retryTransactionUntilIncluded';
    data: {
        transactionId: string;
        interval?: number;
        maxAttempts?: number;
    };
};

export type __SendAmountMethod__ = {
    name: 'sendAmount';
    data: {
        params: SendAmountParams[];
        options?: TransactionOptions;
    };
};

export type __PrepareSendNativeTokensMethod__ = {
    name: 'prepareSendNativeTokens';
    data: {
        params: SendNativeTokensParams[];
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

export type __SignTransactionEssenceMethod__ = {
    name: 'signTransactionEssence';
    data: {
        preparedTransactionData: IPreparedTransactionData;
    };
};

export type __SignAndSubmitTransactionMethod__ = {
    name: 'signAndSubmitTransaction';
    data: {
        preparedTransactionData: IPreparedTransactionData;
    };
};

export type __SubmitAndStoreTransactionMethod__ = {
    name: 'submitAndStoreTransaction';
    data: {
        signedTransactionData: SignedTransactionEssence;
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
        amount: string;
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
        amount: string;
    };
};
