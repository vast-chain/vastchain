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

//! Web3 rpc interface.
use vast_types::H256;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

use v1::types::Bytes;

/// Web3 rpc interface.
#[rpc(server)]
pub trait Web3 {
	/// Returns current client version.
	#[rpc(name = "web3_clientVersion")]
	fn client_version(&self) -> Result<String>;

	/// Returns sha3 of the given data
	#[rpc(name = "web3_sha3")]
	fn sha3(&self, _: Bytes) -> Result<H256>;
}