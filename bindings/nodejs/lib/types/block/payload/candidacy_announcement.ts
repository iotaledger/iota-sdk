import { Payload, PayloadType } from './payload';

/**
 * A payload which is used to indicate candidacy for committee selection for the next epoch.
 */
export class CandidacyAnnouncementPayload extends Payload {
    constructor() {
        super(PayloadType.CandidacyAnnouncement);
    }
}
