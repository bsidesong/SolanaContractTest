/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/ppt2.json`.
 */
export type Ppt2 = {
  "address": "5ccZFdZ3eQxiN6vvYcAurVcSrMPsQHcwNZFZSfFDzv8J",
  "metadata": {
    "name": "ppt2",
    "version": "0.1.0",
    "spec": "0.1.0"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "gameState",
          "writable": true,
          "signer": true
        },
        {
          "name": "player",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "makePlay",
      "discriminator": [
        4,
        183,
        67,
        232,
        160,
        194,
        3,
        179
      ],
      "accounts": [
        {
          "name": "gameState",
          "writable": true
        },
        {
          "name": "player",
          "writable": true,
          "signer": true,
          "relations": [
            "gameState"
          ]
        }
      ],
      "args": [
        {
          "name": "playerMove",
          "type": "u8"
        }
      ]
    },
    {
      "name": "payForPlays",
      "discriminator": [
        145,
        142,
        226,
        147,
        122,
        95,
        247,
        58
      ],
      "accounts": [
        {
          "name": "gameState",
          "writable": true
        },
        {
          "name": "player",
          "writable": true,
          "signer": true,
          "relations": [
            "gameState"
          ]
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "withdrawTreasury",
      "discriminator": [
        40,
        63,
        122,
        158,
        144,
        216,
        83,
        96
      ],
      "accounts": [
        {
          "name": "treasury",
          "writable": true
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "gameState",
      "discriminator": [
        144,
        94,
        208,
        172,
        248,
        99,
        134,
        120
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "noPlaysLeft",
      "msg": "No plays left. Please pay to get more plays."
    },
    {
      "code": 6001,
      "name": "unauthorized",
      "msg": "Unauthorized to perform this action."
    }
  ],
  "types": [
    {
      "name": "gameState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "pubkey"
          },
          {
            "name": "playsLeft",
            "type": "u8"
          },
          {
            "name": "lastPaidSlot",
            "type": "u64"
          },
          {
            "name": "score",
            "type": "u64"
          },
          {
            "name": "history",
            "type": {
              "vec": {
                "defined": {
                  "name": "matchResult"
                }
              }
            }
          }
        ]
      }
    },
    {
      "name": "matchResult",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "playerChoice",
            "type": "u8"
          },
          {
            "name": "programChoice",
            "type": "u8"
          },
          {
            "name": "result",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
