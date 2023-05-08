// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node public participation routes.
//! <https://github.com/iota-community/treasury/blob/main/specifications/hornet-participation-plugin.md#public-node-endpoints>
//! <https://github.com/iotaledger/inx-participation/blob/develop/core/participation/routes.go>

use crate::{
    client::{Client, Result},
    types::{
        api::plugins::participation::{
            responses::{AddressOutputsResponse, EventsResponse, OutputStatusResponse},
            types::{
                AddressStakingStatus, ParticipationEventData, ParticipationEventId, ParticipationEventStatus,
                ParticipationEventType,
            },
        },
        block::{address::Bech32AddressLike, output::OutputId},
    },
};

impl Client {
    /// RouteParticipationEvents is the route to list all events, returning their ID, the event name and status.
    pub async fn events(&self, event_type: Option<ParticipationEventType>) -> Result<EventsResponse> {
        let route = "api/participation/v1/events";

        let query = event_type.map(|event_type| match event_type {
            ParticipationEventType::Voting => "type=0",
            ParticipationEventType::Staking => "type=1",
        });

        self.inner
            .node_manager
            .get_request(route, query, self.get_timeout(), false, false)
            .await
    }

    /// RouteParticipationEvent is the route to access a single participation by its ID.
    pub async fn event(&self, event_id: &ParticipationEventId) -> Result<ParticipationEventData> {
        let route = format!("api/participation/v1/events/{event_id}");

        self.inner
            .node_manager
            .get_request(&route, None, self.get_timeout(), false, false)
            .await
    }

    /// RouteParticipationEventStatus is the route to access the status of a single participation by its ID.
    pub async fn event_status(
        &self,
        event_id: &ParticipationEventId,
        milestone_index: Option<u32>,
    ) -> Result<ParticipationEventStatus> {
        let route = format!("api/participation/v1/events/{event_id}/status");

        self.inner
            .node_manager
            .get_request(
                &route,
                milestone_index.map(|index| index.to_string()).as_deref(),
                self.get_timeout(),
                false,
                false,
            )
            .await
    }

    /// RouteOutputStatus is the route to get the vote status for a given output ID.
    pub async fn output_status(&self, output_id: &OutputId) -> Result<OutputStatusResponse> {
        let route = format!("api/participation/v1/outputs/{output_id}");

        self.inner
            .node_manager
            .get_request(&route, None, self.get_timeout(), false, false)
            .await
    }

    /// RouteAddressBech32Status is the route to get the staking rewards for the given bech32 address.
    pub async fn address_staking_status(&self, bech32_address: impl Bech32AddressLike) -> Result<AddressStakingStatus> {
        let route = format!("api/participation/v1/addresses/{}", bech32_address.as_string());

        self.inner
            .node_manager
            .get_request(&route, None, self.get_timeout(), false, false)
            .await
    }

    /// RouteAddressBech32Outputs is the route to get the outputs for the given bech32 address.
    pub async fn address_participation_output_ids(
        &self,
        bech32_address: impl Bech32AddressLike,
    ) -> Result<AddressOutputsResponse> {
        let route = format!("api/participation/v1/addresses/{}/outputs", bech32_address.as_string());

        self.inner
            .node_manager
            .get_request(&route, None, self.get_timeout(), false, false)
            .await
    }
}
