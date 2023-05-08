import type { IBlock, PayloadTypes } from '@iota/types';
import type { SecretManagerType } from '../../secretManager/secretManager';
import type { IGenerateAddressesOptions } from '../generateAddressesOptions';
import type { IBuildBlockOptions } from '../buildBlockOptions';
import type { BlockId } from '../../core/blockId';
import type {
    IPreparedTransactionData,
    IBip32Chain,
} from '../preparedTransactionData';
import type {
    AliasQueryParameter,
    FoundryQueryParameter,
    NftQueryParameter,
    QueryParameter,
} from '../queryParameters';
import type { IAuth } from '../network';
import type { BasicOutputBuilderOptions } from '../outputBuilderOptions/basicOutputOptions';
import type { AliasOutputBuilderOptions } from '../outputBuilderOptions/aliasOutputOptions';
import type { FoundryOutputBuilderOptions } from '../outputBuilderOptions/foundryOutputOptions';
import type { NftOutputBuilderOptions } from '../outputBuilderOptions/nftOutputOptions';

export interface __GetInfoMethod__ {
    name: 'getInfo';
}

export interface __GetOutputMethod__ {
    name: 'getOutput';
    data: {
        outputId: string;
    };
}

export interface __GetBasicOutputIdsMethod__ {
    name: 'basicOutputIds';
    data: {
        queryParameters: QueryParameter[];
    };
}

export interface __GetOutputsMethod__ {
    name: 'getOutputs';
    data: {
        outputIds: string[];
    };
}

export interface __GenerateAddressesMethod__ {
    name: 'generateAddresses';
    data: {
        secretManager: SecretManagerType;
        options: IGenerateAddressesOptions;
    };
}

export interface __PostBlockMethod__ {
    name: 'postBlock';
    data: {
        block: IBlock;
    };
}

export interface __BuildAndPostBlockMethod__ {
    name: 'buildAndPostBlock';
    data: {
        secretManager?: SecretManagerType;
        options?: IBuildBlockOptions;
    };
}

export interface __GetTipsMethod__ {
    name: 'getTips';
}

export interface __GetNetworkInfoMethod__ {
    name: 'getNetworkInfo';
}

export interface __GetBlockMethod__ {
    name: 'getBlock';
    data: {
        blockId: BlockId;
    };
}

export interface __GetBlockMetadataMethod__ {
    name: 'getBlockMetadata';
    data: {
        blockId: BlockId;
    };
}

export interface __FindInputsMethod__ {
    name: 'findInputs';
    data: {
        addresses: string[];
        amount: number;
    };
}

export interface __FindOutputsMethod__ {
    name: 'findOutputs';
    data: {
        outputIds: string[];
        addresses: string[];
    };
}

export interface __PrepareTransactionMethod__ {
    name: 'prepareTransaction';
    data: {
        secretManager?: SecretManagerType;
        options?: IBuildBlockOptions;
    };
}

export interface __SignTransactionMethod__ {
    name: 'signTransaction';
    data: {
        secretManager: SecretManagerType;
        preparedTransactionData: IPreparedTransactionData;
    };
}

export interface __SignatureUnlockMethod__ {
    name: 'signatureUnlock';
    data: {
        secretManager: SecretManagerType;
        transactionEssenceHash: Array<number>;
        chain: IBip32Chain;
    };
}

export interface __PostBlockPayloadMethod__ {
    name: 'postBlockPayload';
    data: {
        payload: PayloadTypes;
    };
}

export interface __GetNodeMethod__ {
    name: 'getNode';
}

export interface __GetNetworkIdMethod__ {
    name: 'getNetworkId';
}

export interface __GetBech32HrpMethod__ {
    name: 'getBech32Hrp';
}

export interface __GetMinPowScoreMethod__ {
    name: 'getMinPowScore';
}

export interface __GetTipsIntervalMethod__ {
    name: 'getTipsInterval';
}

export interface __GetProtocolParametersMethod__ {
    name: 'getProtocolParameters';
}

export interface __GetLocalPowMethod__ {
    name: 'getLocalPow';
}

export interface __GetFallbackToLocalPowMethod__ {
    name: 'getFallbackToLocalPow';
}

export interface __GetHealthMethod__ {
    name: 'getHealth';
    data: {
        url: string;
    };
}

export interface __GetNodeInfoMethod__ {
    name: 'getNodeInfo';
    data: {
        url: string;
        auth?: IAuth;
    };
}

export interface __GetPeersMethod__ {
    name: 'getPeers';
}

export interface __PostBlockRawMethod__ {
    name: 'postBlockRaw';
    data: {
        block: IBlock;
    };
}

export interface __GetBlockRawMethod__ {
    name: 'getBlockRaw';
    data: {
        blockId: BlockId;
    };
}

