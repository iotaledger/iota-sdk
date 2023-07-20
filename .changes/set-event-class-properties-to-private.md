
---
"wallet-nodejs-binding": patch
---

Make properties in classes `NewOutputWalletEvent`, `SpentOutputWalletEvent` and `TransactionInclusionWalletEvent` private;
Change type of `transactionInputs` from `[IOutputResponse]` `to IOutputResponse[]` in class `NewOutputWalletEvent`;
