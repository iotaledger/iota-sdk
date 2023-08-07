// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type {
    Bip44,
    SecretManagerType,
} from '../../secret_manager/secret-manager';
import type { Block, BlockId, Output, Payload } from '../../block';
import type { PreparedTransactionData } from '../prepared-transaction-data';
import type {
    AliasQueryParameter,
    FoundryQueryParameter,
    NftQueryParameter,
    QueryParameter,
} from '../query-parameters';
import type { IAuth } from '../network';
import type { BasicOutputBuilderParams } from '../output_builder_params/basic-output-params';
import type { AliasOutputBuilderParams } from '../output_builder_params/alias-output-params';
import type { FoundryOutputBuilderParams } from '../output_builder_params/foundry-output-params';
import type { NftOutputBuilderParams } from '../output_builder_params/nft-output-params';
import { HexEncodedString } from '../../utils';

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

export interface __PostBlockMethod__ {
    name: 'postBlock';
    data: {
        block: Block;
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

export interface __SignTransactionMethod__ {
    name: 'signTransaction';
    data: {
        secretManager: SecretManagerType;
        preparedTransactionData: PreparedTransactionData;
    };
}

export interface __SignatureUnlockMethod__ {
    name: 'signatureUnlock';
    data: {
        secretManager: SecretManagerType;
        transactionEssenceHash: HexEncodedString;
        chain: Bip44;
    };
}

export interface __PostBlockPayloadMethod__ {
    name: 'postBlockPayload';
    data: {
        payload: Payload;
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
        block: Block;
    };
}

export interface __GetBlockRawMethod__ {
    name: 'getBlockRaw';
    data: {
        blockId: BlockId;
    };
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

export interface __UnhealthyNodesMethod__ {
    name: 'unhealthyNodes';
}

export interface __BuildBasicOutputMethod__ {
    name: 'buildBasicOutput';
    data: BasicOutputBuilderParams;
}

export interface __BuildAliasOutputMethod__ {
    name: 'buildAliasOutput';
    data: AliasOutputBuilderParams;
}

export interface __BuildFoundryOutputMethod__ {
    name: 'buildFoundryOutput';
    data: FoundryOutputBuilderParams;
}

export interface __BuildNftOutputMethod__ {
    name: 'buildNftOutput';
    data: NftOutputBuilderParams;
}

export interface __ClearListenersMethod__ {
    name: 'clearListeners';
    data: {
        topics: string[];
    };
}

export type __MinimumRequiredStorageDepositMethod__ = {
    name: 'minimumRequiredStorageDeposit';
    data: {
        output: Output;
    };
};

export type __RequestFundsFromFaucetMethod__ = {
    name: 'requestFundsFromFaucet';
    data: {
        url: string;
        address: string;
    };
};

export type __CallPluginRouteMethod__ = {
    name: 'callPluginRoute';
    data: {
        basePluginPath: string;
        method: 'GET' | 'POST';
        endpoint: string;
        queryParams: string[];
        request?: string;
    };
};
