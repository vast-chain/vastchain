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

use std::cmp::PartialEq;
use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Weak};

pub use vast_rpc::signer::SignerService;

use account_utils::{self, AccountProvider};
use vastcore::client::Client;
use vastcore::miner::Miner;
use snapshot::SnapshotService;
use client_traits::BlockChainClient;
use sync::SyncState;
use vastcore_logger::RotatingLogger;
use vastcore_private_tx::Provider as PrivateTransactionManager;
use vastcore_service::PrivateTxService;
use hash_fetch::fetch::Client as FetchClient;
use jsonrpc_core::{self as core, MetaIoHandler};
use light::client::LightChainClient;
use light::{Cache as LightDataCache, TransactionQueue as LightTransactionQueue};
use miner::external::ExternalMiner;
use vast_rpc::dispatch::{FullDispatcher, LightDispatcher};
use vast_rpc::informant::{ActivityNotifier, ClientNotifier};
use vast_rpc::{Host, Metadata, NetworkSettings};
use vast_rpc::v1::traits::TransactionsPool;
use vast_runtime::Executor;
use parking_lot::{Mutex, RwLock};
use sync::{LightSync, ManageNetwork, SyncProvider};
use updater::Updater;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Api {
	/// Web3 (Safe)
	Web3,
	/// Net (Safe)
	Net,
	/// Vast (Safe)
	Vast,
	/// Vast Pub-Sub (Safe)
	VastPubSub,
	/// Gvast-compatible "personal" API (DEPRECATED; only used in `--gvast` mode.)
	Personal,
	/// Signer - Confirm transactions in Signer (UNSAFE: Passwords, List of transactions)
	Signer,
	/// Vast - Custom extensions (Safe)
	Vast,
	/// Traces (Safe)
	Traces,
	/// Rpc (Safe)
	Rpc,
	/// Private transaction manager (Safe)
	Private,
	/// Vast PubSub - Generic Publish-Subscriber (Safety depends on other APIs exposed).
	VastPubSub,
	/// Vast Accounts extensions (UNSAFE: Passwords, Side Effects (new account))
	VastAccounts,
	/// Vast - Set mvastods (UNSAFE: Side Effects affecting node operation)
	VastSet,
	/// SecretStore (UNSAFE: arbitrary hash signing)
	SecretStore,
	/// Gvast-compatible (best-effort) debug API (Potentially UNSAFE)
	/// NOTE We don't aim to support all mvastods, only the ones that are useful.
	Debug,
	/// Vast Transactions pool PubSub
	VastTransactionsPool,
	/// Deprecated api
	Deprecated,
}

impl FromStr for Api {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use self::Api::*;

		match s {
			"debug" => Ok(Debug),
			"vast" => Ok(Vast),
			"net" => Ok(Net),
			"vast" => Ok(Vast),
			"vast_accounts" => Ok(VastAccounts),
			"vast_pubsub" => Ok(VastPubSub),
			"vast_set" => Ok(VastSet),
			"personal" => Ok(Personal),
			"private" => Ok(Private),
			"pubsub" => Ok(VastPubSub),
			"rpc" => Ok(Rpc),
			"secretstore" => Ok(SecretStore),
			"signer" => Ok(Signer),
			"traces" => Ok(Traces),
			"web3" => Ok(Web3),
			"vast_transactions_pool" => Ok(VastTransactionsPool),
			"shh" | "shh_pubsub" => Ok(Deprecated),
			api => Err(format!("Unknown api: {}", api)),
		}
	}
}

#[derive(Debug, Clone)]
pub enum ApiSet {
	// Unsafe context (like jsonrpc over http)
	UnsafeContext,
	// All possible APIs (safe context like token-protected WS interface)
	All,
	// Local "unsafe" context and accounts access
	IpcContext,
	// APIs for Vast Generic Pub-Sub
	PubSub,
	// Fixed list of APis
	List(HashSet<Api>),
}

impl Default for ApiSet {
	fn default() -> Self {
		ApiSet::UnsafeContext
	}
}

