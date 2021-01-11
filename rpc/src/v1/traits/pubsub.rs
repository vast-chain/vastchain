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

//! Vast-specific PUB-SUB rpc interface.

use jsonrpc_core::{Result, Value, Params};
use jsonrpc_pubsub::{typed::Subscriber, SubscriptionId};
use jsonrpc_derive::rpc;

/// Vast-specific PUB-SUB rpc interface.
#[rpc(server)]
pub trait PubSub {
	/// Pub/Sub Metadata
	type Metadata;

	/// Subscribe to changes of any RPC mvastod in Vast.
	#[pubsub(subscription = "vast_subscription", subscribe, name = "vast_subscribe")]
	fn vast_subscribe(&self, _: Self::Metadata, _: Subscriber<Value>, _: String, _: Option<Params>);

	/// Unsubscribe from existing Vast subscription.
	#[pubsub(subscription = "vast_subscription", unsubscribe, name = "vast_unsubscribe")]
	fn vast_unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> Result<bool>;
}
