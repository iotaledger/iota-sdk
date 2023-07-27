import type { AccountId, CreateAccountPayload, SyncOptions } from '../account';
import type { GenerateAddressOptions } from '../address';
import type { WalletEventType, WalletEvent } from '../event';
import type { Auth, ClientOptions } from '../network';

export type __BackupMessage__ = {
    cmd: 'backup';
    payload: {
        destination: string;
        password: string;
    };
};

export type __Bech32ToHex__ = {
    cmd: 'bech32ToHex';
    payload: {
        bech32Address: string;
    };
};

export type __ChangeStrongholdPasswordMessage__ = {
    cmd: 'changeStrongholdPassword';
    payload: {
        currentPassword: string;
        newPassword: string;
    };
};

export type __ClearStrongholdPasswordMessage__ = {
    cmd: 'clearStrongholdPassword';
};

export type __ClearListenersMessage__ = {
    cmd: 'clearListeners';
    payload: { eventTypes: WalletEventType[] };
};

export type __CreateAccountMessage__ = {
    cmd: 'createAccount';
    payload: CreateAccountPayload;
};

export type __EmitTestEventMessage__ = {
    cmd: 'emitTestEvent';
    payload: { event: WalletEvent };
};

export type __GenerateMnemonicMessage__ = {
    cmd: 'generateMnemonic';
};

export type __GetAccountIndexesMessage__ = {
    cmd: 'getAccountIndexes';
};

export type __GetAccountsMessage__ = {
    cmd: 'getAccounts';
};

export type __GetAccountMessage__ = {
    cmd: 'getAccount';
    payload: { accountId: AccountId };
};

export type __GetLedgerNanoStatusMessage__ = {
    cmd: 'getLedgerNanoStatus';
};

export type __GenerateEd25519AddressMessage__ = {
    cmd: 'generateEd25519Address';
    payload: {
        accountIndex: number;
        addressIndex: number;
        options?: GenerateAddressOptions;
        bech32Hrp?: string;
    };
};

export type __GetNodeInfoMessage__ = {
    cmd: 'getNodeInfo';
    payload: {
        url?: string;
        auth?: Auth;
    };
};

export type __HexToBech32__ = {
    cmd: 'hexToBech32';
    payload: {
        hex: string;
        bech32Hrp?: string;
    };
};

export type __IsStrongholdPasswordAvailableMessage__ = {
    cmd: 'isStrongholdPasswordAvailable';
};

export type __RecoverAccountsMessage__ = {
    cmd: 'recoverAccounts';
    payload: {
        accountStartIndex: number;
        accountGapLimit: number;
        addressGapLimit: number;
        syncOptions?: SyncOptions;
    };
};

export type __RemoveLatestAccountMessage__ = {
    cmd: 'removeLatestAccount';
};

export type __RestoreBackupMessage__ = {
    cmd: 'restoreBackup';
    payload: {
        source: string;
        password: string;
        ignoreIfCoinTypeMismatch?: boolean;
        ignoreIfBech32Mismatch?: string;
    };
};

export type __SetClientOptionsMessage__ = {
    cmd: 'setClientOptions';
    payload: { clientOptions: ClientOptions };
};

export type __SetStrongholdPasswordMessage__ = {
    cmd: 'setStrongholdPassword';
    payload: { password: string };
};

export type __SetStrongholdPasswordClearIntervalMessage__ = {
    cmd: 'setStrongholdPasswordClearInterval';
    payload?: { intervalInMilliseconds?: number };
};

export type __StartBackgroundSyncMessage__ = {
    cmd: 'startBackgroundSync';
    payload: {
        options?: SyncOptions;
        intervalInMilliseconds?: number;
    };
};

export type __StopBackgroundSyncMessage__ = {
    cmd: 'stopBackgroundSync';
};

export type __StoreMnemonicMessage__ = {
    cmd: 'storeMnemonic';
    payload: { mnemonic: string };
};

export type __VerifyMnemonicMessage__ = {
    cmd: 'verifyMnemonic';
    payload: { mnemonic: string };
};

export type __UpdateNodeAuthMessage__ = {
    cmd: 'updateNodeAuth';
    payload: { url: string; auth?: Auth };
};