impl PartialEq for ApiSet {
	fn eq(&self, other: &Self) -> bool {
		self.list_apis() == other.list_apis()
	}
}

impl FromStr for ApiSet {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut apis = HashSet::new();

		for api in s.split(',') {
			match api {
				"all" => {
					apis.extend(ApiSet::All.list_apis());
				}
				"safe" => {
					// Safe APIs are those that are safe even in UnsafeContext.
					apis.extend(ApiSet::UnsafeContext.list_apis());
				}
				// Remove the API
				api if api.starts_with("-") => {
					let api = api[1..].parse()?;
					apis.remove(&api);
				}
				api => {
					let api = api.parse()?;
					apis.insert(api);
				}
			}
		}

		Ok(ApiSet::List(apis))
	}
}

fn to_modules(apis: &HashSet<Api>) -> BTreeMap<String, String> {
	let mut modules = BTreeMap::new();
	for api in apis {
		let (name, version) = match *api {
			Api::Debug => ("debug", "1.0"),
			Api::Vast => ("vast", "1.0"),
			Api::VastPubSub => ("pubsub", "1.0"),
			Api::Net => ("net", "1.0"),
			Api::Vast => ("vast", "1.0"),
			Api::VastAccounts => ("vast_accounts", "1.0"),
			Api::VastPubSub => ("vast_pubsub", "1.0"),
			Api::VastSet => ("vast_set", "1.0"),
			Api::Personal => ("personal", "1.0"),
			Api::Private => ("private", "1.0"),
			Api::Rpc => ("rpc", "1.0"),
			Api::SecretStore => ("secretstore", "1.0"),
			Api::Signer => ("signer", "1.0"),
			Api::Traces => ("traces", "1.0"),
			Api::Web3 => ("web3", "1.0"),
			Api::VastTransactionsPool => ("vast_transactions_pool", "1.0"),
			Api::Deprecated => {
				continue;
			}
		};
		modules.insert(name.into(), version.into());
	}
	modules
}

macro_rules! add_signing_mvastods {
	($namespace:ident, $handler:expr, $deps:expr, $dispatch:expr) => {{
		let deps = &$deps;
		let (dispatcher, accounts) = $dispatch;
		if deps.signer_service.is_enabled() {
			$handler.extend_with($namespace::to_delegate(SigningQueueClient::new(
				&deps.signer_service,
				dispatcher.clone(),
				deps.executor.clone(),
				accounts,
			)))
		} else {
			$handler.extend_with($namespace::to_delegate(SigningUnsafeClient::new(
				accounts,
				dispatcher.clone(),
			)))
			}
		}};
}

/// RPC dependencies can be used to initialize RPC endpoints from APIs.
pub trait Dependencies {
	type Notifier: ActivityNotifier;

	/// Create the activity notifier.
	fn activity_notifier(&self) -> Self::Notifier;

	/// Extend the given I/O handler with endpoints for each API.
	fn extend_with_set<S>(&self, handler: &mut MetaIoHandler<Metadata, S>, apis: &HashSet<Api>)
	where
		S: core::Middleware<Metadata>;
}

/// RPC dependencies for a full node.
pub struct FullDependencies {
	pub signer_service: Arc<SignerService>,
	pub client: Arc<Client>,
	pub snapshot: Arc<dyn SnapshotService>,
	pub sync: Arc<dyn SyncProvider>,
	pub net: Arc<dyn ManageNetwork>,
	pub accounts: Arc<AccountProvider>,
	pub private_tx_service: Option<Arc<PrivateTxService>>,
	pub miner: Arc<Miner>,
	pub external_miner: Arc<ExternalMiner>,
	pub logger: Arc<RotatingLogger>,
	pub settings: Arc<NetworkSettings>,
	pub net_service: Arc<dyn ManageNetwork>,
	pub updater: Arc<Updater>,
	pub gvast_compatibility: bool,
	pub experimental_rpcs: bool,
	pub ws_address: Option<Host>,
	pub fetch: FetchClient,
	pub executor: Executor,
	pub gas_price_percentile: usize,
	pub poll_lifetime: u32,
	pub allow_missing_blocks: bool,
	pub no_ancient_blocks: bool,
}

