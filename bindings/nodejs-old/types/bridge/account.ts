import type {
    OutputTypes,
    HexEncodedAmount,
    HexEncodedString,
} from '@iota/types';
import type { SyncOptions, FilterOptions } from '../account';
import type {
    SendParams,
    SendNativeTokensParams,
    SendNftParams,
    GenerateAddressOptions,
    GenerateAddressesOptions,
} from '../address';
import type {
    BuildAliasOutputData,
    BuildBasicOutputData,
    BuildFoundryOutputData,
    BuildNftOutputData,
} from '../buildOutputData';
import type { Node } from '../network';
import type { OutputParams } from '../outputParams';
import type { OutputsToClaim } from '../output';
import type { SignedTransactionEssence } from '../signedTransactionEssence';
import type { PreparedTransactionData } from '../preparedTransactionData';
import type {
    AliasOutputParams,
    CreateNativeTokenParams,
    TransactionOptions,
    MintNftParams,
} from '../transactionOptions';
import type {
    ParticipationEventId,
    ParticipationEventRegistrationOptions,
    ParticipationEventType,
} from '../participation';
import type { ConsolidationParams } from '../consolidationParams';
import { Bip44, Ed25519Signature } from '../secretManager';

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

export type __BurnNativeTokenMethod__ = {
    name: 'burnNativeToken';
    data: {
        tokenId: string;
        burnAmount: HexEncodedAmount;
        options?: TransactionOptions;
    };
};

export type __BurnNftMethod__ = {
    name: 'burnNft';
    data: {
        nftId: string;
        options?: TransactionOptions;
    };
};

export type __ClaimOutputsMethod__ = {
    name: 'claimOutputs';
    data: {
        outputIdsToClaim: string[];
    };
};

export type __ConsolidateOutputsMethod__ = {
    name: 'consolidateOutputs';
    data: {
        params: ConsolidationParams;
    };
};

export type __CreateAliasOutputMethod__ = {
    name: 'createAliasOutput';
    data: {
        params?: AliasOutputParams;
        options?: TransactionOptions;
    };
};

export type __MeltNativeTokenMethod__ = {
    name: 'meltNativeToken';
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

export type __DestroyAliasMethod__ = {
    name: 'destroyAlias';
    data: {
        aliasId: string;
        options?: TransactionOptions;
    };
};

export type __DestroyFoundryMethod__ = {
    name: 'destroyFoundry';
    data: {
        foundryId: string;
        options?: TransactionOptions;
    };
};

export type __GenerateEd25519AddressesMethod__ = {
    name: 'generateEd25519Addresses';
    data: {
        amount: number;
        options?: GenerateAddressOptions;
    };
};

export type __GenerateEvmAddressesMethod__ = {
    name: 'generateEvmAddresses';
    data: {
        options: GenerateAddressesOptions;
    };
};

export type __VerifyEd25519SignatureMethod__ = {
    name: 'verifyEd25519Signature';
    data: {
        signature: Ed25519Signature;
        message: HexEncodedString;
    };
};

export type __VerifySecp256k1EcdsaSignatureMethod__ = {
    name: 'verifySecp256k1EcdsaSignature';
    data: {
        publicKey: HexEncodedString;
        signature: HexEncodedString;
        message: HexEncodedString;
    };
};

export type __SignSecp256k1EcdsaMethod__ = {
    name: 'signSecp256k1Ecdsa';
    data: {
        message: HexEncodedString;
        chain: Bip44;
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

export type __ClaimableOutputsMethod__ = {
    name: 'claimableOutputs';
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
        output: OutputTypes;
    };
};

export type __MintNativeTokenMethod__ = {
    name: 'mintNativeToken';
    data: {
        tokenId: string;
        mintAmount: HexEncodedAmount;
        options?: TransactionOptions;
    };
};

export type __CreateNativeTokenMethod__ = {
    name: 'createNativeToken';
    data: {
        params: CreateNativeTokenParams;
        options?: TransactionOptions;
    };
};

export type __MintNftsMethod__ = {
    name: 'mintNfts';
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
        outputs: OutputTypes[];
        options?: TransactionOptions;
    };
};

export type __RegisterParticipationEventsMethod__ = {
    name: 'registerParticipationEvents';
    data: {
        options: ParticipationEventRegistrationOptions;
    };
};

export type __RequestFundsFromFaucetMethod__ = {
    name: 'requestFundsFromFaucet';
    data: {
        url: string;
        address: string;
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

export type __SendMethod__ = {
    name: 'send';
    data: {
        params: SendParams[];
        options?: TransactionOptions;
    };
};

export type __SendNativeTokensMethod__ = {
    name: 'sendNativeTokens';
    data: {
        params: SendNativeTokensParams[];
        options?: TransactionOptions;
    };
};

export type __SendNftMethod__ = {
    name: 'sendNft';
    data: {
        params: SendNftParams[];
        options?: TransactionOptions;
    };
};

export type __SendOutputsMethod__ = {
    name: 'sendOutputs';
    data: {
        outputs: OutputTypes[];
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
        preparedTransactionData: PreparedTransactionData;
    };
};

export type __SubmitAndStoreTransactionMethod__ = {
    name: 'submitAndStoreTransaction';
    data: {
        signedTransactionData: SignedTransactionEssence;
    };
};

export type __SyncAccountMethod__ = {
    name: 'syncAccount';
    data: {
        options?: SyncOptions;
    };
};

export type __VoteMethod__ = {
    name: 'vote';
    data: {
        eventId?: ParticipationEventId;
        answers?: number[];
    };
};

export type __StopParticipatingMethod__ = {
    name: 'stopParticipating';
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

export type __IncreaseVotingPowerMethod__ = {
    name: 'increaseVotingPower';
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
        node: Node;
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

export type __DecreaseVotingPowerMethod__ = {
    name: 'decreaseVotingPower';
    data: {
        amount: string;
    };
};
