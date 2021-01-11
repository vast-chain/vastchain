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

//! Vast rpc interfaces.

pub mod debug;
pub mod vast;
pub mod vast_pubsub;
pub mod vast_signing;
pub mod net;
pub mod vast;
pub mod vast_accounts;
pub mod vast_set;
pub mod vast_signing;
pub mod personal;
pub mod private;
pub mod pubsub;
pub mod rpc;
pub mod secretstore;
pub mod signer;
pub mod traces;
pub mod transactions_pool;
pub mod web3;

pub use self::debug::Debug;
pub use self::vast::{Vast, VastFilter};
pub use self::vast_pubsub::VastPubSub;
pub use self::vast_signing::VastSigning;
pub use self::net::Net;
pub use self::vast::Vast;
pub use self::vast_accounts::{VastAccounts, VastAccountsInfo};
pub use self::vast_set::{VastSet, VastSetAccounts};
pub use self::vast_signing::VastSigning;
pub use self::personal::Personal;
pub use self::private::Private;
pub use self::pubsub::PubSub;
pub use self::rpc::Rpc;
pub use self::secretstore::SecretStore;
pub use self::signer::Signer;
pub use self::traces::Traces;
pub use self::transactions_pool::TransactionsPool;
pub use self::web3::Web3;
