"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ERC_20_ABI = exports.StateMutabilityType = exports.AbiType = void 0;
var AbiType;
(function (AbiType) {
    AbiType["Function"] = "function";
    AbiType["Constructor"] = "constructor";
    AbiType["Event"] = "event";
    AbiType["Fallback"] = "fallback";
})(AbiType = exports.AbiType || (exports.AbiType = {}));
var StateMutabilityType;
(function (StateMutabilityType) {
    StateMutabilityType["Pure"] = "pure";
    StateMutabilityType["View"] = "view";
    StateMutabilityType["NonPayable"] = "nonpayable";
    StateMutabilityType["Payable"] = "payable";
})(StateMutabilityType = exports.StateMutabilityType || (exports.StateMutabilityType = {}));
exports.ERC_20_ABI = [
    {
        constant: true,
        inputs: [],
        name: 'name',
        outputs: [
            {
                name: '',
                type: 'string',
            },
        ],
        payable: false,
        stateMutability: StateMutabilityType.View,
        type: AbiType.Function,
    },
    {
        constant: false,
        inputs: [
            {
                name: '_spender',
                type: 'address',
            },
            {
                name: '_value',
                type: 'uint256',
            },
        ],
        name: 'approve',
        outputs: [
            {
                name: '',
                type: 'bool',
            },
        ],
        payable: false,
        stateMutability: StateMutabilityType.NonPayable,
        type: AbiType.Function,
    },
    {
        constant: true,
        inputs: [],
        name: 'totalSupply',
        outputs: [
            {
                name: '',
                type: 'uint256',
            },
        ],
        payable: false,
        stateMutability: StateMutabilityType.View,
        type: AbiType.Function,
    },
    {
        constant: false,
        inputs: [
            {
                name: '_from',
                type: 'address',
            },
            {
                name: '_to',
                type: 'address',
            },
            {
                name: '_value',
                type: 'uint256',
            },
        ],
        name: 'transferFrom',
        outputs: [
            {
                name: '',
                type: 'bool',
            },
        ],
        payable: false,
        stateMutability: StateMutabilityType.NonPayable,
        type: AbiType.Function,
    },
    {
        constant: true,
        inputs: [],
        name: 'decimals',
        outputs: [
            {
                name: '',
                type: 'uint8',
            },
        ],
        payable: false,
        stateMutability: StateMutabilityType.View,
        type: AbiType.Function,
    },
    {
        constant: true,
        inputs: [
            {
                name: '_owner',
                type: 'address',
            },
        ],
        name: 'balanceOf',
        outputs: [
            {
                name: 'balance',
                type: 'uint256',
            },
        ],
        payable: false,
        stateMutability: StateMutabilityType.View,
        type: AbiType.Function,
    },
    {
        constant: true,
        inputs: [],
        name: 'symbol',
        outputs: [
            {
                name: '',
                type: 'string',
            },
        ],
        payable: false,
        stateMutability: StateMutabilityType.View,
        type: AbiType.Function,
    },
    {
        constant: false,
        inputs: [
            {
                name: '_to',
                type: 'address',
            },
            {
                name: '_value',
                type: 'uint256',
            },
        ],
        name: 'transfer',
        outputs: [
            {
                name: '',
                type: 'bool',
            },
        ],
        payable: false,
        stateMutability: StateMutabilityType.NonPayable,
        type: AbiType.Function,
    },
    {
        constant: true,
        inputs: [
            {
                name: '_owner',
                type: 'address',
            },
            {
                name: '_spender',
                type: 'address',
            },
        ],
        name: 'allowance',
        outputs: [
            {
                name: '',
                type: 'uint256',
            },
        ],
        payable: false,
        stateMutability: StateMutabilityType.View,
        type: AbiType.Function,
    },
    {
        payable: true,
        stateMutability: StateMutabilityType.Payable,
        type: AbiType.Fallback,
    },
    {
        anonymous: false,
        inputs: [
            {
                indexed: true,
                name: 'owner',
                type: 'address',
            },
            {
                indexed: true,
                name: 'spender',
                type: 'address',
            },
            {
                indexed: false,
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'Approval',
        type: AbiType.Event,
    },
    {
        anonymous: false,
        inputs: [
            {
                indexed: true,
                name: 'from',
                type: 'address',
            },
            {
                indexed: true,
                name: 'to',
                type: 'address',
            },
            {
                indexed: false,
                name: 'value',
                type: 'uint256',
            },
        ],
        name: 'Transfer',
        type: AbiType.Event,
    },
];
