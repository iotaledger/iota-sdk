// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

enum OutputType {
    A = 0,
}

class Output {
    type: OutputType;
    hexEncodedString: String;

    constructor(type: OutputType, hexEncodedString: String) {
        this.type = type;
        this.hexEncodedString = hexEncodedString;
    }
}

class AOutput extends Output {
    constructor(address: String) {
        super(OutputType.A, address);
    }
}

export { Output, AOutput };
