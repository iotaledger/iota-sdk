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
      type: 'doc',
      id: 'overview',
    },
    {
      type: 'category',
      label: 'How To',
      items: [
        {
          type: "category",
          label: 'Sign Ed25519',
          items: [
            {
              type: 'autogenerated',
              dirName: 'how_tos/sign_ed25519',
            }
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