```js
{
  payload: {
    type: 6,
    essence: {
      type: 1,
      networkId: '1856588631910923207',
      inputs: [
        {
          type: 0,
          transactionId: '0xc58e2c937bfe852e69c090ab178f8eb8fc127fbaab812b3833c7f9fc06273fa6',
          transactionOutputIndex: 1
        }
      ],
      inputsCommitment: '0xa71844622d843a2800c4aba62dcb732ff7f0b347faf32f35a81e26db675dfe74',
      outputs: [
        {
          type: 4,
          amount: '50300',
          aliasId: '0x0000000000000000000000000000000000000000000000000000000000000000',
          stateIndex: 0,
          stateMetadata: '0x',
          foundryCounter: 0,
          unlockConditions: [
            {
              type: 4,
              address: {
                type: 0,
                pubKeyHash: '0xa2ac93c845b63ba80f93c31e3b88693ac1d403d3c5967926fa246f960a90ee78'
              }
            },
            {
              type: 5,
              address: {
                type: 0,
                pubKeyHash: '0xa2ac93c845b63ba80f93c31e3b88693ac1d403d3c5967926fa246f960a90ee78'
              }
            }
          ]
        },
        {
          type: 3,
          amount: '999849100',
          unlockConditions: [
            {
              type: 0,
              address: {
                type: 0,
                pubKeyHash: '0xa2ac93c845b63ba80f93c31e3b88693ac1d403d3c5967926fa246f960a90ee78'
              }
            }
          ]
        }
      ]
    },
    unlocks: [
      {
        type: 0,
        signature: {
          type: 0,
          publicKey: '0xbeac18c634df21150c6e02b7be1cd065e44c3702bfce0ff72a9eb82034f9f260',
          signature: '0x21b2a49680389c1f117d52bb92033051bc665c1409aef7bb76eafae7df0e4e1ab1e9611515912f0886b9299e7be2c1aea5488ca125100d8f2c57446dedd5a00d'
        }
      }
    ]
  },
  blockId: '0xda2ab689c68b78c180a54bdd13ffe0b8185b02ad2cff3cc06f273ebfacf42999',
  inclusionState: 'Pending',
  timestamp: '1679689882116',
  transactionId: '0xc174c1a24ed29dabcb441e0ff28813238ffa11e7fc328ab0fb3c68deceb39ddb',
  networkId: '1856588631910923207',
  incoming: false,
  note: null,
  inputs: [
    {
      metadata: {
        blockId: '0x78f19599a407ec07efb6e49a3c26d2b5f4318c4e5e25dabb0aa0075b9ce74c44',
        transactionId: '0xc58e2c937bfe852e69c090ab178f8eb8fc127fbaab812b3833c7f9fc06273fa6',
        outputIndex: 1,
        isSpent: false,
        milestoneIndexBooked: 4297386,
        milestoneTimestampBooked: 1679689686,
        ledgerIndex: 4297424
      },
      output: {
        type: 3,
        amount: '999899400',
        unlockConditions: [
          {
            type: 0,
            address: {
              type: 0,
              pubKeyHash: '0xa2ac93c845b63ba80f93c31e3b88693ac1d403d3c5967926fa246f960a90ee78'
            }
          }
        ]
      }
    }
  ]
}
```