/** The status of a Ledger Nano */
export interface LedgerNanoStatus {
    /** TODO */
    connected: boolean;
    /** TODO */
    locked?: boolean;
    /** TODO */
    blindSigningEnabled: boolean;
    /** TODO */
    app?: LedgerApp;
    /** TODO */
    device?: LedgerDeviceType;
    /** TODO */
    bufferSize?: number;
}

/** The current opened app */
export interface LedgerApp {
    /** TODO */
    name: string;
    /** TODO */
    version: string;
}

/** The Ledger Device Type */
export enum LedgerDeviceType {
    /** TODO */
    LedgerNanoS = 'ledgerNanoS',
    /** TODO */
    LedgerNanoX = 'ledgerNanoX',
    /** TODO */
    LedgerNanoSPlus = 'ledgerNanoSPlus',
}
