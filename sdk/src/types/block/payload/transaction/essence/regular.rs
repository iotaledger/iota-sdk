// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{collections::BTreeSet, vec::Vec};

use hashbrown::HashSet;
use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix, Packable};

use crate::types::{
    block::{
        context_input::{ContextInput, CONTEXT_INPUT_COUNT_RANGE},
        input::{Input, INPUT_COUNT_RANGE},
        mana::{Allotment, Allotments},
        output::{InputsCommitment, NativeTokens, Output, OUTPUT_COUNT_RANGE},
        payload::{OptionalPayload, Payload},
        protocol::ProtocolParameters,
        Error,
    },
    ValidationParams,
};

/// A builder to build a [`RegularTransactionEssence`].
#[derive(Debug, Clone)]
#[must_use]
pub struct RegularTransactionEssenceBuilder {
    network_id: u64,
    context_inputs: Vec<ContextInput>,
    inputs: Vec<Input>,
    inputs_commitment: InputsCommitment,
    outputs: Vec<Output>,
    allotments: BTreeSet<Allotment>,
    payload: OptionalPayload,
    creation_slot: Option<u64>,
}

impl RegularTransactionEssenceBuilder {
    /// Creates a new [`RegularTransactionEssenceBuilder`].
    pub fn new(network_id: u64, inputs_commitment: InputsCommitment) -> Self {
        Self {
            network_id,
            context_inputs: Vec::new(),
            inputs: Vec::new(),
            inputs_commitment,
            outputs: Vec::new(),
            allotments: BTreeSet::new(),
            payload: OptionalPayload::default(),
            creation_slot: None,
        }
    }

    /// Adds creation slot to a [`RegularTransactionEssenceBuilder`].
    pub fn with_creation_slot(mut self, creation_slot: impl Into<Option<u64>>) -> Self {
        self.creation_slot = creation_slot.into();
        self
    }

    /// Adds context inputs to a [`RegularTransactionEssenceBuilder`].
    pub fn with_context_inputs(mut self, context_inputs: impl Into<Vec<ContextInput>>) -> Self {
        self.context_inputs = context_inputs.into();
        self
    }

    /// Adds inputs to a [`RegularTransactionEssenceBuilder`].
    pub fn with_inputs(mut self, inputs: impl Into<Vec<Input>>) -> Self {
        self.inputs = inputs.into();
        self
    }

    /// Add an input to a [`RegularTransactionEssenceBuilder`].
    pub fn add_input(mut self, input: Input) -> Self {
        self.inputs.push(input);
        self
    }

    /// Add outputs to a [`RegularTransactionEssenceBuilder`].
    pub fn with_outputs(mut self, outputs: impl Into<Vec<Output>>) -> Self {
        self.outputs = outputs.into();
        self
    }

    /// Add allotments to a [`RegularTransactionEssenceBuilder`].
    pub fn with_allotments(mut self, allotments: impl IntoIterator<Item = Allotment>) -> Self {
        self.allotments = allotments.into_iter().collect();
        self
    }

    /// Add an output to a [`RegularTransactionEssenceBuilder`].
    pub fn add_output(mut self, output: Output) -> Self {
        self.outputs.push(output);
        self
    }

    /// Add an [`Allotment`] to a [`RegularTransactionEssenceBuilder`].
    pub fn add_allotment(mut self, allotment: Allotment) -> Self {
        self.allotments.insert(allotment);
        self
    }

    /// Replaces an [`Allotment`] of the [`RegularTransactionEssenceBuilder`] with a new one, or adds it.
    pub fn replace_allotment(mut self, allotment: Allotment) -> Self {
        self.allotments.replace(allotment);
        self
    }

    /// Add a payload to a [`RegularTransactionEssenceBuilder`].
    pub fn with_payload(mut self, payload: impl Into<OptionalPayload>) -> Self {
        self.payload = payload.into();
        self
    }

