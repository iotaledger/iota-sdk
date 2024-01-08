import {
    Ed25519Signature,
    HexEncodedString,
    Transaction,
    SignedTransactionPayload,
    TransactionId,
    TokenSchemeType,
    Output,
    StorageScoreParameters,
    Block,
    ProtocolParameters,
    OutputId,
    NftId,
    Bech32Address,
    Unlock,
} from '../../';
import { AccountId } from '../../block/id';
import { SlotCommitment } from '../../block/slot';
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

export interface __ComputeAccountIdMethod__ {
    name: 'computeAccountId';
    data: {
        outputId: OutputId;
    };
}

export interface __ComputeFoundryIdMethod__ {
    name: 'computeFoundryId';
    data: {
        accountId: AccountId;
        serialNumber: number;
        tokenSchemeType: number;
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
        storageScoreParameters: StorageScoreParameters;
    };
}

export interface __ComputeTokenIdMethod__ {
    name: 'computeTokenId';
    data: {
        accountId: AccountId;
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
        protocolParameters: ProtocolParameters;
    };
}

export interface __TransactionIdMethod__ {
    name: 'transactionId';
    data: {
        payload: SignedTransactionPayload;
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

export interface __AccountIdToBech32Method__ {
    name: 'accountIdToBech32';
    data: {
        accountId: AccountId;
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

export interface __ProtocolParametersHashMethod__ {
    name: 'protocolParametersHash';
    data: {
        protocolParameters: ProtocolParameters;
    };
}

export interface __TransactionSigningHashMethod__ {
    name: 'transactionSigningHash';
    data: {
        transaction: Transaction;
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

// TODO we don't do this anywhere else, but it seems necessary, need to reevaluate later.
// Modified `SlotCommitment` with bigint types converted to strings.
type SlotCommitmentConverted = Omit<
    SlotCommitment,
    'cumulativeWeight' | 'referenceManaCost'
> & { cumulativeWeight: string; referenceManaCost: string };
export interface __ComputeSlotCommitmentId__ {
    name: 'computeSlotCommitmentId';
    data: {
        slotCommitment: SlotCommitmentConverted;
    };
}

export interface __VerifyTransactionSemantic__ {
    name: 'verifyTransactionSemantic';
    data: {
        transaction: SignedTransactionPayload;
        inputs: InputSigningData[];
        unlocks?: Unlock[];
    };
}
