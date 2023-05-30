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
      id: 'welcome',
    },
    {
      type: 'category',
      label: 'Overview',
      items: [
        {
          type: 'doc',
          id: 'overview',
          label: 'SDK Overview'
        },
        {
          type: 'doc',
          id: 'explanations/account_approaches'
        }
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
            'how_tos/accounts_and_addresses/create_mnemonic',
            'how_tos/accounts_and_addresses/create_account',
            'how_tos/accounts_and_addresses/list_accounts',
            'how_tos/accounts_and_addresses/check_balance',
            'how_tos/accounts_and_addresses/list_addresses',
            'how_tos/accounts_and_addresses/create_address',
            'how_tos/accounts_and_addresses/list_transactions',
            'how_tos/accounts_and_addresses/list_outputs',
          ]
        },
        {
          type: "category",
          label: 'Simple Transaction',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how_tos/simple_transaction'
            }
          ]
        },
        {
          type: "category",
          label: 'Advanced Transactions',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how_tos/advanced_transactions'
            }
          ]
        },
        {
          type: "category",
          label: 'Sign And Verify Ed25519',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how_tos/sign_and_verify_ed25519',
            }
          ]
        },
        {
          type: "category",
          label: 'Outputs',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how_tos/outputs'
            }
          ]
        }
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