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

//! Vast-specific rpc interface.

use std::collections::BTreeMap;

use vast_types::{H64, H160, H256, H512, U64, U256};
use vastcore::miner::FilterOptions;
use jsonrpc_core::{BoxFuture, Result};
use jsonrpc_derive::rpc;
use v1::types::{
	Bytes, CallRequest,
	Peers, Transaction, RpcSettings, Histogram, RecoveredAccount,
	TransactionStats, LocalTransactionStatus,
	BlockNumber, ConsensusCapability, VersionInfo,
	OperationsInfo, ChainStatus, Log, Filter,
	RichHeader, Receipt,
};

/// Vast-specific rpc interface.
#[rpc(server)]
pub trait Vast {
	/// RPC Metadata
	type Metadata;

	/// Returns current transactions limit.
	#[rpc(name = "vast_transactionsLimit")]
	fn transactions_limit(&self) -> Result<usize>;

	/// Returns mining extra data.
	#[rpc(name = "vast_extraData")]
	fn extra_data(&self) -> Result<Bytes>;

	/// Returns mining gas floor target.
	#[rpc(name = "vast_gasFloorTarget")]
	fn gas_floor_target(&self) -> Result<U256>;

	/// Returns mining gas floor cap.
	#[rpc(name = "vast_gasCeilTarget")]
	fn gas_ceil_target(&self) -> Result<U256>;

	/// Returns minimal gas price for transaction to be included in queue.
	#[rpc(name = "vast_minGasPrice")]
	fn min_gas_price(&self) -> Result<U256>;

	/// Returns latest logs
	#[rpc(name = "vast_devLogs")]
	fn dev_logs(&self) -> Result<Vec<String>>;

	/// Returns logs levels
	#[rpc(name = "vast_devLogsLevels")]
	fn dev_logs_levels(&self) -> Result<String>;

	/// Returns chain name - DEPRECATED. Use `vast_chainName` instead.
	#[rpc(name = "vast_netChain")]
	fn net_chain(&self) -> Result<String>;

	/// Returns peers details
	#[rpc(name = "vast_netPeers")]
	fn net_peers(&self) -> Result<Peers>;

	/// Returns network port
	#[rpc(name = "vast_netPort")]
	fn net_port(&self) -> Result<u16>;

	/// Returns rpc settings
	#[rpc(name = "vast_rpcSettings")]
	fn rpc_settings(&self) -> Result<RpcSettings>;

	/// Returns node name
	#[rpc(name = "vast_nodeName")]
	fn node_name(&self) -> Result<String>;

	/// Returns default extra data
	#[rpc(name = "vast_defaultExtraData")]
	fn default_extra_data(&self) -> Result<Bytes>;

	/// Returns distribution of gas price in latest blocks.
	#[rpc(name = "vast_gasPriceHistogram")]
	fn gas_price_histogram(&self) -> BoxFuture<Histogram>;

	/// Returns number of unsigned transactions waiting in the signer queue (if signer enabled)
	/// Returns error when signer is disabled
	#[rpc(name = "vast_unsignedTransactionsCount")]
	fn unsigned_transactions_count(&self) -> Result<usize>;

	/// Returns a cryptographically random phrase sufficient for securely seeding a secret key.
	#[rpc(name = "vast_generateSecretPhrase")]
	fn generate_secret_phrase(&self) -> Result<String>;

	/// Returns whatever address would be derived from the given phrase if it were to seed a brainwallet.
	#[rpc(name = "vast_phraseToAddress")]
	fn phrase_to_address(&self, _: String) -> Result<H160>;

	/// Returns the value of the registrar for this network.
	#[rpc(name = "vast_registryAddress")]
	fn registry_address(&self) -> Result<Option<H160>>;

	/// Returns all addresses if Fat DB is enabled (`--fat-db`), or null if not.
	#[rpc(name = "vast_listAccounts")]
	fn list_accounts(&self, _: u64, _: Option<H160>, _: Option<BlockNumber>) -> Result<Option<Vec<H160>>>;

	/// Returns all storage keys of the given address (first parameter) if Fat DB is enabled (`--fat-db`),
	/// or null if not.
	#[rpc(name = "vast_listStorageKeys")]
	fn list_storage_keys(
		&self,
		_: H160,
		_: Option<u64>,
		_: Option<H256>,
		_: Option<BlockNumber>,
	) -> Result<Option<Vec<H256>>>;

	/// Encrypt some data with a public key under ECIES.
	/// First parameter is the 512-byte destination public key, second is the message.
	#[rpc(name = "vast_encryptMessage")]
	fn encrypt_message(&self, _: H512, _: Bytes) -> Result<Bytes>;

	/// Returns all pending transactions from transaction queue.
	#[rpc(name = "vast_pendingTransactions")]
	fn pending_transactions(&self, _: Option<usize>, _: Option<FilterOptions>) -> Result<Vec<Transaction>>;

	/// Returns all transactions from transaction queue.
	///
	/// Some of them might not be ready to be included in a block yet.
	#[rpc(name = "vast_allTransactions")]
	fn all_transactions(&self) -> Result<Vec<Transaction>>;

