import { HexEncodedString } from "@iota/types";
import { Feature } from "../../../lib";
import { BasicOutputBuilderOptions } from "./basicOutputOptions";


/**
 * Options for building an Alias Output
 */
export interface AliasOutputBuilderOptions extends BasicOutputBuilderOptions {
    aliasId: HexEncodedString;
    stateIndex?: number;
    stateMetadata?: HexEncodedString;
    foundryCounter?: number;
    immutableFeatures?: Feature[];
}
