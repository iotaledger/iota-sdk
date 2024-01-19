import {
    Ed25519Signature,
    HexEncodedString,
    Block,
    TransactionEssence,
    MilestonePayload,
    TransactionPayload,
    TransactionId,
    TokenSchemeType,
    Output,
    IRent,
    OutputId,
    NftId,
    Bech32Address,
    INodeInfoProtocol,
} from '../../';
import { AliasId } from '../../block/id';
import { InputSigningData } from '../../client';

export interface __GenerateMnemonicMethod__ {
    name: 'generateMnemonic';
}

export interface __MnemonicToHexSeedMethod__ {
    name: 'mnemonicToHexSeed';
    data: {
        mnemonic: string;
    };
}

export interface __ComputeAliasIdMethod__ {
    name: 'computeAliasId';
    data: {
        outputId: OutputId;
    };
}

export interface __ComputeFoundryIdMethod__ {
    name: 'computeFoundryId';
    data: {
        aliasId: AliasId;
        serialNumber: number;
        tokenSchemeType: number;
    };
}

export interface __ComputeInputsCommitmentMethod__ {
    name: 'computeInputsCommitment';
    data: {
        inputs: Output[];
    };
}

export interface __ComputeNftIdMethod__ {
    name: 'computeNftId';
    data: {
        outputId: OutputId;
    };
}

export interface __ComputeOutputIdMethod__ {
    name: 'computeOutputId';
    data: {
        id: TransactionId;
        index: number;
    };
}

export interface __ComputeStorageDepositMethod__ {
    name: 'computeStorageDeposit';
    data: {
        output: Output;
        rent: IRent;
    };
}

export interface __ComputeTokenIdMethod__ {
    name: 'computeTokenId';
    data: {
        aliasId: AliasId;
        serialNumber: number;
        tokenSchemeType: TokenSchemeType;
    };
}

export interface __ParseBech32AddressMethod__ {
    name: 'parseBech32Address';
    data: {
        address: Bech32Address;
    };
}

export interface __BlockIdMethod__ {
    name: 'blockId';
    data: {
        block: Block;
    };
}

export interface __MilestoneIdMethod__ {
    name: 'milestoneId';
    data: {
        payload: MilestonePayload;
    };
}

export interface __TransactionIdMethod__ {
    name: 'transactionId';
    data: {
        payload: TransactionPayload;
    };
}

export interface __Bech32ToHexMethod__ {
    name: 'bech32ToHex';
    data: {
        bech32: Bech32Address;
    };
}

export interface __HexToBech32Method__ {
    name: 'hexToBech32';
    data: {
        hex: HexEncodedString;
        bech32Hrp?: string;
    };
}

export interface __AliasIdToBech32Method__ {
    name: 'aliasIdToBech32';
    data: {
        aliasId: AliasId;
        bech32Hrp?: string;
    };
}

export interface __NftIdToBech32Method__ {
    name: 'nftIdToBech32';
    data: {
        nftId: NftId;
        bech32Hrp?: string;
    };
}

export interface __HexPublicKeyToBech32AddressMethod__ {
    name: 'hexPublicKeyToBech32Address';
    data: {
        hex: HexEncodedString;
        bech32Hrp?: string;
    };
}

export interface __IsAddressValidMethod__ {
    name: 'isAddressValid';
    data: {
        address: string;
    };
}

export interface __HashTransactionEssenceMethod__ {
    name: 'hashTransactionEssence';
    data: {
        essence: TransactionEssence;
    };
}

export interface __VerifyEd25519SignatureMethod__ {
    name: 'verifyEd25519Signature';
    data: {
        signature: Ed25519Signature;
        message: HexEncodedString;
    };
}

export interface __VerifySecp256k1EcdsaSignatureMethod__ {
    name: 'verifySecp256k1EcdsaSignature';
    data: {
        publicKey: HexEncodedString;
        signature: HexEncodedString;
        message: HexEncodedString;
    };
}

export type __VerifyMnemonicMethod__ = {
    name: 'verifyMnemonic';
    data: { mnemonic: string };
};

export type __FaucetMethod__ = {
    name: 'faucet';
    data: {
        url: string;
        address: string;
    };
};

export interface __OutputIdToUtxoInput__ {
    name: 'outputIdToUtxoInput';
    data: {
        outputId: OutputId;
    };
}

export interface __OutputHexBytes__ {
    name: 'outputHexBytes';
    data: {
        output: Output;
    };
}

export interface __VerifyTransactionSemantic__ {
    name: 'verifyTransactionSemantic';
    data: {
        inputs: InputSigningData[];
        transaction: TransactionPayload;
        time: number;
    };
}

export interface __VerifyTransactionSyntax__ {
    name: 'verifyTransactionSyntax';
    data: {
        transaction: TransactionPayload;
        protocolParameters: INodeInfoProtocol;
    };
}

export interface __BlockBytes__ {
    name: 'blockBytes';
    data: {
        block: Block;
    };
}

export interface __BlockHashWithoutNonce__ {
    name: 'blockHashWithoutNonce';
    data: {
        block: Block;
    };
}
