import type { SyncOptions, FilterOptions } from '../wallet';
import type { WalletEventType, WalletEvent } from '../event';
import type {
    Auth,
    ClientOptions,
    Burn,
    Node,
    PreparedTransactionData,
} from '../../client';
import type {
    SendParams,
    SendNativeTokenParams,
    SendNftParams,
    SendManaParams,
} from '../address';
import type { OutputParams } from '../output-params';
import type { OutputsToClaim } from '../output';
import type { SignedTransactionData } from '../signed-transaction-data';
import type {
    AccountOutputParams,
    CreateNativeTokenParams,
    TransactionOptions,
    MintNftParams,
    CreateDelegationParams,
    BeginStakingParams,
} from '../transaction-options';
import type {
    ParticipationEventId,
    ParticipationEventRegistrationOptions,
    ParticipationEventType,
} from '../participation';
import type { ConsolidationParams } from '../consolidation-params';
import {
    AccountId,
    DelegationId,
    HexEncodedAmount,
    NumericString,
    Output,
    OutputId,
    TokenId,
    TransactionId,
} from '../../';

export type __AccountsMethod__ = {
    name: 'accounts';
};

export type __AnnounceCandidacyMethod__ = {
    name: 'announceCandidacy';
    data: {
        accountId: AccountId;
    };
};

export type __BackupToStrongholdSnapshotMethod__ = {
    name: 'backupToStrongholdSnapshot';
    data: {
        destination: string;
        password: string;
    };
};

export type __ChangeStrongholdPasswordMethod__ = {
    name: 'changeStrongholdPassword';
    data: {
        currentPassword: string;
        newPassword: string;
    };
};

export type __ClearStrongholdPasswordMethod__ = {
    name: 'clearStrongholdPassword';
};

export type __ClearListenersMethod__ = {
    name: 'clearListeners';
    data: { eventTypes: WalletEventType[] };
};

export type __EmitTestEventMethod__ = {
    name: 'emitTestEvent';
    data: { event: WalletEvent };
};

export type __GenerateMnemonicMethod__ = {
    name: 'generateMnemonic';
};

export type __GetLedgerNanoStatusMethod__ = {
    name: 'getLedgerNanoStatus';
};

export type __IsStrongholdPasswordAvailableMethod__ = {
    name: 'isStrongholdPasswordAvailable';
};

export type __RestoreFromStrongholdSnapshotMethod__ = {
    name: 'restoreFromStrongholdSnapshot';
    data: {
        source: string;
        password: string;
        ignoreIfCoinTypeMismatch?: boolean;
        ignoreIfBech32Mismatch?: string;
    };
};

export type __SetClientOptionsMethod__ = {
    name: 'setClientOptions';
    data: { clientOptions: ClientOptions };
};

export type __SetStrongholdPasswordMethod__ = {
    name: 'setStrongholdPassword';
    data: { password: string };
};

export type __SetStrongholdPasswordClearIntervalMethod__ = {
    name: 'setStrongholdPasswordClearInterval';
    data?: { intervalInMilliseconds?: number };
};

export type __StartBackgroundSyncMethod__ = {
    name: 'startBackgroundSync';
    data: {
        options?: SyncOptions;
        intervalInMilliseconds?: number;
    };
};

export type __StopBackgroundSyncMethod__ = {
    name: 'stopBackgroundSync';
};

export type __StoreMnemonicMethod__ = {
    name: 'storeMnemonic';
    data: { mnemonic: string };
};

export type __UpdateNodeAuthMethod__ = {
    name: 'updateNodeAuth';
    data: { url: string; auth?: Auth };
};

export type __PrepareBurnMethod__ = {
    name: 'prepareBurn';
    data: {
        burn: Burn;
        options?: TransactionOptions;
    };
};

export type __PrepareClaimOutputsMethod__ = {
    name: 'prepareClaimOutputs';
    data: {
        outputIdsToClaim: OutputId[];
        options?: TransactionOptions;
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

export type __PrepareCreateDelegationMethod__ = {
    name: 'prepareCreateDelegation';
    data: {
        params: CreateDelegationParams;
        options?: TransactionOptions;
    };
};

export type __PrepareDelayDelegationClaimingMethod__ = {
    name: 'prepareDelayDelegationClaiming';
    data: {
        delegationId: DelegationId;
        reclaimExcess: boolean;
    };
};

export type __PrepareBeginStakingMethod__ = {
    name: 'prepareBeginStaking';
    data: {
        params: BeginStakingParams;
        options?: TransactionOptions;
    };
};

export type __PrepareExtendStakingMethod__ = {
    name: 'prepareExtendStaking';
    data: {
        accountId: AccountId;
        additionalEpochs: number;
    };
};

export type __PrepareEndStakingMethod__ = {
    name: 'prepareEndStaking';
    data: {
        accountId: AccountId;
    };
};

export type __DeregisterParticipationEventMethod__ = {
    name: 'deregisterParticipationEvent';
    data: {
        eventId: ParticipationEventId;
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

export type __GetAddressMethod__ = {
    name: 'getAddress';
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

export type __PrepareImplicitAccountTransitionMethod__ = {
    name: 'prepareImplicitAccountTransition';
    data: {
        outputId: OutputId;
    };
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

export type __PrepareSendOutputsMethod__ = {
    name: 'prepareSendOutputs';
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

export type __WaitForTransactionAcceptanceMethod__ = {
    name: 'waitForTransactionAcceptance';
    data: {
        transactionId: TransactionId;
        interval?: number;
        maxAttempts?: number;
    };
};

export type __PrepareSendMethod__ = {
    name: 'prepareSend';
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

export type __PrepareSendManaMethod__ = {
    name: 'prepareSendMana';
    data: {
        params: SendManaParams;
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

export type __PrepareDecreaseVotingPowerMethod__ = {
    name: 'prepareDecreaseVotingPower';
    data: {
        amount: NumericString;
    };
};
