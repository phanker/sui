// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::*;
use sui_indexer::models_v2::events::StoredEvent;

use crate::error::Error;

use super::{
    address::Address, date_time::DateTime, move_module::MoveModuleId, move_value::MoveValue,
    sui_address::SuiAddress,
};

#[derive(SimpleObject)]
#[graphql(complex)]
pub(crate) struct Event {
    #[graphql(skip)]
    pub stored: StoredEvent,
    #[graphql(flatten)]
    pub contents: MoveValue,
}

#[ComplexObject]
impl Event {
    /// Package id and module name of the Move module that the event was emitted in
    async fn sending_module_id(&self) -> Result<Option<MoveModuleId>> {
        let package_id = SuiAddress::from_bytes(&self.stored.package)
            .map_err(|e| Error::Internal(e.to_string()))
            .extend()?;
        Ok(Some(MoveModuleId {
            package: package_id,
            name: self.stored.module.clone(),
        }))
    }

    /// Addresses of the senders of the event
    async fn senders(&self) -> Result<Option<Vec<Address>>> {
        let result: Result<Option<Vec<Address>>, _> = self
            .stored
            .senders
            .iter()
            .map(|sender| {
                sender
                    .as_ref()
                    .map(|sender| {
                        SuiAddress::from_bytes(sender)
                            .map(|sui_address| Address {
                                address: sui_address,
                            })
                            .map_err(|e| Error::Internal(e.to_string()))
                    })
                    .transpose()
            })
            .collect();

        result.extend()
    }

    /// UTC timestamp in milliseconds since epoch (1/1/1970)
    async fn timestamp(&self) -> Option<DateTime> {
        DateTime::from_ms(self.stored.timestamp_ms)
    }
}

#[derive(InputObject, Clone)]
pub(crate) struct EventFilter {
    pub sender: Option<SuiAddress>,
    pub transaction_digest: Option<String>,
    // Enhancement (post-MVP)
    // after_checkpoint
    // before_checkpoint

    // Cascading
    pub emitting_package: Option<SuiAddress>,
    pub emitting_module: Option<String>,

    // Cascading
    pub event_package: Option<SuiAddress>,
    pub event_module: Option<String>,
    pub event_type: Option<String>,
    // Enhancement (post-MVP)
    // pub start_time
    // pub end_time

    // Enhancement (post-MVP)
    // pub any
    // pub all
    // pub not
}
