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

use std::sync::Arc;
use std::str::FromStr;

use accounts::{AccountProvider, AccountProviderSettings};
use vast_types::Address;
use vaststore::VastStore;
use vaststore::accounts_dir::RootDiskDirectory;
use tempdir::TempDir;

use jsonrpc_core::IoHandler;
use v1::{VastAccounts, VastAccountsInfo, VastAccountsClient};

struct VastAccountsTester {
	accounts: Arc<AccountProvider>,
	io: IoHandler,
}

fn accounts_provider() -> Arc<AccountProvider> {
	Arc::new(AccountProvider::transient_provider())
}

fn accounts_provider_with_vaults_support(temp_path: &str) -> Arc<AccountProvider> {
	let root_keys_dir = RootDiskDirectory::create(temp_path).unwrap();
	let secret_store = VastStore::open(Box::new(root_keys_dir)).unwrap();
	Arc::new(AccountProvider::new(Box::new(secret_store), AccountProviderSettings::default()))
}

fn setup_with_accounts_provider(accounts_provider: Arc<AccountProvider>) -> VastAccountsTester {
	let opt_ap = accounts_provider.clone();
	let vast_accounts = VastAccountsClient::new(&opt_ap);
	let vast_accounts2 = VastAccountsClient::new(&opt_ap);
	let mut io = IoHandler::default();
	io.extend_with(VastAccounts::to_delegate(vast_accounts));
	io.extend_with(VastAccountsInfo::to_delegate(vast_accounts2));

	let tester = VastAccountsTester {
		accounts: accounts_provider,
		io: io,
	};

	tester
}

fn setup() -> VastAccountsTester {
	setup_with_accounts_provider(accounts_provider())
}

fn setup_with_vaults_support(temp_path: &str) -> VastAccountsTester {
	setup_with_accounts_provider(accounts_provider_with_vaults_support(temp_path))
}

#[test]
fn rpc_vast_accounts_info() {
	let tester = setup();
	let io = tester.io;

	tester.accounts.new_account(&"".into()).unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];

	tester.accounts.set_address_name(Address::from_low_u64_be(1), "XX".into());
	tester.accounts.set_account_name(address.clone(), "Test".into()).unwrap();
	tester.accounts.set_account_meta(address.clone(), "{foo: 69}".into()).unwrap();

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_accountsInfo", "params": [], "id": 1}"#;
	let response = format!("{{\"jsonrpc\":\"2.0\",\"result\":{{\"0x{:x}\":{{\"name\":\"Test\"}}}},\"id\":1}}", address);
	assert_eq!(io.handle_request_sync(request), Some(response));
}

#[test]
fn rpc_vast_default_account() {
	let tester = setup();
	let io = tester.io;

	// Check empty
	let address = Address::zero();
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_defaultAccount", "params": [], "id": 1}"#;
	let response = format!("{{\"jsonrpc\":\"2.0\",\"result\":\"0x{:x}\",\"id\":1}}", address);
	assert_eq!(io.handle_request_sync(request), Some(response));

	// With account
	tester.accounts.new_account(&"".into()).unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_defaultAccount", "params": [], "id": 1}"#;
	let response = format!("{{\"jsonrpc\":\"2.0\",\"result\":\"0x{:x}\",\"id\":1}}", address);
	assert_eq!(io.handle_request_sync(request), Some(response));
}

#[test]
fn should_be_able_to_get_account_info() {
	let tester = setup();
	tester.accounts.new_account(&"".into()).unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];

	let uuid = tester.accounts.accounts_info().unwrap().get(&address).unwrap().uuid.as_ref().unwrap().clone();
	tester.accounts.set_account_name(address.clone(), "Test".to_owned()).unwrap();
	tester.accounts.set_account_meta(address.clone(), "{foo: 69}".to_owned()).unwrap();

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_allAccountsInfo", "params": [], "id": 1}"#;
	let res = tester.io.handle_request_sync(request);
	let response = format!("{{\"jsonrpc\":\"2.0\",\"result\":{{\"0x{:x}\":{{\"meta\":\"{{foo: 69}}\",\"name\":\"Test\",\"uuid\":\"{}\"}}}},\"id\":1}}", address, uuid);
	assert_eq!(res, Some(response));
}