    /// Finishes a [`RegularTransactionEssenceBuilder`] into a [`RegularTransactionEssence`].
    pub fn finish_with_params<'a>(
        self,
        params: impl Into<ValidationParams<'a>> + Send,
    ) -> Result<RegularTransactionEssence, Error> {
        let params = params.into();
        if let Some(protocol_parameters) = params.protocol_parameters() {
            if self.network_id != protocol_parameters.network_id() {
                return Err(Error::NetworkIdMismatch {
                    expected: protocol_parameters.network_id(),
                    actual: self.network_id,
                });
            }
        }

        let context_inputs: BoxedSlicePrefix<ContextInput, ContextInputCount> = self
            .context_inputs
            .into_boxed_slice()
            .try_into()
            .map_err(Error::InvalidContextInputCount)?;

        let inputs: BoxedSlicePrefix<Input, InputCount> = self
            .inputs
            .into_boxed_slice()
            .try_into()
            .map_err(Error::InvalidInputCount)?;

        verify_inputs(&inputs)?;

        let outputs: BoxedSlicePrefix<Output, OutputCount> = self
            .outputs
            .into_boxed_slice()
            .try_into()
            .map_err(Error::InvalidOutputCount)?;

        if let Some(protocol_parameters) = params.protocol_parameters() {
            verify_outputs::<true>(&outputs, protocol_parameters)?;
        }

        let allotments = Allotments::from_set(self.allotments)?;

        verify_payload(&self.payload)?;

        let creation_slot = self.creation_slot.unwrap_or_else(|| {
            #[cfg(feature = "std")]
            let creation_slot = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos() as u64;
            // TODO no_std way to have a nanosecond timestamp
            // https://github.com/iotaledger/iota-sdk/issues/647
            #[cfg(not(feature = "std"))]
            let creation_slot = 0;

            creation_slot
        });

        Ok(RegularTransactionEssence {
            network_id: self.network_id,
            creation_slot,
            context_inputs,
            inputs,
            inputs_commitment: self.inputs_commitment,
            outputs,
            allotments,
            payload: self.payload,
        })
    }

    /// Finishes a [`RegularTransactionEssenceBuilder`] into a [`RegularTransactionEssence`] without protocol
    /// validation.
    pub fn finish(self) -> Result<RegularTransactionEssence, Error> {
        self.finish_with_params(ValidationParams::default())
    }
}

pub(crate) type ContextInputCount =
    BoundedU16<{ *CONTEXT_INPUT_COUNT_RANGE.start() }, { *CONTEXT_INPUT_COUNT_RANGE.end() }>;
pub(crate) type InputCount = BoundedU16<{ *INPUT_COUNT_RANGE.start() }, { *INPUT_COUNT_RANGE.end() }>;
pub(crate) type OutputCount = BoundedU16<{ *OUTPUT_COUNT_RANGE.start() }, { *OUTPUT_COUNT_RANGE.end() }>;

/// A transaction regular essence consuming inputs, creating outputs and carrying an optional payload.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
#[packable(unpack_error = Error)]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct RegularTransactionEssence {
    /// The unique value denoting whether the block was meant for mainnet, testnet, or a private network.
    #[packable(verify_with = verify_network_id)]
    network_id: u64,
    /// The slot index of the block in which the transaction was created.
    creation_slot: u64,
    #[packable(verify_with = verify_context_inputs_packable)]
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidContextInputCount(p.into())))]
    context_inputs: BoxedSlicePrefix<ContextInput, ContextInputCount>,
    #[packable(verify_with = verify_inputs_packable)]
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidInputCount(p.into())))]
    inputs: BoxedSlicePrefix<Input, InputCount>,
    /// BLAKE2b-256 hash of the serialized outputs referenced in inputs by their OutputId.
    inputs_commitment: InputsCommitment,
    #[packable(verify_with = verify_outputs)]
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidOutputCount(p.into())))]
    outputs: BoxedSlicePrefix<Output, OutputCount>,
    allotments: Allotments,
    #[packable(verify_with = verify_payload_packable)]
    payload: OptionalPayload,
}

impl RegularTransactionEssence {
    /// The essence kind of a [`RegularTransactionEssence`].
    pub const KIND: u8 = 2;

    /// Creates a new [`RegularTransactionEssenceBuilder`] to build a [`RegularTransactionEssence`].
    pub fn builder(network_id: u64, inputs_commitment: InputsCommitment) -> RegularTransactionEssenceBuilder {
        RegularTransactionEssenceBuilder::new(network_id, inputs_commitment)
    }

    /// Returns the network ID of a [`RegularTransactionEssence`].
    pub fn network_id(&self) -> u64 {
        self.network_id
    }

