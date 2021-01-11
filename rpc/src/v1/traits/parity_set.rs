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

//! Vast-specific rpc interface for operations altering the settings.

use vast_types::{H160, H256, U256};
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;

use v1::types::{Bytes, ReleaseInfo, Transaction};

/// Vast-specific rpc interface for operations altering the account-related settings.
#[rpc(server)]
pub trait VastSetAccounts {
	/// Sets account for signing consensus messages.
	#[rpc(name = "vast_setEngineSigner")]
	fn set_engine_signer(&self, _: H160, _: String) -> Result<bool>;
}

/// Vast-specific rpc interface for operations altering the settings.
#[rpc(server)]
pub trait VastSet {
	/// Sets new minimal gas price for mined blocks.
	#[rpc(name = "vast_setMinGasPrice")]
	fn set_min_gas_price(&self, _: U256) -> Result<bool>;

	/// Sets new gas floor target for mined blocks.
	#[rpc(name = "vast_setGasFloorTarget")]
	fn set_gas_floor_target(&self, _: U256) -> Result<bool>;

	/// Sets new gas ceiling target for mined blocks.
	#[rpc(name = "vast_setGasCeilTarget")]
	fn set_gas_ceil_target(&self, _: U256) -> Result<bool>;

	/// Sets new extra data for mined blocks.
	#[rpc(name = "vast_setExtraData")]
	fn set_extra_data(&self, _: Bytes) -> Result<bool>;

	/// Sets new author for mined block.
	#[rpc(name = "vast_setAuthor")]
	fn set_author(&self, _: H160) -> Result<bool>;

	/// Sets the secret of engine signer account.
	#[rpc(name = "vast_setEngineSignerSecret")]
	fn set_engine_signer_secret(&self, _: H256) -> Result<bool>;

	/// Unsets the engine signer account address.
	#[rpc(name = "vast_clearEngineSigner")]
	fn clear_engine_signer(&self) -> Result<bool>;

	/// Sets the limits for transaction queue.
	#[rpc(name = "vast_setTransactionsLimit")]
	fn set_transactions_limit(&self, _: usize) -> Result<bool>;

	/// Sets the maximum amount of gas a single transaction may consume.
	#[rpc(name = "vast_setMaxTransactionGas")]
	fn set_tx_gas_limit(&self, _: U256) -> Result<bool>;

	/// Add a reserved peer.
	#[rpc(name = "vast_addReservedPeer")]
	fn add_reserved_peer(&self, _: String) -> Result<bool>;

	/// Remove a reserved peer.
	#[rpc(name = "vast_removeReservedPeer")]
	fn remove_reserved_peer(&self, _: String) -> Result<bool>;

	/// Drop all non-reserved peers.
	#[rpc(name = "vast_dropNonReservedPeers")]
	fn drop_non_reserved_peers(&self) -> Result<bool>;

	/// Accept non-reserved peers (default behavior)
	#[rpc(name = "vast_acceptNonReservedPeers")]
	fn accept_non_reserved_peers(&self) -> Result<bool>;

	/// Start the network.
	///
	/// @deprecated - Use `set_mode("active")` instead.
	#[rpc(name = "vast_startNetwork")]
	fn start_network(&self) -> Result<bool>;

	/// Stop the network.
	///
	/// @deprecated - Use `set_mode("offline")` instead.
	#[rpc(name = "vast_stopNetwork")]
	fn stop_network(&self) -> Result<bool>;

	/// Set the mode. Argument must be one of: "active", "passive", "dark", "offline".
	#[rpc(name = "vast_setMode")]
	fn set_mode(&self, _: String) -> Result<bool>;

	/// Set the network spec. Argument must be one of pre-configured chains or a filename.
	#[rpc(name = "vast_setChain")]
	fn set_spec_name(&self, _: String) -> Result<bool>;

	/// Hash a file content under given URL.
	#[rpc(name = "vast_hashContent")]
	fn hash_content(&self, _: String) -> BoxFuture<H256>;

	/// Is there a release ready for install?
	#[rpc(name = "vast_upgradeReady")]
	fn upgrade_ready(&self) -> Result<Option<ReleaseInfo>>;

	/// Execute a release which is ready according to upgrade_ready().
	#[rpc(name = "vast_executeUpgrade")]
	fn execute_upgrade(&self) -> Result<bool>;

	/// Removes transaction from transaction queue.
	/// Makes sense only for transactions that were not propagated to other peers yet
	/// like scheduled transactions or transactions in future.
	/// It might also work for some local transactions with to low gas price
	/// or excessive gas limit that are not accepted by other peers whp.
	/// Returns `true` when transaction was removed, `false` if it was not found.
	#[rpc(name = "vast_removeTransaction")]
	fn remove_transaction(&self, _: H256) -> Result<Option<Transaction>>;
}
