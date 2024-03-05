// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::types::block::{
    address::{AccountAddress, Address, Ed25519Address},
    input::{Input, UtxoInput},
    output::{
        unlock_condition::{AddressUnlockCondition, ImmutableAccountAddressUnlockCondition},
        AccountId, AccountOutput, BasicOutput, ChainId, FoundryId, FoundryOutput, NativeToken, NftId, NftOutput,
        Output, SimpleTokenScheme, TokenId, TokenScheme,
    },
    payload::{
        signed_transaction::{
            SignedTransactionPayload, Transaction, TransactionCapabilities, TransactionCapabilityFlag, TransactionId,
        },
        Payload, PayloadError,
    },
    protocol::iota_mainnet_protocol_parameters,
    rand::{mana::rand_mana_allotment, payload::rand_tagged_data_payload},
    signature::{Ed25519Signature, Signature},
    unlock::{ReferenceUnlock, SignatureUnlock, Unlock, Unlocks},
};
use packable::bounded::TryIntoBoundedU16Error;
use pretty_assertions::assert_eq;

const TRANSACTION_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000";
const ED25519_ADDRESS_1: &str = "0xd56da1eb7726ed482dfe9d457cf548c2ae2a6ce3e053dbf82f11223be476adb9";
const ED25519_ADDRESS_2: &str = "0xefda4275375ac3675abff85235fd25a1522a2044cc6027a31b310857246f18c0";
const ED25519_PUBLIC_KEY: &str = "0x1da5ddd11ba3f961acab68fafee3177d039875eaa94ac5fdbff8b53f0c50bfb9";
const ED25519_SIGNATURE: &str = "0xc6a40edf9a089f42c18f4ebccb35fe4b578d93b879e99b87f63573324a710d3456b03fb6d1fcc027e6401cbd9581f790ee3ed7a3f68e9c225fcb9f1cd7b7110d";

#[test]
fn build_valid() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    );

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs([input1, input2])
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(transaction.is_ok());
}

#[test]
fn build_valid_with_payload() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    );

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs([input1, input2])
        .add_output(output)
        .with_payload(rand_tagged_data_payload())
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(transaction.is_ok());
}

#[test]
fn build_valid_add_inputs_outputs() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    );

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs([input1, input2])
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(transaction.is_ok());
}

#[test]
fn build_invalid_payload_kind() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    // Construct a transaction with two inputs and one output.
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    );
    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs([input1.clone(), input2.clone()])
        .add_output(output.clone())
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters)
        .unwrap();

    // Construct a list of two unlocks, whereas we only have 1 tx input.
    let pub_key_bytes: [u8; 32] = prefix_hex::decode(ED25519_PUBLIC_KEY).unwrap();
    let sig_bytes: [u8; 64] = prefix_hex::decode(ED25519_SIGNATURE).unwrap();
    let signature = Ed25519Signature::from_bytes(pub_key_bytes, sig_bytes);
    let sig_unlock = Unlock::from(SignatureUnlock::from(Signature::from(signature)));
    let ref_unlock = Unlock::from(ReferenceUnlock::new(0).unwrap());
    let unlocks = Unlocks::new([sig_unlock, ref_unlock]).unwrap();

    let tx_payload = SignedTransactionPayload::new(transaction, unlocks).unwrap();

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(vec![input1, input2])
        .add_output(output)
        .with_payload(tx_payload)
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(transaction, Err(PayloadError::Kind(1))));
}

#[test]
fn build_invalid_input_count_low() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    );

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(
        transaction,
        Err(PayloadError::InputCount(TryIntoBoundedU16Error::Invalid(0)))
    ));
}

#[test]
fn build_invalid_input_count_high() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    );

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(vec![input; 129])
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(
        transaction,
        Err(PayloadError::InputCount(TryIntoBoundedU16Error::Invalid(129)))
    ));
}

#[test]
fn build_invalid_output_count_low() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input = Input::Utxo(UtxoInput::new(transaction_id, 0));

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .add_input(input)
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(
        transaction,
        Err(PayloadError::OutputCount(TryIntoBoundedU16Error::Invalid(0)))
    ));
}

#[test]
fn build_invalid_output_count_high() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    );

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .add_input(input)
        .with_outputs(vec![output; 129])
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(
        transaction,
        Err(PayloadError::OutputCount(TryIntoBoundedU16Error::Invalid(129)))
    ));
}

#[test]
fn build_invalid_duplicate_utxo() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    );

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(vec![input; 2])
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(transaction, Err(PayloadError::DuplicateUtxo(_))));
}

#[test]
fn build_invalid_accumulated_output() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input = Input::Utxo(UtxoInput::new(transaction_id, 0));

    let bytes1: [u8; 32] = prefix_hex::decode(ED25519_ADDRESS_1).unwrap();
    let address1 = Address::from(Ed25519Address::new(bytes1));
    let amount1 = protocol_parameters.token_supply() - 1_000_000;
    let output1 = Output::Basic(
        BasicOutput::build_with_amount(amount1)
            .add_unlock_condition(AddressUnlockCondition::new(address1))
            .finish()
            .unwrap(),
    );

    let bytes2: [u8; 32] = prefix_hex::decode(ED25519_ADDRESS_2).unwrap();
    let address2 = Address::from(Ed25519Address::new(bytes2));
    let amount2 = 2_000_000;
    let output2 = Output::Basic(
        BasicOutput::build_with_amount(amount2)
            .add_unlock_condition(AddressUnlockCondition::new(address2))
            .finish()
            .unwrap(),
    );

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .add_input(input)
        .with_outputs([output1, output2])
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(transaction, Err(PayloadError::TransactionAmountSum(_))));
}

