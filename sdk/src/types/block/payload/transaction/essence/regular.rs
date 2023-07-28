// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use hashbrown::HashSet;
use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix, Packable};

use crate::types::{
    block::{
        input::{Input, INPUT_COUNT_RANGE},
        mana::{Allotment, ALLOTMENT_COUNT_RANGE},
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
    creation_time: Option<u64>,
    inputs: Vec<Input>,
    inputs_commitment: InputsCommitment,
    outputs: Vec<Output>,
    allotments: Vec<Allotment>,
    payload: OptionalPayload,
}

impl RegularTransactionEssenceBuilder {
    /// Creates a new [`RegularTransactionEssenceBuilder`].
    pub fn new(network_id: u64, inputs_commitment: InputsCommitment) -> Self {
        Self {
            network_id,
            creation_time: None,
            inputs: Vec::new(),
            inputs_commitment,
            outputs: Vec::new(),
            allotments: Vec::new(),
            payload: OptionalPayload::default(),
        }
    }

    /// Adds creation time to a [`RegularTransactionEssenceBuilder`].
    pub fn with_creation_time(mut self, creation_time: impl Into<Option<u64>>) -> Self {
        self.creation_time = creation_time.into();
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
    pub fn with_allotments(mut self, allotments: impl Into<Vec<Allotment>>) -> Self {
        self.allotments = allotments.into();
        self
    }

    /// Add an output to a [`RegularTransactionEssenceBuilder`].
    pub fn add_output(mut self, output: Output) -> Self {
        self.outputs.push(output);
        self
    }

    /// Add an allotment to a [`RegularTransactionEssenceBuilder`].
    pub fn add_allotment(mut self, allotment: Allotment) -> Self {
        self.allotments.push(allotment);
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

        let inputs: BoxedSlicePrefix<Input, InputCount> = self
            .inputs
            .into_boxed_slice()
            .try_into()
            .map_err(Error::InvalidInputCount)?;

        verify_inputs::<true>(&inputs)?;

        let outputs: BoxedSlicePrefix<Output, OutputCount> = self
            .outputs
            .into_boxed_slice()
            .try_into()
            .map_err(Error::InvalidOutputCount)?;

        if let Some(protocol_parameters) = params.protocol_parameters() {
            verify_outputs::<true>(&outputs, protocol_parameters)?;
        }

        let allotments: BoxedSlicePrefix<Allotment, AllotmentCount> = self
            .allotments
            .into_boxed_slice()
            .try_into()
            .map_err(Error::InvalidAllotmentCount)?;

        if let Some(protocol_parameters) = params.protocol_parameters() {
            verify_allotments::<true>(&allotments, protocol_parameters)?;
        }

        verify_payload::<true>(&self.payload)?;

        let creation_time = self.creation_time.unwrap_or_else(|| {
            #[cfg(feature = "std")]
            let creation_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos() as u64;
            // TODO no_std way to have a nanosecond timestamp
            // https://github.com/iotaledger/iota-sdk/issues/647
            #[cfg(not(feature = "std"))]
            let creation_time = 0;

            creation_time
        });

        Ok(RegularTransactionEssence {
            network_id: self.network_id,
            creation_time,
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

pub(crate) type InputCount = BoundedU16<{ *INPUT_COUNT_RANGE.start() }, { *INPUT_COUNT_RANGE.end() }>;
pub(crate) type OutputCount = BoundedU16<{ *OUTPUT_COUNT_RANGE.start() }, { *OUTPUT_COUNT_RANGE.end() }>;
pub(crate) type AllotmentCount = BoundedU16<{ *ALLOTMENT_COUNT_RANGE.start() }, { *ALLOTMENT_COUNT_RANGE.end() }>;

/// A transaction regular essence consuming inputs, creating outputs and carrying an optional payload.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
#[packable(unpack_error = Error)]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct RegularTransactionEssence {
    /// The unique value denoting whether the block was meant for mainnet, testnet, or a private network.
    #[packable(verify_with = verify_network_id)]
    network_id: u64,
    /// The time at which this transaction was created by the client. It's a Unix-like timestamp in nanosecond.
    creation_time: u64,
    #[packable(verify_with = verify_inputs_packable)]
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidInputCount(p.into())))]
    inputs: BoxedSlicePrefix<Input, InputCount>,
    /// BLAKE2b-256 hash of the serialized outputs referenced in inputs by their OutputId.
    inputs_commitment: InputsCommitment,
    #[packable(verify_with = verify_outputs)]
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidOutputCount(p.into())))]
    outputs: BoxedSlicePrefix<Output, OutputCount>,
    #[packable(verify_with = verify_allotments)]
    #[packable(unpack_error_with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidAllotmentCount(p.into())))]
    allotments: BoxedSlicePrefix<Allotment, AllotmentCount>,
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

    /// Returns the creation time of a [`RegularTransactionEssence`].
    pub fn creation_time(&self) -> u64 {
        self.creation_time
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

fn verify_inputs<const VERIFY: bool>(inputs: &[Input]) -> Result<(), Error> {
    if VERIFY {
        let mut seen_utxos = HashSet::new();

        for input in inputs.iter() {
            let Input::Utxo(utxo) = input;
            if !seen_utxos.insert(utxo) {
                return Err(Error::DuplicateUtxo(*utxo));
            }
        }
    }

    Ok(())
}

fn verify_inputs_packable<const VERIFY: bool>(inputs: &[Input], _visitor: &ProtocolParameters) -> Result<(), Error> {
    verify_inputs::<VERIFY>(inputs)
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

fn verify_allotments<const VERIFY: bool>(allotments: &[Allotment], _visitor: &ProtocolParameters) -> Result<(), Error> {
    if VERIFY {
        let mut mana: u64;
        let mut mana_sum: u64 = 0;
        let mut unique_ids = HashSet::with_capacity(allotments.len());
        for allotment in allotments.iter() {
            mana = allotment.mana();
            mana_sum = mana_sum
                .checked_add(mana)
                .ok_or(Error::InvalidAllotmentManaSum(mana_sum as u128 + mana as u128))?;

            // TODO: compare with `max_mana_supply` from visitor once available
            // if mana_sum > visitor.max_mana_supply() {
            //     return Err(Error::InvalidAllotmentManaSum(mana_sum as u128));
            // }

            if !unique_ids.insert(*allotment.account_id()) {
                return Err(Error::DuplicateAllotment(*allotment.account_id()));
            }
        }
    }

    Ok(())
}

fn verify_payload<const VERIFY: bool>(payload: &OptionalPayload) -> Result<(), Error> {
    if VERIFY {
        match &payload.0 {
            Some(Payload::TaggedData(_)) | None => Ok(()),
            Some(payload) => Err(Error::InvalidPayloadKind(payload.kind())),
        }
    } else {
        Ok(())
    }
}

fn verify_payload_packable<const VERIFY: bool>(
    payload: &OptionalPayload,
    _visitor: &ProtocolParameters,
) -> Result<(), Error> {
    verify_payload::<VERIFY>(payload)
}

pub(crate) mod dto {
    use alloc::{
        boxed::Box,
        string::{String, ToString},
    };
    use core::str::FromStr;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::{
        block::{
            input::dto::InputDto, mana::dto::AllotmentDto, output::dto::OutputDto, payload::dto::PayloadDto, Error,
        },
        TryFromDto,
    };

    /// Describes the essence data making up a transaction by defining its inputs and outputs and an optional payload.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RegularTransactionEssenceDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub network_id: String,
        pub creation_time: u64,
        pub inputs: Vec<InputDto>,
        pub inputs_commitment: String,
        pub outputs: Vec<OutputDto>,
        pub allotments: Vec<AllotmentDto>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<PayloadDto>,
    }

    impl From<&RegularTransactionEssence> for RegularTransactionEssenceDto {
        fn from(value: &RegularTransactionEssence) -> Self {
            Self {
                kind: RegularTransactionEssence::KIND,
                network_id: value.network_id().to_string(),
                creation_time: value.creation_time(),
                inputs: value.inputs().iter().map(Into::into).collect::<Vec<_>>(),
                inputs_commitment: value.inputs_commitment().to_string(),
                outputs: value.outputs().iter().map(Into::into).collect::<Vec<_>>(),
                allotments: value.allotments().iter().map(Into::into).collect::<Vec<_>>(),
                payload: match value.payload() {
                    Some(Payload::TaggedData(i)) => Some(PayloadDto::TaggedData(Box::new(i.as_ref().into()))),
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
            let inputs = dto
                .inputs
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<Input>, Error>>()?;
            let outputs = dto
                .outputs
                .into_iter()
                .map(|o| Output::try_from_dto_with_params(o, &params))
                .collect::<Result<Vec<Output>, Error>>()?;
            let allotments = dto
                .allotments
                .into_iter()
                .map(|a| Allotment::try_from_dto_with_params(a, &params))
                .collect::<Result<Vec<Allotment>, Error>>()?;

            let mut builder = Self::builder(network_id, InputsCommitment::from_str(&dto.inputs_commitment)?)
                .with_creation_time(dto.creation_time)
                .with_inputs(inputs)
                .with_outputs(outputs)
                .with_allotments(allotments);

            builder = if let Some(p) = dto.payload {
                if let PayloadDto::TaggedData(i) = p {
                    builder.with_payload(Payload::TaggedData(Box::new((*i).try_into()?)))
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