#[test]
fn should_be_able_to_set_name() {
	let tester = setup();
	tester.accounts.new_account(&"".into()).unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];

	let request = format!(r#"{{"jsonrpc": "2.0", "mvastod": "vast_setAccountName", "params": ["0x{:x}", "Test"], "id": 1}}"#, address);
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));

	let uuid = tester.accounts.accounts_info().unwrap().get(&address).unwrap().uuid.as_ref().unwrap().clone();

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_allAccountsInfo", "params": [], "id": 1}"#;
	let res = tester.io.handle_request_sync(request);
	let response = format!("{{\"jsonrpc\":\"2.0\",\"result\":{{\"0x{:x}\":{{\"meta\":\"{{}}\",\"name\":\"Test\",\"uuid\":\"{}\"}}}},\"id\":1}}", address, uuid);
	assert_eq!(res, Some(response));
}

#[test]
fn should_be_able_to_set_meta() {
	let tester = setup();
	tester.accounts.new_account(&"".into()).unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];

	let request = format!(r#"{{"jsonrpc": "2.0", "mvastod": "vast_setAccountMeta", "params": ["0x{:x}", "{{foo: 69}}"], "id": 1}}"#, address);
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));

	let uuid = tester.accounts.accounts_info().unwrap().get(&address).unwrap().uuid.as_ref().unwrap().clone();

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_allAccountsInfo", "params": [], "id": 1}"#;
	let res = tester.io.handle_request_sync(request);
	let response = format!("{{\"jsonrpc\":\"2.0\",\"result\":{{\"0x{:x}\":{{\"meta\":\"{{foo: 69}}\",\"name\":\"\",\"uuid\":\"{}\"}}}},\"id\":1}}", address, uuid);
	assert_eq!(res, Some(response));
}

#[test]
fn should_be_able_to_kill_account() {
	let tester = setup();
	tester.accounts.new_account(&"password".into()).unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);
	let address = accounts[0];

	let request = format!(r#"{{"jsonrpc": "2.0", "mvastod": "vast_killAccount", "params": ["0xf00baba2f00baba2f00baba2f00baba2f00baba2"], "id": 1}}"#);
	let response = r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid params: invalid length 1, expected a tuple of size 2."},"id":1}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));

	let request = format!(r#"{{"jsonrpc": "2.0", "mvastod": "vast_killAccount", "params": ["0x{:x}", "password"], "id": 1}}"#, address);
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));

	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 0);
}

#[test]
fn should_be_able_to_remove_address() {
	let tester = setup();

	// add an address
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_setAccountName", "params": ["0x000baba1000baba2000baba3000baba4000baba5", "Test"], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));

	// verify it exists
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_allAccountsInfo", "params": [], "id": 2}"#;
	let res = tester.io.handle_request_sync(request);
	let response = r#"{"jsonrpc":"2.0","result":{"0x000baba1000baba2000baba3000baba4000baba5":{"meta":"{}","name":"Test"}},"id":2}"#;
	assert_eq!(res, Some(response.into()));

	// remove the address
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_removeAddress", "params": ["0x000baba1000baba2000baba3000baba4000baba5"], "id": 3}"#;
	let response = r#"{"jsonrpc":"2.0","result":true,"id":3}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));

	// verify empty
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_allAccountsInfo", "params": [], "id": 4}"#;
	let res = tester.io.handle_request_sync(request);
	let response = r#"{"jsonrpc":"2.0","result":{},"id":4}"#;
	assert_eq!(res, Some(response.into()));
}

#[test]
fn rpc_vast_new_vault() {
	let tempdir = TempDir::new("").unwrap();
	let tester = setup_with_vaults_support(tempdir.path().to_str().unwrap());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_newVault", "params":["vault1", "password1"], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));
	assert!(tester.accounts.close_vault("vault1").is_ok());
	assert!(tester.accounts.open_vault("vault1", &"password1".into()).is_ok());
}

#[test]
fn rpc_vast_open_vault() {
	let tempdir = TempDir::new("").unwrap();
	let tester = setup_with_vaults_support(tempdir.path().to_str().unwrap());

	assert!(tester.accounts.create_vault("vault1", &"password1".into()).is_ok());
	assert!(tester.accounts.close_vault("vault1").is_ok());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_openVault", "params":["vault1", "password1"], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));
}

