import type { AccountId } from '../account';
import type {
    __BuildAliasOutputMethod__,
    __BuildBasicOutputMethod__,
    __BuildFoundryOutputMethod__,
    __BuildNftOutputMethod__,
    __ClaimOutputsMethod__,
    __PrepareConsolidateOutputsMethod__,
    __PrepareCreateAliasOutputMethod__,
    __PrepareDecreaseNativeTokenSupplyMethod__,
    __PrepareDestroyAliasMethod__,
    __PrepareDestroyFoundryMethod__,
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
    __PrepareBurnNativeTokenMethod__,
    __PrepareBurnNftMethod__,
    __IncomingTransactionsMethod__,
    __TransactionsMethod__,
    __UnspentOutputsMethod__,
    __MinimumRequiredStorageDepositMethod__,
    __PrepareIncreaseNativeTokenSupplyMethod__,
    __PrepareMintNativeTokenMethod__,
    __PrepareMintNftsMethod__,
    __PrepareOutputMethod__,
    __PrepareSendAmountMethod__,
    __PrepareTransactionMethod__,
    __RegisterParticipationEventsMethod__,
    __RequestFundsFromFaucetMethod__,
    __RetryTransactionUntilIncludedMethod__,
    __SendAmountMethod__,
    __PrepareSendNativeTokensMethod__,
    __PrepareSendNftMethod__,
    __SendOutputsMethod__,
    __SetAliasMethod__,
    __SetDefaultSyncOptionsMethod__,
    __SignAndSubmitTransaction__,
    __SignTransactionEssenceMethod__,
    __SubmitAndStoreTransactionMethod__,
    __SyncAccountMethod__,
    __GetIncomingTransactionDataMethod__,
    __PrepareVoteMethod__,
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
    __BackupMessage__,
    __Bech32ToHex__,
    __ChangeStrongholdPasswordMessage__,
    __ClearStrongholdPasswordMessage__,
    __ClearListenersMessage__,
    __CreateAccountMessage__,
    __EmitTestEventMessage__,
    __GenerateMnemonicMessage__,
    __GetAccountMessage__,
    __GetAccountIndexesMessage__,
    __GetAccountsMessage__,
    __GetLedgerNanoStatusMessage__,
    __GenerateAddressMessage__,
    __GetNodeInfoMessage__,
    __HexToBech32__,
    __IsStrongholdPasswordAvailableMessage__,
    __RecoverAccountsMessage__,
    __RemoveLatestAccountMessage__,
    __RestoreBackupMessage__,
    __SetClientOptionsMessage__,
    __SetStrongholdPasswordClearIntervalMessage__,
    __SetStrongholdPasswordMessage__,
    __StartBackgroundSyncMessage__,
    __StopBackgroundSyncMessage__,
    __StoreMnemonicMessage__,
    __VerifyMnemonicMessage__,
    __UpdateNodeAuthMessage__,
} from './accountManager';

export type __AccountMethod__ =
    | __BuildAliasOutputMethod__
    | __BuildBasicOutputMethod__
    | __BuildFoundryOutputMethod__
    | __BuildNftOutputMethod__
    | __ClaimOutputsMethod__
    | __PrepareConsolidateOutputsMethod__
    | __PrepareCreateAliasOutputMethod__
    | __DeregisterParticipationEventMethod__
    | __PrepareDestroyAliasMethod__
    | __PrepareDestroyFoundryMethod__
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
    | __PrepareBurnNativeTokenMethod__
    | __PrepareBurnNftMethod__
    | __IncomingTransactionsMethod__
    | __TransactionsMethod__
    | __UnspentOutputsMethod__
    | __PrepareDecreaseNativeTokenSupplyMethod__
    | __MinimumRequiredStorageDepositMethod__
    | __PrepareIncreaseNativeTokenSupplyMethod__
    | __PrepareMintNativeTokenMethod__
    | __PrepareMintNftsMethod__
    | __PrepareOutputMethod__
    | __PrepareSendAmountMethod__
    | __PrepareTransactionMethod__
    | __RegisterParticipationEventsMethod__
    | __RequestFundsFromFaucetMethod__
    | __RetryTransactionUntilIncludedMethod__
    | __SendAmountMethod__
    | __PrepareSendNativeTokensMethod__
    | __PrepareSendNftMethod__
    | __SendOutputsMethod__
    | __SetAliasMethod__
    | __SetDefaultSyncOptionsMethod__
    | __SignTransactionEssenceMethod__
    | __SignAndSubmitTransaction__
    | __SubmitAndStoreTransactionMethod__
    | __SyncAccountMethod__
    | __PrepareVoteMethod__
    | __PrepareStopParticipatingMethod__
    | __GetParticipationOverviewMethod__
    | __PrepareIncreaseVotingPowerMethod__
    | __PrepareDecreaseVotingPowerMethod__;

export type __CallAccountMethodMessage__ = {
    cmd: 'callAccountMethod';
    payload: {
        accountId: AccountId;
        method: __AccountMethod__;
    };
};

export type __Message__ =
    | __BackupMessage__
    | __Bech32ToHex__
    | __CallAccountMethodMessage__
    | __ChangeStrongholdPasswordMessage__
    | __ClearListenersMessage__
    | __ClearStrongholdPasswordMessage__
    | __CreateAccountMessage__
    | __EmitTestEventMessage__
    | __GenerateMnemonicMessage__
    | __GetAccountMessage__
    | __GetAccountIndexesMessage__
    | __GetAccountsMessage__
    | __GetLedgerNanoStatusMessage__
    | __GenerateAddressMessage__
    | __GetNodeInfoMessage__
    | __HexToBech32__
    | __IsStrongholdPasswordAvailableMessage__
    | __RecoverAccountsMessage__
    | __RemoveLatestAccountMessage__
    | __RestoreBackupMessage__
    | __SetClientOptionsMessage__
    | __SetStrongholdPasswordClearIntervalMessage__
    | __SetStrongholdPasswordMessage__
    | __StartBackgroundSyncMessage__
    | __StopBackgroundSyncMessage__
    | __StoreMnemonicMessage__
    | __VerifyMnemonicMessage__
    | __UpdateNodeAuthMessage__;
