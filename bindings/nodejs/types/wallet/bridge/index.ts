import type { AccountId } from '../account';
import type {
    __BuildAliasOutputMethod__,
    __BuildBasicOutputMethod__,
    __BuildFoundryOutputMethod__,
    __BuildNftOutputMethod__,
    __BurnNativeTokenMethod__,
    __BurnNftMethod__,
    __ClaimOutputsMethod__,
    __ConsolidateOutputsMethod__,
    __CreateAliasOutputMethod__,
    __DecreaseNativeTokenSupplyMethod__,
    __DestroyAliasMethod__,
    __DestroyFoundryMethod__,
    __DeregisterParticipationEventMethod__,
    __GenerateAddressesMethod__,
    __GetBalanceMethod__,
    __GetOutputMethod__,
    __GetFoundryOutputMethod__,
    __GetOutputsWithAdditionalUnlockConditionsMethod__,
    __GetTransactionMethod__,
    __AddressesMethod__,
    __AddressesWithUnspentOutputsMethod__,
    __OutputsMethod__,
    __PendingTransactionsMethod__,
    __IncomingTransactionsMethod__,
    __TransactionsMethod__,
    __UnspentOutputsMethod__,
    __MinimumRequiredStorageDepositMethod__,
    __IncreaseNativeTokenSupplyMethod__,
    __MintNativeTokenMethod__,
    __MintNftsMethod__,
    __PrepareOutputMethod__,
    __PrepareSendAmountMethod__,
    __PrepareTransactionMethod__,
    __RegisterParticipationEventsMethod__,
    __RequestFundsFromFaucetMethod__,
    __RetryTransactionUntilIncludedMethod__,
    __SendAmountMethod__,
    __SendNativeTokensMethod__,
    __SendNftMethod__,
    __SendOutputsMethod__,
    __SetAliasMethod__,
    __SetDefaultSyncOptionsMethod__,
    __SignTransactionEssenceMethod__,
    __SubmitAndStoreTransactionMethod__,
    __SyncAccountMethod__,
    __GetIncomingTransactionDataMethod__,
    __VoteMethod__,
    __GetParticipationOverviewMethod__,
    __GetParticipationEventMethod__,
    __GetParticipationEventsMethod__,
    __GetParticipationEventStatusMethod__,
    __GetParticipationEventIdsMethod__,
    __IncreaseVotingPowerMethod__,
    __DecreaseVotingPowerMethod__,
    __StopParticipatingMethod__,
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
    __GetLedgerNanoStatusMethod__,
    __GenerateAddressMethod__,
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
    | __BurnNativeTokenMethod__
    | __BurnNftMethod__
    | __ClaimOutputsMethod__
    | __ConsolidateOutputsMethod__
    | __CreateAliasOutputMethod__
    | __DeregisterParticipationEventMethod__
    | __DestroyAliasMethod__
    | __DestroyFoundryMethod__
    | __GenerateAddressesMethod__
    | __GetBalanceMethod__
    | __GetOutputMethod__
    | __GetIncomingTransactionDataMethod__
    | __GetFoundryOutputMethod__
    | __GetOutputsWithAdditionalUnlockConditionsMethod__
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
    | __DecreaseNativeTokenSupplyMethod__
    | __MinimumRequiredStorageDepositMethod__
    | __IncreaseNativeTokenSupplyMethod__
    | __MintNativeTokenMethod__
    | __MintNftsMethod__
    | __PrepareOutputMethod__
    | __PrepareSendAmountMethod__
    | __PrepareTransactionMethod__
    | __RegisterParticipationEventsMethod__
    | __RequestFundsFromFaucetMethod__
    | __RetryTransactionUntilIncludedMethod__
    | __SendAmountMethod__
    | __SendNativeTokensMethod__
    | __SendNftMethod__
    | __SendOutputsMethod__
    | __SetAliasMethod__
    | __SetDefaultSyncOptionsMethod__
    | __SignTransactionEssenceMethod__
    | __SubmitAndStoreTransactionMethod__
    | __SyncAccountMethod__
    | __VoteMethod__
    | __StopParticipatingMethod__
    | __GetParticipationOverviewMethod__
    | __IncreaseVotingPowerMethod__
    | __DecreaseVotingPowerMethod__;

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
    | __GetLedgerNanoStatusMethod__
    | __GenerateAddressMethod__
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