impl FullDependencies {
	fn extend_api<S>(
		&self,
		handler: &mut MetaIoHandler<Metadata, S>,
		apis: &HashSet<Api>,
		for_generic_pubsub: bool,
	) where
		S: core::Middleware<Metadata>,
	{
		use vast_rpc::v1::*;

		let nonces = Arc::new(Mutex::new(dispatch::Reservations::new(
			self.executor.clone(),
		)));
		let dispatcher = FullDispatcher::new(
			self.client.clone(),
			self.miner.clone(),
			nonces.clone(),
			self.gas_price_percentile,
		);
		let account_signer = Arc::new(dispatch::Signer::new(self.accounts.clone())) as _;
		let accounts = account_utils::accounts_list(self.accounts.clone());

		for api in apis {
			match *api {
				Api::Debug => {
					handler.extend_with(DebugClient::new(self.client.clone()).to_delegate());
				}
				Api::Web3 => {
					handler.extend_with(Web3Client::default().to_delegate());
				}
				Api::Net => {
					handler.extend_with(NetClient::new(&self.sync).to_delegate());
				}
				Api::Vast => {
					let client = VastClient::new(
						&self.client,
						&self.snapshot,
						&self.sync,
						&accounts,
						&self.miner,
						&self.external_miner,
						VastClientOptions {
							pending_nonce_from_queue: self.gvast_compatibility,
							allow_pending_receipt_query: !self.gvast_compatibility,
							send_block_number_in_get_work: !self.gvast_compatibility,
							gas_price_percentile: self.gas_price_percentile,
							allow_missing_blocks: self.allow_missing_blocks,
							allow_experimental_rpcs: self.experimental_rpcs,
							no_ancient_blocks: self.no_ancient_blocks
						}
					);
					handler.extend_with(client.to_delegate());

					if !for_generic_pubsub {
						let filter_client = VastFilterClient::new(
							self.client.clone(),
							self.miner.clone(),
							self.poll_lifetime,
						);
						handler.extend_with(filter_client.to_delegate());

						add_signing_mvastods!(VastSigning, handler, self, (&dispatcher, &account_signer));
					}
				}
				Api::VastPubSub => {
					if !for_generic_pubsub {
						let pool_receiver = self.miner.pending_transactions_receiver();
						let mut client =
							VastPubSubClient::new(self.client.clone(), self.executor.clone(), pool_receiver);
						let weak_client = Arc::downgrade(&self.client);

						client.add_sync_notifier(self.sync.sync_notification(), move |state| {
							let client = weak_client.upgrade()?;
							let queue_info = client.queue_info();

							let is_syncing_state = match state { SyncState::Idle | SyncState::NewBlocks => false, _ => true };
							let is_verifying = queue_info.unverified_queue_size + queue_info.verified_queue_size > 3;

							Some(PubSubSyncStatus {
								syncing: is_verifying || is_syncing_state,
							})
						});

						if let Some(h) = client.handler().upgrade() {
							self.client.add_notify(h);
						}
						handler.extend_with(client.to_delegate());
					}
				}
				Api::VastTransactionsPool => {
					if !for_generic_pubsub {
						let receiver = self.miner.full_transactions_receiver();
						let client = TransactionsPoolClient::new(self.executor.clone(), receiver);
						handler.extend_with(TransactionsPoolClient::to_delegate(client));
					}
				}
				Api::Personal => {
					#[cfg(feature = "accounts")]
					handler.extend_with(
						PersonalClient::new(
							&self.accounts,
							dispatcher.clone(),
							self.gvast_compatibility,
							self.experimental_rpcs,
						).to_delegate(),
					);
				}
				Api::Signer => {
					handler.extend_with(
						SignerClient::new(
							account_signer.clone(),
							dispatcher.clone(),
							&self.signer_service,
							self.executor.clone(),
						).to_delegate(),
					);
				}
				Api::Vast => {
					let signer = match self.signer_service.is_enabled() {
						true => Some(self.signer_service.clone()),
						false => None,
					};
					handler.extend_with(
						VastClient::new(
							self.client.clone(),
							self.miner.clone(),
							self.sync.clone(),
							self.updater.clone(),
							self.net_service.clone(),
							self.logger.clone(),
							self.settings.clone(),
							signer,
							self.ws_address.clone(),
							self.snapshot.clone().into(),
						).to_delegate(),
					);
					#[cfg(feature = "accounts")]
					handler.extend_with(VastAccountsInfo::to_delegate(VastAccountsClient::new(&self.accounts)));

					if !for_generic_pubsub {
						add_signing_mvastods!(VastSigning, handler, self, (&dispatcher, &account_signer));
					}
				}
				Api::VastPubSub => {
					if !for_generic_pubsub {
						let mut rpc = MetaIoHandler::default();
						let apis = ApiSet::List(apis.clone())
							.retain(ApiSet::PubSub)
							.list_apis();
						self.extend_api(&mut rpc, &apis, true);
						handler.extend_with(
							PubSubClient::new(rpc, self.executor.clone()).to_delegate(),
						);
					}
				}
				Api::VastAccounts => {
					#[cfg(feature = "accounts")]
					handler.extend_with(VastAccounts::to_delegate(VastAccountsClient::new(&self.accounts)));
				}
				Api::VastSet => {
					handler.extend_with(
						VastSetClient::new(
							&self.client,
							&self.miner,
							&self.updater,
							&self.net_service,
							self.fetch.clone(),
						).to_delegate(),
					);
					#[cfg(feature = "accounts")]
					handler.extend_with(
						VastSetAccountsClient::new(
							&self.accounts,
							&self.miner,
						).to_delegate(),
					);
				}
				Api::Traces => handler.extend_with(TracesClient::new(&self.client).to_delegate()),
				Api::Rpc => {
					let modules = to_modules(&apis);
					handler.extend_with(RpcClient::new(modules).to_delegate());
				}
				Api::SecretStore => {
					#[cfg(feature = "accounts")]
					handler.extend_with(SecretStoreClient::new(&self.accounts).to_delegate());
				}
				Api::Private => {
					handler.extend_with(
						PrivateClient::new(self.private_tx_service.as_ref().map(|p| p.provider()))
							.to_delegate(),
					);
				}
				Api::Deprecated => {},
			}
		}
	}
}