	/// Same as vast_allTransactions, but return only transactions hashes.
	#[rpc(name = "vast_allTransactionHashes")]
	fn all_transaction_hashes(&self) -> Result<Vec<H256>>;

	/// Returns all future transactions from transaction queue (deprecated)
	#[rpc(name = "vast_futureTransactions")]
	fn future_transactions(&self) -> Result<Vec<Transaction>>;

	/// Returns propagation statistics on transactions pending in the queue.
	#[rpc(name = "vast_pendingTransactionsStats")]
	fn pending_transactions_stats(&self) -> Result<BTreeMap<H256, TransactionStats>>;

	/// Returns a list of current and past local transactions with status details.
	#[rpc(name = "vast_localTransactions")]
	fn local_transactions(&self) -> Result<BTreeMap<H256, LocalTransactionStatus>>;

	/// Returns current WS Server interface and port or an error if ws server is disabled.
	#[rpc(name = "vast_wsUrl")]
	fn ws_url(&self) -> Result<String>;

	/// Returns next nonce for particular sender. Should include all transactions in the queue.
	#[rpc(name = "vast_nextNonce")]
	fn next_nonce(&self, _: H160) -> BoxFuture<U256>;

	/// Get the mode. Returns one of: "active", "passive", "dark", "offline".
	#[rpc(name = "vast_mode")]
	fn mode(&self) -> Result<String>;

	/// Get the chain name. Returns one of the pre-configured chain names or a filename.
	#[rpc(name = "vast_chain")]
	fn chain(&self) -> Result<String>;

	/// Get the enode of this node.
	#[rpc(name = "vast_enode")]
	fn enode(&self) -> Result<String>;

	/// Returns information on current consensus capability.
	#[rpc(name = "vast_consensusCapability")]
	fn consensus_capability(&self) -> Result<ConsensusCapability>;

	/// Get our version information in a nice object.
	#[rpc(name = "vast_versionInfo")]
	fn version_info(&self) -> Result<VersionInfo>;

	/// Get information concerning the latest releases if available.
	#[rpc(name = "vast_releasesInfo")]
	fn releases_info(&self) -> Result<Option<OperationsInfo>>;

	/// Get the current chain status.
	#[rpc(name = "vast_chainStatus")]
	fn chain_status(&self) -> Result<ChainStatus>;

	/// Get node kind info.
	#[rpc(name = "vast_nodeKind")]
	fn node_kind(&self) -> Result<::v1::types::NodeKind>;

	/// Get block header.
	/// Same as `vast_getBlockByNumber` but without uncles and transactions.
	#[rpc(name = "vast_getBlockHeaderByNumber")]
	fn block_header(&self, _: Option<BlockNumber>) -> BoxFuture<RichHeader>;

	/// Get block receipts.
	/// Allows you to fetch receipts from the entire block at once.
	/// If no parameter is provided defaults to `latest`.
	#[rpc(name = "vast_getBlockReceipts")]
	fn block_receipts(&self, _: Option<BlockNumber>) -> BoxFuture<Vec<Receipt>>;

	/// Get IPFS CIDv0 given protobuf encoded bytes.
	#[rpc(name = "vast_cidV0")]
	fn ipfs_cid(&self, _: Bytes) -> Result<String>;

	/// Call contract, returning the output data.
	#[rpc(name = "vast_call")]
	fn call(&self, _: Vec<CallRequest>, _: Option<BlockNumber>) -> Result<Vec<Bytes>>;

	/// Used for submitting a proof-of-work solution (similar to `vast_submitWork`,
	/// but returns block hash on success, and returns an explicit error message on failure).
	#[rpc(name = "vast_submitWorkDetail")]
	fn submit_work_detail(&self, _: H64, _: H256, _: H256) -> Result<H256>;

	/// Returns the status of the node. Used as the health endpoint.
	///
	/// The RPC returns successful response if:
	/// - The node have a peer (unless running a dev chain)
	/// - The node is not syncing.
	///
	/// Otherwise the RPC returns error.
	#[rpc(name = "vast_nodeStatus")]
	fn status(&self) -> Result<()>;

	/// Extracts Address and public key from signature using the r, s and v params. Equivalent to Solidity erecover
	/// as well as checks the signature for chain replay protection
	#[rpc(name = "vast_verifySignature")]
	fn verify_signature(&self, _: bool, _: Bytes, _: H256, _: H256, _: U64) -> Result<RecoveredAccount>;

	/// Returns logs matching given filter object.
	/// Is allowed to skip filling transaction hash for faster query.
	#[rpc(name = "vast_getLogsNoTransactionHash")]
	fn logs_no_tx_hash(&self, _: Filter) -> BoxFuture<Vec<Log>>;

	/// Returns raw block RLP with given number.
	#[rpc(name = "vast_getRawBlockByNumber")]
	fn get_raw_block_by_number(&self, _: BlockNumber) -> BoxFuture<Option<Bytes>>;

	/// Submit raw block to be published to the network
	#[rpc(name = "vast_submitRawBlock")]
	fn submit_raw_block(&self, _: Bytes) -> Result<H256>;
}
