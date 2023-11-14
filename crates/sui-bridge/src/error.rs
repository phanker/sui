// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0


pub enum BridgeError {
    InvalidTxHash,
    OriginTxFailed,
    TxNotFound,
    NoBridgeEventsInTx,
    InternalError(String),
}

pub type BridgeResult<T> = Result<T, BridgeError>;