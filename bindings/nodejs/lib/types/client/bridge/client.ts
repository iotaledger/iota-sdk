// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type {
    Bip44,
    SecretManagerType,
} from '../../secret_manager/secret-manager';
import type {
    AccountId,
    Block,
    BlockId,
    FoundryId,
    AnchorId,
    NftId,
    DelegationId,
    Output,
    OutputId,
    Payload,
    SlotIndex,
    SlotCommitmentId,
    EpochIndex,
} from '../../block';
import type { PreparedTransactionData } from '../prepared-transaction-data';
import type {
    AccountOutputQueryParameters,
    AnchorOutputQueryParameters,
    BasicOutputQueryParameters,
    DelegationOutputQueryParameters,
    FoundryOutputQueryParameters,
    NftOutputQueryParameters,
    OutputQueryParameters,
} from '../query-parameters';
import type { Auth } from '../network';
import type { BasicOutputBuilderParams } from '../output_builder_params/basic-output-params';
import type { AccountOutputBuilderParams } from '../output_builder_params/account-output-params';
import type { FoundryOutputBuilderParams } from '../output_builder_params/foundry-output-params';
import type { NftOutputBuilderParams } from '../output_builder_params/nft-output-params';
import { HexEncodedString } from '../../utils';
import { TransactionId } from '../..';

// Node routes.

export interface __GetHealthMethod__ {
    name: 'getHealth';
    data: {
        url: string;
    };
}

export interface __GetInfoMethod__ {
    name: 'getInfo';
}

export interface __GetNodeInfoMethod__ {
    name: 'getNodeInfo';
    data: {
        url: string;
        auth?: Auth;
    };
}

export interface __GetRoutesMethod__ {
    name: 'getRoutes';
}

// Accounts routes.

export interface __GetAccountCongestionMethod__ {
    name: 'getAccountCongestion';
    data: {
        accountId: AccountId;
        workScore?: number;
    };
}

// Rewards routes.

export interface __GetRewardsMethod__ {
    name: 'getRewards';
    data: {
        outputId: OutputId;
        slotIndex?: SlotIndex;
    };
}

// Validators routes.

export interface __GetValidatorsMethod__ {
    name: 'getValidators';
    data: {
        pageSize?: number;
        cursor?: string;
    };
}

export interface __GetValidatorMethod__ {
    name: 'getValidator';
    data: {
        accountId: AccountId;
    };
}

// Committee routes.

export interface __GetCommitteeMethod__ {
    name: 'getCommittee';
    data: {
        epochIndex?: EpochIndex;
    };
}

// Blocks routes.

export interface __GetIssuanceMethod__ {
    name: 'getIssuance';
}

export interface __GetBlockMethod__ {
    name: 'getBlock';
    data: {
        blockId: BlockId;
    };
}

export interface __GetBlockRawMethod__ {
    name: 'getBlockRaw';
    data: {
        blockId: BlockId;
    };
}

export interface __PostBlockMethod__ {
    name: 'postBlock';
    data: {
        block: Block;
    };
}

export interface __PostBlockRawMethod__ {
    name: 'postBlockRaw';
    data: {
        block: Block;
    };
}

export interface __GetBlockMetadataMethod__ {
    name: 'getBlockMetadata';
    data: {
        blockId: BlockId;
    };
}

export interface __GetBlockWithMetadataMethod__ {
    name: 'getBlockWithMetadata';
    data: {
        blockId: BlockId;
    };
}

// UTXO routes.

export interface __GetOutputMethod__ {
    name: 'getOutput';
    data: {
        outputId: OutputId;
    };
}

export interface __GetOutputMetadataMethod__ {
    name: 'getOutputMetadata';
    data: {
        outputId: OutputId;
    };
}

export interface __GetOutputWithMetadataMethod__ {
    name: 'getOutputWithMetadata';
    data: {
        outputId: OutputId;
    };
}

export interface __GetOutputsMethod__ {
    name: 'getOutputs';
    data: {
        outputIds: OutputId[];
    };
}

export interface __GetOutputsIgnoreErrorsMethod__ {
    name: 'getOutputsIgnoreErrors';
    data: {
        outputIds: OutputId[];
    };
}

export interface __GetIncludedBlockMethod__ {
    name: 'getIncludedBlock';
    data: {
        transactionId: TransactionId;
    };
}

