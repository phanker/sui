// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use axum::{http::StatusCode, routing::get, Router};

mod checkpoints;
mod client;
mod eth_client;
pub mod headers;
pub mod error;
// pub mod node_state_getter;
// mod objects;
use axum::{
    extract::{Path, State},
    Json, TypedHeader,
};
use ethers::prelude::LocalWallet;
pub use client::Client;
use ethers::signers::Signer;
use fastcrypto::encoding::{Hex, Encoding};
use fastcrypto::secp256k1::Secp256k1KeyPair;
use sui_types::crypto::{Secp256k1SuiSignature, SuiKeyPair, get_key_pair};

async fn health_check() -> StatusCode {
    println!("health check");
    StatusCode::OK
}

// pub struct Bcs<T>(pub T);

// pub const TEXT_PLAIN_UTF_8: &str = "text/plain; charset=utf-8";
// pub const APPLICATION_BCS: &str = "application/bcs";
pub const APPLICATION_JSON: &str = "application/json";

// impl<T> axum::response::IntoResponse for Bcs<T>
// where
//     T: serde::Serialize,
// {
//     fn into_response(self) -> axum::response::Response {
//         match bcs::to_bytes(&self.0) {
//             Ok(buf) => (
//                 [(
//                     axum::http::header::CONTENT_TYPE,
//                     axum::http::HeaderValue::from_static(APPLICATION_BCS),
//                 )],
//                 buf,
//             )
//                 .into_response(),
//             Err(err) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 [(
//                     axum::http::header::CONTENT_TYPE,
//                     axum::http::HeaderValue::from_static(TEXT_PLAIN_UTF_8),
//                 )],
//                 err.to_string(),
//             )
//                 .into_response(),
//         }
//     }
// }

// pub fn rest_router(state: std::sync::Arc<dyn NodeStateGetter>) -> Router {
pub fn rest_router() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route(checkpoints::ETH_TX_PATH, get(handle_eth_tx_hash))
        .route(checkpoints::SUI_TX_PATH, get(handle_sui_tx_digest))
    // .with_state(state)
}

pub async fn start_service(
    socket_address: std::net::SocketAddr,
    // state: std::sync::Arc<dyn NodeStateGetter>,
) {
    let app = rest_router();

    axum::Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub async fn handle_eth_tx_hash(
    //TODO support digest as well as sequence number
    Path(tx_hash_hex): Path<String>,
    // State(state): State<Arc<dyn NodeStateGetter>>,
    // ) -> Result<Json<CertifiedCheckpointSummary>, AppError> {
// ) -> Result<Json<Secp256k1SuiSignature>, AppError> {
) -> Result<Json<String>, AppError> {
    let key: Secp256k1KeyPair = get_key_pair().1;
    let private_key_bytes = key.secret.as_ref().to_vec();
    let pub_key_bytes = key.public.as_ref().to_vec();
    println!("Eth: {private_key_bytes:?}");
    let key = SuiKeyPair::Secp256k1(key);
    let private_key_hex = Hex::encode(&private_key_bytes);
    let pub_key_hex = Hex::encode(&pub_key_bytes);
    println!("Eth privatek hex: {private_key_hex:?}");
    println!("Eth pubk hex: {pub_key_hex:?}");
    let local_wallet = LocalWallet::from_str(&private_key_hex).unwrap();
    let address = local_wallet.address();
    // let sig = key.sign("hello".as_bytes());
    let message = "Hello, World!";
    println!("Eth message: {message}");
    let sig = local_wallet.sign_message(message).await?;
    let recovered_address = sig.recover(message)?;
    assert_eq!(address, recovered_address);
    let sig_str = sig.to_string();

    // FIXME do more when this error occurs
    // let sig: Secp256k1SuiSignature = sig.try_into().map_err(|_| AppError(anyhow::anyhow!("failed to convert signature")))?;
    println!("Eth: {tx_hash_hex}, {address:?}, {sig_str:?}, {sig:?}");
    Ok(Json(sig_str))
}

pub async fn handle_sui_tx_digest(
    //TODO support digest as well as sequence number
    Path(tx_digest_base58): Path<String>,
    // State(state): State<Arc<dyn NodeStateGetter>>,
    // ) -> Result<Json<CertifiedCheckpointSummary>, AppError> {
) -> Result<Json<()>, AppError> {
    println!("Sui: {tx_digest_base58}");
    Ok(Json(()))
}
