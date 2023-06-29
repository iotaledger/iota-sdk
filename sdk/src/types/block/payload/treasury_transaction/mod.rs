// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Module describing the treasury payload.

use crate::types::block::{
    input::{Input, TreasuryInput},
    output::{Output, TreasuryOutput},
    protocol::ProtocolParameters,
    Error,
};

/// [`TreasuryTransactionPayload`] represents a transaction which moves funds from the treasury.
#[derive(Clone, Debug, Eq, PartialEq, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct TreasuryTransactionPayload {
    #[packable(verify_with = verify_input)]
    input: Input,
    #[packable(verify_with = verify_output)]
    output: Output,
}

impl TreasuryTransactionPayload {
    /// The payload kind of a [`TreasuryTransactionPayload`].
    pub const KIND: u32 = 4;

    /// Creates a new [`TreasuryTransactionPayload`].
    pub fn new(input: TreasuryInput, output: TreasuryOutput) -> Result<Self, Error> {
        Ok(Self {
            input: input.into(),
            output: output.into(),
        })
    }

    /// Returns the input of a [`TreasuryTransactionPayload`].
    pub fn input(&self) -> &TreasuryInput {
        if let Input::Treasury(ref input) = self.input {
            input
        } else {
            // It has already been validated at construction that `input` is a `TreasuryInput`.
            unreachable!()
        }
    }

    /// Returns the output of a [`TreasuryTransactionPayload`].
    pub fn output(&self) -> &TreasuryOutput {
        // It has already been validated at construction that `output` is a `TreasuryOutput`.
        self.output.as_treasury()
    }
}

fn verify_input<const VERIFY: bool>(input: &Input, _: &ProtocolParameters) -> Result<(), Error> {
    if VERIFY && !matches!(input, Input::Treasury(_)) {
        Err(Error::InvalidInputKind(input.kind()))
    } else {
        Ok(())
    }
}

fn verify_output<const VERIFY: bool>(output: &Output, _: &ProtocolParameters) -> Result<(), Error> {
    if VERIFY && !matches!(output, Output::Treasury(_)) {
        Err(Error::InvalidOutputKind(output.kind()))
    } else {
        Ok(())
    }
}

#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::{
        input::dto::{InputDto, TreasuryInputDto},
        output::dto::{OutputDto, TreasuryOutputDto},
        Error,
    };

    /// The payload type to define a treasury transaction.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct TreasuryTransactionPayloadDto {
        #[serde(rename = "type")]
        pub kind: u32,
        pub input: InputDto,
        pub output: OutputDto,
    }

    impl From<&TreasuryTransactionPayload> for TreasuryTransactionPayloadDto {
        fn from(value: &TreasuryTransactionPayload) -> Self {
            Self {
                kind: TreasuryTransactionPayload::KIND,
                input: InputDto::Treasury(TreasuryInputDto::from(value.input())),
                output: OutputDto::Treasury(TreasuryOutputDto::from(value.output())),
            }
        }
    }

    impl TreasuryTransactionPayload {
        fn _try_from_dto(value: TreasuryTransactionPayloadDto, output: TreasuryOutput) -> Result<Self, Error> {
            Self::new(
                if let InputDto::Treasury(input) = value.input {
                    input.try_into()?
                } else {
                    return Err(Error::InvalidField("input"));
                },
                output,
            )
        }

        pub fn try_from_dto(value: TreasuryTransactionPayloadDto, token_supply: u64) -> Result<Self, Error> {
            if let OutputDto::Treasury(output) = value.output {
                if let InputDto::Treasury(input) = value.input {
                    Self::new(input.try_into()?, TreasuryOutput::try_from_dto(output, token_supply)?)
                } else {
                    Err(Error::InvalidField("input"))
                }
            } else {
                Err(Error::InvalidField("output"))
            }
        }

        pub fn try_from_dto_unverified(value: TreasuryTransactionPayloadDto) -> Result<Self, Error> {
            if let OutputDto::Treasury(output) = value.output {
                if let InputDto::Treasury(input) = value.input {
                    Self::new(input.try_into()?, TreasuryOutput::try_from_dto_unverified(output)?)
                } else {
                    Err(Error::InvalidField("input"))
                }
            } else {
                Err(Error::InvalidField("output"))
            }
        }
    }
}