    /// Returns the slot index of the block in which the transaction was created. [`RegularTransactionEssence`].
    pub fn creation_slot(&self) -> u64 {
        self.creation_slot
    }

    /// Returns the context inputs of a [`RegularTransactionEssence`].
    pub fn context_inputs(&self) -> &[ContextInput] {
        &self.context_inputs
    }

    /// Returns the inputs of a [`RegularTransactionEssence`].
    pub fn inputs(&self) -> &[Input] {
        &self.inputs
    }

    /// Returns the inputs commitment of a [`RegularTransactionEssence`].
    pub fn inputs_commitment(&self) -> &InputsCommitment {
        &self.inputs_commitment
    }

    /// Returns the outputs of a [`RegularTransactionEssence`].
    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }

    /// Returns the allotments of a [`RegularTransactionEssence`].
    pub fn allotments(&self) -> &[Allotment] {
        &self.allotments
    }

    /// Returns the optional payload of a [`RegularTransactionEssence`].
    pub fn payload(&self) -> Option<&Payload> {
        self.payload.as_ref()
    }
}

fn verify_network_id<const VERIFY: bool>(network_id: &u64, visitor: &ProtocolParameters) -> Result<(), Error> {
    if VERIFY {
        let expected = visitor.network_id();

        if *network_id != expected {
            return Err(Error::NetworkIdMismatch {
                expected,
                actual: *network_id,
            });
        }
    }

    Ok(())
}

fn verify_context_inputs_packable<const VERIFY: bool>(
    context_inputs: &[ContextInput],
    _visitor: &ProtocolParameters,
) -> Result<(), Error> {
    if VERIFY {
        verify_context_inputs(context_inputs)?;
    }
    Ok(())
}

fn verify_context_inputs(context_inputs: &[ContextInput]) -> Result<(), Error> {
    // There must be zero or one Commitment Input.
    if context_inputs
        .iter()
        .filter(|i| matches!(i, ContextInput::Commitment(_)))
        .count()
        > 1
    {
        return Err(Error::TooManyCommitmentInputs);
    }

    let mut reward_index_set = HashSet::new();
    let mut bic_account_id_set = HashSet::new();
    for input in context_inputs.iter() {
        match input {
            ContextInput::BlockIssuanceCredit(bic) => {
                let account_id = bic.account_id();
                // All Block Issuance Credit Inputs must reference a different Account ID.
                if !bic_account_id_set.insert(account_id) {
                    return Err(Error::DuplicateBicAccountId(account_id));
                }
            }
            ContextInput::Reward(r) => {
                let idx = r.index();
                // All Rewards Inputs must reference a different Index
                if !reward_index_set.insert(idx) {
                    return Err(Error::DuplicateRewardInputIndex(idx));
                }
            }
            _ => (),
        }
    }

    Ok(())
}

fn verify_inputs(inputs: &[Input]) -> Result<(), Error> {
    let mut seen_utxos = HashSet::new();

    for input in inputs.iter() {
        let Input::Utxo(utxo) = input;
        if !seen_utxos.insert(utxo) {
            return Err(Error::DuplicateUtxo(*utxo));
        }
    }

    Ok(())
}

fn verify_inputs_packable<const VERIFY: bool>(inputs: &[Input], _visitor: &ProtocolParameters) -> Result<(), Error> {
    if VERIFY {
        verify_inputs(inputs)?;
    }
    Ok(())
}

