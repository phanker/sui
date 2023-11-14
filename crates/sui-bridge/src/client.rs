// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new<S: Into<String>>(base_url: S) -> Self {
        Self {
            inner: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    // pub async fn get_latest_checkpoint(&self) -> Result<CertifiedCheckpointSummary> {
    //     let url = format!("{}/checkpoints", self.base_url);
    //     let checkpoint = self
    //         .inner
    //         .get(url)
    //         .header(reqwest::header::ACCEPT, crate::APPLICATION_JSON)
    //         .send()
    //         .await?
    //         .json()
    //         .await?;
    //     Ok(checkpoint)
    // }

    // pub async fn get_full_checkpoint(
    //     &self,
    //     checkpoint_sequence_number: CheckpointSequenceNumber,
    // ) -> Result<CheckpointData> {
    //     let url = format!(
    //         "{}/checkpoints/{checkpoint_sequence_number}/full",
    //         self.base_url
    //     );

    //     let bytes = self
    //         .inner
    //         .get(url)
    //         .header(reqwest::header::ACCEPT, crate::APPLICATION_BCS)
    //         .send()
    //         .await?
    //         .bytes()
    //         .await?;

    //     bcs::from_bytes(&bytes).map_err(Into::into)
    // }
}