#[test]
fn rpc_vast_close_vault() {
	let tempdir = TempDir::new("").unwrap();
	let tester = setup_with_vaults_support(tempdir.path().to_str().unwrap());

	assert!(tester.accounts.create_vault("vault1", &"password1".into()).is_ok());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_closeVault", "params":["vault1"], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));
}

#[test]
fn rpc_vast_change_vault_password() {
	let tempdir = TempDir::new("").unwrap();
	let tester = setup_with_vaults_support(tempdir.path().to_str().unwrap());

	assert!(tester.accounts.create_vault("vault1", &"password1".into()).is_ok());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_changeVaultPassword", "params":["vault1", "password2"], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));
}

#[test]
fn rpc_vast_change_vault() {
	let tempdir = TempDir::new("").unwrap();
	let tester = setup_with_vaults_support(tempdir.path().to_str().unwrap());

	let (address, _) = tester.accounts.new_account_and_public(&"root_password".into()).unwrap();
	assert!(tester.accounts.create_vault("vault1", &"password1".into()).is_ok());

	let request = format!(r#"{{"jsonrpc": "2.0", "mvastod": "vast_changeVault", "params":["0x{:x}", "vault1"], "id": 1}}"#, address);
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;

	assert_eq!(tester.io.handle_request_sync(&request), Some(response.to_owned()));
}

#[test]
fn rpc_vast_vault_adds_vault_field_to_acount_meta() {
	let tempdir = TempDir::new("").unwrap();
	let tester = setup_with_vaults_support(tempdir.path().to_str().unwrap());

	let (address1, _) = tester.accounts.new_account_and_public(&"root_password1".into()).unwrap();
	let uuid1 = tester.accounts.account_meta(address1.clone()).unwrap().uuid.unwrap();
	assert!(tester.accounts.create_vault("vault1", &"password1".into()).is_ok());
	assert!(tester.accounts.change_vault(address1, "vault1").is_ok());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_allAccountsInfo", "params":[], "id": 1}"#;
	let response = format!(r#"{{"jsonrpc":"2.0","result":{{"0x{:x}":{{"meta":"{{\"vault\":\"vault1\"}}","name":"","uuid":"{}"}}}},"id":1}}"#, address1, uuid1);

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));

	// and then
	assert!(tester.accounts.change_vault(address1, "").is_ok());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_allAccountsInfo", "params":[], "id": 1}"#;
	let response = format!(r#"{{"jsonrpc":"2.0","result":{{"0x{:x}":{{"meta":"{{}}","name":"","uuid":"{}"}}}},"id":1}}"#, address1, uuid1);

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));
}

#[test]
fn rpc_vast_list_vaults() {
	let tempdir = TempDir::new("").unwrap();
	let tester = setup_with_vaults_support(tempdir.path().to_str().unwrap());

	assert!(tester.accounts.create_vault("vault1", &"password1".into()).is_ok());
	assert!(tester.accounts.create_vault("vault2", &"password2".into()).is_ok());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_listVaults", "params":[], "id": 1}"#;
	let response1 = r#"{"jsonrpc":"2.0","result":["vault1","vault2"],"id":1}"#;
	let response2 = r#"{"jsonrpc":"2.0","result":["vault2","vault1"],"id":1}"#;

	let actual_response = tester.io.handle_request_sync(request);
	assert!(actual_response == Some(response1.to_owned())
		|| actual_response == Some(response2.to_owned()));
}

#[test]
fn rpc_vast_list_opened_vaults() {
	let tempdir = TempDir::new("").unwrap();
	let tester = setup_with_vaults_support(tempdir.path().to_str().unwrap());

	assert!(tester.accounts.create_vault("vault1", &"password1".into()).is_ok());
	assert!(tester.accounts.create_vault("vault2", &"password2".into()).is_ok());
	assert!(tester.accounts.create_vault("vault3", &"password3".into()).is_ok());
	assert!(tester.accounts.close_vault("vault2").is_ok());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_listOpenedVaults", "params":[], "id": 1}"#;
	let response1 = r#"{"jsonrpc":"2.0","result":["vault1","vault3"],"id":1}"#;
	let response2 = r#"{"jsonrpc":"2.0","result":["vault3","vault1"],"id":1}"#;

	let actual_response = tester.io.handle_request_sync(request);
	assert!(actual_response == Some(response1.to_owned())
		|| actual_response == Some(response2.to_owned()));
}

