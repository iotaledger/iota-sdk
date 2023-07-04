import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/iota-sdk/',
    component: ComponentCreator('/iota-sdk/', '5bb'),
    routes: [
      {
        path: '/iota-sdk/contribute/',
        component: ComponentCreator('/iota-sdk/contribute/', '8ad'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/explanations/account_approaches/',
        component: ComponentCreator('/iota-sdk/explanations/account_approaches/', '36b'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/accounts_and_addresses/check_balance/',
        component: ComponentCreator('/iota-sdk/how_tos/accounts_and_addresses/check_balance/', '32c'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/accounts_and_addresses/create_account/',
        component: ComponentCreator('/iota-sdk/how_tos/accounts_and_addresses/create_account/', '008'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/accounts_and_addresses/create_address/',
        component: ComponentCreator('/iota-sdk/how_tos/accounts_and_addresses/create_address/', '05b'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/accounts_and_addresses/create_mnemonic/',
        component: ComponentCreator('/iota-sdk/how_tos/accounts_and_addresses/create_mnemonic/', '02e'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/accounts_and_addresses/list_accounts/',
        component: ComponentCreator('/iota-sdk/how_tos/accounts_and_addresses/list_accounts/', '2a4'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/accounts_and_addresses/list_addresses/',
        component: ComponentCreator('/iota-sdk/how_tos/accounts_and_addresses/list_addresses/', '556'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/accounts_and_addresses/list_outputs/',
        component: ComponentCreator('/iota-sdk/how_tos/accounts_and_addresses/list_outputs/', '707'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/accounts_and_addresses/list_transactions/',
        component: ComponentCreator('/iota-sdk/how_tos/accounts_and_addresses/list_transactions/', '040'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/nfts/burn_nft/',
        component: ComponentCreator('/iota-sdk/how_tos/nfts/burn_nft/', '53a'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/nfts/mint_nft/',
        component: ComponentCreator('/iota-sdk/how_tos/nfts/mint_nft/', '73a'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/nfts/send_nft/',
        component: ComponentCreator('/iota-sdk/how_tos/nfts/send_nft/', '2e5'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/outputs/features/',
        component: ComponentCreator('/iota-sdk/how_tos/outputs/features/', 'a25'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/outputs/unlock_conditions/',
        component: ComponentCreator('/iota-sdk/how_tos/outputs/unlock_conditions/', '176'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/sign_and_verify_ed25519/sign_ed25519/',
        component: ComponentCreator('/iota-sdk/how_tos/sign_and_verify_ed25519/sign_ed25519/', '078'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/sign_and_verify_ed25519/verify_ed25519_signature/',
        component: ComponentCreator('/iota-sdk/how_tos/sign_and_verify_ed25519/verify_ed25519_signature/', '2fe'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/simple_transaction/',
        component: ComponentCreator('/iota-sdk/how_tos/simple_transaction/', '8cb'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/how_tos/simple_transaction/request_funds/',
        component: ComponentCreator('/iota-sdk/how_tos/simple_transaction/request_funds/', '1d3'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/overview/',
        component: ComponentCreator('/iota-sdk/overview/', '2d4'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/troubleshooting/',
        component: ComponentCreator('/iota-sdk/troubleshooting/', '6b0'),
        exact: true,
        sidebar: "docs"
      },
      {
        path: '/iota-sdk/welcome/',
        component: ComponentCreator('/iota-sdk/welcome/', '971'),
        exact: true,
        sidebar: "docs"
      }
    ]
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
