// Copyright 2015-2020 Vast Technologies (UK) Ltd.
// This file is part of Vast.

// Vast is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Vast is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Vast.  If not, see <http://www.gnu.org/licenses/>.

//! Vast PUB-SUB rpc interface.

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use jsonrpc_pubsub::{typed, SubscriptionId};

use v1::types::pubsub;

/// Vast PUB-SUB rpc interface.
#[rpc(server)]
pub trait VastPubSub {
	/// RPC Metadata
	type Metadata;

	/// Subscribe to Vast subscription.
	#[pubsub(subscription = "vast_subscription", subscribe, name = "vast_subscribe")]
	fn subscribe(
		&self,
		_: Self::Metadata,
		_: typed::Subscriber<pubsub::Result>,
		_: pubsub::Kind,
		_: Option<pubsub::Params>,
	);

	/// Unsubscribe from existing Vast subscription.
	#[pubsub(subscription = "vast_subscription", unsubscribe, name = "vast_unsubscribe")]
	fn unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> Result<bool>;
}
