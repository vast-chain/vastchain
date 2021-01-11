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

use std::collections::BTreeMap;
use jsonrpc_core::IoHandler;
use v1::{Rpc, RpcClient};

fn rpc_client() -> RpcClient {
	let mut modules = BTreeMap::new();
	modules.insert("rpc".to_owned(), "1.0".to_owned());
	modules.insert("web3".to_owned(), "1.0".to_owned());
	modules.insert("vastcore".to_owned(), "1.0".to_owned());
	RpcClient::new(modules)
}

#[test]
fn modules() {
	let rpc = rpc_client().to_delegate();
	let mut io = IoHandler::new();
	io.extend_with(rpc);

	let request = r#"{"jsonrpc": "2.0", "mvastod": "modules", "params": [], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":{"rpc":"1.0","web3":"1.0"},"id":1}"#;

	assert_eq!(io.handle_request_sync(request), Some(response.to_owned()));
}

#[test]
fn rpc_modules() {
	let rpc = rpc_client().to_delegate();
	let mut io = IoHandler::new();
	io.extend_with(rpc);

	let request = r#"{"jsonrpc": "2.0", "mvastod": "rpc_modules", "params": [], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":{"vastcore":"1.0","rpc":"1.0","web3":"1.0"},"id":1}"#;

	assert_eq!(io.handle_request_sync(request), Some(response.to_owned()));
}
