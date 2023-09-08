import type { AccountId, CreateAccountPayload, SyncOptions } from '../account';
import type { GenerateAddressOptions } from '../address';
import type { WalletEventType, WalletEvent } from '../event';
import type { IAuth, IClientOptions } from '../../client';

export type __BackupMethod__ = {
    name: 'backup';
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

export type __CreateAccountMethod__ = {
    name: 'createAccount';
    data: CreateAccountPayload;
};

export type __EmitTestEventMethod__ = {
    name: 'emitTestEvent';
    data: { event: WalletEvent };
};

export type __GenerateMnemonicMethod__ = {
    name: 'generateMnemonic';
};

export type __GetAccountIndexesMethod__ = {
    name: 'getAccountIndexes';
};

export type __GetAccountsMethod__ = {
    name: 'getAccounts';
};

export type __GetAccountMethod__ = {
    name: 'getAccount';
    data: { accountId: AccountId };
};

export type __GetChrysalisDataMethod__ = {
    name: 'getChrysalisData';
};

export type __GetLedgerNanoStatusMethod__ = {
    name: 'getLedgerNanoStatus';
};

export type __GenerateEd25519AddressMethod__ = {
    name: 'generateEd25519Address';
    data: {
        accountIndex: number;
        addressIndex: number;
        options?: GenerateAddressOptions;
        bech32Hrp?: string;
    };
};

export type __IsStrongholdPasswordAvailableMethod__ = {
    name: 'isStrongholdPasswordAvailable';
};

export type __RecoverAccountsMethod__ = {
    name: 'recoverAccounts';
    data: {
        accountStartIndex: number;
        accountGapLimit: number;
        addressGapLimit: number;
        syncOptions?: SyncOptions;
    };
};

export type __RemoveLatestAccountMethod__ = {
    name: 'removeLatestAccount';
};

export type __RestoreBackupMethod__ = {
    name: 'restoreBackup';
    data: {
        source: string;
        password: string;
        ignoreIfCoinTypeMismatch?: boolean;
        ignoreIfBech32Mismatch?: string;
    };
};

export type __SetClientOptionsMethod__ = {
    name: 'setClientOptions';
    data: { clientOptions: IClientOptions };
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
    data: { url: string; auth?: IAuth };
};
