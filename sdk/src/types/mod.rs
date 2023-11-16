// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Common types required by nodes and clients APIs like blocks, responses and DTOs.

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "serde")]
pub mod api;
pub mod block;

use self::block::protocol::ProtocolParameters;

pub trait TryFromDto<Dto>: Sized {
    type Error;

    fn try_from_dto(dto: Dto) -> Result<Self, Self::Error> {
        Self::try_from_dto_with_params_inner(dto, None)
    }

    fn try_from_dto_with_params(dto: Dto, params: &ProtocolParameters) -> Result<Self, Self::Error> {
        Self::try_from_dto_with_params_inner(dto, Some(params))
    }

    fn try_from_dto_with_params_inner(dto: Dto, params: Option<&ProtocolParameters>) -> Result<Self, Self::Error>;
}