impl Dependencies for FullDependencies {
	type Notifier = ClientNotifier;

	fn activity_notifier(&self) -> ClientNotifier {
		ClientNotifier {
			client: self.client.clone(),
		}
	}

	fn extend_with_set<S>(&self, handler: &mut MetaIoHandler<Metadata, S>, apis: &HashSet<Api>)
	where
		S: core::Middleware<Metadata>,
	{
		self.extend_api(handler, apis, false)
	}
}

/// Light client notifier. Doesn't do anything yet, but might in the future.
pub struct LightClientNotifier;

impl ActivityNotifier for LightClientNotifier {
	fn active(&self) {}
}

/// RPC dependencies for a light client.
pub struct LightDependencies<T> {
	pub signer_service: Arc<SignerService>,
	pub client: Arc<T>,
	pub sync: Arc<LightSync>,
	pub net: Arc<dyn ManageNetwork>,
	pub accounts: Arc<AccountProvider>,
	pub logger: Arc<RotatingLogger>,
	pub settings: Arc<NetworkSettings>,
	pub on_demand: Arc<::light::on_demand::OnDemand>,
	pub cache: Arc<Mutex<LightDataCache>>,
	pub transaction_queue: Arc<RwLock<LightTransactionQueue>>,
	pub ws_address: Option<Host>,
	pub fetch: FetchClient,
	pub gvast_compatibility: bool,
	pub experimental_rpcs: bool,
	pub executor: Executor,
	pub private_tx_service: Option<Arc<PrivateTransactionManager>>,
	pub gas_price_percentile: usize,
	pub poll_lifetime: u32,
}

