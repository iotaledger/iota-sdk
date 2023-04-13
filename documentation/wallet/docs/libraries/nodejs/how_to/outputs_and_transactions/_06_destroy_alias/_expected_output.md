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
          transactionId: '0x15f72b6509d34179e0cfef2436caa1f49d8b9fc5a7564b5a5b484aa162f8dee7',
          transactionOutputIndex: 0
        }
      ],
      inputsCommitment: '0x7e30140d6761097569be539fc767c3780bcc3fc4a65b7fb1bf286fd35032a57a',
      outputs: [
        {
          type: 3,
          amount: '50300',
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
          signature: '0x07b08d1ae0db3bf88aa59c3421521f1b8e20d8cc74a3dd2a07e11241ba00f57de38bb5bc09eb7e5ba4a4b5d353a7d200127a8f4515b8c1ec396a04690c4a7505'
        }
      }
    ]
  },
  blockId: '0x6d00425aecd6c044d87d9c0b7dfdda329cd6bea9e4ae879500c63062254bf796',
  inclusionState: 'Pending',
  timestamp: '1679691498914',
  transactionId: '0xaf2aaa2df34fd956a33917fbbf2cf4acfab64ee9faf7755618c6ee4d2e72fd89',
  networkId: '1856588631910923207',
  incoming: false,
  note: null,
  inputs: [
    {
      metadata: {
        blockId: '0xc215aac7d24d46abe1ca53ca5054593db5ad69c13dc16ec2875c3fe5ea3ec3fb',
        transactionId: '0x15f72b6509d34179e0cfef2436caa1f49d8b9fc5a7564b5a5b484aa162f8dee7',
        outputIndex: 0,
        isSpent: false,
        milestoneIndexBooked: 4297677,
        milestoneTimestampBooked: 1679691147,
        ledgerIndex: 4297744
      },
      output: {
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
      }
    }
  ]
}
```