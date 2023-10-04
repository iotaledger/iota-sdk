/**
 * Convert arrays to and from different formats.
 */
export class Converter {
    /**
     * Lookup table for encoding.
     * @internal
     */
    private static ENCODE_LOOKUP: string[] | undefined;

    /**
     * Lookup table for decoding.
     * @internal
     */
    private static DECODE_LOOKUP: number[] | undefined;

    /**
     * Encode a raw array to hex string.
     * @param array The bytes to encode.
     * @param includePrefix Include the 0x prefix on the returned hex.
     * @param startIndex The index to start in the bytes.
     * @param length The length of bytes to read.
     * @param reverse Reverse the combine direction.
     * @returns The array formated as hex.
     */
    public static bytesToHex(
        array: ArrayLike<number>,
        includePrefix: boolean = false,
        startIndex?: number,
        length?: number | undefined,
        reverse?: boolean,
    ): string {
        let hex = '';
        this.buildHexLookups();
        if (Converter.ENCODE_LOOKUP) {
            const len = length ?? array.length;
            const start = startIndex ?? 0;
            if (reverse) {
                for (let i = 0; i < len; i++) {
                    hex = Converter.ENCODE_LOOKUP[array[start + i]] + hex;
                }
            } else {
                for (let i = 0; i < len; i++) {
                    hex += Converter.ENCODE_LOOKUP[array[start + i]];
                }
            }
        }
        return includePrefix ? hex.replace(/^0x/, '') : hex;
    }

    /**
     * Decode a hex string to raw array.
     * @param hex The hex to decode.
     * @param reverse Store the characters in reverse.
     * @returns The array.
     */
    public static hexToBytes(hex: string, reverse?: boolean): Uint8Array {
        const strippedHex = hex.replace(/^0x/, '');
        const sizeof = strippedHex.length >> 1;
        const length = sizeof << 1;
        const array = new Uint8Array(sizeof);

        this.buildHexLookups();
        if (Converter.DECODE_LOOKUP) {
            let i = 0;
            let n = 0;
            while (i < length) {
                array[n++] =
                    (Converter.DECODE_LOOKUP[strippedHex.charCodeAt(i++)] <<
                        4) |
                    Converter.DECODE_LOOKUP[strippedHex.charCodeAt(i++)];
            }

            if (reverse) {
                array.reverse();
            }
        }
        return array;
    }

    /**
     * Build the static lookup tables.
     * @internal
     */
    private static buildHexLookups(): void {
        if (!Converter.ENCODE_LOOKUP) {
            const alphabet = '0123456789abcdef';
            Converter.ENCODE_LOOKUP = [];
            Converter.DECODE_LOOKUP = [];

            for (let i = 0; i < 256; i++) {
                Converter.ENCODE_LOOKUP[i] =
                    alphabet[(i >> 4) & 0xf] + alphabet[i & 0xf];
                if (i < 16) {
                    if (i < 10) {
                        Converter.DECODE_LOOKUP[0x30 + i] = i;
                    } else {
                        Converter.DECODE_LOOKUP[0x61 - 10 + i] = i;
                    }
                }
            }
        }
    }
}
