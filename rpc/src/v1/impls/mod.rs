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

//! Vast rpc interface implementation.

mod debug;
mod vast;
mod vast_filter;
mod vast_pubsub;
mod net;
mod vast;
#[cfg(any(test, feature = "accounts"))]
mod vast_accounts;
mod vast_set;
#[cfg(any(test, feature = "accounts"))]
mod personal;
mod private;
mod pubsub;
mod rpc;
#[cfg(any(test, feature = "accounts"))]
mod secretstore;
mod signer;
mod signing;
mod signing_unsafe;
mod traces;
mod transactions_pool;
mod web3;

pub mod light;

pub use self::debug::DebugClient;
pub use self::vast::{VastClient, VastClientOptions};
pub use self::vast_filter::VastFilterClient;
pub use self::vast_pubsub::VastPubSubClient;
pub use self::transactions_pool::TransactionsPoolClient;
pub use self::net::NetClient;
pub use self::vast::VastClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::vast_accounts::VastAccountsClient;
pub use self::vast_set::VastSetClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::vast_set::accounts::VastSetAccountsClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::personal::PersonalClient;
pub use self::private::PrivateClient;
pub use self::pubsub::PubSubClient;
pub use self::rpc::RpcClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::secretstore::SecretStoreClient;
pub use self::signer::SignerClient;
pub use self::signing::SigningQueueClient;
pub use self::signing_unsafe::SigningUnsafeClient;
pub use self::traces::TracesClient;
pub use self::web3::Web3Client;
