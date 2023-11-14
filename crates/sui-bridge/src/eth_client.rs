// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use ethers::providers::{Provider, Http, Middleware};
use ethers::types::TxHash;
use std::str::FromStr;
use crate::error::{BridgeError, BridgeResult};
use tap::tap::TapFallible;

pub(crate) struct EthClient {
    provider: Provider::<Http>,
}

impl EthClient {
    pub async fn new(provider_url: &str) -> anyhow::Result<Self> {
        let provider = Provider::<Http>::try_from(provider_url)?;
        let self_ = Self { provider };
        self_.describe().await;
        Ok(self_)
    }

    async fn describe(&self) -> anyhow::Result<()> {
        let chain_id = self.provider.get_chainid().await?;
        let block_number = self.provider.get_block_number().await?;
        // FIXME
        println!("EthClient is connected to chain {chain_id}, current block number: {block_number}");
        Ok(())
    }

    pub async fn get_bridge_events_maybe(&self, tx_hash: &str) -> BridgeResult<()> {
        let tx_hash = TxHash::from_str(tx_hash).map_err(|_| BridgeError::InvalidTxHash)?;
        let receipt = self.provider.get_transaction_receipt(tx_hash).await
        // FIXME
        .tap_err(|e| println!("Error getting transaction receipt from provider: {:?}", e))
        .map_err(|e| BridgeError::InternalError(e.to_string()))?
        .ok_or(BridgeError::TxNotFound)?;
        Ok(())
    }
}

pub const ABI_JSON: &str = r#"
[
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "address",
                "name": "owner",
                "type": "address"
            },
            {
                "indexed": true,
                "internalType": "address",
                "name": "spender",
                "type": "address"
            },
            {
                "indexed": false,
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
            }
        ],
        "name": "Approval",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "address",
                "name": "delegator",
                "type": "address"
            },
            {
                "indexed": true,
                "internalType": "address",
                "name": "fromDelegate",
                "type": "address"
            },
            {
                "indexed": true,
                "internalType": "address",
                "name": "toDelegate",
                "type": "address"
            }
        ],
        "name": "DelegateChanged",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "address",
                "name": "delegate",
                "type": "address"
            },
            {
                "indexed": false,
                "internalType": "uint256",
                "name": "previousBalance",
                "type": "uint256"
            },
            {
                "indexed": false,
                "internalType": "uint256",
                "name": "newBalance",
                "type": "uint256"
            }
        ],
        "name": "DelegateVotesChanged",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": false,
                "internalType": "address",
                "name": "minter",
                "type": "address"
            },
            {
                "indexed": false,
                "internalType": "address",
                "name": "newMinter",
                "type": "address"
            }
        ],
        "name": "MinterChanged",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "address",
                "name": "from",
                "type": "address"
            },
            {
                "indexed": true,
                "internalType": "address",
                "name": "to",
                "type": "address"
            },
            {
                "indexed": false,
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
            }
        ],
        "name": "Transfer",
        "type": "event"
    }
]
"#;