fn verify_outputs<const VERIFY: bool>(outputs: &[Output], visitor: &ProtocolParameters) -> Result<(), Error> {
    if VERIFY {
        let mut amount_sum: u64 = 0;
        let mut native_tokens_count: u8 = 0;
        let mut chain_ids = HashSet::new();

        for output in outputs.iter() {
            let (amount, native_tokens, chain_id) = match output {
                Output::Basic(output) => (output.amount(), Some(output.native_tokens()), None),
                Output::Account(output) => (output.amount(), Some(output.native_tokens()), Some(output.chain_id())),
                Output::Foundry(output) => (output.amount(), Some(output.native_tokens()), Some(output.chain_id())),
                Output::Nft(output) => (output.amount(), Some(output.native_tokens()), Some(output.chain_id())),
                Output::Delegation(output) => (output.amount(), None, Some(output.chain_id())),
            };

            amount_sum = amount_sum
                .checked_add(amount)
                .ok_or(Error::InvalidTransactionAmountSum(amount_sum as u128 + amount as u128))?;

            // Accumulated output balance must not exceed the total supply of tokens.
            if amount_sum > visitor.token_supply() {
                return Err(Error::InvalidTransactionAmountSum(amount_sum as u128));
            }

            if let Some(native_tokens) = native_tokens {
                native_tokens_count = native_tokens_count.checked_add(native_tokens.len() as u8).ok_or(
                    Error::InvalidTransactionNativeTokensCount(native_tokens_count as u16 + native_tokens.len() as u16),
                )?;

                if native_tokens_count > NativeTokens::COUNT_MAX {
                    return Err(Error::InvalidTransactionNativeTokensCount(native_tokens_count as u16));
                }
            }

            if let Some(chain_id) = chain_id {
                if !chain_id.is_null() && !chain_ids.insert(chain_id) {
                    return Err(Error::DuplicateOutputChain(chain_id));
                }
            }

            output.verify_storage_deposit(*visitor.rent_structure(), visitor.token_supply())?;
        }
    }

    Ok(())
}

fn verify_payload(payload: &OptionalPayload) -> Result<(), Error> {
    match &payload.0 {
        Some(Payload::TaggedData(_)) | None => Ok(()),
        Some(payload) => Err(Error::InvalidPayloadKind(payload.kind())),
    }
}

fn verify_payload_packable<const VERIFY: bool>(
    payload: &OptionalPayload,
    _visitor: &ProtocolParameters,
) -> Result<(), Error> {
    if VERIFY {
        verify_payload(payload)?;
    }
    Ok(())
}

pub(crate) mod dto {
    use alloc::string::{String, ToString};
    use core::str::FromStr;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        types::{
            block::{output::dto::OutputDto, payload::dto::PayloadDto, Error},
            TryFromDto,
        },
        utils::serde::string,
    };

    /// Describes the essence data making up a transaction by defining its inputs and outputs and an optional payload.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RegularTransactionEssenceDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub network_id: String,
        #[serde(with = "string")]
        pub creation_slot: u64,
        pub context_inputs: Vec<ContextInput>,
        pub inputs: Vec<Input>,
        pub inputs_commitment: String,
        pub outputs: Vec<OutputDto>,
        pub allotments: Vec<Allotment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<PayloadDto>,
    }

    impl From<&RegularTransactionEssence> for RegularTransactionEssenceDto {
        fn from(value: &RegularTransactionEssence) -> Self {
            Self {
                kind: RegularTransactionEssence::KIND,
                network_id: value.network_id().to_string(),
                creation_slot: value.creation_slot(),
                context_inputs: value.context_inputs().to_vec(),
                inputs: value.inputs().to_vec(),
                inputs_commitment: value.inputs_commitment().to_string(),
                outputs: value.outputs().iter().map(Into::into).collect::<Vec<_>>(),
                allotments: value.allotments().to_vec(),
                payload: match value.payload() {
                    Some(p @ Payload::TaggedData(_)) => Some(p.into()),
                    Some(_) => unimplemented!(),
                    None => None,
                },
            }
        }
    }

    impl TryFromDto for RegularTransactionEssence {
        type Dto = RegularTransactionEssenceDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            let network_id = dto
                .network_id
                .parse::<u64>()
                .map_err(|_| Error::InvalidField("network_id"))?;
            let outputs = dto
                .outputs
                .into_iter()
                .map(|o| Output::try_from_dto_with_params(o, &params))
                .collect::<Result<Vec<Output>, Error>>()?;

            let mut builder = Self::builder(network_id, InputsCommitment::from_str(&dto.inputs_commitment)?)
                .with_creation_slot(dto.creation_slot)
                .with_context_inputs(dto.context_inputs)
                .with_inputs(dto.inputs)
                .with_outputs(outputs)
                .with_allotments(dto.allotments);

            builder = if let Some(p) = dto.payload {
                if let PayloadDto::TaggedData(i) = p {
                    builder.with_payload(*i)
                } else {
                    return Err(Error::InvalidField("payload"));
                }
            } else {
                builder
            };

            builder.finish_with_params(params).map_err(Into::into)
        }
    }
}
