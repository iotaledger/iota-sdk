import type { AccountId } from '../account';
import type {
    __BuildAliasOutputMethod__,
    __BuildBasicOutputMethod__,
    __BuildFoundryOutputMethod__,
    __BuildNftOutputMethod__,
    __PrepareBurnMethod__,
    __PrepareClaimOutputsMethod__,
    __ClaimOutputsMethod__,
    __PrepareConsolidateOutputsMethod__,
    __PrepareCreateAliasOutputMethod__,
    __DeregisterParticipationEventMethod__,
    __GenerateEd25519AddressesMethod__,
    __GetBalanceMethod__,
    __GetOutputMethod__,
    __GetFoundryOutputMethod__,
    __ClaimableOutputsMethod__,
    __GetTransactionMethod__,
    __AddressesMethod__,
    __AddressesWithUnspentOutputsMethod__,
    __OutputsMethod__,
    __PendingTransactionsMethod__,
    __IncomingTransactionsMethod__,
    __TransactionsMethod__,
    __UnspentOutputsMethod__,
    __PrepareCreateNativeTokenMethod__,
    __PrepareMeltNativeTokenMethod__,
    __PrepareMintNativeTokenMethod__,
    __PrepareMintNftsMethod__,
    __PrepareOutputMethod__,
    __PrepareSendMethod__,
    __PrepareTransactionMethod__,
    __RegisterParticipationEventsMethod__,
    __RetryTransactionUntilIncludedMethod__,
    __SendMethod__,
    __SendWithParamsMethod__,
    __PrepareSendNativeTokensMethod__,
    __PrepareSendNftMethod__,
    __SendOutputsMethod__,
    __SetAliasMethod__,
    __SetDefaultSyncOptionsMethod__,
    __SignTransactionEssenceMethod__,
    __SignAndSubmitTransactionMethod__,
    __SubmitAndStoreTransactionMethod__,
    __SyncAccountMethod__,
    __PrepareVoteMethod__,
    __GetIncomingTransactionMethod__,
    __GetParticipationOverviewMethod__,
    __GetParticipationEventMethod__,
    __GetParticipationEventsMethod__,
    __GetParticipationEventStatusMethod__,
    __GetParticipationEventIdsMethod__,
    __PrepareIncreaseVotingPowerMethod__,
    __PrepareDecreaseVotingPowerMethod__,
    __PrepareStopParticipatingMethod__,
} from './account';
import type {
    __BackupMethod__,
    __ChangeStrongholdPasswordMethod__,
    __ClearStrongholdPasswordMethod__,
    __ClearListenersMethod__,
    __CreateAccountMethod__,
    __EmitTestEventMethod__,
    __GenerateMnemonicMethod__,
    __GetAccountMethod__,
    __GetAccountIndexesMethod__,
    __GetAccountsMethod__,
    __GetChrysalisDataMethod__,
    __GetLedgerNanoStatusMethod__,
    __GenerateEd25519AddressMethod__,
    __IsStrongholdPasswordAvailableMethod__,
    __RecoverAccountsMethod__,
    __RemoveLatestAccountMethod__,
    __RestoreBackupMethod__,
    __SetClientOptionsMethod__,
    __SetStrongholdPasswordClearIntervalMethod__,
    __SetStrongholdPasswordMethod__,
    __StartBackgroundSyncMethod__,
    __StopBackgroundSyncMethod__,
    __StoreMnemonicMethod__,
    __UpdateNodeAuthMethod__,
} from './wallet';

export type __AccountMethod__ =
    | __BuildAliasOutputMethod__
    | __BuildBasicOutputMethod__
    | __BuildFoundryOutputMethod__
    | __BuildNftOutputMethod__
    | __PrepareBurnMethod__
    | __ClaimOutputsMethod__
    | __PrepareClaimOutputsMethod__
    | __PrepareConsolidateOutputsMethod__
    | __PrepareCreateAliasOutputMethod__
    | __DeregisterParticipationEventMethod__
    | __GenerateEd25519AddressesMethod__
    | __GetBalanceMethod__
    | __GetOutputMethod__
    | __GetIncomingTransactionMethod__
    | __GetFoundryOutputMethod__
    | __ClaimableOutputsMethod__
    | __GetParticipationEventMethod__
    | __GetParticipationEventIdsMethod__
    | __GetParticipationEventsMethod__
    | __GetParticipationEventStatusMethod__
    | __GetTransactionMethod__
    | __AddressesMethod__
    | __AddressesWithUnspentOutputsMethod__
    | __OutputsMethod__
    | __PendingTransactionsMethod__
    | __IncomingTransactionsMethod__
    | __TransactionsMethod__
    | __UnspentOutputsMethod__
    | __PrepareCreateNativeTokenMethod__
    | __PrepareMeltNativeTokenMethod__
    | __PrepareMintNativeTokenMethod__
    | __PrepareMintNftsMethod__
    | __PrepareOutputMethod__
    | __PrepareSendMethod__
    | __PrepareTransactionMethod__
    | __RegisterParticipationEventsMethod__
    | __RetryTransactionUntilIncludedMethod__
    | __SendMethod__
    | __SendWithParamsMethod__
    | __PrepareSendNativeTokensMethod__
    | __PrepareSendNftMethod__
    | __SendOutputsMethod__
    | __SetAliasMethod__
    | __SetDefaultSyncOptionsMethod__
    | __SignTransactionEssenceMethod__
    | __SignAndSubmitTransactionMethod__
    | __SubmitAndStoreTransactionMethod__
    | __SyncAccountMethod__
    | __PrepareVoteMethod__
    | __PrepareStopParticipatingMethod__
    | __GetParticipationOverviewMethod__
    | __PrepareIncreaseVotingPowerMethod__
    | __PrepareDecreaseVotingPowerMethod__;

export type __CallAccountMethodMethod__ = {
    name: 'callAccountMethod';
    data: {
        accountId: AccountId;
        method: __AccountMethod__;
    };
};

export type __Method__ =
    | __BackupMethod__
    | __CallAccountMethodMethod__
    | __ChangeStrongholdPasswordMethod__
    | __ClearListenersMethod__
    | __ClearStrongholdPasswordMethod__
    | __CreateAccountMethod__
    | __EmitTestEventMethod__
    | __GenerateMnemonicMethod__
    | __GetAccountMethod__
    | __GetAccountIndexesMethod__
    | __GetAccountsMethod__
    | __GetChrysalisDataMethod__
    | __GetLedgerNanoStatusMethod__
    | __GenerateEd25519AddressMethod__
    | __IsStrongholdPasswordAvailableMethod__
    | __RecoverAccountsMethod__
    | __RemoveLatestAccountMethod__
    | __RestoreBackupMethod__
    | __SetClientOptionsMethod__
    | __SetStrongholdPasswordClearIntervalMethod__
    | __SetStrongholdPasswordMethod__
    | __StartBackgroundSyncMethod__
    | __StopBackgroundSyncMethod__
    | __StoreMnemonicMethod__
    | __UpdateNodeAuthMethod__;
