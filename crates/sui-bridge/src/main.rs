// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use sui_bridge::rest_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = rest_router();
    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 9000);
    axum::Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