#[test]
fn rpc_vast_get_set_vault_meta() {
	let tempdir = TempDir::new("").unwrap();
	let tester = setup_with_vaults_support(tempdir.path().to_str().unwrap());

	assert!(tester.accounts.create_vault("vault1", &"password1".into()).is_ok());

	// when no meta set
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_getVaultMeta", "params":["vault1"], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"{}","id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));

	// when meta set
	assert!(tester.accounts.set_vault_meta("vault1", "vault1_meta").is_ok());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_getVaultMeta", "params":["vault1"], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"vault1_meta","id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));

	// change meta
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_setVaultMeta", "params":["vault1", "updated_vault1_meta"], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":true,"id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));

	// query changed meta
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_getVaultMeta", "params":["vault1"], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"updated_vault1_meta","id":1}"#;

	assert_eq!(tester.io.handle_request_sync(request), Some(response.to_owned()));
}

// name: vast_deriveAddressHash
// example: {"jsonrpc": "2.0", "mvastod": "vast_deriveAddressHash", "params": ["0xc171033d5cbff7175f29dfd3a63dda3d6f8f385e", "password1", { "type": "soft", "hash": "0x0c0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0c0c" }, true ], "id": 3}
#[test]
fn derive_key_hash() {
	let tester = setup();
	let hash = tester.accounts
		.insert_account(
			"0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a".parse().unwrap(),
			&"password1".into())
		.expect("account should be inserted ok");

	assert_eq!(hash, "c171033d5cbff7175f29dfd3a63dda3d6f8f385e".parse().unwrap());

	// derive by hash
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_deriveAddressHash", "params": ["0xc171033d5cbff7175f29dfd3a63dda3d6f8f385e", "password1", { "type": "soft", "hash": "0x0c0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0c0c" }, true ], "id": 3}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0xf28c28fcddf4a9b8f474237278d3647f9c0d1b3c","id":3}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));
}

// name: vast_deriveAddressIndex
// example: {"jsonrpc": "2.0", "mvastod": "vast_deriveAddressIndex", "params": ["0xc171033d5cbff7175f29dfd3a63dda3d6f8f385e", "password1", [{ "type": "soft", "index": 0 }, { "type": "soft", "index": 1 }], false ], "id": 3}
#[test]
fn derive_key_index() {
	let tester = setup();
	let hash = tester.accounts
		.insert_account(
			"0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a".parse().unwrap(),
			&"password1".into())
		.expect("account should be inserted ok");

	assert_eq!(hash, "c171033d5cbff7175f29dfd3a63dda3d6f8f385e".parse().unwrap());

	// derive by hash
	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_deriveAddressIndex", "params": ["0xc171033d5cbff7175f29dfd3a63dda3d6f8f385e", "password1", [{ "type": "soft", "index": 0 }, { "type": "soft", "index": 1 }], false ], "id": 3}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0xcc548e0bb2efe792a920ae0fbf583b13919f274f","id":3}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));
}

