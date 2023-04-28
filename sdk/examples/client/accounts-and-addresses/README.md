# Accounts and Addresses

The IOTA ecosystem has
reusable [addresses](https://wiki.iota.org/shimmer/develop/how-tos/seeds-accounts-and-addresses/accounts-and-addresses/)
that can be mapped to users. Accounts are used to generate addresses and can be created in any number per seed for a
specific IOTA Layer 1 Network.

## BIP32 Structure

The [BIP32](https://wiki.iota.org/shimmer/develop/how-tos/seeds-accounts-and-addresses/accounts-and-addresses/#bip32---tree-structure)
standard defines a hierarchical deterministic wallet approach for address and key space generation, and the BIP44 
improvement further enhances it. The derivation path of address/key space in IOTA is `[seed]/44/4218/{int}/{0,1}/{int}`,
where `purpose` and `coin_type` are constants. There are two chains of addresses or keys, and the hierarchy can manage 
different coins secured by the same seed.

## Account Approaches

You can implement
different [account approaches](https://wiki.iota.org/shimmer/develop/how-tos/seeds-accounts-and-addresses/accounts-and-addresses/#account-approaches)
to split addresses/keys into independent spaces. The
multi-account approach assigns an account for each user, while the single account approach creates a single account and
generates multiple addresses linked to user IDs.

## Code Examples

### Create Mnemonic
You can use this example to generate a mnemonic for a seed.

### List Addresses
List all the addresses related to an account.

### Create an Account


### Create an Address


### List Accounts

### List Outputs

### Check Unlock Conditions


### List Transactions:

