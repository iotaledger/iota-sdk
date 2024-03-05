// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{collections::BTreeSet, vec::Vec};

use crypto::hashes::{blake2b::Blake2b256, Digest};
use hashbrown::HashSet;
use packable::{
    bounded::BoundedU16,
    prefix::{BoxedSlicePrefix, UnpackPrefixError},
    Packable, PackableExt,
};

use crate::{
    types::block::{
        capabilities::{Capabilities, CapabilityFlag},
        context_input::{ContextInput, ContextInputs},
        input::{Input, InputError, INPUT_COUNT_RANGE},
        mana::{verify_mana_allotments_sum, ManaAllotment, ManaAllotments},
        output::{Output, OutputCommitmentProof, OutputError, OutputIdProof, ProofError, OUTPUT_COUNT_RANGE},
        payload::{
            signed_transaction::{TransactionHash, TransactionId, TransactionSigningHash},
            OptionalPayload, Payload, PayloadError,
        },
        protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
        slot::SlotIndex,
    },
    utils::merkle_hasher,
};

/// A builder to build a [`Transaction`].
#[derive(Debug, Clone)]
#[must_use]
pub struct TransactionBuilder {
    network_id: u64,
    creation_slot: Option<SlotIndex>,
    context_inputs: Vec<ContextInput>,
    inputs: Vec<Input>,
    allotments: BTreeSet<ManaAllotment>,
    capabilities: TransactionCapabilities,
    payload: OptionalPayload,
    outputs: Vec<Output>,
}

impl TransactionBuilder {
    /// Creates a new [`TransactionBuilder`].
    pub fn new(network_id: u64) -> Self {
        Self {
            network_id,
            creation_slot: None,
            context_inputs: Vec::new(),
            inputs: Vec::new(),
            allotments: BTreeSet::new(),
            capabilities: Default::default(),
            payload: OptionalPayload::default(),
            outputs: Vec::new(),
        }
    }

    /// Sets the creation slot of a [`TransactionBuilder`].
    pub fn with_creation_slot(mut self, creation_slot: impl Into<Option<SlotIndex>>) -> Self {
        self.creation_slot = creation_slot.into();
        self
    }

    /// Sets the context inputs of a [`TransactionBuilder`].
    pub fn with_context_inputs(mut self, context_inputs: impl IntoIterator<Item = ContextInput>) -> Self {
        self.context_inputs = context_inputs.into_iter().collect();
        self
    }

    /// Sets the inputs of a [`TransactionBuilder`].
    pub fn with_inputs(mut self, inputs: impl IntoIterator<Item = Input>) -> Self {
        self.inputs = inputs.into_iter().collect();
        self
    }

    /// Adds an input to a [`TransactionBuilder`].
    pub fn add_input(mut self, input: Input) -> Self {
        self.inputs.push(input);
        self
    }

    /// Sets the [`ManaAllotment`]s of a [`TransactionBuilder`].
    pub fn with_mana_allotments(mut self, allotments: impl IntoIterator<Item = ManaAllotment>) -> Self {
        self.allotments = allotments.into_iter().collect();
        self
    }

    /// Adds a [`ManaAllotment`] to a [`TransactionBuilder`].
    pub fn add_mana_allotment(mut self, allotment: ManaAllotment) -> Self {
        self.allotments.insert(allotment);
        self
    }

    /// Replaces a [`ManaAllotment`] of the [`TransactionBuilder`] with a new one, or adds it.
    pub fn replace_mana_allotment(mut self, allotment: ManaAllotment) -> Self {
        self.allotments.replace(allotment);
        self
    }

    pub fn with_capabilities(mut self, capabilities: impl Into<TransactionCapabilities>) -> Self {
        self.capabilities = capabilities.into();
        self
    }

    pub fn add_capabilities(mut self, capabilities: impl IntoIterator<Item = TransactionCapabilityFlag>) -> Self {
        self.capabilities.add_capabilities(capabilities);
        self
    }

    /// Sets the payload of a [`TransactionBuilder`].
    pub fn with_payload(mut self, payload: impl Into<OptionalPayload>) -> Self {
        self.payload = payload.into();
        self
    }