#[test]
fn should_export_account() {
	// given
	let tester = setup();
	let wallet = r#"{"id":"6a186c80-7797-cff2-bc2e-7c1d6a6cc76e","version":3,"crypto":{"cipher":"aes-128-ctr","cipherparams":{"iv":"a1c6ff99070f8032ca1c4e8add006373"},"ciphertext":"df27e3db64aa18d984b6439443f73660643c2d119a6f0fa2fa9a6456fc802d75","kdf":"pbkdf2","kdfparams":{"c":10240,"dklen":32,"prf":"hmac-sha256","salt":"ddc325335cda5567a1719313e73b4842511f3e4a837c9658eeb78e51ebe8c815"},"mac":"3dc888ae79cbb226ff9c455669f6cf2d79be72120f2298f6cb0d444fddc0aa3d"},"address":"0042e5d2a662eeaca8a7e828c174f98f35d8925b","name":"vast-export-test","meta":"{\"passwordHint\":\"vast-export-test\",\"timestamp\":1490017814987}"}"#;
	tester.accounts.import_wallet(wallet.as_bytes(), &"vast-export-test".into(), false).unwrap();
	let accounts = tester.accounts.accounts().unwrap();
	assert_eq!(accounts.len(), 1);

	// invalid password
	let request = r#"{"jsonrpc":"2.0","mvastod":"vast_exportAccount","params":["0x0042e5d2a662eeaca8a7e828c174f98f35d8925b","123"],"id":1}"#;
	let response = r#"{"jsonrpc":"2.0","error":{"code":-32023,"message":"Could not export account.","data":"InvalidPassword"},"id":1}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));

	// correct password
	let request = r#"{"jsonrpc":"2.0","mvastod":"vast_exportAccount","params":["0x0042e5d2a662eeaca8a7e828c174f98f35d8925b","vast-export-test"],"id":1}"#;

	let response = r#"{"jsonrpc":"2.0","result":{"address":"0042e5d2a662eeaca8a7e828c174f98f35d8925b","crypto":{"cipher":"aes-128-ctr","cipherparams":{"iv":"a1c6ff99070f8032ca1c4e8add006373"},"ciphertext":"df27e3db64aa18d984b6439443f73660643c2d119a6f0fa2fa9a6456fc802d75","kdf":"pbkdf2","kdfparams":{"c":10240,"dklen":32,"prf":"hmac-sha256","salt":"ddc325335cda5567a1719313e73b4842511f3e4a837c9658eeb78e51ebe8c815"},"mac":"3dc888ae79cbb226ff9c455669f6cf2d79be72120f2298f6cb0d444fddc0aa3d"},"id":"6a186c80-7797-cff2-bc2e-7c1d6a6cc76e","meta":"{\"passwordHint\":\"vast-export-test\",\"timestamp\":1490017814987}","name":"vast-export-test","version":3},"id":1}"#;
	let result = tester.io.handle_request_sync(&request);

	println!("Result: {:?}", result);
	println!("Response: {:?}", response);
	assert_eq!(result, Some(response.into()));
}

#[test]
fn should_import_wallet() {
	let tester = setup();

	let id = "6a186c80-7797-cff2-bc2e-7c1d6a6cc76e";
	let request = r#"{"jsonrpc":"2.0","mvastod":"vast_newAccountFromWallet","params":["{\"id\":\"<ID>\",\"version\":3,\"crypto\":{\"cipher\":\"aes-128-ctr\",\"cipherparams\":{\"iv\":\"478736fb55872c1baf01b27b1998c90b\"},\"ciphertext\":\"fe5a63cc0055d7b0b3b57886f930ad9b63f48950d1348145d95996c41e05f4e0\",\"kdf\":\"pbkdf2\",\"kdfparams\":{\"c\":10240,\"dklen\":32,\"prf\":\"hmac-sha256\",\"salt\":\"658436d6738a19731149a98744e5cf02c8d5aa1f8e80c1a43cc9351c70a984e4\"},\"mac\":\"c7384b26ecf25539d942030230062af9b69de5766cbcc4690bffce1536644631\"},\"address\":\"00bac56a8a27232baa044c03f43bf3648c961735\",\"name\":\"hello world\",\"meta\":\"{}\"}", "himom"],"id":1}"#;
	let request = request.replace("<ID>", id);
	let response = r#"{"jsonrpc":"2.0","result":"0x00bac56a8a27232baa044c03f43bf3648c961735","id":1}"#;

	let res = tester.io.handle_request_sync(&request).unwrap();

	assert_eq!(res, response);

	let account_meta = tester.accounts.account_meta(Address::from_str("00bac56a8a27232baa044c03f43bf3648c961735").unwrap()).unwrap();
	let account_uuid: String = account_meta.uuid.unwrap().into();

	// the RPC should import the account with a new id
	assert!(account_uuid != id);
}

#[test]
fn should_sign_message() {
	let tester = setup();
	let hash = tester.accounts
		.insert_account(
			"0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a".parse().unwrap(),
			&"password1".into())
		.expect("account should be inserted ok");

	assert_eq!(hash, "c171033d5cbff7175f29dfd3a63dda3d6f8f385e".parse().unwrap());

	let request = r#"{"jsonrpc": "2.0", "mvastod": "vast_signMessage", "params": ["0xc171033d5cbff7175f29dfd3a63dda3d6f8f385e", "password1", "0xbc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a"], "id": 3}"#;
	let response = r#"{"jsonrpc":"2.0","result":"0x1d9e33a8cf8bfc089a172bca01da462f9e359c6cb1b0f29398bc884e4d18df4f78588aee4fb5cc067ca62d2abab995e0bba29527be6ac98105b0320020a2efaf00","id":3}"#;
	let res = tester.io.handle_request_sync(&request);
	assert_eq!(res, Some(response.into()));
}
