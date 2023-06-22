/**
 * * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

module.exports = {
  docs: [
    {
      type: 'doc',
      id:'about-the-iota-sdk',
    },
    {
      type: 'category',
      label: 'Getting Started',
      items:[
        {
          label: 'Rust',
          type: 'doc',
          id: 'getting-started/rust',
        },

        {
          label: 'Node.js',
          type: 'doc',
          id: 'getting-started/nodejs',
        },
        {
          label: 'Python',
          type: 'doc',
          id:  'getting-started/python',
        },
        {
          label: 'Wasm',
          type: 'doc',
          id:'getting-started/wasm',
        },
      ]
    },
    {
    type: 'category',
    label: 'Explanations',
    items: [
      {
        type: 'doc',
        id: 'explanations/account-approaches'
      },
      {
        type: 'doc',
        id:'explanations/testnet-and-test-tokens',
      },
    ]
    },
    {
      type: 'category',
      label: 'How To',
      items: [
        {
          type: "category",
          label: 'Accounts and Addresses',
          items: [
            'how-tos/accounts-and-addresses/create-mnemonic',
            'how-tos/accounts-and-addresses/create-account',
            'how-tos/accounts-and-addresses/list-accounts',
            'how-tos/accounts-and-addresses/check-balance',
            'how-tos/accounts-and-addresses/list-addresses',
            'how-tos/accounts-and-addresses/create-address',
            'how-tos/accounts-and-addresses/list-transactions',
            'how-tos/accounts-and-addresses/list-outputs',
          ]
        },
        {
          type: "category",
          label: 'Simple Transaction',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how-tos/simple-transaction'
            }
          ]
        },
        {
          type: "category",
          label: 'Advanced Transactions',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how-tos/advanced-transactions'
            }
          ]
        },
        {
          type: "category",
          label: 'Use Aliases/NFTs as Wallets',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how-tos/alias-wallet'
            }
          ]
        },
        {
          type: "category",
          label: 'Native Tokens',
          items: [
            'how-tos/native-tokens/mint',
            'how-tos/native-tokens/decrease-supply',
            'how-tos/native-tokens/increase-supply',
            'how-tos/native-tokens/send',
            'how-tos/native-tokens/burn',
            'how-tos/native-tokens/destroy-foundry',
          ]
        },
        {
          type: "category",
          label: 'Sign And Verify Ed25519',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how-tos/sign-and-verify-ed25519',
            }
          ]
        },
        {
          type: "category",
          label: 'Outputs',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how-tos/outputs'
            }
          ]
        }
      ]
    },
    {

      type: 'category',
      label: 'API Reference',
      items:[
        {
          type: 'doc',
          id: 'references/rust-api-reference',
          label: 'Rust'
        },
        {
          type: 'category',
          label: 'Node.js',
          items:[
            {
              type: "doc",
              id: "references/nodejs/api_ref",
            },
            {
              type: "category",
              label: "Classes",
              items:[
                {
                  type: 'autogenerated',
                  dirName: 'references/nodejs/classes'
                },
              ]
            },
            {
              type: "category",
              label: "Enums",
              items:[
                {
                  type: 'autogenerated',
                  dirName: 'references/nodejs/enums'
                },
              ]
            },
            {
              type: "category",
              label: "Interfaces",
              items:[
                {
                  type: 'autogenerated',
                  dirName: 'references/nodejs/interfaces'
                },
              ]
            },
          ]
        },
        {
          type: 'category',
          label: 'Python',
          items:[
            {
              type: 'autogenerated',
              dirName: 'references/python/iota_sdk'
            },
          ]
        },
      ]
    },
    {
      type: 'doc',
      id: 'troubleshooting',
    },
    {
      type: 'doc',
      id: 'contribute',
      label: 'Contribute',
    }
  ]
};