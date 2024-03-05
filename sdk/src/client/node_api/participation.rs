// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node public participation routes.
//! <https://github.com/iota-community/treasury/blob/main/specifications/hornet-participation-plugin.md#public-node-endpoints>
//! <https://github.com/iotaledger/inx-participation/blob/develop/components/participation/routes.go>

use crate::{
    client::{node_api::query_tuples_to_query_string, ClientError, ClientInner},
    types::{
        api::plugins::participation::{
            responses::{AddressOutputsResponse, EventsResponse, OutputStatusResponse},
            types::{
                AddressStakingStatus, ParticipationEventData, ParticipationEventId, ParticipationEventStatus,
                ParticipationEventType,
            },
        },
        block::{address::Bech32Address, output::OutputId},
    },
    utils::ConvertTo,
};

impl ClientInner {
    /// RouteParticipationEvents is the route to list all events, returning their ID, the event name and status.
    pub async fn events(&self, event_type: Option<ParticipationEventType>) -> Result<EventsResponse, ClientError> {
        let route = "api/participation/v1/events";

        let query = query_tuples_to_query_string([event_type.map(|t| ("type", (t as u8).to_string()))]);

        self.get_request(route, query.as_deref(), false).await
    }

    /// RouteParticipationEvent is the route to access a single participation by its ID.
    pub async fn event(&self, event_id: &ParticipationEventId) -> Result<ParticipationEventData, ClientError> {
        let route = format!("api/participation/v1/events/{event_id}");

        self.get_request(&route, None, false).await
    }

    /// RouteParticipationEventStatus is the route to access the status of a single participation by its ID.
    pub async fn event_status(
        &self,
        event_id: &ParticipationEventId,
        milestone_index: Option<u32>,
    ) -> Result<ParticipationEventStatus, ClientError> {
        let route = format!("api/participation/v1/events/{event_id}/status");

        let query = query_tuples_to_query_string([milestone_index.map(|i| ("milestoneIndex", i.to_string()))]);

        self.get_request(&route, query.as_deref(), false).await
    }

    /// RouteOutputStatus is the route to get the vote status for a given output ID.
    pub async fn output_status(&self, output_id: &OutputId) -> Result<OutputStatusResponse, ClientError> {
        let route = format!("api/participation/v1/outputs/{output_id}");

        self.get_request(&route, None, false).await
    }

    /// RouteAddressBech32Status is the route to get the staking rewards for the given bech32 address.
    pub async fn address_staking_status(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> Result<AddressStakingStatus, ClientError> {
        let route = format!("api/participation/v1/addresses/{}", bech32_address.convert()?);

        self.get_request(&route, None, false).await
    }

    /// RouteAddressBech32Outputs is the route to get the outputs for the given bech32 address.
    pub async fn address_participation_output_ids(
        &self,
        bech32_address: impl ConvertTo<Bech32Address>,
    ) -> Result<AddressOutputsResponse, ClientError> {
        let route = format!("api/participation/v1/addresses/{}/outputs", bech32_address.convert()?);

        self.get_request(&route, None, false).await
    }
}