    /// Sets the outputs of a [`TransactionBuilder`].
    pub fn with_outputs(mut self, outputs: impl IntoIterator<Item = Output>) -> Self {
        self.outputs = outputs.into_iter().collect();
        self
    }

    /// Adds an output to a [`TransactionBuilder`].
    pub fn add_output(mut self, output: Output) -> Self {
        self.outputs.push(output);
        self
    }

    /// Finishes a [`TransactionBuilder`] into a [`Transaction`].
    pub fn finish_with_params<'a>(
        self,
        params: impl Into<Option<&'a ProtocolParameters>>,
    ) -> Result<Transaction, PayloadError> {
        let params = params.into();

        if let Some(protocol_parameters) = params {
            if self.network_id != protocol_parameters.network_id() {
                return Err(PayloadError::NetworkIdMismatch {
                    expected: protocol_parameters.network_id(),
                    actual: self.network_id,
                });
            }
        }

        let creation_slot = self
            .creation_slot
            .or_else(|| {
                #[cfg(feature = "std")]
                let creation_slot = params.map(|params| {
                    params.slot_index(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    )
                });
                #[cfg(not(feature = "std"))]
                let creation_slot = None;
                creation_slot
            })
            .ok_or(PayloadError::MissingCreationSlot)?;

        let inputs: BoxedSlicePrefix<Input, InputCount> = self
            .inputs
            .into_boxed_slice()
            .try_into()
            .map_err(PayloadError::InputCount)?;

        verify_inputs(&inputs)?;

        let allotments = ManaAllotments::from_set(self.allotments)?;

        if let Some(protocol_parameters) = params {
            verify_mana_allotments_sum(allotments.iter(), protocol_parameters)?;
        }

        verify_payload(&self.payload)?;

        let outputs: BoxedSlicePrefix<Output, OutputCount> = self
            .outputs
            .into_boxed_slice()
            .try_into()
            .map_err(PayloadError::OutputCount)?;

        if let Some(protocol_parameters) = params {
            verify_outputs(&outputs, protocol_parameters)?;
        }

        Ok(Transaction {
            network_id: self.network_id,
            creation_slot,
            context_inputs: ContextInputs::from_vec(self.context_inputs)?,
            inputs,
            allotments,
            capabilities: self.capabilities,
            payload: self.payload,
            outputs,
        })
    }

    /// Finishes a [`TransactionBuilder`] into a [`Transaction`] without protocol
    /// validation.
    pub fn finish(self) -> Result<Transaction, PayloadError> {
        self.finish_with_params(None)
    }
}

pub(crate) type InputCount = BoundedU16<{ *INPUT_COUNT_RANGE.start() }, { *INPUT_COUNT_RANGE.end() }>;
pub(crate) type OutputCount = BoundedU16<{ *OUTPUT_COUNT_RANGE.start() }, { *OUTPUT_COUNT_RANGE.end() }>;

/// A transaction consuming inputs, creating outputs and carrying an optional payload.
#[derive(Clone, Debug, Eq, PartialEq, Packable)]
#[packable(unpack_error = PayloadError)]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct Transaction {
    /// The unique value denoting whether the block was meant for mainnet, testnet, or a private network.
    #[packable(verify_with = verify_network_id)]
    network_id: u64,
    /// The slot index in which the transaction was created.
    creation_slot: SlotIndex,
    context_inputs: ContextInputs,
    #[packable(verify_with = verify_inputs_packable)]
    #[packable(unpack_error_with = unpack_inputs_err)]
    inputs: BoxedSlicePrefix<Input, InputCount>,
    allotments: ManaAllotments,
    capabilities: TransactionCapabilities,
    #[packable(verify_with = verify_payload_packable)]
    payload: OptionalPayload,
    #[packable(verify_with = verify_outputs)]
    #[packable(unpack_error_with = unpack_outputs_err)]
    outputs: BoxedSlicePrefix<Output, OutputCount>,
}

fn unpack_inputs_err<E: Into<<InputCount as TryFrom<usize>>::Error>>(
    e: UnpackPrefixError<InputError, E>,
) -> PayloadError {
    match e {
        UnpackPrefixError::Item(i) => i.into(),
        UnpackPrefixError::Prefix(p) => PayloadError::InputCount(p.into()),
    }
}

fn unpack_outputs_err<E: Into<<OutputCount as TryFrom<usize>>::Error>>(
    e: UnpackPrefixError<OutputError, E>,
) -> PayloadError {
    match e {
        UnpackPrefixError::Item(i) => i.into(),
        UnpackPrefixError::Prefix(p) => PayloadError::OutputCount(p.into()),
    }
}

impl Transaction {
    /// Creates a new [`TransactionBuilder`] to build a [`Transaction`].
    pub fn builder(network_id: u64) -> TransactionBuilder {
        TransactionBuilder::new(network_id)
    }

    /// Returns the network ID of a [`Transaction`].
    pub fn network_id(&self) -> u64 {
        self.network_id
    }

    /// Returns the slot index in which the [`Transaction`] was created.
    pub fn creation_slot(&self) -> SlotIndex {
        self.creation_slot
    }

    /// Returns the context inputs of a [`Transaction`].
    pub fn context_inputs(&self) -> &ContextInputs {
        &self.context_inputs
    }

    /// Returns the inputs of a [`Transaction`].
    pub fn inputs(&self) -> &[Input] {
        &self.inputs
    }

    /// Returns the [`ManaAllotment`]s of a [`Transaction`].
    pub fn allotments(&self) -> &ManaAllotments {
        &self.allotments
    }

    pub fn capabilities(&self) -> &TransactionCapabilities {
        &self.capabilities
    }

    /// Returns whether a given [`TransactionCapabilityFlag`] is enabled.
    pub fn has_capability(&self, flag: TransactionCapabilityFlag) -> bool {
        self.capabilities.has_capability(flag)
    }

    /// Returns the optional payload of a [`Transaction`].
    pub fn payload(&self) -> Option<&Payload> {
        self.payload.as_ref()
    }

    /// Returns the outputs of a [`Transaction`].
    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }

    /// Return the Blake2b hash of the transaction that can be used to create a
    /// [`SignedTransactionPayload`](crate::types::block::payload::SignedTransactionPayload).
    pub fn signing_hash(&self) -> TransactionSigningHash {
        TransactionSigningHash::new(Blake2b256::digest(self.pack_to_vec()).into())
    }

    /// Return the Blake2b hash of the transaction commitment and output commitment.
    fn hash(&self) -> TransactionHash {
        TransactionHash::new(
            Blake2b256::digest([self.transaction_commitment(), self.output_commitment()].concat()).into(),
        )
    }

    /// Returns the transaction commitment.
    /// I.E. The hash of the serialized transaction excluding the outputs.
    fn transaction_commitment(&self) -> [u8; 32] {
        let mut packer = Vec::new();
        self.network_id.pack(&mut packer).unwrap();
        self.creation_slot.pack(&mut packer).unwrap();
        self.context_inputs.pack(&mut packer).unwrap();
        self.inputs.pack(&mut packer).unwrap();
        self.allotments.pack(&mut packer).unwrap();
        self.capabilities.pack(&mut packer).unwrap();
        self.payload.pack(&mut packer).unwrap();
        Blake2b256::digest(packer).into()
    }

    /// Returns the transaction's output commitment, which is the root of the
    /// merkle tree that contains the transaction's serialized outputs as leaves.
    fn output_commitment(&self) -> [u8; 32] {
        let outputs_serialized = self.outputs.iter().map(|o| o.pack_to_vec()).collect::<Vec<_>>();
        merkle_hasher::MerkleHasher::digest::<Blake2b256>(&outputs_serialized).into()
    }

    /// Returns a proof for the output in the transaction at the given index,
    /// if the transaction has an output at that index.
    pub fn output_id_proof(&self, index: u16) -> Result<OutputIdProof, ProofError> {
        let output_commitment_proof = OutputCommitmentProof::new(&self.outputs, index)?;
        Ok(OutputIdProof {
            slot: self.creation_slot(),
            output_index: index,
            transaction_commitment: self.transaction_commitment(),
            output_commitment_proof,
        })
    }

    /// Computes the identifier of a [`Transaction`].
    pub fn id(&self) -> TransactionId {
        self.hash().into_transaction_id(self.creation_slot())
    }
}

impl WorkScore for Transaction {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        self.context_inputs().work_score(params)
            + self.inputs().work_score(params)
            + self.allotments().work_score(params)
            + self.outputs().work_score(params)
    }
}

fn verify_network_id(network_id: &u64, visitor: &ProtocolParameters) -> Result<(), PayloadError> {
    let expected = visitor.network_id();

    if *network_id != expected {
        return Err(PayloadError::NetworkIdMismatch {
            expected,
            actual: *network_id,
        });
    }

    Ok(())
}

fn verify_inputs(inputs: &[Input]) -> Result<(), PayloadError> {
    let mut seen_utxos = HashSet::new();

    for input in inputs.iter() {
        let Input::Utxo(utxo) = input;
        if !seen_utxos.insert(utxo) {
            return Err(PayloadError::DuplicateUtxo(*utxo));
        }
    }

    Ok(())
}

fn verify_inputs_packable(inputs: &[Input], _visitor: &ProtocolParameters) -> Result<(), PayloadError> {
    verify_inputs(inputs)?;
    Ok(())
}

fn verify_payload(payload: &OptionalPayload) -> Result<(), PayloadError> {
    match &payload.0 {
        Some(Payload::TaggedData(_)) | None => Ok(()),
        Some(payload) => Err(PayloadError::Kind(payload.kind())),
    }
}

fn verify_payload_packable(payload: &OptionalPayload, _visitor: &ProtocolParameters) -> Result<(), PayloadError> {
    verify_payload(payload)?;
    Ok(())
}

fn verify_outputs(outputs: &[Output], visitor: &ProtocolParameters) -> Result<(), PayloadError> {
    let mut amount_sum: u64 = 0;
    let mut chain_ids = HashSet::new();

    for output in outputs.iter() {
        let (amount, chain_id) = match output {
            Output::Basic(output) => (output.amount(), None),
            Output::Account(output) => (output.amount(), Some(output.chain_id())),
            Output::Anchor(output) => (output.amount(), Some(output.chain_id())),
            Output::Foundry(output) => (output.amount(), Some(output.chain_id())),
            Output::Nft(output) => (output.amount(), Some(output.chain_id())),
            Output::Delegation(output) => (output.amount(), Some(output.chain_id())),
        };

        amount_sum = amount_sum
            .checked_add(amount)
            .ok_or(PayloadError::TransactionAmountSum(amount_sum as u128 + amount as u128))?;

        // Accumulated output balance must not exceed the total supply of tokens.
        if amount_sum > visitor.token_supply() {
            return Err(PayloadError::TransactionAmountSum(amount_sum as u128));
        }

        if let Some(chain_id) = chain_id {
            if !chain_id.is_null() && !chain_ids.insert(chain_id) {
                return Err(PayloadError::DuplicateOutputChain(chain_id));
            }
        }

        output.verify_storage_deposit(visitor.storage_score_parameters())?;
    }

    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum TransactionCapabilityFlag {
    BurnNativeTokens,
    BurnMana,
    DestroyAccountOutputs,
    DestroyAnchorOutputs,
    DestroyFoundryOutputs,
    DestroyNftOutputs,
}

impl TransactionCapabilityFlag {
    const BURN_NATIVE_TOKENS: u8 = 0b00000001;
    const BURN_MANA: u8 = 0b00000010;
    const DESTROY_ACCOUNT_OUTPUTS: u8 = 0b00000100;
    const DESTROY_ANCHOR_OUTPUTS: u8 = 0b00001000;
    const DESTROY_FOUNDRY_OUTPUTS: u8 = 0b00010000;
    const DESTROY_NFT_OUTPUTS: u8 = 0b00100000;
}

impl CapabilityFlag for TransactionCapabilityFlag {
    type Iterator = core::array::IntoIter<Self, 6>;

    fn as_byte(&self) -> u8 {
        match self {
            Self::BurnNativeTokens => Self::BURN_NATIVE_TOKENS,
            Self::BurnMana => Self::BURN_MANA,
            Self::DestroyAccountOutputs => Self::DESTROY_ACCOUNT_OUTPUTS,
            Self::DestroyAnchorOutputs => Self::DESTROY_ANCHOR_OUTPUTS,
            Self::DestroyFoundryOutputs => Self::DESTROY_FOUNDRY_OUTPUTS,
            Self::DestroyNftOutputs => Self::DESTROY_NFT_OUTPUTS,
        }
    }

    fn index(&self) -> usize {
        match self {
            Self::BurnNativeTokens
            | Self::BurnMana
            | Self::DestroyAccountOutputs
            | Self::DestroyAnchorOutputs
            | Self::DestroyFoundryOutputs
            | Self::DestroyNftOutputs => 0,
        }
    }

    fn all() -> Self::Iterator {
        [
            Self::BurnNativeTokens,
            Self::BurnMana,
            Self::DestroyAccountOutputs,
            Self::DestroyAnchorOutputs,
            Self::DestroyFoundryOutputs,
            Self::DestroyNftOutputs,
        ]
        .into_iter()
    }
}

pub type TransactionCapabilities = Capabilities<TransactionCapabilityFlag>;

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::string::{String, ToString};

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::{
        block::payload::{dto::PayloadDto, CandidacyAnnouncementPayload, SignedTransactionPayload},
        TryFromDto,
    };

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TransactionDto {
        pub network_id: String,
        pub creation_slot: SlotIndex,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub context_inputs: Vec<ContextInput>,
        pub inputs: Vec<Input>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub allotments: Vec<ManaAllotment>,
        #[serde(default, skip_serializing_if = "TransactionCapabilities::is_none")]
        pub capabilities: TransactionCapabilities,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<PayloadDto>,
        pub outputs: Vec<Output>,
    }

    impl From<&Transaction> for TransactionDto {
        fn from(value: &Transaction) -> Self {
            Self {
                network_id: value.network_id().to_string(),
                creation_slot: value.creation_slot(),
                context_inputs: value.context_inputs().to_vec(),
                inputs: value.inputs().to_vec(),
                allotments: value.allotments().to_vec(),
                capabilities: value.capabilities().clone(),
                payload: match value.payload() {
                    Some(p @ Payload::TaggedData(_)) => Some(p.into()),
                    Some(_) => unimplemented!(),
                    None => None,
                },
                outputs: value.outputs().to_vec(),
            }
        }
    }

    impl TryFromDto<TransactionDto> for Transaction {
        type Error = PayloadError;

        fn try_from_dto_with_params_inner(
            dto: TransactionDto,
            params: Option<&ProtocolParameters>,
        ) -> Result<Self, Self::Error> {
            let network_id = dto
                .network_id
                .parse::<u64>()
                .map_err(|e| PayloadError::NetworkId(e.to_string()))?;

            let mut builder = Self::builder(network_id)
                .with_creation_slot(dto.creation_slot)
                .with_context_inputs(dto.context_inputs)
                .with_inputs(dto.inputs)
                .with_mana_allotments(dto.allotments)
                .with_capabilities(dto.capabilities)
                .with_outputs(dto.outputs);

            builder = if let Some(p) = dto.payload {
                match p {
                    PayloadDto::TaggedData(i) => builder.with_payload(*i),
                    PayloadDto::SignedTransaction(_) => {
                        return Err(PayloadError::Kind(SignedTransactionPayload::KIND));
                    }
                    PayloadDto::CandidacyAnnouncement => {
                        return Err(PayloadError::Kind(CandidacyAnnouncementPayload::KIND));
                    }
                }
            } else {
                builder
            };

            builder.finish_with_params(params).map_err(Into::into)
        }
    }
}