impl<C: LightChainClient + 'static> LightDependencies<C> {
	fn extend_api<T: core::Middleware<Metadata>>(
		&self,
		handler: &mut MetaIoHandler<Metadata, T>,
		apis: &HashSet<Api>,
		for_generic_pubsub: bool,
	) {
		use vast_rpc::v1::*;

		let dispatcher = LightDispatcher::new(
			self.sync.clone(),
			self.client.clone(),
			self.on_demand.clone(),
			self.cache.clone(),
			self.transaction_queue.clone(),
			Arc::new(Mutex::new(dispatch::Reservations::new(
				self.executor.clone(),
			))),
			self.gas_price_percentile,
		);
		let account_signer = Arc::new(dispatch::Signer::new(self.accounts.clone())) as _;
		let accounts = account_utils::accounts_list(self.accounts.clone());

		for api in apis {
			match *api {
				Api::Debug => {
					warn!(target: "rpc", "Debug API is not available in light client mode.")
				}
				Api::Web3 => {
					handler.extend_with(Web3Client::default().to_delegate());
				}
				Api::Net => {
					handler.extend_with(light::NetClient::new(self.sync.clone()).to_delegate());
				}
				Api::Vast => {
					let client = light::VastClient::new(
						self.sync.clone(),
						self.client.clone(),
						self.on_demand.clone(),
						self.transaction_queue.clone(),
						accounts.clone(),
						self.cache.clone(),
						self.gas_price_percentile,
						self.poll_lifetime,
					);
					handler.extend_with(Vast::to_delegate(client.clone()));

					if !for_generic_pubsub {
						handler.extend_with(VastFilter::to_delegate(client));
						add_signing_mvastods!(VastSigning, handler, self, (&dispatcher, &account_signer));
					}
				}
				Api::VastPubSub => {
					let receiver = self.transaction_queue.write().pending_transactions_receiver();

					let mut client = VastPubSubClient::light(
						self.client.clone(),
						self.on_demand.clone(),
						self.sync.clone(),
						self.cache.clone(),
						self.executor.clone(),
						self.gas_price_percentile,
						receiver
					);

					let weak_client = Arc::downgrade(&self.client);

					client.add_sync_notifier(self.sync.sync_notification(), move |state| {
						let client = weak_client.upgrade()?;
						let queue_info = client.queue_info();

						let is_syncing_state = match state { SyncState::Idle | SyncState::NewBlocks => false, _ => true };
						let is_verifying = queue_info.unverified_queue_size + queue_info.verified_queue_size > 3;

						Some(PubSubSyncStatus {
							syncing: is_verifying || is_syncing_state,
						})
					});

					self.client.add_listener(client.handler() as Weak<_>);
					handler.extend_with(VastPubSub::to_delegate(client));
				}
				Api::VastTransactionsPool => {
					if !for_generic_pubsub {
						let receiver = self.transaction_queue.write().full_transactions_receiver();
						let client = TransactionsPoolClient::new(self.executor.clone(), receiver);
						handler.extend_with(TransactionsPoolClient::to_delegate(client));
					}
				}
				Api::Personal => {
					#[cfg(feature = "accounts")]
					handler.extend_with(
						PersonalClient::new(
							&self.accounts,
							dispatcher.clone(),
							self.gvast_compatibility,
							self.experimental_rpcs,
						).to_delegate(),
					);
				}
				Api::Signer => {
					handler.extend_with(
						SignerClient::new(
							account_signer.clone(),
							dispatcher.clone(),
							&self.signer_service,
							self.executor.clone(),
						).to_delegate(),
					);
				}
				Api::Vast => {
					let signer = match self.signer_service.is_enabled() {
						true => Some(self.signer_service.clone()),
						false => None,
					};
					handler.extend_with(
						light::VastClient::new(
							Arc::new(dispatcher.clone()),
							self.logger.clone(),
							self.settings.clone(),
							signer,
							self.ws_address.clone(),
							self.gas_price_percentile,
						).to_delegate(),
					);
					#[cfg(feature = "accounts")]
					handler.extend_with(
						VastAccountsInfo::to_delegate(VastAccountsClient::new(&self.accounts))
					);

					if !for_generic_pubsub {
						add_signing_mvastods!(VastSigning, handler, self, (&dispatcher, &account_signer));
					}
				}
				Api::VastPubSub => {
					if !for_generic_pubsub {
						let mut rpc = MetaIoHandler::default();
						let apis = ApiSet::List(apis.clone())
							.retain(ApiSet::PubSub)
							.list_apis();
						self.extend_api(&mut rpc, &apis, true);
						handler.extend_with(
							PubSubClient::new(rpc, self.executor.clone()).to_delegate(),
						);
					}
				}
				Api::VastAccounts => {
					#[cfg(feature = "accounts")]
					handler.extend_with(VastAccounts::to_delegate(VastAccountsClient::new(&self.accounts)));
				}
				Api::VastSet => handler.extend_with(
					light::VastSetClient::new(self.client.clone(), self.sync.clone(), self.fetch.clone())
						.to_delegate(),
				),
				Api::Traces => handler.extend_with(light::TracesClient.to_delegate()),
				Api::Rpc => {
					let modules = to_modules(&apis);
					handler.extend_with(RpcClient::new(modules).to_delegate());
				}
				Api::SecretStore => {
					#[cfg(feature = "accounts")]
					handler.extend_with(SecretStoreClient::new(&self.accounts).to_delegate());
				}
				Api::Private => {
					if let Some(ref tx_manager) = self.private_tx_service {
						let private_tx_service = Some(tx_manager.clone());
						handler.extend_with(PrivateClient::new(private_tx_service).to_delegate());
					}
				}
				Api::Deprecated => {},
			}
		}
	}
}

