/** The status of a Ledger Nano */
export interface LedgerNanoStatus {
    /** Ledger is available and ready to be used. */
    connected: boolean;
    /** Ledger is connected and locked, true/false for IOTA/Shimmer, undefined for the rest. */
    locked?: boolean;
    /** Ledger blind signing enabled */
    blindSigningEnabled: boolean;
    /** Ledger opened app. */
    app?: LedgerApp;
    /** Ledger device */
    device?: LedgerDeviceType;
    /** Buffer size on device */
    bufferSize?: number;
}

/** The current opened app */
export interface LedgerApp {
    /** Opened app name. */
    name: string;
    /** Opened app version. */
    version: string;
}

/** The Ledger Device Type */
export enum LedgerDeviceType {
    /** Device Type Nano S */
    LedgerNanoS = 'ledgerNanoS',
    /** Device Type Nano X */
    LedgerNanoX = 'ledgerNanoX',
    /** Device Type Nano S Plus */
    LedgerNanoSPlus = 'ledgerNanoSPlus',
}
