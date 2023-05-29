import type { HexEncodedString } from '../utils/hexEncodedTypes';
import { Payload } from './payload';
/**
 * The default protocol version.
 */
export declare const DEFAULT_PROTOCOL_VERSION: number;
/**
 * Block layout.
 */
export interface IBlock {
    /**
     * The protocol version under which this block operates.
     */
    protocolVersion: number;
    /**
     * The parent block ids.
     */
    parents: HexEncodedString[];
    /**
     * The payload contents.
     */
    payload?: Payload;
    /**
     * The nonce for the block.
     */
    nonce: string;
}