impl<T: LightChainClient + 'static> Dependencies for LightDependencies<T> {
	type Notifier = LightClientNotifier;

	fn activity_notifier(&self) -> Self::Notifier {
		LightClientNotifier
	}

	fn extend_with_set<S>(&self, handler: &mut MetaIoHandler<Metadata, S>, apis: &HashSet<Api>)
	where
		S: core::Middleware<Metadata>,
	{
		self.extend_api(handler, apis, false)
	}
}

impl ApiSet {
	/// Retains only APIs in given set.
	pub fn retain(self, set: Self) -> Self {
		ApiSet::List(&self.list_apis() & &set.list_apis())
	}

	pub fn list_apis(&self) -> HashSet<Api> {
		let mut public_list: HashSet<Api> = [
			Api::Web3,
			Api::Net,
			Api::Vast,
			Api::VastPubSub,
			Api::Vast,
			Api::Rpc,
			Api::Private,
		]
			.iter()
			.cloned()
			.collect();

		match *self {
			ApiSet::List(ref apis) => apis.into_iter()
				.filter(|api| *api != &Api::Deprecated)
				.cloned()
				.collect(),
			ApiSet::UnsafeContext => {
				public_list.insert(Api::Traces);
				public_list.insert(Api::VastPubSub);
				public_list.insert(Api::VastTransactionsPool);
				public_list
			}
			ApiSet::IpcContext => {
				public_list.insert(Api::Traces);
				public_list.insert(Api::VastPubSub);
				public_list.insert(Api::VastAccounts);
				public_list.insert(Api::VastTransactionsPool);
				public_list
			}
			ApiSet::All => {
				public_list.insert(Api::Debug);
				public_list.insert(Api::Traces);
				public_list.insert(Api::VastPubSub);
				public_list.insert(Api::VastAccounts);
				public_list.insert(Api::VastSet);
				public_list.insert(Api::Signer);
				public_list.insert(Api::Personal);
				public_list.insert(Api::SecretStore);
				public_list.insert(Api::VastTransactionsPool);
				public_list
			}
			ApiSet::PubSub => [
				Api::Vast,
				Api::Vast,
				Api::VastAccounts,
				Api::VastSet,
				Api::Traces,
				Api::VastTransactionsPool,
			]
				.iter()
				.cloned()
				.collect(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::{Api, ApiSet};

	#[test]
	fn test_api_parsing() {
		assert_eq!(Api::Debug, "debug".parse().unwrap());
		assert_eq!(Api::Web3, "web3".parse().unwrap());
		assert_eq!(Api::Net, "net".parse().unwrap());
		assert_eq!(Api::Vast, "vast".parse().unwrap());
		assert_eq!(Api::VastPubSub, "pubsub".parse().unwrap());
		assert_eq!(Api::Personal, "personal".parse().unwrap());
		assert_eq!(Api::Signer, "signer".parse().unwrap());
		assert_eq!(Api::Vast, "vast".parse().unwrap());
		assert_eq!(Api::VastAccounts, "vast_accounts".parse().unwrap());
		assert_eq!(Api::VastSet, "vast_set".parse().unwrap());
		assert_eq!(Api::Traces, "traces".parse().unwrap());
		assert_eq!(Api::Rpc, "rpc".parse().unwrap());
		assert_eq!(Api::SecretStore, "secretstore".parse().unwrap());
		assert_eq!(Api::Private, "private".parse().unwrap());
		assert_eq!(Api::VastTransactionsPool, "vast_transactions_pool".parse().unwrap());
		assert!("rp".parse::<Api>().is_err());
	}

	#[test]
	fn test_api_set_default() {
		assert_eq!(ApiSet::UnsafeContext, ApiSet::default());
	}

	#[test]
	fn test_api_set_parsing() {
		assert_eq!(
			ApiSet::List(vec![Api::Web3, Api::Vast].into_iter().collect()),
			"web3,vast".parse().unwrap()
		);
	}

	#[test]
	fn test_api_set_unsafe_context() {
		let expected = vec![
			// make sure this list contains only SAFE mvastods
			Api::Web3,
			Api::Net,
			Api::Vast,
			Api::VastPubSub,
			Api::Vast,
			Api::VastPubSub,
			Api::Traces,
			Api::Rpc,
			Api::Private,
			Api::VastTransactionsPool,
		].into_iter()
		.collect();
		assert_eq!(ApiSet::UnsafeContext.list_apis(), expected);
	}

	#[test]
	fn test_api_set_ipc_context() {
		let expected = vec![
			// safe
			Api::Web3,
			Api::Net,
			Api::Vast,
			Api::VastPubSub,
			Api::Vast,
			Api::VastPubSub,
			Api::Traces,
			Api::Rpc,
			Api::Private,
			Api::VastTransactionsPool,
			// semi-safe
			Api::VastAccounts,
		].into_iter()
		.collect();
		assert_eq!(ApiSet::IpcContext.list_apis(), expected);
	}

	#[test]
	fn test_all_apis() {
		assert_eq!(
			"all".parse::<ApiSet>().unwrap(),
			ApiSet::List(
				vec![
					Api::Web3,
					Api::Net,
					Api::Vast,
					Api::VastPubSub,
					Api::Vast,
					Api::VastPubSub,
					Api::Traces,
					Api::Rpc,
					Api::SecretStore,
					Api::VastAccounts,
					Api::VastSet,
					Api::Signer,
					Api::Personal,
					Api::Private,
					Api::Debug,
					Api::VastTransactionsPool,
				].into_iter()
				.collect()
			)
		);
	}

	#[test]
	fn test_all_without_personal_apis() {
		assert_eq!(
			"personal,all,-personal".parse::<ApiSet>().unwrap(),
			ApiSet::List(
				vec![
					Api::Web3,
					Api::Net,
					Api::Vast,
					Api::VastPubSub,
					Api::Vast,
					Api::VastPubSub,
					Api::Traces,
					Api::Rpc,
					Api::SecretStore,
					Api::VastAccounts,
					Api::VastSet,
					Api::Signer,
					Api::Private,
					Api::Debug,
					Api::VastTransactionsPool,
				].into_iter()
				.collect()
			)
		);
	}

	#[test]
	fn test_safe_parsing() {
		assert_eq!(
			"safe".parse::<ApiSet>().unwrap(),
			ApiSet::List(
				vec![
					Api::Web3,
					Api::Net,
					Api::Vast,
					Api::VastPubSub,
					Api::Vast,
					Api::VastPubSub,
					Api::Traces,
					Api::Rpc,
					Api::Private,
					Api::VastTransactionsPool,
				].into_iter()
				.collect()
			)
		);
	}
}