export interface __GetMilestoneByIdMethod__ {
    name: 'getMilestoneById';
    data: {
        milestoneId: string;
    };
}

export interface __GetUtxoChangesByIdMethod__ {
    name: 'getUtxoChangesById';
    data: {
        milestoneId: string;
    };
}
export interface __GetMilestoneByIndexMethod__ {
    name: 'getMilestoneByIndex';
    data: {
        index: number;
    };
}

export interface __GetUtxoChangesByIndexMethod__ {
    name: 'getUtxoChangesByIndex';
    data: {
        index: number;
    };
}

export interface __GetReceiptsMethod__ {
    name: 'getReceipts';
}

export interface __GetReceiptsMigratedAtMethod__ {
    name: 'getReceiptsMigratedAt';
    data: {
        milestoneIndex: number;
    };
}

export interface __GetTreasuryMethod__ {
    name: 'getTreasury';
}

export interface __GetIncludedBlockMethod__ {
    name: 'getIncludedBlock';
    data: {
        transactionId: string;
    };
}

export interface __GetIncludedBlockMetadataMethod__ {
    name: 'getIncludedBlockMetadata';
    data: {
        transactionId: string;
    };
}

export interface __HexToBech32Method__ {
    name: 'hexToBech32';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __AliasIdToBech32Method__ {
    name: 'aliasIdToBech32';
    data: {
        aliasId: string;
        bech32Hrp?: string;
    };
}

export interface __NftIdToBech32Method__ {
    name: 'nftIdToBech32';
    data: {
        nftId: string;
        bech32Hrp?: string;
    };
}

export interface __HexPublicKeyToBech32AddressMethod__ {
    name: 'hexPublicKeyToBech32Address';
    data: {
        hex: string;
        bech32Hrp?: string;
    };
}

export interface __AliasOutputIdsMethod__ {
    name: 'aliasOutputIds';
    data: {
        queryParameters: AliasQueryParameter[];
    };
}

export interface __AliasOutputIdMethod__ {
    name: 'aliasOutputId';
    data: {
        aliasId: string;
    };
}

export interface __NftOutputIdsMethod__ {
    name: 'nftOutputIds';
    data: {
        queryParameters: NftQueryParameter[];
    };
}

export interface __NftOutputIdMethod__ {
    name: 'nftOutputId';
    data: {
        nftId: string;
    };
}

export interface __FoundryOutputIdsMethod__ {
    name: 'foundryOutputIds';
    data: {
        queryParameters: FoundryQueryParameter[];
    };
}

export interface __FoundryOutputIdMethod__ {
    name: 'foundryOutputId';
    data: {
        foundryId: string;
    };
}

export interface __GetOutputsIgnoreErrorsMethod__ {
    name: 'getOutputsIgnoreErrors';
    data: {
        outputIds: string[];
    };
}

export interface __FindBlocksMethod__ {
    name: 'findBlocks';
    data: {
        blockIds: string[];
    };
}

export interface __RetryMethod__ {
    name: 'retry';
    data: {
        blockId: string;
    };
}

export interface __RetryUntilIncludedMethod__ {
    name: 'retryUntilIncluded';
    data: {
        blockId: string;
        interval?: number;
        maxAttempts?: number;
    };
}

export interface __ConsolidateFundsMethod__ {
    name: 'consolidateFunds';
    data: {
        secretManager: SecretManagerType;
        generateAddressesOptions: IGenerateAddressesOptions;
    };
}

export interface __ReattachMethod__ {
    name: 'reattach';
    data: {
        blockId: BlockId;
    };
}

export interface __ReattachUncheckedMethod__ {
    name: 'reattachUnchecked';
    data: {
        blockId: BlockId;
    };
}

export interface __PromoteMethod__ {
    name: 'promote';
    data: {
        blockId: BlockId;
    };
}

export interface __PromoteUncheckedMethod__ {
    name: 'promoteUnchecked';
    data: {
        blockId: BlockId;
    };
}

export interface __UnhealthyNodesMethod__ {
    name: 'unhealthyNodes';
}

export interface __BuildBasicOutputMethod__ {
    name: 'buildBasicOutput';
    data: BasicOutputBuilderOptions;
}

export interface __BuildAliasOutputMethod__ {
    name: 'buildAliasOutput';
    data: AliasOutputBuilderOptions;
}

export interface __BuildFoundryOutputMethod__ {
    name: 'buildFoundryOutput';
    data: FoundryOutputBuilderOptions;
}

export interface __BuildNftOutputMethod__ {
    name: 'buildNftOutput';
    data: NftOutputBuilderOptions;
}

export interface __ClearListenersMethod__ {
    name: 'clearListeners';
    data: {
        topics: string[];
    };
}

export type __RequestFundsFromFaucetMethod__ = {
    name: 'requestFundsFromFaucet';
    data: {
        url: string;
        address: string;
    };
};
