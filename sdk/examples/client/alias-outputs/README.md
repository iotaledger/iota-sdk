# Alias Outputs

Stardust introduces a new type of ledger account,
called [Alias Output](https://wiki.iota.org/shimmer/develop/how-tos/alias/introduction/), which represents a UTXO state
machine suitable for committee-ran smart contract chains. Alias Outputs store state commitments of the second-layer
smart contract chains, have a globally unique address, and can issue custom tokens.

## State Controller and Governor

The State Controller and the Governor are two controllers that can be rotated and have different account privileges. The
State Controller can change the state data stored in the output and manipulate token balances of the alias account,
while the Governor can destroy the alias and change the controller entities. To prove ownership of funds locked under an
alias address, one must require the unlocking of the alias output in the same transaction that tries to unlock the
funds.

## [Crete an Alias](https://wiki.iota.org/shimmer/develop/how-tos/alias/create/)

Just like a Basic Output, an Alias Output also must have:

* **Amount** to hold base token.
* **Unlock Conditions** that define how the output can be unlocked.
* **Features** that don't modify the unlocking.

On top of these, it also defines:

* **State Index** that has to be incremented in every transaction initiated by the State Controller.
* **State Metadata** that may hold binary data.
* **Foundry Counter** that defines how many foundries the alias has created.
* **Immutable Features** that are regular Features defined upon creation which can never be modified afterward.

### Available Unlock Conditions

Alias outputs only support two type of unlock conditions:

* **State Controller Address Unlock Condition** that defines the state controller.
* **Governor Address Unlock Condition** that defines the governor of the alias.

## [State Transitions](https://wiki.iota.org/shimmer/develop/how-tos/alias/state-transitions/)

As the name suggests, a state transition must be initiated by the State Controller, therefore it is the
stateControllerAddress that needs to sign the transaction.

During a state transition, the following must happen:

* **The alias can't be destroyed.**
* **The State Index** must be incremented.
* **The Foundry Counter** must be incremented by the number of foundries created in the transaction.
* **The Unlock Conditions** must not be changed, therefore the state controller can't update ownership of the alias
  account.
* **The Metadata Feature** must not be updated.

The following can happen, but is optional:

* **Token balances of the output may be changed**, the State Controller can transfer funds in- and out of the alias
  account.
* **The State Metadata** may be updated,
* **The Sender Feature** may be updated,

## [Governance Transitions](https://wiki.iota.org/shimmer/develop/how-tos/alias/governance-transitions/)

A valid governance can do the following:

* **The alias output may be destroyed by the governor**.
* **The State Controller Address Unlock Condition** may be changed.
* **The Governor Address Unlock Condition** may be changed.
* **The Metadata Feature may be changed.**

No other fields are allowed to change in the next alias state, therefore a governance transition can't modify token
balances or create foundries.

## [Destroy an Alias](https://wiki.iota.org/shimmer/develop/how-tos/alias/destroy/)

When an alias is destroyed, the storage deposit is
refunded to a governor defined address. Keep in mind that once the
alias is destroyed, any funds locked in the ledger in outputs that belong to the alias address (derived from Alias ID)
are essentially lost, as it is not possible to unlock them.

## [Unlock Funds Owned by an Alias](https://wiki.iota.org/shimmer/develop/how-tos/alias/unlock-alias-funds/)

An alias account may own funds in the ledger two ways:

* Locked directly in its alias output.
* Locked in other outputs to the address of the alias.

The global alias address is derived from the unique Alias ID. Anyone can send funds in the ledger to the address of the
alias, while only the current state controller is able to unlock those funds by including the alias itself in the very
same transaction. An alias address doesn't have a private key for signing unlocks, therefore one has to prove that they
can unlock the alias output that created the alias address.