#[test]
fn getters() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let outputs = [Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    )];
    let payload = Payload::from(rand_tagged_data_payload());

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs([input1, input2])
        .with_outputs(outputs.clone())
        .with_payload(payload.clone())
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters)
        .unwrap();

    assert_eq!(transaction.outputs(), outputs.as_slice());
    assert_eq!(transaction.payload().unwrap(), &payload);
}

#[test]
fn duplicate_output_nft() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let basic = BasicOutput::build_with_amount(amount)
        .add_unlock_condition(AddressUnlockCondition::new(address.clone()))
        .finish_output()
        .unwrap();
    let nft_id = NftId::from_str(ED25519_ADDRESS_1).unwrap();
    let nft = NftOutput::build_with_amount(1_000_000, nft_id)
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .finish_output()
        .unwrap();

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs([input1, input2])
        .with_outputs([basic, nft.clone(), nft])
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(
        transaction,
        Err(PayloadError::DuplicateOutputChain(ChainId::Nft(nft_id_0))) if nft_id_0 == nft_id
    ));
}

#[test]
fn duplicate_output_nft_null() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let basic = BasicOutput::build_with_amount(amount)
        .add_unlock_condition(AddressUnlockCondition::new(address.clone()))
        .finish_output()
        .unwrap();
    let nft_id = NftId::null();
    let nft = NftOutput::build_with_amount(1_000_000, nft_id)
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .finish_output()
        .unwrap();

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs([input1, input2])
        .with_outputs([basic, nft.clone(), nft])
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(transaction.is_ok());
}

#[test]
fn duplicate_output_account() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let basic = BasicOutput::build_with_amount(amount)
        .add_unlock_condition(AddressUnlockCondition::new(address.clone()))
        .finish_output()
        .unwrap();
    let account_id = AccountId::from_str(ED25519_ADDRESS_1).unwrap();
    let account = AccountOutput::build_with_amount(1_000_000, account_id)
        .add_unlock_condition(AddressUnlockCondition::new(address.clone()))
        .finish_output()
        .unwrap();

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs([input1, input2])
        .with_outputs([basic, account.clone(), account])
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(
        transaction,
        Err(PayloadError::DuplicateOutputChain(ChainId::Account(account_id_0))) if account_id_0 == account_id
    ));
}

#[test]
fn duplicate_output_foundry() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::from_str(ED25519_ADDRESS_1).unwrap());
    let amount = 1_000_000;
    let basic = BasicOutput::build_with_amount(amount)
        .add_unlock_condition(AddressUnlockCondition::new(address))
        .finish_output()
        .unwrap();
    let account_id = AccountId::from_str(ED25519_ADDRESS_1).unwrap();
    let token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(70, 0, 100).unwrap());
    let foundry_id = FoundryId::build(&AccountAddress::from(account_id), 1, token_scheme.kind());
    let token_id = TokenId::from(foundry_id);
    let foundry = FoundryOutput::build_with_amount(1_000_000, 1, token_scheme)
        .with_native_token(NativeToken::new(token_id, 70).unwrap())
        .add_unlock_condition(ImmutableAccountAddressUnlockCondition::new(AccountAddress::from(
            account_id,
        )))
        .finish_output()
        .unwrap();

    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs([input1, input2])
        .with_outputs([basic, foundry.clone(), foundry])
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters);

    assert!(matches!(
        transaction,
        Err(PayloadError::DuplicateOutputChain(ChainId::Foundry(foundry_id_0))) if foundry_id_0 == foundry_id
    ));
}

#[test]
fn transactions_capabilities() {
    let protocol_parameters = iota_mainnet_protocol_parameters();
    let transaction_id = TransactionId::new(prefix_hex::decode(TRANSACTION_ID).unwrap());
    let input1 = Input::Utxo(UtxoInput::new(transaction_id, 0));
    let input2 = Input::Utxo(UtxoInput::new(transaction_id, 1));
    let address = Address::from(Ed25519Address::new(prefix_hex::decode(ED25519_ADDRESS_1).unwrap()));
    let amount = 1_000_000;
    let output = Output::Basic(
        BasicOutput::build_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(address))
            .finish()
            .unwrap(),
    );
    let transaction = Transaction::builder(protocol_parameters.network_id())
        .with_inputs(vec![input1, input2])
        .add_output(output)
        .add_mana_allotment(rand_mana_allotment(protocol_parameters))
        .finish_with_params(protocol_parameters)
        .unwrap();
    let mut capabilities = transaction.capabilities().clone();

    use TransactionCapabilityFlag as Flag;

    assert!(capabilities.is_none());

    assert!(!capabilities.has_capability(Flag::BurnNativeTokens));
    capabilities.add_capability(Flag::BurnNativeTokens);
    assert!(capabilities.has_capabilities([Flag::BurnNativeTokens]));

    assert!(!capabilities.has_capability(Flag::BurnMana));
    capabilities.set_capabilities([Flag::BurnMana, Flag::DestroyAccountOutputs]);
    assert!(capabilities.has_capabilities([Flag::BurnMana, Flag::DestroyAccountOutputs]));
    assert!(!capabilities.has_capability(Flag::BurnNativeTokens));

    assert!(!capabilities.is_none());

    assert!(!capabilities.has_capabilities(TransactionCapabilities::all().capabilities_iter()));
    capabilities.set_all();
    assert!(capabilities.has_capabilities(TransactionCapabilities::all().capabilities_iter()));
    assert!(capabilities.has_capabilities([Flag::DestroyFoundryOutputs, Flag::DestroyNftOutputs]));
}
