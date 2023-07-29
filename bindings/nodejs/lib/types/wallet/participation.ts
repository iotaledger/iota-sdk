// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { INode } from '../client';
import type { OutputId } from '../block/output';

/**
 * TODO.
 */
export interface ParticipationOverview {
    /** TODO */
    participations: Participations;
}

/**
 * TODO.
 */
export interface Participations {
    [eventId: ParticipationEventId]: {
        [outputId: OutputId]: TrackedParticipationOverview;
    };
}

/**
 * TODO.
 */
export interface TrackedParticipationOverview {
    /** TODO */
    amount: string;
    /** TODO */
    answers: number[];
    /** TODO */
    blockId: string;
    /** TODO */
    endMilestoneIndex: number;
    /** TODO */
    startMilestoneIndex: number;
}

/**
 * TODO.
 */
export interface ParticipationEvent {
    /** TODO */
    id: ParticipationEventId;
    /** TODO */
    data: ParticipationEventData;
}

/**
 * TODO.
 */
export interface ParticipationEventRegistrationOptions {
    /** TODO */
    node: INode;
    /** TODO */
    eventsToRegister?: ParticipationEventId[];
    /** TODO */
    eventsToIgnore?: ParticipationEventId[];
}

/**
 * TODO.
 */
export interface ParticipationEventWithNodes {
    /** TODO */
    id: ParticipationEventId;
    /** TODO */
    data: ParticipationEventData;
    /** TODO */
    nodes: INode[];
}

/**
 * TODO.
 */
export type ParticipationEventId = string;

/**
 * TODO.
 */
export type ParticipationEventMap = {
    [id: ParticipationEventId]: ParticipationEventWithNodes;
};

/**
 * TODO.
 */
export interface ParticipationEventStatus {
    /** TODO */
    milestoneIndex: number;
    /** TODO */
    status: EventStatus;
    /** TODO */
    questions?: QuestionStatus[];
    /** TODO */
    checksum: string;
}

/**
 * TODO.
 */
export enum EventStatus {
    /** TODO */
    Upcoming = 'upcoming',
    /** TODO */
    Commencing = 'commencing',
    /** TODO */
    Holding = 'holding',
    /** TODO */
    Ended = 'ended',
}

/**
 * TODO.
 */
export interface ParticipationEventData {
    /** TODO */
    name: string;
    /** TODO */
    milestoneIndexCommence: number;
    /** TODO */
    milestoneIndexStart: number;
    /** TODO */
    milestoneIndexEnd: number;
    /** TODO */
    payload: ParticipationEventPayload;
    /** TODO */
    additionalInfo: string;
}

/**
 * TODO.
 */
export type ParticipationEventPayload =
    | VotingEventPayload
    | StakingEventPayload;

/**
 * TODO.
 */
export interface VotingEventPayload {
    /** TODO */
    type: ParticipationEventType.Voting;
    /** TODO */
    questions: Question[];
}

/**
 * TODO.
 */
export interface StakingEventPayload {
    /** TODO */
    type: ParticipationEventType.Staking;
    /** TODO */
    text: string;
    /** TODO */
    symbol: string;
    /** TODO */
    numerator: string;
    /** TODO */
    denominator: string;
    /** TODO */
    requiredMinimumRewards: string;
    /** TODO */
    additionalInfo: string;
}

/**
 * TODO.
 */
export interface Question {
    /** TODO */
    text: string;
    /** TODO */
    answers: Answer[];
    /** TODO */
    additionalInfo: string;
}

/**
 * TODO.
 */
export interface Answer {
    /** TODO */
    value: number;
    /** TODO */
    text: string;
    /** TODO */
    additionalInfo: string;
}

/**
 * TODO.
 */
export interface QuestionStatus {
    /** TODO */
    answers: AnswerStatus[];
}

/**
 * TODO.
 */
export interface AnswerStatus {
    /** TODO */
    value: number;
    /** TODO */
    current: number;
    /** TODO */
    accumulated: number;
}

/**
 * TODO.
 */
export enum ParticipationEventType {
    /** TODO */
    Voting = 0,
    /** TODO */
    Staking = 1,
}
