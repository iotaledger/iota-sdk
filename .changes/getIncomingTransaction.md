---
"wallet-nodejs-binding": patch
---

Rename `getIncomingTransactionData` to `getIncomingTransaction`.
Change `incomingTransactions()` return type from `[string, Transaction][]`to `Transaction[]`.