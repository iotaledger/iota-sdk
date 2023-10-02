// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { INode } from '../client';
import type { OutputId } from '../block/output';

/**
 * An object containing an account's entire participation overview.
 */
export interface ParticipationOverview {
    participations: Participations;
}

/**
 * Output participations for events.
 */
export interface Participations {
    [eventId: ParticipationEventId]: {
        [outputId: OutputId]: TrackedParticipationOverview;
    };
}

/**
 * Holds the information for each tracked participation.
 */
export interface TrackedParticipationOverview {
    /** Amount of tokens that were included in the output the participation was made. */
    amount: string;
    /** IDs of the answers to the questions of a ballot, in the same order. */
    answers: number[];
    /** ID of the block that included the transaction that created the output the participation was made. */
    blockId: string;
    /** Milestone index the participation ended. 0 if the participation is still active. */
    endMilestoneIndex: number;
    /** Milestone index the participation started. */
    startMilestoneIndex: number;
}

/**
 * A participation event.
 */
export interface ParticipationEvent {
    /** The event ID. */
    id: ParticipationEventId;
    /** Information about a voting or staking event. */
    data: ParticipationEventData;
}

/**
 * Options when registering participation events.
 */
export interface ParticipationEventRegistrationOptions {
    /** The node to register participation events. */
    node: INode;
    /**
     * The events to register.
     * If empty, then every event being tracked by the node will be registered. */
    eventsToRegister?: ParticipationEventId[];
    /** The events to ignore. */
    eventsToIgnore?: ParticipationEventId[];
}

/**
 * A participation event with the provided client nodes.
 */
export interface ParticipationEventWithNodes {
    /** The event id */
    id: ParticipationEventId;
    /** Information about a voting or staking event */
    data: ParticipationEventData;
    /** Provided client nodes for this event. */
    nodes: INode[];
}

/**
 * A participation event ID represented as hex-encoded string.
 */
export type ParticipationEventId = string;

/**
 * Maps participation event IDs to their corresponding event data with nodes.
 */
export type ParticipationEventMap = {
    [id: ParticipationEventId]: ParticipationEventWithNodes;
};

/**
 * The participation event status.
 */
export interface ParticipationEventStatus {
    /** The milestone index the status was calculated for. */
    milestoneIndex: number;
    /** The overall status of the event. */
    status: EventStatus;
    /** The answer status of the different questions of the event. */
    questions?: QuestionStatus[];
    /** Checksum is the SHA256 checksum of all the question and answer status or the staking amount and rewards calculated for this milestone index. */
    checksum: string;
}

/**
 * All possible event status.
 */
export enum EventStatus {
    Upcoming = 'upcoming',
    Commencing = 'commencing',
    Holding = 'holding',
    Ended = 'ended',
}

/**
 * Metadata about a participation event.
 */
export interface ParticipationEventData {
    /** The name of the event. */
    name: string;
    /** The milestone index the commencing period starts. */
    milestoneIndexCommence: number;
    /** The milestone index the holding period starts. */
    milestoneIndexStart: number;
    /** The milestone index the event ends. */
    milestoneIndexEnd: number;
    /** The payload of the event (voting or staking). */
    payload: ParticipationEventPayload;
    /** Some additional description text about the event. */
    additionalInfo: string;
}

/**
 * Possible participation event payloads (voting or staking).
 */
export type ParticipationEventPayload =
    | VotingEventPayload
    | StakingEventPayload;

/**
 * A voting event payload.
 */
export interface VotingEventPayload {
    /** The type of the event (voting). */
    type: ParticipationEventType.Voting;
    /** The questions to vote on. */
    questions: Question[];
}

/**
 * A staking event payload.
 */
export interface StakingEventPayload {
    /** The type of the event (statking). */
    type: ParticipationEventType.Staking;
    /** The description text of the staking event. */
    text: string;
    /** The symbol of the rewarded tokens. */
    symbol: string;
    /** Used in combination with Denominator to calculate the rewards per milestone per staked tokens. */
    numerator: string;
    /** Used in combination with Numerator to calculate the rewards per milestone per staked tokens. */
    denominator: string;
    /** The minimum rewards required to be included in the staking results. */
    requiredMinimumRewards: string;
    /** Additional description text about the staking event. */
    additionalInfo: string;
}

/**
 * Defines a single question inside a Ballot that can have multiple answers.
 */
export interface Question {
    /** The text of the question. */
    text: string;
    /** The possible answers to the question. */
    answers: Answer[];
    /** Some additional description text about the question. */
    additionalInfo: string;
}

/**
 * The answer in a voting event.
 */
export interface Answer {
    /** The value that should be used to pick this answer. It must be unique for each answer in a given question. Reserved values are 0 and 255. */
    value: number;
    /** The text of the answer. */
    text: string;
    /** Some additional description text about the answer. */
    additionalInfo: string;
}

/**
 * The question status.
 */
export interface QuestionStatus {
    /** The status of the answers. */
    answers: AnswerStatus[];
}

/**
 * The answer status.
 */
export interface AnswerStatus {
    /** The value that identifies this answer. */
    value: number;
    /** The current voting weight of the answer. */
    current: number;
    /** The accumulated voting weight of the answer. */
    accumulated: number;
}

/**
 * The types of participation events.
 */
export enum ParticipationEventType {
    /** A voting event. */
    Voting = 0,
    /** A staking event. */
    Staking = 1,
}