export interface __GetIncludedBlockMetadataMethod__ {
    name: 'getIncludedBlockMetadata';
    data: {
        transactionId: TransactionId;
    };
}

export interface __GetTransactionMetadataMethod__ {
    name: 'getTransactionMetadata';
    data: {
        transactionId: TransactionId;
    };
}

// Commitments routes.

export interface __GetCommitmentMethod__ {
    name: 'getCommitment';
    data: {
        commitmentId: SlotCommitmentId;
    };
}

export interface __GetUtxoChangesMethod__ {
    name: 'getUtxoChanges';
    data: {
        commitmentId: SlotCommitmentId;
    };
}

export interface __GetUtxoChangesFullMethod__ {
    name: 'getUtxoChangesFull';
    data: {
        commitmentId: SlotCommitmentId;
    };
}

export interface __GetCommitmentBySlotMethod__ {
    name: 'getCommitmentBySlot';
    data: {
        slot: SlotIndex;
    };
}

export interface __GetUtxoChangesBySlotMethod__ {
    name: 'getUtxoChangesBySlot';
    data: {
        slot: SlotIndex;
    };
}

export interface __GetUtxoChangesFullBySlotMethod__ {
    name: 'getUtxoChangesFullBySlot';
    data: {
        slot: SlotIndex;
    };
}

// Other routes.

export interface __GetNetworkInfoMethod__ {
    name: 'getNetworkInfo';
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
        transactionSigningHash: HexEncodedString;
        chain: Bip44;
    };
}

export interface __BuildBasicBlockMethod__ {
    name: 'buildBasicBlock';
    data: {
        issuerId: AccountId;
        payload?: Payload;
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

export interface __GetProtocolParametersMethod__ {
    name: 'getProtocolParameters';
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

export interface __AnchorIdToBech32Method__ {
    name: 'anchorIdToBech32';
    data: {
        anchorId: AnchorId;
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

export interface __FindBlocksMethod__ {
    name: 'findBlocks';
    data: {
        blockIds: BlockId[];
    };
}

export interface __UnhealthyNodesMethod__ {
    name: 'unhealthyNodes';
}

export interface __BuildBasicOutputMethod__ {
    name: 'buildBasicOutput';
    data: BasicOutputBuilderParams;
}

export interface __BuildAccountOutputMethod__ {
    name: 'buildAccountOutput';
    data: AccountOutputBuilderParams;
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

export type __ComputeMinimumOutputAmountMethod__ = {
    name: 'computeMinimumOutputAmount';
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

// inx-indexer methods

export interface __GetOutputIdsMethod__ {
    name: 'outputIds';
    data: {
        queryParameters: OutputQueryParameters;
    };
}

export interface __GetBasicOutputIdsMethod__ {
    name: 'basicOutputIds';
    data: {
        queryParameters: BasicOutputQueryParameters;
    };
}

export interface __AccountOutputIdsMethod__ {
    name: 'accountOutputIds';
    data: {
        queryParameters: AccountOutputQueryParameters;
    };
}

export interface __AccountOutputIdMethod__ {
    name: 'accountOutputId';
    data: {
        accountId: AccountId;
    };
}

export interface __AnchorOutputIdsMethod__ {
    name: 'anchorOutputIds';
    data: {
        queryParameters: AnchorOutputQueryParameters;
    };
}

export interface __AnchorOutputIdMethod__ {
    name: 'anchorOutputId';
    data: {
        anchorId: AnchorId;
    };
}

export interface __DelegationOutputIdsMethod__ {
    name: 'delegationOutputIds';
    data: {
        queryParameters: DelegationOutputQueryParameters;
    };
}

export interface __DelegationOutputIdMethod__ {
    name: 'delegationOutputId';
    data: {
        delegationId: DelegationId;
    };
}

export interface __FoundryOutputIdsMethod__ {
    name: 'foundryOutputIds';
    data: {
        queryParameters: FoundryOutputQueryParameters;
    };
}

export interface __FoundryOutputIdMethod__ {
    name: 'foundryOutputId';
    data: {
        foundryId: FoundryId;
    };
}

export interface __NftOutputIdsMethod__ {
    name: 'nftOutputIds';
    data: {
        queryParameters: NftOutputQueryParameters;
    };
}

export interface __NftOutputIdMethod__ {
    name: 'nftOutputId';
    data: {
        nftId: NftId;
    };
}

export interface __BlockIdMethod__ {
    name: 'blockId';
    data: {
        block: Block;
    };
}
