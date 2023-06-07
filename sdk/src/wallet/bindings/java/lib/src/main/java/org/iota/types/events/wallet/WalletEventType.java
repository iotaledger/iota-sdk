package org.iota.types.events.wallet;

public enum WalletEventType {
    ConsolidationRequired(0),
    LedgerAddressGeneration(1),
    NewOutput(2),
    SpentOutput(3),
    TransactionInclusion(4),
    TransactionProgress(5);
    
    private int value;

    private WalletEventType(int value) {
        this.value = value;
    }

    public int getValue() { return value; }
}
